//! Where does this show up?
#![no_std]
#![feature(const_type_id)]
#![feature(allocator_api)]

// TODO: Remove this ..
#![allow(unused)]

extern crate alloc;
extern crate smallvec;
extern crate hashbrown;
extern crate tracing;
extern crate ahash;
extern crate uuid;
//--
#[cfg(feature="chizel")]
pub extern crate chizel;

pub mod surface;
pub mod scaffold;
pub mod element;
pub mod event;
pub mod component;
pub mod style;
pub mod log;
pub mod arena;
pub mod collections {
    /// Convenience for getting a hashmap with a custom allocator.
    /// 
    /// Note: We're using AHash for potential speed improvements when hashing.
    ///   We should prove it's working at some point ..
    /// 
    /// TODO: GxHash doesn't yet support custom allocators (Nov. 2023).
    ///   Ref: https://github.com/ogxd/gxhash/blob/main/src/hasher.rs#L69)
    pub type HashMap<K, V, A = alloc::alloc::Global> = hashbrown::HashMap<K, V, core::hash::BuildHasherDefault<ahash::AHasher>, A>;
    
    /// Convenience for getting a hashset with a custom allocator.
    pub type HashSet<V> = hashbrown::HashSet<V>;
    
    /// Convenience for getting a hashmap entry with a custom allocator.
    pub type Entry<'a, K, V, A = alloc::alloc::Global> = hashbrown::hash_map::Entry<'a, K, V, core::hash::BuildHasherDefault<ahash::AHasher>, A>;
    
    /// Convenience to get the drain while we're at it ..
    pub use hashbrown::hash_map::Drain;
}
