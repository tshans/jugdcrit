//! Types and methods for controlling the game's battle system.
use std::collections::HashSet;
use std::ops::Deref;

use bitflags::bitflags;

use unity::il2cpp::object::Array;
use unity::prelude::*;
use unity::system::List;

use engage::gamedata::item::UnitItem;
use engage::gamedata::unit::Unit;
use engage::gamedata::skill::SkillArray;
use engage::gamedata::skill::SkillData;
use engage::gamedata::terrain::TerrainData;

use crate::hooks::combatdetails::jugdral_critical;

use super::bitfields::BitFieldTemplate32Fields;
use super::pool::PoolHierarchy;
use super::pool::PoolList;


// Classes
#[unity::class("App", "BattleCalculator")]
#[static_fields(BattleCalculatorStaticFields)]
pub struct BattleCalculator {
    pub mode: BattleCalculatorMode,
    pub info: &'static BattleInfo,
    pub flag: &'static BattleCalculatorFlagField,
    pub scene_list: &'static BattleSceneList,
    pub orders: &'static BattleCalculatorOrderList,
    pub next_order_index: i32,
    pub equip_skill: &'static SkillData,
    pub chain_offenses: &'static List<BattleInfoSide>,
    pub chain_defenses: &'static List<BattleInfoSide>,
    pub hit_skill_pool: &'static BattleCalculatorHitSkillPool,
    pub commit_skill_units: &'static HashSet<i32>,
}

#[unity::class("App", "BattleDetail")]
pub struct BattleDetail {
    pub capability: &'static mut CapabilityInt,
    pub base_params: &'static mut Array<i32>,
    pub battle_params: &'static mut Array<&'static BattleParam>,
    pub attack_attribute: BattleCalculatorAttributes,
    pub skill_layers: i32,
    pub active_skill: &'static mut SkillArray,
}

#[unity::class("App", "CapabilityInt")]
pub struct CapabilityInt {
    pub data: &'static mut Array<i32>,
}

#[unity::class("App", "BattleInfo")]
pub struct BattleInfo {
    pub flag: &'static BattleInfoFlagField,
    pub sides: &'static BattleInfoSideArray,
    pub supports: &'static BattleInfoSupportList,
    pub main_unit_enum: BattleInfoEnumFields,
    pub whole_unit_enum: BattleInfoEnumFields,
    pub chain_offense_enum: BattleInfoEnumFields,
    pub chain_defense_enum: BattleInfoEnumFields,
    pub chain_unit_enum: BattleInfoEnumFields,
    pub temp_skills: &'static List<SkillData>,
    pub range: i32,
    pub battle_count: i32,
    pub scene_result: BattleSceneResult,
    pub chain_attack_side: BattleSideType,
    pub chain_attack_count: i32,
    pub chain_guard_count: i32,
    pub chain_attack_defeat: i32,
    pub chain_attack_hit: i32,
    pub chain_attack_critical: i32,
    pub chain_attack_damage: i32,
    pub summon_rank: i32, // PersonData.Ranks enum
    pub summon_color: i32, // PersonData.Colors enum
    pub guard_side: BattleSideType,
    pub guard_func: u64
}

#[unity::class("App", "BattleInfoEnum")]
pub struct BattleInfoEnum {
    pub info: &'static BattleInfo,
    pub min: BattleSideType,
    pub max: BattleSideType,
    pub current: &'static BattleInfoSide,
}

