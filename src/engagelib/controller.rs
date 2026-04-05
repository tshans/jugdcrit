//! Types for the in-combat UI.
use unity::prelude::*;
use unity::il2cpp::object::Array;
use unity::system::List;
use unity::engine::Sprite;
use unity::engine::ui::Image;
use unity::engine::Color;

use super::tmpro::TextMeshProUGUI;

use super::triggeredskill::TriggeredSkillListSetter;
use super::forcetexture::ForceTextureSetter;

use crate::unitylib::engine::Animator;
use crate::unitylib::engine::GameObject;
use crate::unitylib::engine::MonoBehaviourFields;

// Classes
#[unity::class("", "CombatGaugeController")]
#[static_fields(CombatGaugeControllerStaticFields)]
pub struct CombatGaugeController {
    pub sup: MonoBehaviourFields,
    pub is_left: bool,
    pub chara_name: &'static TextMeshProUGUI,
    pub item_root: &'static GameObject,
    pub item_name: &'static TextMeshProUGUI,
    pub item: &'static GameObject,
    pub hp_root: &'static GameObject,
    pub hp: &'static mut TextMeshProUGUI,
    pub gaue_root: &'static GameObject, // Note the typo
    pub gauge_hp_base: &'static GameObject,
    pub gauge_hp_move: &'static GameObject,
    pub gauge_hp_now: &'static GameObject,
    pub damage_color: Color,
    pub heal_color: Color,
    pub max_hp_gauge_width: i32,
    pub min_hp_gauge_width: i32,
    pub hp_gauge_max: i32,
    pub hp_move_wait: f32,
    pub hp_move_speed: f32,
    pub hp_stock_root: &'static GameObject,
    pub hp_stock: &'static Array<&'static Image>,
    pub hp_stock_sprites: &'static Array<&'static Sprite>,
    pub now_hp_width: f32,
    pub moving_hp_width: f32,
    pub hp_move_wait_timer: f32,
    pub is_hp_move: bool,
    pub hp_move_scale: f32,
    pub param_root: &'static GameObject,
    pub btl_hp_title: &'static TextMeshProUGUI, // Set when is_left == true
    pub btl_atk_title: &'static TextMeshProUGUI, // Set when is_left == false
    pub btl_hit_title: &'static TextMeshProUGUI, // Set when is_left == false
    pub btl_crit_title: &'static TextMeshProUGUI, // Set when is_left == false
    pub btl_atk: &'static List<CombatGaugeControllerValue>,
    pub btl_hit: &'static TextMeshProUGUI,
    pub btl_crit: &'static TextMeshProUGUI,
    pub chain_root: &'static GameObject,
    pub chain_btl_atk: &'static TextMeshProUGUI,
    pub chain_btl_hit: &'static TextMeshProUGUI,
    pub chain_btl_crit: &'static TextMeshProUGUI,
    pub god_info_root: &'static GameObject,
    pub god_name: &'static TextMeshProUGUI,
    pub engage_count: &'static Array<&'static GameObject>,
    pub triggered_skill_list_setter: &'static TriggeredSkillListSetter,
    pub battle_start_skill_list: &'static List<Image>,
    pub battle_start_skill_index: i32,
    pub force_texture: &'static List<ForceTextureSetter>,
    pub animator: &'static Animator,
}

// Derived Classes
#[unity::class("", "Value")]
#[nested_from_type(CombatGaugeController)]
pub struct CombatGaugeControllerValue {
    pub root: &'static GameObject,
    pub text: &'static TextMeshProUGUI,
}


// Static Fields
#[repr(C)]
pub struct CombatGaugeControllerStaticFields {
    pub anime_nameh_in: &'static Il2CppString, // Typo is in dump.cs
    pub anime_name_out: &'static Il2CppString,
    pub param_clamp: i32,
}