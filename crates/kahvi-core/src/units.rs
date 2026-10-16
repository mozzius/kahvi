//! [`Units`]: a string's UTF-16 code units behind a single pointer.
//!
//! The string table keeps one entry per string, so an entry's size counts
//! once per string in the heap. A `Box<[u16]>` is a pointer and a length
//! (16 bytes); `Units` stores the length in front of the code units instead
//! and is one pointer (8 bytes).

use std::alloc::{Layout, alloc, dealloc, handle_alloc_error};
use std::ptr::NonNull;

/// The header of every allocation: the length in code units, then the
/// units.
#[repr(C)]
struct Header {
    len: u32,
}

/// The empty string's header, shared (never freed).
static EMPTY: Header = Header { len: 0 };

pub(crate) struct Units(NonNull<Header>);

// SAFETY: `Units` owns its allocation like a `Box<[u16]>`, and the units
// are never changed after construction.
unsafe impl Send for Units {}
unsafe impl Sync for Units {}

impl Units {
    fn layout(len: usize) -> Layout {
        Layout::new::<Header>()
            .extend(Layout::array::<u16>(len).expect("string length overflows"))
            .expect("string length overflows")
            .0
            .pad_to_align()
    }

    /// The bytes allocated for `len` code units.
    pub(crate) fn heap_bytes(&self) -> usize {
        match self.len() {
            0 => 0,
            n => Self::layout(n).size(),
        }
    }

    fn units_ptr(&self) -> *const u16 {
        // The units follow the 4-byte header (2-byte aligned).
        unsafe { (self.0.as_ptr() as *const u8).add(size_of::<Header>()) as *const u16 }
    }
}

impl From<&[u16]> for Units {
    fn from(units: &[u16]) -> Units {
        if units.is_empty() {
            return Units::default();
        }
        let len = u32::try_from(units.len()).expect("string longer than 2^32 units");
        let layout = Self::layout(units.len());
        // SAFETY: the layout is non-zero in size; the header and units are
        // written before the value is used.
        unsafe {
            let p = alloc(layout) as *mut Header;
            let Some(p) = NonNull::new(p) else {
                handle_alloc_error(layout)
            };
            p.as_ptr().write(Header { len });
            let out = Units(p);
            std::ptr::copy_nonoverlapping(units.as_ptr(), out.units_ptr() as *mut u16, units.len());
            out
        }
    }
}

impl Default for Units {
    fn default() -> Units {
        Units(NonNull::from(&EMPTY))
    }
}

impl Drop for Units {
    fn drop(&mut self) {
        let len = self.len();
        if len != 0 {
            // SAFETY: a non-empty `Units` was allocated with this layout.
            unsafe { dealloc(self.0.as_ptr() as *mut u8, Self::layout(len)) }
        }
    }
}

impl std::ops::Deref for Units {
    type Target = [u16];
    fn deref(&self) -> &[u16] {
        // SAFETY: the header holds the number of units that follow it.
        unsafe {
            let len = (*self.0.as_ptr()).len as usize;
            std::slice::from_raw_parts(self.units_ptr(), len)
        }
    }
}

impl std::fmt::Debug for Units {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        String::from_utf16_lossy(self).fmt(f)
    }
}

const _: () = assert!(size_of::<Units>() == 8);