#[unity::class("App", "BattleInfoSide")]
pub struct BattleInfoSide {
    pub info: &'static mut BattleInfo,
    pub side_type: BattleSideType,
    __ : i32,
    pub unit: Option<&'static Unit>,
    pub unit_item: Option<&'static UnitItem>,
    pub specified_item: &'static UnitItem,
    pub x: i32,
    pub z: i32,
    pub terrain: &'static TerrainData,
    pub overlap: &'static TerrainData,
    pub status: &'static mut BattleInfoSideBitFieldStatus,
    pub detail: &'static BattleDetail,
    pub hierarchy: &'static PoolHierarchy<BattleDetail>,
    support: u64,
    pub parent: &'static BattleInfoSide,
    pub reverse: Option<&'static BattleInfoSide>,
    destroy: *const u8,
    pub mask_skill: &'static SkillArray,
    pub level: i32,
    pub hp: i32,
    pub gain_exp: i32,
    pub gain_gold: i32,
    pub drop_item_ratio: f32,
    pub pick_up_item: i32,
    pub damage: i32,
    pub heal: i32,
    pub battle_times: i32,
    pub total_order: i32,
    pub total_action: i32,
    pub total_attack: i32,
    pub total_damage: i32,
    pub total_result: BattleSceneResult,
    pub temporary: i32,
    pub stun: i32,
    pub engage_count: i32,
    pub engage_first_count: i32,
    pub blown_distance: i32,
    pub weapon_expend: i32,
    pub expend_count: i32,
}

#[unity::class("App", "BattleMath")]
pub struct BattleMath{ }

#[unity::class("App", "BattleParam")]
#[static_fields(BattleParamStaticFields)]
pub struct BattleParam {
    // These two fields contain skill-based changes to the base BattleParam formula.
    pub add: f32,
    pub scale: f32,
    // Base value as calculated using the CalculatorCommand formulas
    pub value: f32,
}

#[unity::class("App", "BattleScene")]
pub struct BattleScene {
    pub list: &'static BattleSceneList,
    pub side: BattleSideType,
    pub target: BattleSideType,
    pub kind: BattleSceneKind,
    pub skill: i32,
    pub item: i32,
    pub god: i32,
    pub index: i32,
    pub result: &'static BattleSceneFieldResult,
    pub guardian: BattleSideType,
    pub hps: &'static BattleSideShortArray,
    pub engages: &'static BattleSideSbyteArray,
    pub damages: &'static BattleSideShortArray,
}

#[unity::class("App", "BattleSceneList")]
pub struct BattleSceneList {
    pub sup: PoolList<BattleScene>,
    pub info: &'static BattleInfo,
    pub index: i32,
}

#[unity::class("App", "BattleSide")]
#[static_fields(BattleSideStaticFields)]
pub struct BattleSide { }


// Derived Classes
// BattleCalculator derived classes
#[repr(C)] // #[unity::class("App", "BattleCalculator.FlagField")]
pub struct BattleCalculatorFlagField {
    pub klass: &'static mut Il2CppClass,
    monitor: *const u8,
    pub fields: BattleCalculatorFlagFieldFields,
}
#[repr(C)]
pub struct BattleCalculatorFlagFieldFields {
    pub sup: BitFieldTemplate32Fields<BattleCalculatorFlags>,
}

#[repr(C)] // #[unity::class("App", "BattleCalculator.Order")]
pub struct BattleCalculatorOrder {
    pub klass: &'static mut Il2CppClass,
    monitor: *const u8,
    pub fields: BattleCalculatorOrderFields,
}
#[repr(C)]
pub struct BattleCalculatorOrderFields {
    pub side: &'static BattleSideType,
}

#[repr(C)] // #[unity::class("App", "BattleCalculator.OrderList")]
pub struct BattleCalculatorOrderList {
    pub klass: &'static mut Il2CppClass,
    monitor: *const u8,
    pub fields: BattleCalculatorOrderListFields,
}
#[repr(C)]
pub struct BattleCalculatorOrderListFields {
    pub sup: PoolList<BattleCalculatorOrder>,
}

#[repr(C)] // #[unity::class("App", "BattleCalculator.HitSkill")]
pub struct BattleCalculatorHitSkill {
    pub klass: &'static mut Il2CppClass,
    monitor: *const u8,
    pub fields: BattleCalculatorHitSkillFields,
}
#[repr(C)]
pub struct BattleCalculatorHitSkillFields {
    pub side: &'static BattleInfoSide,
    pub action: i32,
    pub skill: &'static SkillData,
}

