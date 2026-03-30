use std::ops::Deref;
use unity::il2cpp::method::OptionalMethod;

use crate::engagelib::battle::BattleCalculator;
use crate::engagelib::battle::BattleInfoSideStatus;
use crate::engagelib::battle::BattleSideType;
use crate::engagelib::controller::CombatGaugeController;
use crate::engagelib::tmpro::Text;
use crate::engagelib::tmpro::TextMeshProUGUI;

use crate::unitylib::engine::RectTransform;
use crate::unitylib::engine::Transformation;
use crate::unitylib::engine::UEComponent;
use crate::unitylib::engine::UEObject;
use crate::unitylib::engine::Vector3;
use crate::unitylib::engine::Vector3Int;

// We modify the In-Combat UI to contain new information on Critical Damage.

// The hierarchy of param_root/"ParameterRoot".
// - Possesses 6 children by default
// - The positon values (x, y) of the corresponding transform are in global coordinates for
//   the left side CombatGaugeController
//      - For the right side, the correct x coordinates are given by 1920 - x (left side value).
//      - The y coordinates are unchanged for the right side.
// - We add one child named CrtDTitle (component Image) with child Title (component TextMeshProUGUI)
// - We add one child to each of Parameter.Main and Parameter.Chain
//      - CrtDValue (component RectTransform) with 2 children:
//          - Wdw (component Image)
//          - Value (component TextMeshProUGUI)

// ParameterRoot (960, 540)
//  -> HP (900, 206)
//      -> Wdw (900, 206)
//      -> HPGauge (900, 206)
//          -> HPGaugeBase (900, 206)
//          -> HPGaugeDamage (900, 206)
//          -> HPGaugeMask (900, 206)
//              -> HPGaugeFront (900, 206)
//          -> Frm (898, 206)
//      -> HPValue (900, 206)
//      -> HPStock (652, 186)
//          -> HPStock0 (652, 186)
//          -> HPStock1 (652, 186)
//          -> HPStock2 (652, 186)
//  -> Parameter (900, 166)
//      -> Main (900, 166)
//          -> AtkValue (900, 166)
//              -> Value5 (900, 166)
//                  -> Wdw (875, 166)
//                  -> Value (896, 166)
//              -> Value4 (900, 166)
//                  -> Wdw (900, 166)
//                  -> Value (896, 166)
//              -> Value3 (900, 166)
//                  -> Wdw (900, 166)
//                  -> Value (896, 166)
//              -> Value2 (900, 166)
//                  -> Wdw  (900, 166)
//                  -> Value (896, 166)
//              -> Value1 (900, 166)
//                  -> Wdw (900, 166)
//                  -> Value (896, 166)
//              -> Value0 (900, 166)
//                  -> Wdw (900, 166)
//                  -> Value (896, 166)
//          -> HitValue (900, 166)
//              -> Wdw (875, 166)
//              -> Value (900, 166)
//              -> % (900, 166)
//          -> CritValue (900, 166)
//              -> Wdw (875, 166)
//              -> Value (900, 166)
//              -> % (900, 166)

//          -> CrtDValue (900, 166)                         // New
//              -> Wdw (875, 166)                           // New
//              -> Value (900, 166)                         // New

//      -> Chain (900, 166)
//          -> AtkValue (900, 166)
//              -> Wdw (900, 166)
//              -> Value (896, 166)
//          -> HitValue (900, 166)
//              -> Wdw (900, 166)
//              -> Value (900, 166)
//              -> % (900, 166)
//          -> CritValue (900, 166)
//              -> Wdw (900, 166)
//              -> Value (900, 166)
//              -> % (900, 166)

//          -> CrtDValue (900, 166)                         // New
//              -> Wdw (875, 166)                           // New
//              -> Value (900, 166)                         // New

//  -> HPTitle (960, 206)
//      -> Title (960, 206)
//  -> AtkTitle (960, 166)
//      -> Title (960, 148)
//  -> HitTitle (960, 126)
//      -> Title (960, 108)
//  -> CritTitle (960, 86)
//      -> Title (960, 68)

