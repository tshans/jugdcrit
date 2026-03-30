use std::ops::{Deref, Shr};
use unity::il2cpp::method::OptionalMethod;

use crate::engagelib::battle::BattleInfoSide;
use crate::engagelib::battle::BattleSideType;
use crate::engagelib::battle::BattleMath;
use crate::engagelib::simulator::AIBattleSimulator;

use crate::hooks::combatdetails::CRITICAL_FACTOR;

// We modify the game's AI behavioural calculations to account for non-standard crits.

#[unity::hook("App", "AIBattleSimulator", "CalculateIndication")]
pub fn calculate_indication_hook(
    this: &AIBattleSimulator,
    side: BattleSideType,
    method_info: OptionalMethod
) {
    // The original function populates all of the fields we need, so it is called.
    call_original!(this, side, method_info);

    let indications = this
        .fields
        .a_indication
        .fields
        .deref();
    let indication = indications[side as usize];

    let power = indication.get_power();
    let hit = indication.get_hit();
    let critical = indication.get_critical();
    let prevent = indication.get_prevent();

    let current = match this
        .fields
        .sup
        .battle_info
        .get_side_type(side) {
            Some(infoside) => infoside,
            None => panic!("BattleSideType failed to identify a corresponding BattleInfoSide."),
        };
    let critical_power = current.get_simple_power(true);

    // The simple expected value of an attack can now be calculated.
    let mut expectation = hit * power as f32
        + prevent * hit * power.shr(1) as f32
        + critical * critical_power as f32
        + prevent * critical * critical_power.shr(1) as f32;

    // Next, the expected value calculation is adjusted for a skill activation. The Ghidra
    // decomp code appears to set skill, skill_power, and skill_critical equal to zero but
    // the calculations have been included (and appropriately modified) for completeness.
    let skill_power = indication.get_skill_power();
    let skill = indication.get_skill();
    let skill_critical = indication.get_skill_critical();
    if skill_power > 0 {
        // The Ghidra setup for this if-block suggests that skill_power is a flat modifier,
        // so skill_critical_power is adjusted accordingly. (skill_power is treated as a
        // bonus to Attack.)
        let skill_critical_power = CRITICAL_FACTOR * skill_power;

        // Skill_kill means protection effects are ignored. Negation operator (!) is used to
        // obtain the correct outcome since (true as i32) = 1 and (false as i32) = 0 in Rust.
        let skill_kill = !(indication.get_is_skill_kill()) as i32 as f32;
        let skill_correction = skill * skill_power as f32
            + skill_kill * (prevent * skill * skill_power.shr(1) as f32)
            + skill_critical * skill_critical_power as f32
            + skill_kill * (prevent * skill_critical * skill_critical_power.shr(1) as f32);

        expectation = expectation + skill_correction;

        // Finally, the expected value is multiplied by the number of attacks.
        let current_detail = current.get_detail();
        let action_count = current_detail.get_action_count() as f32;
        
        expectation = expectation * action_count;

        indication.set_expectation(expectation);
    } else {
        let current_detail = current.get_detail();
        let action_count = current_detail.get_action_count() as f32;
        expectation = expectation * action_count;

        indication.set_expectation(expectation);
    }

    return
}