#[repr(C)] // #[unity::class("App", "BattleCalculator.HitSkillPool")]
pub struct BattleCalculatorHitSkillPool {
    pub klass: &'static mut Il2CppClass,
    monitor: *const u8,
    pub fields: BattleCalculatorHitSkillPoolFields,
}
#[repr(C)]
pub struct BattleCalculatorHitSkillPoolFields {
    pub sup: PoolList<BattleCalculatorHitSkill>,
}


// BattleInfo derived classes
#[repr(C)] // #[unity::class("App", "BattleInfo.BattleInfoSideArray")]
pub struct BattleInfoSideArray {
    pub klass: &'static mut Il2CppClass,
    monitor: *const u8,
    pub fields: BattleInfoSideArrayFields,
}
#[repr(C)]
pub struct BattleInfoSideArrayFields {
    pub sup: BattleSideContainerArrayFields<BattleInfoSide>
}

#[repr(C)] // #[unity::class("App", "BattleInfo.FlagField")]
pub struct BattleInfoFlagField {
    pub klass: &'static mut Il2CppClass,
    monitor: *const u8,
    pub fields: BattleInfoFlagFieldFields,
}
#[repr(C)]
pub struct BattleInfoFlagFieldFields {
    pub sup: BitFieldTemplate32Fields<BattleInfoFlags>
}

#[repr(C)] // #[unity::class("App", "BattleInfo.SupportList")]
pub struct BattleInfoSupportList {
    pub klass: &'static mut Il2CppClass,
    monitor: *const u8,
    pub fields: BattleInfoSupportListFields,
}
#[repr(C)]
pub struct BattleInfoSupportListFields {
    pub sup: List<BattleInfoSupportData>,
    pub offense: &'static Unit,
    pub defense: &'static Unit,
    pub compare: u64
}

#[repr(C)] // #[unity::class("App", "BattleInfo.SupportData")]
pub struct BattleInfoSupportData {
    pub klass: &'static mut Il2CppClass,
    monitor: *const u8,
    pub fields: BattleInfoSupportDataFields,
}
#[repr(C)]
pub struct BattleInfoSupportDataFields {
    pub unit: &'static Unit,
    pub status: BattleInfoSideStatus,
}


// BattleInfoSide derived classes
#[repr(C)] // #[unity::class("App", "BattleInfoSide.BitFieldStatus")]
pub struct BattleInfoSideBitFieldStatus {
    pub klass: &'static mut Il2CppClass,
    monitor: *const u8,
    pub fields: BattleInfoSideBitFieldStatusFields,
}
#[repr(C)]
pub struct BattleInfoSideBitFieldStatusFields {
    pub sup: BitFieldTemplate32Fields<BattleInfoSideStatus>,
}


// BattleScene derived classes
#[repr(C)] // #[unity::class("App", "BattleScene.FieldResult")]
pub struct BattleSceneFieldResult {
    pub klass: &'static mut Il2CppClass,
    monitor: *const u8,
    pub fields: BattleSceneFieldResultFields,
}
#[repr(C)]
pub struct BattleSceneFieldResultFields {
    pub sup: BitFieldTemplate32Fields<BattleSceneResult>,
}

// BattleSide derived classes
#[repr(C)] // #[unity::class("App", "BattleSide.ContainerArray`1")]
pub struct BattleSideContainerArray<T: 'static> {
    pub klass: &'static mut Il2CppClass,
    monitor: *const u8,
    pub fields: BattleSideContainerArrayFields<T>,
}
#[repr(C)]
pub struct BattleSideContainerArrayFields<T: 'static> {
    pub array: &'static mut Array<&'static T>,
}

