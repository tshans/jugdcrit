use std::ops::Deref;
use unity::il2cpp::class::Il2CppClassData;
use unity::il2cpp::method::OptionalMethod;

use crate::engagelib::battle::BattleCalculator;
use crate::engagelib::battle::BattleInfoSide;
use crate::engagelib::battle::BattleParam;
use crate::engagelib::battle::BattleParamStaticFields;
use crate::engagelib::battle::BattleSceneResult;


// CRITICAL_FACTOR is a constant used to determine how "true damage" skills are affected by
// crits. A value of 2 matches the Jugdral formula's attack doubling and is the default.
// (The vanilla game uses the BattleInfo field value const int CriticalFactor = 3.)
pub const CRITICAL_FACTOR: i32 = 2;


// Critical Formula
// Damage = [Max(2 * Attack - Defense, 0) + Add * CRITICAL_FACTOR] * Scale
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

    // Vanilla formula uses
    // let calc = attack as i32 - defense as i32;
    let calc = 2 * attack as i32 - defense as i32;
    let max_calc = calc.max(0);

    let damage = ((max_calc + CRITICAL_FACTOR * add as i32) as f32 * scale)
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