//  -> CrtDTitle (960, 46)                                  // New
//      -> Title (960, 28)                                  // New



#[unity::hook("", "CombatGaugeController", "Setup")]
pub fn combat_gauge_controller_setup_hook(
    this: &mut CombatGaugeController,
    side_type: BattleSideType,
    calculator: &BattleCalculator,
    method_info: OptionalMethod
) {
    call_original!(this, side_type, calculator, method_info);

    // Confirm that the current side_type is one of the primaries in a combat animation scene.
    if (side_type != BattleSideType::Offense) && (side_type != BattleSideType::Defense) {
        return
    }

    let center_params = this.fields.param_root;

    // If center_param is not active, return as the standard HUD will not appear.
    if center_params.get_active() == false {
        return
    }

    // If param_root's position is (960.0, 540.0, 0,0), then standard combat animations are playing and the combat
    // UI needs to be shifted up by 36.0 pixels to accomodate the CriticalDamage UI elements. If the position is
    // (960.0, 640.0, 0.0), then a map animation is playing and none of the vanilla UI elements need to be moved.
    // "Map animation" in this instance refers to the on-map combat animations, e.g., setting (support) Animations
    // to off in the Settings menu.
    //
    // The game creates/destroys the CombatGaugeController GameObjects during each scene transition into/out of combat.
    // The on-map combat animations create and re-use a single set of CGC GameObjects until a scene transition occurs.
    let param_transform = center_params.get_transform::<RectTransform>();
    let shift_ui = Vector3::new(0.0, 36.0, 0.0);
    let shift = Vector3::new(0.0, -36.0, 0.0);
    let shift_title = Vector3::new(0.0, -40.0, 0.0);

    // The Vector3::to_int() method rounds each f32 value to the nearest integer before casting as i32, which should
    // prevent floating point-derived errors, e.g., Rust's primitive type casting by default rounds towards zero, so
    // it interprets 959.9994 as i32 == 959 and the below code would accidentally skip a needed UI position shift.
    //
    // It might be faster to check if the transform's position is within some error of the exact location, e.g.,
    // for some epsilon << 1.0, check the truth of 960 - epsilon < param_transform.get_position().x < 960 + epsilon,
    // repeating for the y and z coordinates.
    if param_transform.get_position().to_int() == Vector3Int::new(960, 540, 0) {
        let parent_param_transform = match param_transform.get_parent() {
            Some(transform) => transform,
            None => panic!("ParameterRoot's parent transform was not found."),
        };

        let param_sibling_count = parent_param_transform.get_child_count();
        for i in 0..param_sibling_count {
            // The .unwrap() method should never panic as we are iterating over 0..ChildCount.
            let sibling_transform = parent_param_transform.get_child(i).unwrap();
            sibling_transform.translate_vec3(shift_ui);
        }
    }

    // Check if the new game objects have been created already. If so, modify them appropriately (this should only occur
    // for map animations). If not, initialize them and then modify them appropriately.
    if let Some(transform) = param_transform.find_child("CrtDTitle/Title") {
        // Ensure CrtDTitle/Title displays the correct text "CrtD".
        match transform
            .get_game_object()
            .get_component_name::<TextMeshProUGUI>("TextMeshProUGUI") {
                Some(text) => text.set_text("CrtD", true),
                None => panic!("ParameterRoot/CrtDTitle/Title textbox could not be found."),
            }

        // Change the Main.CrtDValue.Value text to reflect the current battle.
        let main_crtd_value_transform = match param_transform
            .find_child("Parameter/Main/CrtDValue/Value") {
                Some(rect) => rect,
                None => panic!("ParameterRoot/Parameter/Main/CrtDValue/Value transform could not be found."),
            };
        let main_crtd_value_text = match main_crtd_value_transform
            .get_game_object()
            .get_component_name::<TextMeshProUGUI>("TextMeshProUGUI") {
                Some(text) => text,
                None => panic!("ParameterRoot/Parameter/Main/CrtDValue/Value textbox could not be found."),
            };
        
        // Set Critical Damage text value using JugdralCrits
        let current = match calculator.get_infoside_type(side_type) {
            Some(infoside) => infoside,
            None => panic!("BattleSideType failed to identify a BattleInfoSide."),
        };
        let status = current.get_status();

        // If the Combat contains no attacks or the only BtlAtk text field is "--", we set the CrtD value to "--".
        // The .unwrap() method won't panic here because BtlAtk's list is always fully populated with 6 entries.
        // (By default, the unused entries are left deactivated and only activated when they need to be displayed.)
        if status.intersects(BattleInfoSideStatus::MaskNoAttack) || 
            (this.fields.btl_atk.fields.deref().get(5).unwrap().fields.text.get_text().to_string() == "--" ) {
            main_crtd_value_text.set_text("--", true);
        } else {
            // .get_simple_power(true) grabs JugdralCrits value.
            main_crtd_value_text.set_text(current.get_simple_power(true).to_string(), true);
        }

        // Update Chain.CrtDValue.Value text to reflect the current battle, assuming the ChainRoot GameObject is active.
        if this.fields.chain_root.get_active() {
            let chain_crtd_value_transform = match param_transform.find_child("Parameter/Chain/CrtDValue/Value") {
                Some(rect) => rect,
                None => panic!("ParameterRoot/Parameter/Chain/CrtDValue/Value transform could not be found."),
            };
            let chain_crtd_value_text = match chain_crtd_value_transform
                .get_game_object()
                .get_component_name::<TextMeshProUGUI>("TextMeshProUGUI") {
                    Some(text) => text,
                    None => panic!("ParameterRoot/Parameter/Chain/CrtDValue/Value textbox could not be found."),
                };
        
            // ChainAttacks cannot crit in vanilla Engage, so we set this text to the default of "--". Mods with
            // ChainAttacks that can crit should modify this code so their damage values are correctly printed here.
            chain_crtd_value_text.set_text("--", true);
        }
    } else {
        // Create the needed GameObject hierarchy.
        // Identify the pre-existing transforms which we will set as parents.
        let main_transform = match param_transform.find_child("Parameter/Main") {
            Some(transform) => transform,
            None => panic!("ParameterRoot/Parameter/Main transform could not be found."),
        };

        let chain_transform = match param_transform.find_child("Parameter/Chain") {
            Some(transform) => transform,
            None => panic!("ParameterRoot/Parameter/Chain transform could not be found."),
        };
    
        // Create copies of the three game objects (and their attached components + children) we need. First, we
        // create the CrtDTitle and CrtDTitle/Title objects by copying the CritTitle object, setting the name of
        // the CrtDTitle so it can easily be accessed later if needed.
        let crit_title_rect = match param_transform.find_child("CritTitle") {
            Some(transform) => transform,
            None => panic!("ParameterRoot/CritTitle transform could not be found."),
        };

        let crtd_title_go = crit_title_rect.get_game_object().copy();
        crtd_title_go.set_name("CrtDTitle");
        let crtd_title_rect = crtd_title_go.get_transform::<RectTransform>();

        // For some reason, the CrtDTitle GameObject isn't placed at CritTitle's position by default. The following code
        // is some vector algebra to shift CrtDTitle's position to CritTitle's position.
        let crit_title_p = crit_title_rect.get_position();
        let crtd_title_p = crtd_title_rect.get_position();
        crtd_title_rect.translate_vec3(crit_title_p.add(crtd_title_p.scale(-1.0)));

        let crtd_title_title_rect = match crtd_title_rect.find_child("Title") {
            Some(rect) => rect,
            None => panic!("ParameterRoot/CrtDTitle/Title transform could not be found."),
        };

        let crtd_title_title_text = match crtd_title_title_rect
            .get_game_object()
            .get_component_name::<TextMeshProUGUI>("TextMeshProUGUI") {
                Some(text) => text,
                None => panic!("ParameterRoot/CrtDTitle/Title textbox could not be found."),
            };
        
        crtd_title_title_text.set_text("CrtD", true);
        crtd_title_rect.translate_vec3(shift_title); // Shift copied objects down.
        crtd_title_rect.set_parent(param_transform);


        // Then, we copy Parameter/Main/CritValue (and its children /Wdw, /Value, and /%) to create
        // CrtDValue, /Wdw, /Value, and /%.
        let main_cvalue_transform = match main_transform.find_child("CritValue") {
            Some(rect) => rect,
            None => panic!("ParameterRoot/Parameter/Main/CritValue transform could not be found."),
        };

        let main_crtdvalue_go = main_cvalue_transform.get_game_object().copy();
        main_crtdvalue_go.set_name("CrtDValue");
        let main_crtdvalue_transform = main_crtdvalue_go.get_transform::<RectTransform>();

        // Ensure CrtDValue's starting position is equal to CritValue's position. (Some initial tests showed that this
        // may not be necessary but, just to be safe, we perform the same vector trick used for CrtDTitle.)
        let main_cvalue_transform_p = main_cvalue_transform.get_position();
        let main_crtdvalue_transform_p = main_crtdvalue_transform.get_position();
        main_crtdvalue_transform.translate_vec3(main_cvalue_transform_p.add(main_crtdvalue_transform_p.scale(-1.0)));

        // The /Wdw object is not changed. We destroy the /% game object as it is not needed. Thankfully, the game
        // automatically adjusts the position of /Value relative to the other GameObjects, so no horizontal aligning
        // of the text is needed.
        match main_crtdvalue_transform.find_child("%") {
            Some(rect) => rect.get_game_object().destroy(),
            None => panic!("ParameterRoot/Parameter/Main/CrtDValue/% transform could not be found."),
        };

        let main_crtdvalue_value_transform = match main_crtdvalue_transform.find_child("Value") {
            Some(rect) => rect,
            None => panic!("ParameterRoot/Parameter/Main/CrtDValue/Value transform could not be found."),
        };
        let main_crtdvalue_value_text = match main_crtdvalue_value_transform
            .get_game_object()
            .get_component_name::<TextMeshProUGUI>("TextMeshProUGUI") {
                Some(text) => text,
                None => panic!("ParameterRoot/Parameter/Main/CrtDValue/Value textbox could not be found."),
            };
        
        // Set Critical Damage text value using JugdralCrits
        let current = match calculator.get_infoside_type(side_type) {
            Some(infoside) => infoside,
            None => panic!("BattleSideType failed to identify a corresponding BattleInfoSide."),
        };
        let status = current.get_status();

        // If the combat contains no attacks or the only BtlAtk text field is "--", we set the CrtD value to "--".
        // The .unwrap() method won't panic here because BtlAtk's List is always fully populated with 6 entries.
        if status.intersects(BattleInfoSideStatus::MaskNoAttack) || 
            (this.fields.btl_atk.fields.deref().get(5).unwrap().fields.text.get_text().to_string() == "--" ) {
            main_crtdvalue_value_text.set_text("--", true);
        } else {
            // .get_simple_power(true) grabs JugdralCrits value.
            main_crtdvalue_value_text.set_text(current.get_simple_power(true).to_string(), true);
        }

        main_crtdvalue_transform.translate_vec3(shift); // Shift copied objects down.
        main_crtdvalue_transform.set_parent(main_transform);

        // Finally, we copy Parameter/Chain/CritValue (and its children /Wdw, /Value, and /%) to create CrtDValue,
        // /Wdw, /Value, and /%. We initialize these GameObjects even if the current combat doesn't have ChainAttacks.
        // This is to ensure that the "if" component of this "if-else" statement won't panic if the first map animation
        // lacks ChainAttacks and a later map animation tries to modify the ChainAttack text.
        let chain_cvalue_transform = match chain_transform.find_child("CritValue") {
            Some(rect) => rect,
            None => panic!("ParameterRoot/Parameter/Chain/CritValue transform could not be found."),
        };

        let chain_crtdvalue_go = chain_cvalue_transform.get_game_object().copy();
        chain_crtdvalue_go.set_name("CrtDValue");
        let chain_crtdvalue_transform = chain_crtdvalue_go.get_transform::<RectTransform>();
        
        // Ensure CrtDValue's starting position is equal to CritValue's position. (Some initial tests showed that this
        // may not be necessary but, just to be safe, we perform the same vector trick used for CrtDTitle.)
        let chain_cvalue_transform_p = chain_cvalue_transform.get_position();
        let chain_crtdvalue_transform_p = chain_crtdvalue_transform.get_position();
        chain_crtdvalue_transform.translate_vec3(chain_cvalue_transform_p.add(chain_crtdvalue_transform_p.scale(-1.0)));

        // The /Wdw object is not changed. We destroy the /% game object as it is not needed. As before, the game
        // automatically adjusts the positon of /Value, so no horizontal alignment changes are required.
        let chain_crtdvalue_percent_transform = match chain_crtdvalue_transform.find_child("%") {
            Some(rect) => rect,
            None => panic!("ParameterRoot/Parameter/Chain/CrtDValue/% transform could not be found."),
        };
        chain_crtdvalue_percent_transform.get_game_object().destroy();

        // Set the Value text.
        let chain_crtdvalue_value_transform = match chain_crtdvalue_transform.find_child("Value") {
            Some(rect) => rect,
            None => panic!("ParameterRoot/Parameter/Chain/CrtDValue/Value transform could not be found."),
        };
        let chain_crtdvalue_value_text = match chain_crtdvalue_value_transform
            .get_game_object()
            .get_component_name::<TextMeshProUGUI>("TextMeshProUGUI") {
                Some(text) => text,
                None => panic!("ParameterRoot/Parameter/Chain/CrtDValue/Value textbox could not be found."),
            };
        
        // ChainAttacks cannot crit in vanilla Engage, so we set this text to the default of "--". Mods with
        // ChainAttacks that can crit should modify this code so their damage values are correctly printed here.
        chain_crtdvalue_value_text.set_text("--", true);

        chain_crtdvalue_transform.translate_vec3(shift); // Shift copied objects down.
        chain_crtdvalue_transform.set_parent(chain_transform);
    }

    // // Old version which co-opts a BtlAtk value to display information. Kept for posterity.
    // let btl_atk_slice = this
    //     .fields
    //     .btl_atk
    //     .fields
    //     .deref();

    // // We find the entry to be modified by iterating over the slice in reverse.
    // for i in btl_atk_slice.iter().rev() {
    //     let text = i.fields.text.get_text().to_string();

    //     // The default text value is "99+99", so the first instance of this value will
    //     // be replaced by our function. The second condition is to ensure, against all
    //     // odds, that this is not an actual attack of "99+99".
    //     if (text == String::from("99+99")) && ((i.fields.root).get_active() == false) {
    //         let battleinfosides = calculator
    //             .fields
    //             .info
    //             .fields
    //             .sides
    //             .fields
    //             .sup
    //             .array
    //             .fields
    //             .deref();

    //         let current = battleinfosides[side_type as usize];
    //         let status = current
    //             .fields
    //             .status
    //             .fields
    //             .sup
    //             .sup
    //             .value;

    //         if status.intersects(BattleInfoSideStatus::MaskNoAttack) {
    //             return
    //         } else {
    //             // Calculate and write the critical damage text for the UI.
    //             let jugdral_dmg = current.get_simple_power(true).to_string();
    //             let left = String::from("(");
    //             let right = String::from(")");
    //             let text_slice = &[left, jugdral_dmg, right];
    //             let new_text = text_slice.concat();

    //             // The new text game element is activated so it appears in the UI,
    //             // then the new text is set over the old.
    //             InfoUtil::try_set_active_gameobject(i.fields.root, true);
    //             InfoUtil::try_set_text_ugui_string(i.fields.text, new_text);

    //             return
    //         }
    //     } else if text == String::from("--") {
    //         // No attacks or negated attack, continue to provide critical information.
    //         continue
    //     } else {
    //         // Standard attack, continue the loop.
    //         continue
    //     }
    // }
    // If we've reached this far, then all 6 entries are filled by attacks, so we just
    // return the function without changing anything. Some sacrficies must be made...
    // before figuring out how to add new game elements.


    return
}