#[repr(C)] // #[unity::class("App", "BattleSide.StructArray`1")]
pub struct BattleSideStructArray<T: 'static> {
    pub klass: &'static mut Il2CppClass,
    monitor: *const u8,
    pub fields: BattleSideStructArrayFields<T>,
}
#[repr(C)]
pub struct BattleSideStructArrayFields<T: 'static> {
    pub array: &'static Array<T>,
}

#[repr(C)] // #[unity::class("App", "BattleSide.SbyteArray")]
pub struct BattleSideSbyteArray {
    pub klass: &'static mut Il2CppClass,
    monitor: *const u8,
    pub fields: BattleSideSbyteArrayFields,
}
#[repr(C)]
pub struct BattleSideSbyteArrayFields {
    pub sup: BattleSideStructArrayFields<i8>,
}

#[repr(C)] // #[unity::class("App", "BattleSide.ShortArray")]
pub struct BattleSideShortArray {
    pub klass: &'static mut Il2CppClass,
    monitor: *const u8,
    pub fields: BattleSideShortArrayFields,
}
#[repr(C)]
pub struct BattleSideShortArrayFields {
    pub sup: BattleSideStructArrayFields<i16>,
}


// Static Fields
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BattleCalculatorStaticFields {
    cannon_conditions: &'static Array<&'static Il2CppString>, // 0x0
    dance_conditions: &'static Array<&'static Il2CppString>, // 0x8
    engage_summon_3: &'static Array<&'static Il2CppString>, // 0x10
    engage_summon_5: &'static Array<&'static Il2CppString> // 0x18
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BattleParamStaticFields {
    pub invalid: f32, // 0x0
    pub mins: &'static Array<f32>, // 0x8
    pub maxs: &'static Array<f32>, // 0x10
    pub clamps: &'static Array<f32>, // 0x18
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BattleSideStaticFields {
    pub parents: &'static Array<BattleSideType>, // 0x0
    pub reverses: &'static Array<BattleSideType>, // 0x8
    pub stands: &'static Array<BattleSideType> // 0x10
}


// Methods
impl BattleCalculator {
    pub fn get_infoside_type(&self, ty: BattleSideType) -> Option<&'static BattleInfoSide> {
        self.get_info().get_side_type(ty)
    }
    pub fn get_info(&self) -> &'static BattleInfo {
        unsafe {
            battlecalculator_getinfo(self, None)
        }
    }
}

impl BattleDetail {
    pub fn get_action_count(&self) -> i32 {
        unsafe {
            battledetail_getactioncount(self, None)
        }
    }
    pub fn get_attack(&self) -> Option<&'static BattleParam> {
        unsafe {
            battledetail_getattack(self, None)
        }
    }
    pub fn get_defense(&self) -> Option<&'static BattleParam> {
        unsafe {
            battledetail_getdefense(self, None)
        }
    }
    pub fn get_simple_power(&self) -> Option<&'static BattleParam> {
        unsafe {
            battledetail_getsimplepower(self, None)
        }
    }
    pub fn get_simple_hit(&self) -> Option<&'static BattleParam> {
        unsafe {
            battledetail_getsimplehit(self, None)
        }
    }
    pub fn get_simple_critical(&self) -> Option<&'static BattleParam> {
        unsafe {
            battledetail_getsimplecritical(self, None)
        }
    }
}

impl BattleInfo {
    pub fn get_side_type(&self, ty: BattleSideType) -> Option<&'static BattleInfoSide> {
        unsafe {
            battleinfo_getside_type(self, ty, None)
        }
    }
    pub fn get_offense(&self) -> &'static BattleInfoSide {
        self.get_side_type(BattleSideType::Offense).unwrap()
    }
}

