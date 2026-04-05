use unity::system::{List, Stack};

#[unity::class("App", "Pool")]
pub struct Pool {}

// Derived Classes
#[unity::class("", "List`1")]
#[nested_from_type(Pool)]
pub struct PoolList<T: 'static> {
    pub list: &'static mut List<T>,
    pub stack: &'static mut Stack<T>,
}

#[unity::class("", "Hierarchy`1")]
#[nested_from_type(Pool)]
pub struct PoolHierarchy<T: 'static> {
    pub pool: &'static mut Stack<T>,
    pub used: &'static mut Stack<T>
}