use unity::engine::ui::Image;
use unity::il2cpp::object::Array;
use unity::prelude::*;
use unity::system::List;

use engage::gamedata::unit::GodUnit;
use engage::gamedata::unit::UnitRing;
use engage::gamedata::unit::Unit;

use crate::engagelib::battle::BattleInfo;
use crate::engagelib::tmpro::TextMeshProUGUI;
use crate::unitylib::engine::GameObject;
use crate::unitylib::engine::MonoBehaviourFields;
use crate::unitylib::engine::UEComponent;
use crate::unitylib::engine::Vector2;


// Classes
#[unity::class("App", "HelpItemBase")]
pub struct HelpItemBase {
    pub sup: MonoBehaviourFields,
    pub is_temp_god: bool,
    pub temp_god: &'static GodUnit,
    pub is_temp_ring: bool,
    pub temp_ring: &'static UnitRing,
    pub is_temp_unit: bool,
    pub temp_unit: &'static Unit,
    pub start_item_priority: i32,
    pub help_item_type: HelpItemType,
}

#[unity::class("App", "HelpItemFixedText")]
pub struct HelpItemFixedText {
    pub sup: HelpItemBaseFields,
    pub mid: &'static Il2CppString,
}

#[unity::class("App", "HelpItemList")]
pub struct HelpItemList {
    pub sup: MonoBehaviourFields,
    pub situation_type: SituationType,
    pub list: &'static mut Array<Option<&'static HelpItemBase>>,
}

#[unity::class("App", "HelpManager")]
pub struct HelpManager {
    pub sup: MonoBehaviourFields,
    binder: *const (),
    pub help_param_setter: &'static HelpParamSetter,
    pub help_list: &'static mut List<HelpItemList>,
    pub item_list: &'static mut List<HelpManagerItem>,
    pub current_index: i32,
    pub situation_type: SituationType,
    pub previous_dir: ItemDir,
    pub axis_pos: Vector2,
}

#[unity::class("App", "HelpParamSetter")]
pub struct HelpParamSetter {
    pub sup: MonoBehaviourFields,
    pub cursor_obj: &'static GameObject,
    pub window_obj: &'static GameObject,
    pub title_root: &'static GameObject,
    pub weapon_root: &'static GameObject,
    pub message_root: &'static GameObject,
    pub item_icon: &'static GameObject,
    pub skill_icon: &'static Image,
    pub content_name: &'static TextMeshProUGUI,
    pub endurance: &'static TextMeshProUGUI,
    pub title_atk: &'static TextMeshProUGUI,
    pub value_atk: &'static TextMeshProUGUI,
    pub icon_atk: &'static Image,
    pub value_hit: &'static TextMeshProUGUI,
    pub icon_hit: &'static Image,
    pub value_crit: &'static TextMeshProUGUI,
    pub icon_crit: &'static Image,
    pub value_spd: &'static TextMeshProUGUI,
    pub icon_spd: &'static Image,
    pub value_avo: &'static TextMeshProUGUI,
    pub icon_avo: &'static Image,
    pub value_crit_avo: &'static TextMeshProUGUI,
    pub icon_crit_avo: &'static Image,
    pub title_range: &'static TextMeshProUGUI,
    pub value_range: &'static TextMeshProUGUI,
    pub efficacy_nothing: &'static TextMeshProUGUI,
    pub efficacy_icons: &'static Array<&'static Image>,
    pub title_weapon_level: &'static TextMeshProUGUI,
    pub value_weapon_level: &'static TextMeshProUGUI,
    pub icon_weapon_level: &'static Image,
    pub contents_text: &'static TextMeshProUGUI,
    pub contents_eng_wep: &'static TextMeshProUGUI,
    pub contents_enchant: &'static TextMeshProUGUI,
    pub contents_sub_text: &'static TextMeshProUGUI,
    pub prev_position: Vector2,
    pub prev_size: Vector2,
    pub next_position: Vector2,
    pub next_size: Vector2,
    pub cursor_move_timer: f32,
    pub default_window_pos: Vector2,
    pub prev_window_pos: Vector2,
    pub next_window_pos: Vector2,
    pub prev_window_pivot: Vector2,
    pub next_window_pivot: Vector2,
    pub prev_window_anchor: Vector2,
    pub next_window_anchor: Vector2,
    pub cursor_move_time: f32,
    pub battle_info: &'static BattleInfo,
    pub tmp_battle_info: &'static BattleInfo,
    pub tmp_calc_unit: &'static Unit,
    pub enhanced_value: &'static Array<i32>,
    pub enhanced_title: &'static Array<&'static Il2CppString>,
    pub is_god_change: bool,
    pub is_ring_change: bool,
    pub ring_select_god: &'static GodUnit,
    pub ring_select_common: &'static UnitRing,
}


// Derived Classes
#[unity::class("", "Item")]
#[nested_from_type(HelpManager)]
pub struct HelpManagerItem {
    pub slf: &'static HelpItemBase,
    pub pos: Vector2,
    pub dir_line: &'static Array<&'static Array<Vector2>>,
    pub is_move_to_no_touch_item: &'static Array<bool>,
}


// Methods
impl HelpItemFixedText {
    pub fn set_data(&self, mid: impl AsRef<str>) {
        unsafe {
            helpitemfixedtext_setdata(self, mid.into(), None)
        }
    }
    pub fn cast_as_base(&self) -> Option<&'static HelpItemBase> {
        self.get_game_object().get_component_name::<HelpItemBase>("HelpItemBase")
    }
}

impl HelpItemList {
    pub fn get_situation_type(&self) -> SituationType {
        unsafe {
            helpitemlist_getsituationtype(self, None)
        }
    }
}


// Traits
// Trait to simulate inheritance for the App.HelpItemBase class
impl UEComponent for HelpItemBase {}
impl UEComponent for HelpItemFixedText {}


// External Functions
// HelpItemFixedText functions
#[unity::from_offset("App", "HelpItemFixedText", "SetData")] // 0x710297D2A0
fn helpitemfixedtext_setdata(this: &HelpItemFixedText, mid: &Il2CppString, method_info: OptionalMethod);


// HelpItemList functions
#[unity::from_offset("App", "HelpItemList", "get_SituationType")] // 0x710297D950
fn helpitemlist_getsituationtype(this: &HelpItemList, method_info: OptionalMethod) -> SituationType;


// Enums
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HelpItemType {
    None = 0,
    UnitName = 1,
    GodName = 2,
    BondLv = 3,
    JobTitle = 4,
    Efficacy = 5,
    BattleType = 6,
    UnitLv = 7,
    Move = 8,
    WeaponLv = 9,
    Parameter = 10,
    Hp = 11,
    HpStock = 12,
    WeaponItem = 13,
    GodWeaponItem = 14,
    Skill = 15,
    GodSkill = 16,
    TerrainInfoSkill = 17,
    Enchantment = 18,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SituationType {
    None = 0,
    UnitStatus = 1,
    RingSelect = 2,
    UnitInfo = 3,
    BattleInfo = 4,
    BattleEngageInfo = 5,
    BattleAlternativeInfo = 6,
    OnlyMapTerrainInfo = 7,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ItemDir {
    Up = 0,
    Down = 1,
    Right = 2,
    Left = 3,
    Num = 4,
}