impl BattleInfoSide {
    pub fn set_damage(&self, value: i32) {
        unsafe {
            battleinfoside_setdamage(self, value, None)
        }
    }
    pub fn get_simple_power(&self, is_critical: bool) -> i32 {
        // This method is modified to use the correct damage value for criticals.
        if is_critical {
            jugdral_critical(self)
        } else {
            unsafe {
                battleinfoside_getsimplepower(self, false, None)
            }
        }
    }
    pub fn get_reverse(&self) -> Option<&'static Self> {
        unsafe {
            battleinfoside_getreverse(self, None)
        }
    }
    pub fn get_detail(&self) -> &'static BattleDetail {
        unsafe {
            battleinfoside_getdetail(self, None)
        }
    }
    pub fn get_status(&self) -> BattleInfoSideStatus {
        unsafe {
            battleinfoside_getstatus(self, None).get_value()
        }
    }
}

impl BattleMath {
    pub fn get_hit_real_ratio(ratio: i32) -> f32 {
        unsafe {
            battlemath_gethitrealratio(ratio, None)
        }
    }
}

impl BattleParam {
    pub fn get_kind(&self) -> BattleParamKind {
        let method = self
            .get_class()
            .get_virtual_method("get_Kind")
            .expect("The provided BattleParam does not implement 'get_Kind'.");
        let get_kind = unsafe {
            std::mem::transmute::<_, extern "C" fn(
                &BattleParam, &MethodInfo
            ) -> BattleParamKind>(
                method.method_info.method_ptr,
            )
        };

        get_kind(self, method.method_info)
    }
    // Returns the skill-adjusted (final) value of the BattleParam.
    pub fn get_result(&self, side: &BattleInfoSide) -> f32 {
        unsafe {
            battleparam_getresult(self, side,None)
        }
    }
}

impl BattleInfoSideBitFieldStatus {
    pub fn get_value(&self) -> BattleInfoSideStatus {
        self.fields.sup.get_value()
    }
}


// Traits
// BattleSideContainerArrayFields trait implementations
impl<T> Deref for BattleSideContainerArrayFields<T> {
    type Target = [&'static T];

    fn deref(&self) -> &Self::Target {
        self.array.fields.deref()
    }
}


// External Functions
// BattleCalculator functions
#[unity::from_offset("App", "BattleCalculator", "get_Info")] // 0x710246D7A0
fn battlecalculator_getinfo(this: &BattleCalculator, method_info: OptionalMethod) -> &'static BattleInfo;

// BattleDetail functions
#[unity::from_offset("App", "BattleDetail", "get_ActionCount")] // 0x7101E75FF0
fn battledetail_getactioncount(this: &BattleDetail, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "BattleDetail", "get_Attack")] // 0x7101E766E0
fn battledetail_getattack(this: &BattleDetail, method_info: OptionalMethod) -> Option<&'static BattleParam>;

#[unity::from_offset("App", "BattleDetail", "get_Defense")] // 0x7101E76710
fn battledetail_getdefense(this: &BattleDetail, method_info: OptionalMethod) -> Option<&'static BattleParam>;

#[unity::from_offset("App", "BattleDetail", "get_SimplePower")] // 0x7101E76830
fn battledetail_getsimplepower(this: &BattleDetail, method_info: OptionalMethod) -> Option<&'static BattleParam>;

#[unity::from_offset("App", "BattleDetail", "get_SimpleHit")] // 0x7101E76860
fn battledetail_getsimplehit(this: &BattleDetail, method_info: OptionalMethod) -> Option<&'static BattleParam>;

#[unity::from_offset("App", "BattleDetail", "get_SimpleCritical")] // 0x7101E76890
fn battledetail_getsimplecritical(this: &BattleDetail, method_info: OptionalMethod) -> Option<&'static BattleParam>;


// BattleInfo functions
#[skyline::from_offset(0x01E7F210)] // 0x7101E7F210
fn battleinfo_getside_type(this: &BattleInfo, ty: BattleSideType, method_info: OptionalMethod) -> Option<&'static BattleInfoSide>;


