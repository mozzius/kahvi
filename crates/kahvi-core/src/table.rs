//! [`Table`]: one of the heap's tables (objects, environments, strings),
//! indexed by handle.
//!
//! The table is kept in chunks of [`CHUNK`] slots rather than one array, so
//! that memory is given back wherever a whole chunk of it is free, not only
//! at its end: one live object near the end of a single array kept all of
//! it allocated after the collector had freed most of what it held.

use crate::gc::BitSet;

const SHIFT: usize = 12;
/// Slots per chunk.
pub(crate) const CHUNK: usize = 1 << SHIFT;
const MASK: usize = CHUNK - 1;

/// What a free slot holds.
pub(crate) trait Vacant {
    fn vacant() -> Self;
}

pub(crate) struct Table<T> {
    /// Every chunk but the last holds `CHUNK` slots, or none once all of
    /// them were free and it was released. The last holds the rest (at
    /// least one), and is never released.
    chunks: Vec<Vec<T>>,
    len: usize,
}

impl<T> Default for Table<T> {
    fn default() -> Self {
        Table {
            chunks: Vec::new(),
            len: 0,
        }
    }
}

impl<T: Vacant> Table<T> {
    pub(crate) fn len(&self) -> usize {
        self.len
    }

    /// Adds `value` in a new slot at the end; returns its index.
    pub(crate) fn push(&mut self, value: T) -> u32 {
        if self.len & MASK == 0 {
            self.chunks.push(Vec::new());
        }
        let c = self.chunks.last_mut().expect("a chunk was added");
        if c.len() == c.capacity() {
            // The last chunk grows like a vector, up to a chunk.
            let to = (c.capacity() * 2).clamp(16, CHUNK);
            c.reserve_exact(to - c.len());
        }
        c.push(value);
        self.len += 1;
        (self.len - 1) as u32
    }

    /// Puts `value` in slot `i` (which is below `len`), allocating its
    /// chunk again if it was released.
    pub(crate) fn set(&mut self, i: u32, value: T) {
        let c = &mut self.chunks[i as usize >> SHIFT];
        if c.is_empty() {
            c.reserve_exact(CHUNK);
            c.resize_with(CHUNK, T::vacant);
        }
        c[i as usize & MASK] = value;
    }

    /// Slot `i`, unless its chunk was released.
    pub(crate) fn get_mut(&mut self, i: u32) -> Option<&mut T> {
        self.chunks
            .get_mut(i as usize >> SHIFT)
            .and_then(|c| c.get_mut(i as usize & MASK))
    }

    /// The slots whose chunks are allocated, with their indices.
    pub(crate) fn iter(&self) -> impl Iterator<Item = (u32, &T)> {
        self.chunks.iter().enumerate().flat_map(|(n, c)| {
            c.iter()
                .enumerate()
                .map(move |(i, v)| ((n << SHIFT | i) as u32, v))
        })
    }

    /// The bytes the table has allocated for its slots.
    pub(crate) fn heap_bytes(&self) -> usize {
        self.chunks.capacity() * size_of::<Vec<T>>()
            + self
                .chunks
                .iter()
                .map(|c| c.capacity() * size_of::<T>())
                .sum::<usize>()
    }

    /// After a sweep, given the free slots: drops the free slots at the end
    /// and releases each chunk whose slots are all free. Lists the free
    /// slots in `free` so that the lowest in an allocated chunk is reused
    /// first, then the lowest in a released one. New things then fill the
    /// table from the start, and its end, where the newest things are, is
    /// left free to be dropped at the next collection.
    pub(crate) fn compact(&mut self, is_free: &mut BitSet, free: &mut Vec<u32>) {
        while self.len > 0 && is_free.get(self.len as u32 - 1) {
            let c = self.chunks.last_mut().expect("len > 0");
            if c.is_empty() {
                // A released chunk, now the last: all of its slots are free.
                self.chunks.pop();
                for i in self.len - CHUNK..self.len {
                    is_free.remove(i as u32);
                }
                self.len -= CHUNK;
                continue;
            }
            c.pop();
            if c.is_empty() {
                self.chunks.pop();
            }
            self.len -= 1;
            is_free.remove(self.len as u32);
        }
        let last = self.chunks.len().saturating_sub(1);
        for (n, c) in self.chunks[..last].iter_mut().enumerate() {
            if !c.is_empty() && (n << SHIFT..(n + 1) << SHIFT).all(|i| is_free.get(i as u32)) {
                *c = Vec::new();
            }
        }
        if let Some(c) = self.chunks.last_mut()
            && c.capacity() >= 2 * c.len().max(16)
        {
            c.shrink_to(c.len() + c.len() / 4);
        }
        if self.chunks.capacity() >= 2 * self.chunks.len().max(16) {
            self.chunks.shrink_to_fit();
        }
        free.clear();
        let (allocated, released): (Vec<_>, Vec<_>) = (0..self.chunks.len())
            .rev()
            .partition(|&n| !self.chunks[n].is_empty());
        for n in released.into_iter().chain(allocated) {
            let end = ((n + 1) << SHIFT).min(self.len);
            free.extend(
                ((n << SHIFT) as u32..end as u32)
                    .rev()
                    .filter(|&i| is_free.get(i)),
            );
        }
        if free.capacity() >= 2 * free.len().max(4096) {
            free.shrink_to(free.len() + free.len() / 4);
        }
    }
}

impl<T> std::ops::Index<usize> for Table<T> {
    type Output = T;
    #[inline]
    fn index(&self, i: usize) -> &T {
        debug_assert!(
            self.chunks[i >> SHIFT].len() > i & MASK,
            "slot {i} not allocated"
        );
        // SAFETY: handles index live slots, and a live slot's chunk is
        // allocated (the collector releases only chunks of free slots).
        unsafe {
            self.chunks
                .get_unchecked(i >> SHIFT)
                .get_unchecked(i & MASK)
        }
    }
}

impl<T> std::ops::IndexMut<usize> for Table<T> {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut T {
        debug_assert!(
            self.chunks[i >> SHIFT].len() > i & MASK,
            "slot {i} not allocated"
        );
        // SAFETY: as for `index`.
        unsafe {
            self.chunks
                .get_unchecked_mut(i >> SHIFT)
                .get_unchecked_mut(i & MASK)
        }
    }
}

impl Vacant for crate::object::ObjectData {
    fn vacant() -> Self {
        crate::object::ObjectData::new(None, crate::object::ObjectKind::Ordinary)
    }
}

impl Vacant for crate::env::Environment {
    fn vacant() -> Self {
        crate::env::Environment::dead()
    }
}

impl Vacant for crate::units::Units {
    fn vacant() -> Self {
        Self::default()
    }
}
