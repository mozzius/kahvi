//! `t262`: the test262 runner and the project's single source of truth.
//!
//! Owned by the harness. Workers may run it but never change it, its skip
//! list or the baseline. See `docs/ENGINE-CLI.md` for the engine contract.

mod canary;
mod classify;
mod exec;
mod meta;
mod prepare;
mod util;

use std::collections::{BTreeMap, HashMap, HashSet};
use std::io::{IsTerminal, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use classify::Status;
use exec::Engine;
use prepare::Harness;
use util::{Rng, json_str, truncate};

const USAGE: &str = "\
usage: t262 run [FILTER]... [options]
       t262 canaries [options]

FILTER is a path prefix under test262's test/ directory, e.g.
  built-ins/Array/prototype/flatMap   or   language/expressions/addition/

options:
  --engine CMD          engine command (default: target/debug/kahvi)
  --jobs N              parallel engine processes (default: CPU count)
  --timeout SECS        per-test timeout (default: 10)
  --mem-limit-mb N      virtual memory cap per engine process, 0 = off (default: 4096)
  --show N              print up to N failures (default: 10)
  --by-dir              print a pass-rate table per directory
  --json FILE           write one JSON object per test run
  --summary-json FILE   write a JSON summary (used by the orchestrator)
  --baseline FILE       compare against a pass list; exit 1 on regressions
  --write-baseline FILE update a pass list with this run's results (merge bot only)
  --list                list matching test files and exit
  --no-canaries         skip the canary self-check
  --quiet               no progress output";

struct Skip {
    features: HashSet<String>,
    flags: HashSet<String>,
    paths: Vec<String>,
}

impl Skip {
    fn load(path: &Path) -> Result<Skip, String> {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let mut skip = Skip {
            features: HashSet::new(),
            flags: HashSet::new(),
            paths: Vec::new(),
        };
        for (n, line) in text.lines().enumerate() {
            let line = line.split('#').next().unwrap_or("").trim();
            if line.is_empty() {
                continue;
            }
            match line
                .split_once(':')
                .map(|(k, v)| (k.trim(), v.trim().to_string()))
            {
                Some(("feature", v)) => {
                    skip.features.insert(v);
                }
                Some(("flag", v)) => {
                    skip.flags.insert(v);
                }
                Some(("path", v)) => skip.paths.push(v),
                _ => {
                    return Err(format!(
                        "{}:{}: expected feature:/flag:/path:",
                        path.display(),
                        n + 1
                    ));
                }
            }
        }
        Ok(skip)
    }
}

struct Options {
    filters: Vec<String>,
    engine_cmd: Option<String>,
    jobs: usize,
    timeout: u64,
    mem_limit_mb: u64,
    show: usize,
    by_dir: bool,
    json: Option<PathBuf>,
    summary_json: Option<PathBuf>,
    baseline: Option<PathBuf>,
    write_baseline: Option<PathBuf>,
    list: bool,
    canaries: bool,
    quiet: bool,
}

fn parse_options(args: &[String]) -> Result<Options, String> {
    let mut o = Options {
        filters: Vec::new(),
        engine_cmd: None,
        jobs: std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4),
        timeout: 10,
        mem_limit_mb: 4096,
        show: 10,
        by_dir: false,
        json: None,
        summary_json: None,
        baseline: None,
        write_baseline: None,
        list: false,
        canaries: true,
        quiet: false,
    };
    let mut it = args.iter();
    let num = |v: Option<&String>, name: &str| -> Result<u64, String> {
        v.ok_or(format!("{name} needs a value"))?
            .parse()
            .map_err(|_| format!("{name} needs a number"))
    };
    let path = |v: Option<&String>, name: &str| -> Result<PathBuf, String> {
        v.map(PathBuf::from).ok_or(format!("{name} needs a value"))
    };
    while let Some(a) = it.next() {
        match a.as_str() {
            "--engine" => o.engine_cmd = Some(it.next().ok_or("--engine needs a value")?.clone()),
            "--jobs" => o.jobs = num(it.next(), a)?.max(1) as usize,
            "--timeout" => o.timeout = num(it.next(), a)?.max(1),
            "--mem-limit-mb" => o.mem_limit_mb = num(it.next(), a)?,
            "--show" => o.show = num(it.next(), a)? as usize,
            "--by-dir" => o.by_dir = true,
            "--json" => o.json = Some(path(it.next(), a)?),
            "--summary-json" => o.summary_json = Some(path(it.next(), a)?),
            "--baseline" => o.baseline = Some(path(it.next(), a)?),
            "--write-baseline" => o.write_baseline = Some(path(it.next(), a)?),
            "--list" => o.list = true,
            "--no-canaries" => o.canaries = false,
            "--quiet" => o.quiet = true,
            "-h" | "--help" => return Err(USAGE.to_string()),
            _ if a.starts_with("--") => return Err(format!("unknown option {a}\n\n{USAGE}")),
            _ => o.filters.push(normalize_filter(a)),
        }
    }
    Ok(o)
}

