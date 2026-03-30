//! Generic types for manipulating 32 and 64 bit C-style bitfields.
use bitflags::Flags;

// Classes
#[unity::class("App", "BitFieldCommon")]
pub struct BitFieldCommon { }

#[unity::class("App", "BitField32")]
pub struct BitField32<T: Flags + Clone + Copy> {
    pub value: T // 0x0
}

#[unity::class("App", "BitFieldTemplate32`1")]
pub struct BitFieldTemplate32<T: Flags + Clone + Copy> {
    pub sup: BitField32Fields<T>
}


// Methods
// BitFieldTemplate32 methods
impl<T: Flags + Clone + Copy> BitFieldTemplate32Fields<T> {
    pub fn get_value(&self) -> T {
        self.sup.value
    }
}