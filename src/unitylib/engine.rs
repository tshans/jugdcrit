use unity::prelude::*;
use unity::il2cpp::object::Array;
use unity::il2cpp::object::ArrayInstantiator;
use unity::il2cpp::class::Il2CppClassData;


// Classes
#[unity::class("UnityEngine", "Object")]
pub struct Object {
    pub cached_ptr: *const u8,
}

#[unity::class("UnityEngine", "GameObject")]
pub struct GameObject {
    pub sup: ObjectFields,
}

#[unity::class("UnityEngine", "Component")]
pub struct Component {
    pub sup: ObjectFields,
}

#[unity::class("UnityEngine", "Behaviour")]
pub struct Behaviour {
    pub sup: ComponentFields,
}

#[unity::class("UnityEngine", "MonoBehaviour")]
pub struct MonoBehaviour {
    pub sup: BehaviourFields,
}

#[unity::class("UnityEngine", "Animator")]
pub struct Animator {
    pub sup: BehaviourFields,
}

#[unity::class("UnityEngine", "Transform")]
pub struct Transform {
    pub sup: ComponentFields,
}

#[unity::class("UnityEngine", "RectTransform")]
pub struct RectTransform {
    pub sup: TransformFields,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Vector3Int {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}


// Methods
impl RectTransform {
    // Horizontal (x) and vertical (y) dimensions of the RectTransform as a Vector2.
    // Use these methods to change the size of the RectTransform.
    pub fn get_size_delta(&self) -> Vector2 {
        unsafe {
            rectransform_getsizedelta(self, None)
        }
    }
    pub fn set_size_delta(&self, value: Vector2) {
        unsafe {
            rectransform_setsizedelta(self, value, None)
        }
    }
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self {x, y}
    }
    pub fn add(&self, add: Self) -> Self {
        Self::new(
            self.x + add.x,
            self.y + add.y,
        )
    }
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {x, y, z}
    }
    pub fn add(&self, add: Self) -> Self {
        Self::new(
            self.x + add.x,
            self.y + add.y,
            self.z + add.z,
        )
    }
    pub fn scale(&self, mult: f32) -> Self {
        Self::new(
            self.x * mult,
            self.y * mult,
            self.z * mult,
        )
    }
    pub fn to_int(&self) -> Vector3Int {
        Vector3Int::new(
            self.x.round() as i32,
            self.y.round() as i32,
            self.z.round() as i32,
        )
    }
}

impl Vector3Int {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self {x, y, z}
    }
}

impl GameObject {
    pub fn get_active(&self) -> bool {
        unsafe {
            gameobject_getactive(self, None)
        }
    }
    pub fn set_active(&self, value: bool) {
        unsafe {
            gameobject_setactive(self, value, None)
        }
    }
    pub fn get_component_name<T: UEComponent + ?Sized>(&self, ty: impl AsRef<str>) -> Option<&'static T> {
        unsafe {
            gameobject_getcomponent_string::<T>(self, ty.into(), None)
        }
    }
    pub fn get_transform<T: Transformation + ?Sized>(&self) -> &'static T {
        unsafe {
            gameobject_gettransform::<T>(self, None)
        }
    }
}


// Traits
/// Trait to simulate UnityEngine.Component inheritance
pub trait UEComponent {
    fn get_game_object(&self) -> &'static GameObject {
        unsafe {
            component_getgameobject::<Self>(&self, None)
        }
    }
}

impl UEComponent for Component {}
impl UEComponent for Transform {}
impl UEComponent for RectTransform {}


/// Trait to simulate UnityEngine.Object inheritance
pub trait UEObject {
    fn get_name(&self) -> Option<&'static Il2CppString> {
        unsafe {
            object_getname::<Self>(self, None)
        }
    }
    fn set_name(&self, value: impl AsRef<str>) {
        unsafe {
            object_setname::<Self>(self, value.into(), None)
        }
    }
    fn copy(&self) -> &'static Self {
        unsafe {
            object_instantiate::<Self>(self, None)
        }
    }
    fn destroy(&self) {
        unsafe {
            object_destroy::<Self>(self, None)
        }
    }
}

impl UEObject for Object {}
impl UEObject for GameObject {}
impl UEObject for Component {}
impl UEObject for Transform {}
impl UEObject for RectTransform {}