fn normalize_filter(f: &str) -> String {
    let mut f = f.trim();
    for prefix in ["vendor/test262/test/", "test262/test/", "test/"] {
        if let Some(rest) = f.strip_prefix(prefix) {
            f = rest;
        }
    }
    f.trim_end_matches('*').to_string()
}

fn find_root() -> Result<PathBuf, String> {
    let mut dir = std::env::current_dir().map_err(|e| e.to_string())?;
    loop {
        if dir.join("vendor/test262/test").is_dir() {
            return Ok(dir);
        }
        if !dir.pop() {
            return Err("run t262 inside the kahvi repository (vendor/test262 not found)".into());
        }
    }
}

fn discover(test_dir: &Path, filters: &[String], skip: &Skip) -> Vec<String> {
    let mut out = Vec::new();
    let mut stack = vec![PathBuf::new()];
    while let Some(rel) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(test_dir.join(&rel)) else {
            continue;
        };
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            let child = rel.join(&name);
            let child_str = child.to_string_lossy().replace('\\', "/");
            if skip.paths.iter().any(|p| child_str.starts_with(p.as_str())) {
                continue;
            }
            let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
            if is_dir {
                let as_dir = format!("{child_str}/");
                if filters.is_empty()
                    || filters
                        .iter()
                        .any(|f| as_dir.starts_with(f.as_str()) || f.starts_with(&as_dir))
                {
                    stack.push(child);
                }
            } else if name.ends_with(".js")
                && !name.contains("_FIXTURE")
                && (filters.is_empty() || filters.iter().any(|f| child_str.starts_with(f.as_str())))
            {
                out.push(child_str);
            }
        }
    }
    out.sort();
    out
}

struct TestResult {
    id: String,
    status: Status,
    reason: String,
    not_implemented: Option<String>,
    millis: u64,
}

struct Ctx {
    test_dir: PathBuf,
    harness: Harness,
    engine: Engine,
    tmp_root: PathBuf,
    timeout: u64,
}

