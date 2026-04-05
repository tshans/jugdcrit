//! Methods relating to AI battle priority calculations
use unity::prelude::*;
use unity::il2cpp::object::Array;

use engage::gamedata::unit::Unit;

use super::battle::BattleInfo;
use super::battle::BattleInfoSide;
use super::battle::BattleSideType;

// Classes
#[unity::class("App", "AIBattleSimulator")]
pub struct AIBattleSimulator {
    pub sup: AISimulatorBaseFields,
    pub a_indication: &'static Array<&'static AIBattleSimulatorIndication>,
    pub a_kill_without_interference: &'static Array<f32>,
    pub kill: f32,
    pub dead: f32,
    pub expectation: f32,
    pub expecation_received: f32,
    pub attack: f32,
    pub follow: f32,
    pub a_chain_attack_candidate:
        &'static Array<&'static AIBattleSimulatorChainAttackCandidate>,
    pub chain_attack_expectation: f32,
    pub a_break: &'static Array<&'static AIBattleSimulatorBreak>,
    pub battle_sides: &'static Array<BattleSideType>,
    pub offense_battle_times: i32,
    pub defense_battle_times: i32,
}

#[unity::class("App", "AISimulatorBase")]
pub struct AISimulatorBase {
    pub offense: &'static Unit,
    pub offense_index: i32,
    pub defense: &'static Unit,
    pub battle_info: &'static BattleInfo,
    pub score: u32,
}

// Derived Classes
#[unity::class("", "Indication")]
#[nested_from_type(AIBattleSimulator)]
pub struct AIBattleSimulatorIndication {
    pub power: i32,             // Standard SimplePower damage value
    pub skill_power: i32,       // Additional damage from a skill activation
    pub hit: f32,               // % chance of an attack resulting in a normal hit
    pub critical: f32,          // % chance of critical hit
    pub skill: f32,             // % chance of hit + skill activation
    pub skill_critical: f32,    // % chance of hit + crit + skill activation
    pub prevent: f32,           // % chance to negate attack? (chain guard/bonded shield?)
    pub miss: f32,              // % chance of attack resulting in a miss
    pub expectation: f32,       // Expected damage from a round of combat
    pub is_skill_kill: bool,    // Skill ignores protection effects?
}

#[unity::class("", "ChainAttackCandidate")]
#[nested_from_type(AIBattleSimulator)]
pub struct AIBattleSimulatorChainAttackCandidate {
    pub side: Option<&'static BattleInfoSide>
}

#[unity::class("","Break")]
#[nested_from_type(AIBattleSimulator)]
pub struct AIBattleSimulatorBreak {
    pub break_attack: f32,
    pub stun: f32,
}


// Methods
// AIBattleSimulator methods
impl AIBattleSimulator {
    pub fn calculate_kill_probability_3(
        &self,
        side: BattleSideType,
        count: i32,
        times: i32,
        now_probability: f32,
        rest_hp: i32,
        damage: i32
    ) {
        unsafe {
            aibattlesimulator_calculatekillprobabilitywithoutinterference3(
                self,
                side,
                count,
                times,
                now_probability,
                rest_hp,
                damage,
                None
            )
        }
    }
}


// AIBattleSimulator.Indication methods
impl AIBattleSimulatorIndication {
    pub fn get_power(&self) -> i32 {
        unsafe {
            aibattlesimulator_indication_getpower(self, None)
        }
    }
    pub fn get_skill_power(&self) -> i32 {
        unsafe {
            aibattlesimulator_indication_getskillpower(self, None)
        }
    }
    pub fn get_hit(&self) -> f32 {
        unsafe {
            aibattlesimulator_indication_gethit(self, None)
        }
    }
    pub fn get_critical(&self) -> f32 {
        unsafe {
            aibattlesimulator_indication_getcritical(self, None)
        }
    }
    pub fn get_skill(&self) -> f32 {
        unsafe {
            aibattlesimulator_indication_getskill(self, None)
        }
    }
    pub fn get_skill_critical(&self) -> f32 {
        unsafe {
            aibattlesimulator_indication_getskillcritical(self, None)
        }
    }
    pub fn get_prevent(&self) -> f32 {
        unsafe {
            aibattlesimulator_indication_getprevent(self, None)
        }
    }
    pub fn get_miss(&self) -> f32 {
        unsafe {
            aibattlesimulator_indication_getmiss(self, None)
        }
    }
    pub fn set_expectation(&self, value: f32) {
        unsafe {
            aibattlesimulator_indication_setexpectation(self, value, None)
        }
    }
    pub fn get_is_skill_kill(&self) -> bool {
        unsafe {
            aibattlesimulator_indication_getisskillkill(self, None)
        }
    }
}


