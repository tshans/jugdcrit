use unity::prelude::*;
use unity::il2cpp::object::Array;

use crate::unitylib::engine::UEComponent;
use crate::unitylib::engine::UEObject;
use crate::unitylib::engine::Vector3;


#[unity::class("TMPro", "TextMeshProUGUI")]
pub struct TextMeshProUGUI {
    to_do_1: [u8; 0x288],
    pub text_container_local_corners: &'static Array<Vector3>, // 0x288
    to_do_2: [u8; 0x428],
    pub sub_text_objects: &'static Array<&'static TMPSubMeshUI>, // 0x6B8
    pub previous_lossy_scale_y: f32, // 0x6C0
    pub rect_transform_corners: &'static Array<Vector3>, // 0x6C8
    to_do_3: [u8; 0xC0],
}

#[unity::class("TMPro", "TMP_SubMeshUI")]
pub struct TMPSubMeshUI {
    // TODO
}


// Traits
// Trait for simulating TMP_Text inheritance
pub trait Text {
    fn set_text(&self, source_text: impl AsRef<str>, sync_text_input_box: bool) {
        unsafe {
            tmptext_settext(self, source_text.into(), sync_text_input_box, None)
        }
    }
    fn get_text(&self) -> &'static Il2CppString {
        unsafe {
            tmptext_gettext(self, None)
        }
    }
}

impl Text for TextMeshProUGUI {}
impl UEComponent for TextMeshProUGUI {}
impl UEObject for TextMeshProUGUI {}


// External Functions
// TMP_Text external functions
#[skyline::from_offset(0x02837690)] // 0x7102837690
fn tmptext_settext<T: Text + ?Sized>(this: &T, source_text: &Il2CppString, sync_text_input_box: bool, method_info: OptionalMethod);

#[skyline::from_offset(0x028316D0)] // 0x71028316D0
fn tmptext_gettext<T: Text + ?Sized>(this: &T, method_info: OptionalMethod) -> &'static Il2CppString;