//! Types for manipulating the pre-combat forecast UI.
use unity::il2cpp::object::Array;
use unity::engine::ui::Image;
use unity::engine::Material;
use unity::engine::Sprite;
use unity::prelude::*;
use unity::system::List;

use engage::force::ForceType;
use engage::gamedata::skill::SkillArray;
use engage::gamedata::unit::Unit;

use super::battle::BattleInfo;
use super::battle::BattleInfoSide;
use super::battle::BattleSceneList;
use super::battle::BattleSideType;
use super::help::HelpItemFixedText;
use super::forcetexture::ForceTextureSetter;
use super::tmpro::TextMeshProUGUI;

use crate::unitylib::engine::GameObject;
use crate::unitylib::engine::MonoBehaviourFields;


// Classes
#[unity::class("App", "MapBattleInfoWindow")]
#[static_fields(MapBattleInfoWindowStaticFields)]
pub struct MapBattleInfoWindow {
    pub sup: [u8; 0x10], // SingletonClass<MapBattleInfoWindow>Fields
    pub prefab_handle: *const u8, // App.TResourceHandle.GameObject
    pub is_valid: bool,
    pub game_object: &'static GameObject,
    pub battle_info_l: &'static GameObject,
    pub battle_info_r: &'static GameObject,
    pub singles: &'static Array<&'static MapBattleInfoWindowSingle>,
    pub battle_sequence: &'static MapBattleInfoSequence,
    pub support_infos: &'static Array<&'static SupportInfo>,
}

#[unity::class("App", "SupportInfo")]
pub struct SupportInfo {
    pub sup: MonoBehaviourFields,
    pub side_type: BattleSideType,
    pub hit: i32,
    pub avoid: i32,
    pub critical: i32,
    pub secure: i32,
    pub support_units: &'static Array<&'static Unit>,
    pub skill_array: &'static SkillArray,
}

#[unity::class("App", "MapBattleInfoWindowSingle")]
pub struct MapBattleInfoWindowSingle {
    pub map_battle_info_param_setter: &'static MapBattleInfoParamSetter,
    pub side_type: BattleSideType,
}

#[unity::class("App", "MapBattleInfoParamSetter")]
pub struct MapBattleInfoParamSetter {
    pub sup: MonoBehaviourFields,
    pub param_clamp: i32,
    pub info_root: &'static GameObject,
    pub chara_name_root: &'static GameObject,
    pub chara_name: &'static TextMeshProUGUI,
    pub god_name_root: &'static GameObject,
    pub god_name: &'static TextMeshProUGUI,
    pub name_only_chara_name_root: &'static Image,
    pub name_only_chara_name: &'static TextMeshProUGUI,
    pub name_only_god_name_root: &'static GameObject,
    pub name_only_god_name: &'static TextMeshProUGUI,
    pub max_hp_gauge_width: i32,
    pub min_hp_gauge_width: i32,
    pub hp_gauge_max: i32,
    pub hp_root: &'static GameObject,
    pub hp_gauge_root: &'static GameObject,
    pub now_hp: &'static TextMeshProUGUI,
    pub after_hp_root: &'static GameObject,
    pub after_hp: &'static TextMeshProUGUI,
    pub after_hp_heal_root: &'static GameObject,
    pub after_hp_heal: &'static TextMeshProUGUI,
    pub hp_gauge_after: &'static GameObject,
    pub hp_gauge_add: &'static GameObject,
    pub damage_material: &'static Material,
    pub heal_material: &'static Material,
    pub engage_material: &'static Material,
    pub hp_stock_root: &'static GameObject,
    pub hp_stock: &'static Array<&'static Image>,
    pub hp_stock_sprites: &'static Array<&'static Sprite>,
    pub hp_stock_add: &'static GameObject,
    pub damage_space: f32,
    pub status_root: &'static GameObject,
    pub btl_atk_help: &'static HelpItemFixedText,
    pub btl_atk_title: &'static TextMeshProUGUI,
    pub btl_atk: &'static TextMeshProUGUI,
    pub btl_hit: &'static TextMeshProUGUI,
    pub btl_crit: &'static TextMeshProUGUI,
    pub chain_btl_atk: &'static TextMeshProUGUI,
    pub chain_btl_hit_root: &'static GameObject,
    pub chain_btl_hit: &'static TextMeshProUGUI,
    pub chain_btl_crit_root: &'static GameObject,
    pub chain_btl_crit: &'static TextMeshProUGUI,
    pub weapon_root: &'static GameObject,
    pub weapon_icon_root: &'static GameObject,
    pub weapon_icon: &'static Image,
    pub weapon_arrow: &'static Image,
    pub weapon_name: &'static mut TextMeshProUGUI,
    pub weapon_nothing: &'static GameObject,
    pub weapon_endurance: &'static mut TextMeshProUGUI,
    pub weapon_change_l: &'static GameObject,
    pub weapon_change_r: &'static GameObject,
    pub item_list_root: &'static GameObject,
    pub space_root: &'static GameObject,
    pub force_texture: &'static List<ForceTextureSetter>,
    pub side_type: BattleSideType,
    pub battle_info: &'static BattleInfo,
    pub battle_scene_list: &'static BattleSceneList,
    pub side: &'static BattleInfoSide,
    pub reverse_side: &'static BattleInfoSide,
}