// AIBattleSimulatorChainAttackCandidate methods
impl AIBattleSimulatorChainAttackCandidate {
    pub fn get_side(&self) -> Option<&'static BattleInfoSide> {
        unsafe {
            aibattlesimulator_chainattackcandidate_getside(self, None)
        }
    }
}


// External Functions
// AIBattleSimulator functions
#[unity::from_offset(
    "App",
    "AIBattleSimulator",
    "CalculateKillProbabilityWithoutInterference3"
)]
fn aibattlesimulator_calculatekillprobabilitywithoutinterference3(
    this: &AIBattleSimulator,
    side: BattleSideType,
    count: i32,
    times: i32,
    now_probability: f32,
    rest_hp: i32,
    damage: i32,
    method_info: OptionalMethod
);

// AIBattleSimulator.Indication functions
#[skyline::from_offset(0x02941140)] // 0x7102941140
fn aibattlesimulator_indication_getpower(
    this: &AIBattleSimulatorIndication,
    method_info: OptionalMethod
) -> i32;

#[skyline::from_offset(0x02941160)] // 0x7102941160
fn aibattlesimulator_indication_getskillpower(
    this: &AIBattleSimulatorIndication,
    method_info: OptionalMethod
) -> i32;

#[skyline::from_offset(0x02941180)] // 0x7102941180
fn aibattlesimulator_indication_gethit(
    this: &AIBattleSimulatorIndication,
    method_info: OptionalMethod
) -> f32;

#[skyline::from_offset(0x029411A0)] // 0x71029411A0
fn aibattlesimulator_indication_getcritical(
    this: &AIBattleSimulatorIndication,
    method_info: OptionalMethod
) -> f32;

#[skyline::from_offset(0x029411C0)] // 0x71029411C0
fn aibattlesimulator_indication_getskill(
    this: &AIBattleSimulatorIndication,
    method_info: OptionalMethod
) -> f32;

#[skyline::from_offset(0x029411E0)] // 0x71029411E0
fn aibattlesimulator_indication_getskillcritical(
    this: &AIBattleSimulatorIndication,
    method_info: OptionalMethod
) -> f32;

#[skyline::from_offset(0x02941200)] //  0x7102941200
fn aibattlesimulator_indication_getprevent(
    this: &AIBattleSimulatorIndication,
    method_info: OptionalMethod
) -> f32;

#[skyline::from_offset(0x02941220)] // 0x7102941220
fn aibattlesimulator_indication_getmiss(
    this: &AIBattleSimulatorIndication,
    method_info: OptionalMethod
) -> f32;

#[skyline::from_offset(0x02941250)] // 0x7102941250
fn aibattlesimulator_indication_setexpectation(
    this: &AIBattleSimulatorIndication,
    value: f32,
    method_info: OptionalMethod
);

#[skyline::from_offset(0x02941260)] // 0x7102941260
fn aibattlesimulator_indication_getisskillkill(
    this: &AIBattleSimulatorIndication,
    method_info: OptionalMethod
) -> bool;


// AIBattleSimulatorChainAttackCandidate functions
#[skyline::from_offset(0x02941100)] // 0x7102941100
fn aibattlesimulator_chainattackcandidate_getside(
    this: &AIBattleSimulatorChainAttackCandidate,
    method_info: OptionalMethod
) -> Option<&'static BattleInfoSide>;