// BattleInfoSide functions
#[unity::from_offset("App", "BattleInfoSide", "set_Damage")] // 0x7101E85370
fn battleinfoside_setdamage(this: &BattleInfoSide, value: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "BattleInfoSide", "GetSimplePower")] // 0x7101E89560
fn battleinfoside_getsimplepower(this: &BattleInfoSide, is_critical: bool, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "BattleInfoSide", "get_Reverse")] // 0x7101E85060
fn battleinfoside_getreverse(this: &BattleInfoSide, method_info: OptionalMethod) -> Option<&'static BattleInfoSide>;

#[unity::from_offset("App", "BattleInfoSide", "get_Detail")] // 0x7101E8B340
fn battleinfoside_getdetail(this: &BattleInfoSide, method_info: OptionalMethod) -> &'static BattleDetail;

#[unity::from_offset("App", "BattleInfoSide", "GetStatus")] // 0x7101E8B240
fn battleinfoside_getstatus(this: &BattleInfoSide, method_info: OptionalMethod) -> &'static BattleInfoSideBitFieldStatus;


// BattleMath functions
#[unity::from_offset("App", "BattleMath", "GetHitRealRatio")] // 0x7101E8D4A0
fn battlemath_gethitrealratio(ratio: i32, method_info: OptionalMethod) -> f32;


// BattleParam functions
#[unity::from_offset("App", "BattleParam", "GetResult")] // 0x7101E8DA30
fn battleparam_getresult(this: &BattleParam, side: &BattleInfoSide, method_info: OptionalMethod) -> f32;


// Enums
// BattleCalculator enums
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BattleCalculatorAttributes {
    None = 0,
    Physical = 1,
    Magic = 2,
}

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    /// Bitflags for BattleCalculator.Flags. The combo flags (which are non-power of two)
    /// are provided by the game and included here for completeness.
    pub struct BattleCalculatorFlags: i32 {
        const InterruptOffense = 1;
        const InterruptDefense = 2;
        const Interrupting = 4;
        const ContinueBattle = 8;
        const Swap_Order = 16;
        const Dead = 32;
        const ChainAttacked = 64;
        const Commited = 128;
        const MaskInterrupt = 7;
        // Workaround for all undefined flags/combos of flags
        const _ = !0;
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BattleCalculatorMode {
    Battle = 0,
    JobIntro = 1,
    ClassChange = 2,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BattleCalculatorTrainingResult {
    Win = 0,
    Lose = 1,
}

// BattleInfo enums
bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    /// Bitflags for BattleInfo.Flags. The combo flags (which are non-power of two) are provided by the game and included here for completeness.
    pub struct BattleInfoFlags: i32 {
        const Simulation = 1;
        const Warmup = 2;
        const Alone = 4;
        const Event = 8;
        const Dance = 16;
        const Training = 32;
        const Rod = 64;
        const Destroy = 128;
        const MultiBattle = 256;
        const BowCannon = 512;
        const MagicCannon = 1024;
        const FireCannon = 2048;
        const EngageAttack = 4096;
        const Traial = 8192;
        const IgnoreRange = 16384;
        const IgnorePosition = 32768;
        const IgnoreOffensePosition = 65536;
        const IgnoreRevenge = 131072;
        const IgnoreBreak = 262144;
        const IgnoreTerrain = 524288;
        const IgnoreSupport = 1048576;
        const IgnoreBlow = 2097152;
        const IgnoreChain = 4194304;
        const IgnoreSkill = 8388608;
        const HideCombatGauge = 16777216;
        const SkipCombatGrow = 33554432;
        const FullBullet = 67108864;
        const Summon = 134217728;
        const Enchant = 268435456;
        const CannonMask = 3584;
        // Workaround for all undefined flags/combos of flags
        const _ = !0;
    }
}

// BattleInfoSide enums
bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    /// Bitflags for BattleInfoSide.Status. The combo flags (which are non-power of two)
    /// are provided by the game and included here for completeness.
    pub struct BattleInfoSideStatus: i32 {
        const Offense = 1;
        const Defense = 2;
        const ChainAttack = 4;
        const ChainGuard = 8;
        const EngageLink = 16;
        const IgnorePosition = 32;
        const IgnoreRange = 64;
        const Magic = 128;
        const Rod = 256;
        const HealRod = 512;
        const InterferenceRod = 1024;
        const LongRange = 2048;
        const NotWeapon = 4096;
        const NotAttack = 8192;
        const NoStun = 16384;
        const MoveChainAttack = 32768;
        const HauntChainAttack = 65536;
        const ExpBattle = 131072;
        const ExpDestroy = 262144;
        const ExpRod = 524288;
        const ExpRodMiss = 1048576;
        const GiveExpBattle = 2097152;
        const Gained = 8388608;
        const Dead = 16777216;
        const ChainAttacked = 33554432;
        const ChainGuarded = 67108864;
        const Blown = 134217728;
        const Bounced = 268435456;
        const Breaked = 536870912;
        const Interrupting = 1073741824;
        const ChangeDragon = -2147483648;
        const MaskNoAttack = 12544;
        const MaskExp = 1966080;
        const MaskChain = 12;
        // Workaround for all undefined flags/combos of flags
        const _ = !0;
    }
}

