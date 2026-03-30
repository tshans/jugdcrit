use unity::prelude::*;
use unity::system::{List, Stack};

// Derived Classes
#[repr(C)] // #[unity::class("App", "Pool.List")]
pub struct PoolList<T: 'static> {
    pub klass: &'static mut Il2CppClass,
    monitor: *const u8,
    pub fields: PoolListFields<T>,
}
#[repr(C)]
pub struct PoolListFields<T: 'static> {
    pub list: &'static mut List<T>,
    pub stack: &'static mut Stack<T>,
}

#[repr(C)] // #[unity::class("App", "Pool.Hierarchy")]
pub struct PoolHierarchy<T: 'static> {
    pub klass: &'static mut Il2CppClass,
    monitor: *const u8,
    pub fields: PoolHierarchyFields<T>,
}
#[repr(C)]
pub struct PoolHierarchyFields<T: 'static> {
    pub pool: &'static mut Stack<T>,
    pub used: &'static mut Stack<T>
}