/// Trait to simulate UnityEngine.Transformation inheritance
pub trait Transformation {
    // Absolute position of the transformation measured in pixels. (0.0, 0.0, 0.0)
    // is the bottom left corner of the Switch screen (1920 x 1080).
    fn get_position(&self) -> Vector3 {
        unsafe {
            transform_getposition::<Self>(self, None)
        }
    }
    // Shifts the absolute position of the Transformation.
    fn translate_vec3(&self, translation: Vector3) {
        unsafe {
            transform_translate_vec3::<Self>(self, translation, None);
        }
    }
    fn get_parent(&self) -> Option<&'static Self> {
        unsafe {
            transform_getparent::<Self, Self>(self, None)
        }
    }
    fn set_parent(&self, value: &Self) {
        unsafe {
            transform_setparent::<Self, Self>(self, value, None)
        }
    }
    fn get_child(&self, index: i32) -> Option<&'static Self> {
        unsafe {
            transform_getchild::<Self, Self>(self, index, None)
        }
    }
    fn get_child_count(&self) -> i32 {
        unsafe {
            transform_getchildcount::<Self>(self, None)
        }
    }
    fn get_root(&self) -> &'static Self {
        unsafe {
            transform_getroot::<Self>(self, None)
        }
    }
    fn find(&self, n: impl AsRef<str>) -> Option<&'static Self> {
        unsafe {
            transform_find::<Self, Self>(self, n.into(), None)
        }
    }
    fn find_child(&self, n: impl AsRef<str>) -> Option<&'static Self> {
        unsafe {
            transform_findchild::<Self, Self>(self, n.into(), None)
        }
    }
}

impl Transformation for Transform { }
impl Transformation for RectTransform { }


impl Il2CppClassData for Vector2 {
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS: &'static str = "Vector2";

    fn class() -> &'static Il2CppClass {
        static CLASS_TYPE: std::sync::LazyLock<&'static mut Il2CppClass> = std::sync::LazyLock::new(|| {
            Il2CppClass::from_name("UnityEngine", "Vector2")
                .expect(&format!("Failed to find class {}.{}", "UnityEngine", "Vector2"))
        });

        &CLASS_TYPE
    }

    fn class_mut() -> &'static mut Il2CppClass {
        Self::class().clone()
    }
}

impl Il2CppClassData for Vector3 {
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS: &'static str = "Vector3";

    fn class() -> &'static Il2CppClass {
        static CLASS_TYPE: std::sync::LazyLock<&'static mut Il2CppClass> = std::sync::LazyLock::new(|| {
            Il2CppClass::from_name("UnityEngine", "Vector3")
                .expect(&format!("Failed to find class {}.{}", "UnityEngine", "Vector3"))
        });

        &CLASS_TYPE
    }

    fn class_mut() -> &'static mut Il2CppClass {
        Self::class().clone()
    }
}

impl ArrayInstantiator<Vector2> for Array<Vector2> {
    fn new(capacity: usize) -> Il2CppResult<&'static mut Self> {
        array_new(Vector2::class(), capacity)
    }
    fn from_slice(mut slice: impl AsMut<[Vector2]>) -> Il2CppResult<&'static mut Self> {
        let new_array = array_new::<Vector2>(Vector2::class(), slice.as_mut().len())?;
        new_array.swap_with_slice(slice.as_mut());
        Ok(new_array)
    }
}

impl ArrayInstantiator<Vector3> for Array<Vector3> {
    fn new(capacity: usize) -> Il2CppResult<&'static mut Self> {
        array_new(Vector3::class(), capacity)
    }
    fn from_slice(mut slice: impl AsMut<[Vector3]>) -> Il2CppResult<&'static mut Self> {
        let new_array = array_new::<Vector3>(Vector3::class(), slice.as_mut().len())?;
        new_array.swap_with_slice(slice.as_mut());
        Ok(new_array)
    }
}


// Array_New function
pub fn array_new<T>(
    element_typeinfo: &Il2CppClass,
    length: usize,
) -> Il2CppResult<&'static mut Il2CppArray<T>> {
    unsafe {
        api_array_new(element_typeinfo, length)
            .ok_or(Il2CppError::FailedArrayInstantiation)
    }
}


// External Functions
// Array external functions
#[skyline::from_offset(0x00428DD0)]
fn api_array_new<T>(
    element_typeinfo: &Il2CppClass,
    length: usize,
) -> Option<&'static mut Il2CppArray<T>>;


// Component external functions
#[unity::from_offset("UnityEngine", "Component", "get_gameObject")] // 0x7102C46440
fn component_getgameobject<T: UEComponent + ?Sized>(
    this: &T,
    method_info: OptionalMethod,
) -> &'static GameObject;


// Transform external functions
#[unity::from_offset("UnityEngine", "Transform", "get_position")] // 0x710378F890
fn transform_getposition<T: Transformation + ?Sized>(
    this: &T,
    method_info: OptionalMethod,
) -> Vector3;

