use std::ops::Deref;
use unity::il2cpp::class::Il2CppClassData;
use unity::il2cpp::method::OptionalMethod;

use crate::engagelib::battle::BattleCalculator;
use crate::engagelib::battle::BattleInfoSide;
use crate::engagelib::battle::BattleParam;
use crate::engagelib::battle::BattleParamStaticFields;
use crate::engagelib::battle::BattleSceneResult;


// Set CRITICAL_FACTOR equal to 1 or 2 for the corresponding Critical Formula Option.
// This is used to apply Skill changes to damage for criticals.

// (The vanilla game uses the BattleInfo field value const int CriticalFactor = 3.)
// pub const CRITICAL_FACTOR: i32 = 1;
pub const CRITICAL_FACTOR: i32 = 2;


// Critical Formula Option #1
// Damage = [Max(2 * Attack - Defense, 0) + Add] * Scale

// fn jugdral_critical(current: &BattleInfoSide) -> i32 {
//     let reverse = match current.get_reverse() {
//         Some(infoside) => infoside,
//         None => panic!("No attack target for jugdral_critical function."),
//     };
//     let current_details = current.get_detail();
//     let reverse_details = reverse.get_detail();

//     let attack_param = current_details
//         .get_attack()
//         .expect("Attack BattleParam should be initialized.");
//     let defense_param = reverse_details
//         .get_defense()
//         .expect("Defense BattleParam should be initialized.");
//     let power_param = current_details
//         .get_simple_power()
//         .expect("SimplePower BattleParam should be initialized.");

//     let statics = BattleParam::class()
//         .get_static_fields::<BattleParamStaticFields>();
//     let min = statics.mins.fields.deref();
//     let max = statics.maxs.fields.deref();

//     let attack = attack_param.get_result(current);
//     let defense = defense_param.get_result(reverse);

//     let add = power_param.fields.add;
//     let scale = power_param.fields.scale;

//     let kind = power_param.get_kind();

//     let calc = CRITICAL_FACTOR * attack as i32 - defense as i32;
//     let max_calc = calc.max(0);
        
//     let damage = ((max_calc as f32 + add) * scale)
//         .clamp(min[kind as usize], max[kind as usize]) as i32;

//     return damage
// }


// Critical Formula Option #2
// Damage = Max{[2 * (Attack + Add) - Defense], 0} * Scale

pub fn jugdral_critical(current: &BattleInfoSide) -> i32 {
    let reverse = match current.get_reverse() {
        Some(infoside) => infoside,
        None => panic!("No attack target for jugdral_critical function."),
    };
    let current_details = current.get_detail();
    let reverse_details = reverse.get_detail();

    let attack_param = current_details
        .get_attack()
        .expect("Attack BattleParam should be initialized.");
    let defense_param = reverse_details
        .get_defense()
        .expect("Defense BattleParam should be initialized.");
    let power_param = current_details
        .get_simple_power()
        .expect("SimplePower BattleParam should be initialized.");

    let statics = BattleParam::class()
        .get_static_fields::<BattleParamStaticFields>();

    let min = statics.mins.fields.deref();
    let max = statics.maxs.fields.deref();

    let attack = attack_param.get_result(current);
    let defense = defense_param.get_result(reverse);

    let add = power_param.fields.add;
    let scale = power_param.fields.scale;

    let kind = power_param.get_kind();
    
    let calc = CRITICAL_FACTOR * (attack as i32 + add as i32) - defense as i32;
    let max_calc = calc.max(0);

    let damage = (max_calc as f32 * scale)
        .clamp(min[kind as usize], max[kind as usize]) as i32;
    
    return damage
}


#[unity::hook("App", "BattleCalculator", "CalcAttackHit")]
pub fn calc_attack_hit_hook(
    this: &BattleCalculator,
    current: &BattleInfoSide,
    reverse: &mut BattleInfoSide,
    critical: &mut i32,
    method_info: OptionalMethod
) -> BattleSceneResult {
    let result = call_original!(
        this,
        current,
        reverse,
        critical,
        method_info
    );

    if result.contains(BattleSceneResult::Critical) {
        reverse.set_damage(current.get_simple_power(true));
    }

    return result
}