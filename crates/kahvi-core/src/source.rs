//! [`SourceText`]: the text of a script or module, which the engine keeps
//! for compiling lazy functions, `Function.prototype.toString` and error
//! positions.
//!
//! A large text with a code cache directory (an app bundle) is kept as a
//! read-only mapping of a copy in that directory rather than in memory the
//! engine allocates: the system loads its pages when they are read (a lazy
//! function's body, the first time it is compiled without cached code) and
//! can drop them again under memory pressure, so the parts that are not
//! read do not count against the process.

use std::sync::Arc;

pub struct SourceText(Repr);

enum Repr {
    Owned(Box<str>),
    #[cfg(all(unix, target_pointer_width = "64"))]
    Mapped(map::Mapping),
}

impl SourceText {
    /// A copy of `text`.
    pub fn new(text: &str) -> Arc<SourceText> {
        Arc::new(SourceText(Repr::Owned(text.into())))
    }

    /// `text`, from the copy at `path` (written there first if it is not
    /// there yet), if it can be mapped.
    pub(crate) fn mapped(path: &std::path::Path, text: &str) -> Option<SourceText> {
        #[cfg(all(unix, target_pointer_width = "64"))]
        {
            map::Mapping::of(path, text).map(|m| SourceText(Repr::Mapped(m)))
        }
        #[cfg(not(all(unix, target_pointer_width = "64")))]
        {
            let _ = (path, text);
            None
        }
    }

    /// Lets the system drop the text's pages from memory (they are read
    /// back from the file when needed), after a pass over all of it.
    pub(crate) fn release_pages(&self) {
        #[cfg(all(unix, target_pointer_width = "64"))]
        if let Repr::Mapped(m) = &self.0 {
            m.release_pages();
        }
    }
}

impl std::ops::Deref for SourceText {
    type Target = str;
    fn deref(&self) -> &str {
        match &self.0 {
            Repr::Owned(s) => s,
            #[cfg(all(unix, target_pointer_width = "64"))]
            Repr::Mapped(m) => m.text(),
        }
    }
}

impl std::fmt::Debug for SourceText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SourceText({} bytes)", self.len())
    }
}

#[cfg(all(unix, target_pointer_width = "64"))]
pub(crate) mod map {
    use std::ffi::c_void;
    use std::io::Write;
    use std::os::fd::AsRawFd;
    use std::path::Path;
    use std::ptr::NonNull;

    // The same values on Linux, Android and Apple systems.
    const PROT_READ: i32 = 1;
    const MAP_PRIVATE: i32 = 2;
    const MADV_DONTNEED: i32 = 4;

    unsafe extern "C" {
        fn mmap(
            addr: *mut c_void,
            len: usize,
            prot: i32,
            flags: i32,
            fd: i32,
            off: i64,
        ) -> *mut c_void;
        fn munmap(addr: *mut c_void, len: usize) -> i32;
        fn madvise(addr: *mut c_void, len: usize, advice: i32) -> i32;
    }

    /// A read-only private mapping of a whole file (for a [`super::SourceText`],
    /// one whose bytes are a `str`).
    pub(crate) struct Mapping {
        ptr: NonNull<u8>,
        len: usize,
    }

    // SAFETY: the mapping is read-only and owned by this value.
    unsafe impl Send for Mapping {}
    unsafe impl Sync for Mapping {}

    impl Mapping {
        /// Maps the file at `path` if it holds exactly `text`, writing it
        /// there first if it does not.
        pub(super) fn of(path: &Path, text: &str) -> Option<Mapping> {
            if text.is_empty() {
                return None;
            }
            if let Some(m) = Mapping::open(path, text) {
                return Some(m);
            }
            // Written whole under another name, then renamed: a reader never
            // sees part of it, and a mapping of an older file keeps its own
            // copy.
            let tmp = path.with_extension(format!("tmp{}", std::process::id()));
            let written = std::fs::File::create(&tmp)
                .and_then(|mut f| f.write_all(text.as_bytes()).and_then(|()| f.sync_all()))
                .and_then(|()| std::fs::rename(&tmp, path));
            if written.is_err() {
                let _ = std::fs::remove_file(&tmp);
                return None;
            }
            Mapping::open(path, text)
        }

        fn open(path: &Path, text: &str) -> Option<Mapping> {
            let m = Mapping::file(&std::fs::File::open(path).ok()?)?;
            // The bytes must be `text`'s: only then are they valid UTF-8.
            if m.bytes() != text.as_bytes() {
                return None;
            }
            m.release_pages();
            Some(m)
        }

        /// Maps the whole of `file`, unless it is empty. The engine only
        /// ever appends to a file it maps, or replaces it whole by rename,
        /// so the mapped bytes never change.
        pub(crate) fn file(file: &std::fs::File) -> Option<Mapping> {
            let len = usize::try_from(file.metadata().ok()?.len()).ok()?;
            if len == 0 {
                return None;
            }
            // SAFETY: a read-only private mapping of a file we opened; it
            // outlives the file handle.
            let p = unsafe {
                mmap(
                    std::ptr::null_mut(),
                    len,
                    PROT_READ,
                    MAP_PRIVATE,
                    file.as_raw_fd(),
                    0,
                )
            };
            if p as isize == -1 {
                return None;
            }
            Some(Mapping {
                ptr: NonNull::new(p as *mut u8)?,
                len,
            })
        }

        pub(crate) fn bytes(&self) -> &[u8] {
            // SAFETY: `len` bytes are mapped for reading.
            unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
        }

        pub(super) fn text(&self) -> &str {
            // SAFETY: the bytes were checked equal to a `str` when mapped.
            // The engine only ever replaces the file whole, by rename, which
            // leaves this mapping's file as it was.
            unsafe { std::str::from_utf8_unchecked(self.bytes()) }
        }

        pub(crate) fn release_pages(&self) {
            // SAFETY: advice on our own mapping; its pages are read back
            // from the file when touched again.
            unsafe { madvise(self.ptr.as_ptr() as *mut c_void, self.len, MADV_DONTNEED) };
        }
    }

    impl Drop for Mapping {
        fn drop(&mut self) {
            // SAFETY: unmaps the mapping this value owns.
            unsafe { munmap(self.ptr.as_ptr() as *mut c_void, self.len) };
        }
    }
}