#[skyline::from_offset(0x03791140)] // 0x7103791120
fn transform_translate_vec3<T: Transformation + ?Sized>(
    this: &T,
    translation: Vector3,
    method_info: OptionalMethod,
);

#[unity::from_offset("UnityEngine", "Transform", "get_parent")] // 0x71037909F0
fn transform_getparent<T: Transformation + ?Sized, U: Transformation + ?Sized>(
    this: &T,
    method_info: OptionalMethod,
) -> Option<&'static U>;

#[unity::from_offset("UnityEngine", "Transform", "set_parent")] // 0x7103790A90
fn transform_setparent<T: Transformation + ?Sized, U: Transformation + ?Sized>(
    this: &T,
    value: &U,
    method_info: OptionalMethod,
);

#[unity::from_offset("UnityEngine", "Transform", "Find")] // 0x7103792840
fn transform_find<T: Transformation + ?Sized, U: Transformation + ?Sized>(
    this: &T,
    n: &Il2CppString,
    method_info: OptionalMethod,
) ->  Option<&'static U>;

#[unity::from_offset("UnityEngine", "Transform", "GetChildCount")] // 0x7103792D90
fn transform_getchildcount<T: Transformation + ?Sized>(
    this: &T,
    method_info: OptionalMethod,
) -> i32;

#[unity::from_offset("UnityEngine", "Transform", "GetRoot")] // 0x7103792550
fn transform_getroot<T: Transformation + ?Sized>(
    this: &T,
    method_info: OptionalMethod,
) -> &'static T;

#[unity::from_offset("UnityEngine", "Transform", "GetChild")] // 0x7103792D40
fn transform_getchild<T: Transformation + ?Sized, U: Transformation + ?Sized>(
    this: &T,
    value: i32,
    method_info: OptionalMethod,
) -> Option<&'static U>;

#[unity::from_offset("UnityEngine", "Transform", "FindChild")] // 0x7103792AD0
fn transform_findchild<T: Transformation + ?Sized, U: Transformation + ?Sized>(
    this: &T,
    n: &Il2CppString,
    method_info: OptionalMethod,
) -> Option<&'static U>;


// RectTransform external functions
#[unity::from_offset("UnityEngine", "RectTransform", "get_sizeDelta")] // 0x7102F7C8C0
fn rectransform_getsizedelta(
    this: &RectTransform,
    method_info: OptionalMethod,
) -> Vector2;

#[unity::from_offset("UnityEngine", "RectTransform", "set_sizeDelta")] // 0x7102F7C970
fn rectransform_setsizedelta(
    this: &RectTransform,
    value: Vector2,
    method_info: OptionalMethod,
);


// GameObject external functions
#[unity::from_offset("UnityEngine", "GameObject", "get_active")] // 0x7102C4E970
fn gameobject_getactive(this: &GameObject, method_info: OptionalMethod) -> bool;

#[unity::from_offset("UnityEngine", "GameObject", "set_active")] // 0x7102C4E9C0
fn gameobject_setactive(this: &GameObject, value: bool, method_info: OptionalMethod);

#[unity::from_offset("UnityEngine", "GameObject", "get_transform")] // 0x7102C4E880
fn gameobject_gettransform<T: Transformation + ?Sized>(
    this: &GameObject,
    method_info: OptionalMethod
) -> &'static T;

#[skyline::from_offset(0x02C4DD40)] // 0x7102C4DD40
fn gameobject_getcomponent_string<T: UEComponent + ?Sized>(
    this: &GameObject,
    ty: &Il2CppString,
    method_info: OptionalMethod
) -> Option<&'static T>;


// Object external functions
#[unity::from_offset("UnityEngine", "Object", "get_name")] // 0x71032E8E90
fn object_getname<T: UEObject + ?Sized>(this: &T, method_info: OptionalMethod) -> Option<&'static Il2CppString>;

#[unity::from_offset("UnityEngine", "Object", "set_name")] // 0x71032EE5F0
fn object_setname<T: UEObject + ?Sized>(this: &T, value: &Il2CppString, method_info: OptionalMethod);

#[skyline::from_offset(0x032EEFB0)] // 0x71032EEFB0
fn object_instantiate<T: UEObject + ?Sized>(original: &T, method_info: OptionalMethod) -> &'static T;

#[skyline::from_offset(0x032EF640)] // 0x71032EF640
fn object_destroy<T: UEObject + ?Sized>(obj: &T, method_info: OptionalMethod);