/// Runs every mode of one test file. `None` means the file was skipped.
fn run_file(
    ctx: &Ctx,
    test_dir: &Path,
    rel: &str,
    src: &[u8],
    skip: &Skip,
    rng: &mut Rng,
) -> Option<Vec<TestResult>> {
    let path = test_dir.join(rel);
    let meta = match meta::parse(src) {
        Ok(m) => m,
        Err(e) => {
            return Some(vec![TestResult {
                id: format!("{rel}:sloppy"),
                status: Status::Fail,
                reason: format!("runner could not read frontmatter: {e}"),
                not_implemented: None,
                millis: 0,
            }]);
        }
    };
    if meta.features.iter().any(|f| skip.features.contains(f))
        || meta.flags.iter().any(|f| skip.flags.contains(f))
    {
        return None;
    }
    let mut results = Vec::new();
    for mode in prepare::modes_for(&meta) {
        let id = format!("{rel}:{}", mode.as_str());
        let prepared =
            match prepare::prepare(&path, src, &meta, mode, &ctx.harness, &ctx.tmp_root, rng) {
                Ok(p) => p,
                Err(e) => {
                    results.push(TestResult {
                        id,
                        status: Status::Fail,
                        reason: format!("runner error: {e}"),
                        not_implemented: None,
                        millis: 0,
                    });
                    continue;
                }
            };
        let out = ctx.engine.run(&prepared.args, &prepared.dir);
        let v = classify::classify(&meta, prepared.nonce.as_deref(), &out, ctx.timeout);
        results.push(TestResult {
            id,
            status: v.status,
            reason: v.reason,
            not_implemented: v.not_implemented,
            millis: out.millis,
        });
    }
    Some(results)
}

struct CanaryReport {
    false_passes: Vec<String>,
    should_pass_ok: usize,
    should_pass_total: usize,
}

fn run_canaries(ctx: &Ctx, rng: &mut Rng, verbose: bool) -> Result<CanaryReport, String> {
    let src_dir = ctx.tmp_root.join(format!("canary-src-{}", rng.hex()));
    std::fs::create_dir_all(&src_dir).map_err(|e| e.to_string())?;
    let no_skip = Skip {
        features: HashSet::new(),
        flags: HashSet::new(),
        paths: Vec::new(),
    };
    let mut report = CanaryReport {
        false_passes: Vec::new(),
        should_pass_ok: 0,
        should_pass_total: 0,
    };
    for c in canary::generate(rng) {
        let name = format!("{}.js", rng.hex());
        std::fs::write(src_dir.join(&name), &c.source).map_err(|e| e.to_string())?;
        let results =
            run_file(ctx, &src_dir, &name, c.source.as_bytes(), &no_skip, rng).unwrap_or_default();
        let all_pass = !results.is_empty() && results.iter().all(|r| r.status == Status::Pass);
        let any_pass = results.iter().any(|r| r.status == Status::Pass);
        if verbose {
            for r in &results {
                let mode = r.id.rsplit(':').next().unwrap_or("");
                println!(
                    "  {:<28} {:<7} expected {:<4} got {:<7} {}",
                    c.name,
                    mode,
                    if c.should_pass { "pass" } else { "fail" },
                    r.status.as_str(),
                    truncate(&r.reason, 100)
                );
            }
        }
        if c.should_pass {
            report.should_pass_total += 1;
            if all_pass {
                report.should_pass_ok += 1;
            }
        } else if any_pass {
            report.false_passes.push(c.name.to_string());
        }
    }
    let _ = std::fs::remove_dir_all(&src_dir);
    Ok(report)
}

