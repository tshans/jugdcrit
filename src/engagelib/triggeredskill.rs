//! Methods for the in-combat triggered skill icon list
use unity::prelude::*;
use unity::il2cpp::object::Array;

use engage::gamedata::skill::SkillData;

use crate::unitylib::engine::Animator;
use crate::unitylib::engine::GameObject;
use crate::unitylib::engine::MonoBehaviourFields;

// Classes
#[unity::class("App", "TriggeredSkillListSetter")]
pub struct TriggeredSkillListSetter {
    pub sup: MonoBehaviourFields,
    pub show_skill_list: &'static Array<&'static TriggeredSkillListSetterShowSkill>
}

// Derived Classes
#[repr(C)] // #[unity::class("App", "TriggeredSkillListSetter.ShowSkill")]
pub struct TriggeredSkillListSetterShowSkill {
    pub klass: &'static mut Il2CppClass,
    monitor: *const u8,
    pub fields: TriggeredSkillListSetterShowSkillFields,
}
#[repr(C)]
pub struct TriggeredSkillListSetterShowSkillFields {
    pub object: &'static GameObject,
    pub data: &'static SkillData,
    pub time: f32,
    pub ani: &'static Animator,
}