#[unity::class("App", "MapBattleInfoSequence")]
pub struct MapBattleInfoSequence {
    pub sup: MonoBehaviourFields,
    pub attack_list: &'static Array<&'static MapBattleInfoSequenceAttack>,
    pub heal_root: &'static GameObject,
    pub heal: &'static TextMeshProUGUI,
}

#[unity::class("App", "MapBattleInfoSequenceAttack")]
pub struct MapBattleInfoSequenceAttack {
    pub sup: MonoBehaviourFields,
    pub defense_damage: &'static TextMeshProUGUI,
    pub defense_arrow: &'static Image,
    pub defense_dead: &'static Image,
    pub offense_damage: &'static TextMeshProUGUI,
    pub offense_arrow: &'static Image,
    pub offense_dead: &'static Image,
    pub defense_result: &'static TextMeshProUGUI,
    pub offense_result: &'static TextMeshProUGUI,
    pub damage_material: &'static Material,
    pub heal_material: &'static Material,
    pub arrow_sprite: &'static Array<&'static Sprite>,
    pub inst_damage_material: &'static Material,
    pub inst_heal_material: &'static Material,
}

// Derived Classes
#[unity::class("", "Info")]
#[nested_from_type(MapBattleInfoSequenceAttack)]
pub struct MapBattleInfoSequenceAttackInfo {
    pub side_type: BattleSideType,
    pub dead_side: BattleSideType,
    pub offense_result: MapBattleInfoSequenceAttackInfoSceneResult,
    pub defense_result: MapBattleInfoSequenceAttackInfoSceneResult,
    pub force: ForceType,
    pub atk_string: &'static Il2CppString,
    pub self_damage: i32,
    pub receive_first_damage: i32,
    pub send_first_damage: i32,
    pub is_heal: bool,
    pub is_chain_atk: bool,
}


// Static Fields
#[repr(C)]
pub struct MapBattleInfoWindowStaticFields {
    pub prefab_path: &'static Il2CppString,
    pub left_side_obj_name: &'static Il2CppString,
    pub right_side_obj_name: &'static Il2CppString,
    pub battle_sequence_obj_name: &'static Il2CppString,
}


// Methods
impl MapBattleInfoWindow {
    pub fn get_battle_info_l(&self) -> &'static GameObject {
        unsafe {
            mapbattleinfowindow_getbattleinfol(self, None)
        }
    }
    pub fn get_battle_info_r(&self) -> &'static GameObject {
        unsafe {
            mapbattleinfowindow_getbattleinfor(self, None)
        }
    }
}


// External Functions
// MapBattleInfoWindow external functions
#[unity::from_offset("App", "MapBattleInfoWindow", "get_BattleInfoL")] // 0x71029991E0
fn mapbattleinfowindow_getbattleinfol(this: &MapBattleInfoWindow, method_info: OptionalMethod) -> &'static GameObject;

#[unity::from_offset("App", "MapBattleInfoWindow", "get_BattleInfoR")] // 0x71029991F0
fn mapbattleinfowindow_getbattleinfor(this: &MapBattleInfoWindow, method_info: OptionalMethod) -> &'static GameObject;


// Enums
#[repr(C)]
pub enum MapBattleInfoSequenceAttackInfoSceneResult {
    Break = 0,
    ChainAttack = 1,
    ChainGuard = 2,
    ChainAttackGuard = 3,
    None = 4,
}