fn read_baseline(path: &Path) -> Result<HashSet<String>, String> {
    match std::fs::read_to_string(path) {
        Ok(text) => Ok(text
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
            .map(String::from)
            .collect()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(HashSet::new()),
        Err(e) => Err(format!("{}: {e}", path.display())),
    }
}

fn write_baseline(path: &Path, results: &[TestResult]) -> Result<usize, String> {
    let mut set = read_baseline(path)?;
    for r in results {
        set.remove(&r.id);
    }
    set.extend(
        results
            .iter()
            .filter(|r| r.status == Status::Pass)
            .map(|r| r.id.clone()),
    );
    let mut ids: Vec<_> = set.into_iter().collect();
    ids.sort();
    let mut text = String::from(
        "# test262 runs that kahvi passes. Written by the merge bot via `t262 run --write-baseline`.\n\
         # Do not edit by hand: merges may not remove lines from this file.\n",
    );
    for id in &ids {
        text.push_str(id);
        text.push('\n');
    }
    std::fs::write(path, text).map_err(|e| format!("{}: {e}", path.display()))?;
    Ok(ids.len())
}

fn main() -> ExitCode {
    match real_main() {
        Ok(code) => code,
        Err(msg) => {
            eprintln!("t262: {msg}");
            ExitCode::from(2)
        }
    }
}

fn real_main() -> Result<ExitCode, String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (cmd, rest) = args.split_first().ok_or(USAGE)?;
    if cmd != "run" && cmd != "canaries" {
        return Err(USAGE.into());
    }
    let opts = parse_options(rest)?;
    let root = find_root()?;
    let test_dir = root.join("vendor/test262/test");
    let skip = Skip::load(&root.join("conformance/skip.txt"))?;

    let engine_cmd = opts
        .engine_cmd
        .clone()
        .unwrap_or_else(|| "target/debug/kahvi".into());
    let mut parts = engine_cmd.split_whitespace().map(String::from);
    let program_name = parts.next().ok_or("--engine is empty")?;
    let program_path = if program_name.contains('/') && !Path::new(&program_name).is_absolute() {
        root.join(&program_name).to_string_lossy().into_owned()
    } else {
        program_name.clone()
    };
    let program = exec::resolve_program(&program_path).ok_or(format!(
        "engine not found: {program_name} (did you `cargo build -p kahvi-cli`?)"
    ))?;
    let engine_args = parts
        .map(|a| {
            let p = root.join(&a);
            if !a.starts_with('-') && p.exists() {
                p.to_string_lossy().into_owned()
            } else {
                a
            }
        })
        .collect();

    let mut rng = Rng::from_entropy();
    let tmp_root =
        std::env::temp_dir().join(format!("kahvi-t262-{}-{}", std::process::id(), rng.hex()));
    std::fs::create_dir_all(&tmp_root).map_err(|e| format!("temp dir: {e}"))?;
    let ctx = Arc::new(Ctx {
        harness: Harness::new(root.join("vendor/test262/harness")),
        test_dir: test_dir.clone(),
        engine: Engine {
            program,
            args: engine_args,
            timeout: Duration::from_secs(opts.timeout),
            mem_limit_mb: opts.mem_limit_mb,
        },
        tmp_root: tmp_root.clone(),
        timeout: opts.timeout,
    });
    let result = run(&opts, &ctx, &skip, &test_dir, cmd == "canaries", &mut rng);
    let _ = std::fs::remove_dir_all(&tmp_root);
    result
}