// BattleParam enums
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BattleParamKind {
    Value = 0,
    Ratio = 1,
    Num = 2,
}

// BattleScene enums
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BattleSceneKind {
    None = 0,
    Attack = 1,
    Rod = 2,
    Dance = 3,
    Skill = 4,
    GiveDirect = 5,
    GiveDelay = 6,
    Strip = 7,
    Equip = 8,
    God = 9,
    Dead = 10,
    EngageAttack = 11,
    Separator = 12,
    PushBattle = 13,
    PushOrder = 14,
    PushAction = 15,
    PushAttack = 16,
    PopAttack = 17,
    PopAction = 18,
    PopOrder = 19,
    PopBattle = 20,
    Heal = 21,
    Num = 22,
}

bitflags! {
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    /// Bitflags for the type of Battle Scene.
    pub struct BattleSceneResult: i32 {
        const None = 0;
        const Hit = 1;
        const Critical = 2;
        const Guard = 4;
        const Suicide = 8;
        const Efficacy = 16;
        const Break = 32;
        const Blow = 64;
        const Bounce = 128;
        const ChainAttack = 256;
        const ChainGuard = 512;
        const DualGuard = 1024;
        const EngageAttack = 2048;
        const Physical = 4096;
        const Magic = 8192;
        const Ignore = 16384;
        // Workaround for all undefined flags/combos of flags
        const _ = !0;
    }
}

// BattleSide enum
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BattleSideType {
    None = -1,
    Offense = 0,
    Defense = 1,
    ChainOffense1 = 2,
    ChainOffense2 = 3,
    ChainOffense3 = 4,
    ChainOffense4 = 5,
    ChainOffense5 = 6,
    ChainOffense6 = 7,
    ChainOffense7 = 8,
    ChainOffense8 = 9,
    ChainOffense9 = 10,
    ChainOffense10 = 11,
    ChainOffense11 = 12,
    ChainOffense12 = 13,
    ChainOffense13 = 14,
    ChainOffense14 = 15,
    ChainOffense15 = 16,
    ChainOffense16 = 17,
    ChainOffense17 = 18,
    ChainOffense18 = 19,
    ChainOffense19 = 20,
    ChainOffense20 = 21,
    ChainOffense21 = 22,
    ChainOffense22 = 23,
    ChainOffense23 = 24,
    ChainOffense24 = 25,
    ChainDefense1 = 26,
    ChainDefense2 = 27,
    ChainDefense3 = 28,
    ChainDefense4 = 29,
}