#[unity::hook("App", "AIBattleSimulator", "CalculateChainAttackIndication")]
pub fn calculate_chain_indication_hook(
    this: &mut AIBattleSimulator,
    current: &BattleInfoSide,
    reverse: &BattleInfoSide,
    method_info: OptionalMethod,
) {
    // The original function populates all of the fields we need, so it is called.
    call_original!(this, current, reverse, method_info);

    if this.fields.chain_attack_expectation != 0.0 {
        this.fields.chain_attack_expectation = 0.0;
    }

    let chain_slice = this
        .fields
        .a_chain_attack_candidate
        .fields
        .deref();

    for i in chain_slice {
        let infoside = match i.get_side() {
                Some(side_chain) => side_chain,
                None => continue,
            };
        let details = infoside.get_detail();

        let power_param = details
            .get_simple_power()
            .expect("SimplePower BattleParam should be initialized.");
        let hit_param = details
            .get_simple_hit()
            .expect("SimpleHit BattleParam should be initialized.");
        let critical_param = details
            .get_simple_critical()
            .expect("SimpleCritical BattleParam should be initialized.");

        let power = power_param.get_result(infoside);
        let hit = hit_param.get_result(infoside);
        let true_hit = BattleMath::get_hit_real_ratio(hit as i32);

        let critical_ratio = critical_param.get_result(infoside);
        let critical = critical_ratio / 100.0;
        let critical_damage = infoside.get_simple_power(true);
        
        let chain_expectation_correction = true_hit * power
            + true_hit * critical * critical_damage as f32;

        this.fields.chain_attack_expectation += chain_expectation_correction;
    }

    return
}

// Full reproduction of original function but with critical damage math changed.
#[unity::hook("App", "AIBattleSimulator", "CalculateKillProbabilityWithoutInterference2")]
pub fn calculate_kill_probability_2_hook(
    this: &AIBattleSimulator,
    side: BattleSideType,
    count: i32,
    times: i32,
    now_probability: f32,
    rest_hp: i32,
    _method_info: OptionalMethod
) {
    let indication_array = this
        .fields
        .a_indication
        .fields
        .deref();
    let indication = indication_array[side as usize];
    let miss = indication.get_miss();

    this.calculate_kill_probability_3(
        side,
        count,
        times,
        now_probability * miss,
        rest_hp,
        0
    );

    let hit = now_probability * indication.get_hit();
    let prevent = hit * indication.get_prevent();
    let power = indication.get_power();

    this.calculate_kill_probability_3(
        side,
        count,
        times,
        hit - prevent,
        rest_hp,
        power
    );
    this.calculate_kill_probability_3(
        side,
        count,
        times,
        prevent,
        rest_hp,
        power.shr(1)
    );

    let critical = now_probability * indication.get_critical();
    let critical_prevent = critical * indication.get_prevent();
    
    let current = match this
        .fields
        .sup
        .battle_info
        .get_side_type(side) {
            Some(infoside) => infoside,
            None => panic!("BattleSideType failed to identify a corresponding BattleInfoSide."),
        };
    let critical_power = current.get_simple_power(true);

    this.calculate_kill_probability_3(
        side,
        count,
        times,
        critical - critical_prevent,
        rest_hp,
        critical_power
    );
    this.calculate_kill_probability_3(
        side,
        count,
        times,
        critical_prevent,
        rest_hp,
        critical_power.shr(1)
    );

    let skill_power = indication.get_skill_power();
    if skill_power > 0 {
        let skill = now_probability * indication.get_skill();
        if indication.get_is_skill_kill() == false {
            let skill_prevent = skill * indication.get_prevent();
            
            this.calculate_kill_probability_3(
                side,
                count,
                times,
                skill - skill_prevent,
                rest_hp,
                skill_power
            );
            this.calculate_kill_probability_3(
                side,
                count,
                times,
                skill_prevent,
                rest_hp,
                skill_power.shr(1)
            );
        } else {
            this.calculate_kill_probability_3(
                side,
                count,
                times,
                skill,
                rest_hp,
                skill_power
            );
        }

        if skill_power > 0 {
            let skill_critical = now_probability * indication.get_skill_critical();
            let skill_critical_power = CRITICAL_FACTOR * skill_power;
            let skill_critical_prevent = skill_critical * indication.get_prevent();

            if indication.get_is_skill_kill() == false {
                this.calculate_kill_probability_3(
                    side,
                    count,
                    times,
                    skill_critical - skill_critical_prevent,
                    rest_hp,
                    skill_critical_power
                );
            }

            this.calculate_kill_probability_3(
                side,
                count,
                times,
                skill_critical_prevent,
                rest_hp,
                skill_critical_power.shr(1)
            );

            return
        }
    }
    
    return
}