fn run(
    opts: &Options,
    ctx: &Arc<Ctx>,
    skip: &Skip,
    test_dir: &Path,
    canaries_only: bool,
    rng: &mut Rng,
) -> Result<ExitCode, String> {
    if canaries_only {
        println!("canaries ({} engine):", ctx.engine.program.display());
        let rep = run_canaries(ctx, rng, true)?;
        println!(
            "should-fail canaries that passed: {}; should-pass canaries passing: {}/{}",
            rep.false_passes.len(),
            rep.should_pass_ok,
            rep.should_pass_total
        );
        return Ok(ExitCode::from(if rep.false_passes.is_empty() {
            0
        } else {
            3
        }));
    }

    let files = discover(test_dir, &opts.filters, skip);
    if opts.list {
        for f in &files {
            println!("{f}");
        }
        return Ok(ExitCode::SUCCESS);
    }
    if files.is_empty() {
        return Err(format!("no tests match {:?}", opts.filters));
    }

    let start = Instant::now();
    let canaries = if opts.canaries {
        Some(run_canaries(ctx, rng, false)?)
    } else {
        None
    };

    let files = Arc::new(files);
    let next = Arc::new(AtomicUsize::new(0));
    let done = Arc::new(AtomicUsize::new(0));
    let skipped = Arc::new(AtomicUsize::new(0));
    let results = Arc::new(Mutex::new(Vec::new()));
    let skip = Arc::new(Skip {
        features: skip.features.clone(),
        flags: skip.flags.clone(),
        paths: skip.paths.clone(),
    });
    let mut handles = Vec::new();
    for _ in 0..opts.jobs.min(files.len()) {
        let (ctx, files, next, done, skipped, results, skip) = (
            ctx.clone(),
            files.clone(),
            next.clone(),
            done.clone(),
            skipped.clone(),
            results.clone(),
            skip.clone(),
        );
        let mut rng = Rng::new(rng.next());
        handles.push(std::thread::spawn(move || {
            loop {
                let i = next.fetch_add(1, Ordering::Relaxed);
                let Some(rel) = files.get(i) else { break };
                let src = std::fs::read(ctx.test_dir.join(rel)).unwrap_or_default();
                match run_file(&ctx, &ctx.test_dir, rel, &src, &skip, &mut rng) {
                    Some(rs) => results.lock().unwrap().extend(rs),
                    None => {
                        skipped.fetch_add(1, Ordering::Relaxed);
                    }
                }
                done.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }
    let progress = !opts.quiet && std::io::stderr().is_terminal();
    let mut last = Instant::now();
    while done.load(Ordering::Relaxed) < files.len() && handles.iter().any(|h| !h.is_finished()) {
        std::thread::sleep(Duration::from_millis(100));
        if progress && last.elapsed() > Duration::from_secs(2) {
            eprint!(
                "\rt262: {}/{} files",
                done.load(Ordering::Relaxed),
                files.len()
            );
            let _ = std::io::stderr().flush();
            last = Instant::now();
        }
    }
    for h in handles {
        h.join().map_err(|_| "worker thread panicked")?;
    }
    if progress {
        eprint!("\r\x1b[K");
    }
    let elapsed = start.elapsed();
    let mut results = std::mem::take(&mut *results.lock().unwrap());
    results.sort_by(|a, b| a.id.cmp(&b.id));
    report(
        opts,
        &results,
        files.len(),
        skipped.load(Ordering::Relaxed),
        elapsed,
        canaries.as_ref(),
        ctx,
    )
}

fn report(
    opts: &Options,
    results: &[TestResult],
    file_count: usize,
    skipped: usize,
    elapsed: Duration,
    canaries: Option<&CanaryReport>,
    ctx: &Ctx,
) -> Result<ExitCode, String> {
    let count = |s: Status| results.iter().filter(|r| r.status == s).count();
    let (pass, fail, timeout, crash) = (
        count(Status::Pass),
        count(Status::Fail),
        count(Status::Timeout),
        count(Status::Crash),
    );
    let total = results.len();
    let pct = |n: usize| {
        if total == 0 {
            0.0
        } else {
            100.0 * n as f64 / total as f64
        }
    };

    if let Some(path) = &opts.json {
        let mut out = String::new();
        for r in results {
            out.push_str(&format!(
                "{{\"id\":{},\"status\":\"{}\",\"reason\":{},\"not_implemented\":{},\"ms\":{}}}\n",
                json_str(&r.id),
                r.status.as_str(),
                json_str(&r.reason),
                r.not_implemented
                    .as_deref()
                    .map(json_str)
                    .unwrap_or("null".into()),
                r.millis
            ));
        }
        std::fs::write(path, out).map_err(|e| format!("{}: {e}", path.display()))?;
    }

    println!(
        "t262: {} files, {} runs ({} files skipped) in {:.1}s",
        file_count,
        total,
        skipped,
        elapsed.as_secs_f64()
    );
    println!(
        "  pass {pass} ({:.1}%)   fail {fail}   timeout {timeout}   crash {crash}",
        pct(pass)
    );

    let mut missing: HashMap<&str, usize> = HashMap::new();
    for r in results {
        if let Some(what) = &r.not_implemented {
            *missing.entry(what.as_str()).or_default() += 1;
        }
    }
    if !missing.is_empty() {
        let mut v: Vec<_> = missing.into_iter().collect();
        v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
        let n: usize = v.iter().map(|x| x.1).sum();
        println!("  not implemented ({n} runs):");
        for (what, c) in v.iter().take(15) {
            println!("    {c:>7}  {what}");
        }
    }

    if opts.by_dir {
        let mut dirs: BTreeMap<String, (usize, usize)> = BTreeMap::new();
        for r in results {
            let key: Vec<&str> = r.id.split('/').take(2).collect();
            let e = dirs.entry(key.join("/")).or_default();
            e.1 += 1;
            if r.status == Status::Pass {
                e.0 += 1;
            }
        }
        println!("  by directory:");
        for (dir, (p, t)) in dirs {
            println!(
                "    {:>6}/{:<6} {:>5.1}%  {dir}",
                p,
                t,
                100.0 * p as f64 / t as f64
            );
        }
    }

    let failures: Vec<_> = results
        .iter()
        .filter(|r| r.status != Status::Pass)
        .collect();
    if opts.show > 0 && !failures.is_empty() {
        println!(
            "  failures (first {} of {}):",
            opts.show.min(failures.len()),
            failures.len()
        );
        for r in failures.iter().take(opts.show) {
            println!(
                "    {:<7} {}  {}",
                r.status.as_str().to_uppercase(),
                r.id,
                truncate(&r.reason, 160)
            );
        }
    }

    let mut code = 0u8;
    let canaries_ok = canaries.is_none_or(|c| c.false_passes.is_empty());
    if let Some(c) = canaries {
        if c.false_passes.is_empty() {
            println!(
                "canaries: ok (should-pass {}/{} passing)",
                c.should_pass_ok, c.should_pass_total
            );
        } else {
            println!(
                "canaries: RUN INVALID — should-fail canaries passed: {}\n  \
                 The engine or runner reports success for code that throws. Results are untrustworthy.",
                c.false_passes.join(", ")
            );
            code = 3;
        }
    }

    let mut regressions = Vec::new();
    let mut new_passes = Vec::new();
    if let Some(path) = &opts.baseline {
        let base = read_baseline(path)?;
        for r in results {
            match (base.contains(&r.id), r.status == Status::Pass) {
                (true, false) => regressions.push(r),
                (false, true) => new_passes.push(r),
                _ => {}
            }
        }
        let in_scope = results.iter().filter(|r| base.contains(&r.id)).count();
        println!(
            "baseline: {in_scope} expected passes in scope, {} regressions, {} new passes",
            regressions.len(),
            new_passes.len()
        );
        for r in regressions.iter().take(50) {
            println!("  REGRESSED {}  {}", r.id, truncate(&r.reason, 160));
        }
        if !regressions.is_empty() && code == 0 {
            code = 1;
        }
    }

    if let Some(path) = &opts.write_baseline {
        if !canaries_ok {
            return Err("refusing to write a baseline from an invalid run".into());
        }
        let n = write_baseline(path, results)?;
        println!("baseline written: {n} passing runs in {}", path.display());
    }

    if let Some(path) = &opts.summary_json {
        let ids = |v: &[&TestResult]| {
            let items: Vec<String> = v.iter().map(|r| json_str(&r.id)).collect();
            format!("[{}]", items.join(","))
        };
        let json = format!(
            "{{\"engine\":{},\"files\":{file_count},\"skipped_files\":{skipped},\"runs\":{total},\
             \"pass\":{pass},\"fail\":{fail},\"timeout\":{timeout},\"crash\":{crash},\
             \"not_implemented\":{},\"canaries_ok\":{canaries_ok},\"elapsed_ms\":{},\
             \"regressions\":{},\"new_passes\":{}}}\n",
            json_str(&ctx.engine.program.to_string_lossy()),
            results
                .iter()
                .filter(|r| r.not_implemented.is_some())
                .count(),
            elapsed.as_millis(),
            ids(&regressions),
            ids(&new_passes),
        );
        std::fs::write(path, json).map_err(|e| format!("{}: {e}", path.display()))?;
    }
    Ok(ExitCode::from(code))
}
