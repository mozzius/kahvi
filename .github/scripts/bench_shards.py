#!/usr/bin/env python3
"""The parallel jobs of bench.yml, as a JSON matrix for GitHub Actions.

  bench_shards.py [--all]

Each shard runs some workloads on one group of engines: Kahvi, or the other
engines. Both groups include the baseline engines (QuickJS, and node --jitless
should QuickJS fail to install), so every ratio compares two engines timed on
the same runner, and bench/run.py --merge joins the shards into one result.
The slow workloads get shards of their own so that no job runs for more than a
few minutes; keep every workload of bench/run.py's default set in one shard
per group.
"""

import json
import sys

KAHVI = "kahvi,quickjs,node-jitless"
OTHERS = "node,node-jitless,quickjs,boa,hermes"

SHARDS = [
    ("kahvi", KAHVI, "octane/crypto"),
    ("kahvi", KAHVI, "octane/earley-boyer"),
    ("kahvi", KAHVI, "octane/regexp octane/richards"),
    ("kahvi", KAHVI, "octane/raytrace octane/deltablue"),
    ("kahvi", KAHVI, "octane/navier-stokes realworld/prettier realworld/terser"),
    ("kahvi", KAHVI, "startup/empty octane/splay realworld/acorn realworld/marked"),
    ("others", OTHERS, "octane/crypto"),
    ("others", OTHERS, "octane/earley-boyer"),
    ("others", OTHERS, "octane/regexp"),
    ("others", OTHERS, "octane/richards octane/deltablue octane/raytrace"),
    ("others", OTHERS, "startup/empty octane/splay octane/navier-stokes realworld/acorn realworld/marked "
                       "realworld/prettier realworld/terser"),
]
# The large Octane workloads (--all, on manual runs only), one per shard.
LARGE = ["pdfjs", "mandreel", "gbemu", "code-load", "box2d", "zlib", "typescript"]


def main():
    shards = list(SHARDS)
    if "--all" in sys.argv[1:]:
        shards += [(group, engines, f"octane/{w}") for w in LARGE for group, engines in (("kahvi", KAHVI),
                                                                                          ("others", OTHERS))]
    include = [{"id": i, "group": g, "engines": e, "workloads": w} for i, (g, e, w) in enumerate(shards)]
    print(json.dumps({"include": include}))


if __name__ == "__main__":
    main()
