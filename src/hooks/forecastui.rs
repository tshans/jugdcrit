use unity::il2cpp::method::OptionalMethod;

use crate::engagelib::battle::BattleInfo;
use crate::engagelib::battle::BattleInfoSideStatus;
use crate::engagelib::battle::BattleSceneList;
use crate::engagelib::help::HelpItemFixedText;
use crate::engagelib::mapbattleinfo::MapBattleInfoWindow;
use crate::engagelib::tmpro::Text;
use crate::engagelib::tmpro::TextMeshProUGUI;

use crate::unitylib::engine::RectTransform;
use crate::unitylib::engine::Transformation;
use crate::unitylib::engine::UEComponent;

// We modify the Pre-Battle-Forecast UI to contain new information on Critical Damage.


// Default positions of the BattleInfoRoot GameObject Hierarchy
// BattleInfoRoot(Clone) (960, 540)
//  -> ShadowBottom (960, 0)            // Shadowed area at bottom of screen?
//  -> BattleInfoL (740, 194)           // Populates with left unit's combat stats
//  -> BattleInfoR (1180, 194)          // Populates with right unit's combat stats
//  -> Command (960, 194)               // Central Attack/Engage command options
//  -> BattleSequence (960, 228)        // Controls combat forecast's central Arrow graphics
//  -> WdwItemListL (360, 240)          // Controls left unit's item selection
//  -> WdwItemListR (1560, 240)         // Controls right unit's item selection
//  -> SupportInfo (960, 540)
//  -> SupportInfo (960, 540)

// BattleInfoL (740, 194)
//  -> FrmName (1140, 114)
//      -> Name (1140, 114)
//          -> UnitName (1010, 114)
//          -> God (1260, 114)
//              -> and (1140, 114)
//              -> GodName (1270, 114)
//  -> FrmNameItem (740, 194)
//      -> Name (740, 224)
//          -> UnitName (490, 224)
//          -> God (490, 224)
//              -> and (490, 224)
//              -> GodName (490, 224)           // In addition to the Item GameObject, the following GameObject is
//          -> Border (740, 200)                // added for single items (?) or enemy only (?), with no KeyHelp GameObjects
//      -> Item (740, 168)                      -> ItemNothing
//          -> KeyHelpY (548, 168)                  -> ItemName
//              -> ArrowL (532, 168)                    -> TMP SubMeshUI [System SDF Material + LiberationSans SDF Atlas]
//              -> Y (572, 168)
//          -> FrmItem (608, 168)
//              -> ItemIcon (608, 168)
//              -> Arrow (626, 156)
//          -> ItemName (644, 168)
//          -> ItemNumber (581, 168) // Variable Location
//          -> KeyHelpX (932, 168)
//              -> X (908, 168)
//              -> ArrowR (948, 168)
//  -> Info (740, 194)
//      -> Wdw (740, 194)
//      -> HP (830, 142)
//          -> HPTitle (854, 142)
//          -> HPValue (990, 142)
//          -> HPGaugeBase (1000, 142)
//              -> Gauge (1004, 142)
//                  -> GaugeAfter (1004, 135)
//                      -> Front (1004, 135)
//                  -> AfterValue (1004, 135)
//                      -> GaugePointer (1004, 133)
//                          -> Value (1012, 135)
//                  -> GaugeAdd (1004, 135)
//                      -> Add (1053.3151, 135) // Variable Location
//                  -> AfterValueHeal (1004, 135)
//                      -> GaugePointer (1004, 133)
//                          -> Value (1452, 135)
//              -> Frm (1180, 142)
//          -> HPStock (1204, 122)
//              -> HPStock0 (1204, 122)
//              -> HPStock1 (1204, 122)
//              -> HPStock2 (1204, 122)
//      -> Status (740, 98)
//          -> Atk (740, 83)
//              -> Border (740, 68)
//              -> Title (602, 83)
//              -> Value (801, 83)
//                  -> Value0 (801, 83)
//                  -> Value1 (801, 83)
//              -> Icon (804, 87)
//                  -> Icon0 (724, 87)
//                  -> Icon1 (764, 87)
//                  -> Icon2 (804, 87)
//                  -> Icon3 (844, 87)
//                  -> Icon4 (884, 87)
//                  -> Icon5 (924, 87)
//          -> Hit (740, 51)
//              -> Border (740, 36)
//              -> Title (602, 51)
//              -> Value (801, 51)
//                  -> Value0 (801, 51)
//                      -> Value (801, 51)
//                      -> & (801, 51)
//                  -> Value1 (801, 51)
//                      -> Value (801, 51)
//                      -> % (801, 51)
//          -> Crit (740, 19)
//              -> Border (740, 4)
//              -> Title (602, 19)
//              -> Value (801, 19)
//                  -> Value0 (801, 19)
//                      -> Value (801, 19)
//                      -> % (801, 19)
//                  -> Value1 (801, 19)
//                      -> Value (801, 19)
//                      -> % (801, 19)


//          -> CrtD (740, 83)                       // New, position listed is the default location of Status/Atk
//              -> Border (740, 68)                 // New
//              -> Title (602, 83)                  // New
//              -> Value (801, 83)                  // New
//                  -> Value0 (801, 83)             // New
//                  -> Value1 (801, 83)             // New

#[unity::hook("App", "MapBattleInfoWindow", "SetBattleInfo")]
pub fn set_battle_info_hook(
    this: &MapBattleInfoWindow,
    info: &BattleInfo,
    scene_list: &BattleSceneList,
    method_info: OptionalMethod,
) {
    call_original!(this, info, scene_list, method_info);

    let left_go = this.get_battle_info_l();
    let left_go_t = left_go.get_transform::<RectTransform>();
    
    let right_go = this.get_battle_info_r();
    let right_go_t = right_go.get_transform::<RectTransform>();

    let current = info.get_offense();
    let status = current.get_status();


    // Update the text to reflect the current situation, assuming the Info/Status/CrtD GameObjects have been created.
    // The "else" clause shouldn't be needed as the GameObjects are initialized when the HelpManager is populated
    // with its entries (i.e., before MapBattleInfoWindow is created) but just in case we add a panic escape...
    if let Some(left_crtd_t) = left_go_t.find_child("Info/Status/CrtD") {
        // Set the Title Text
        match left_crtd_t.find_child("Title") {
            Some(rect) => match rect
                .get_game_object()
                .get_component_name::<TextMeshProUGUI>("TextMeshProUGUI") {
                    Some(text) => text.set_text("CrtD", true),
                    None => panic!("Left CrtD/Title textbox could not be found."),
                },
            None => panic!("Left CrtD/Title transform could not be found."),
        };
        
        // Setting Help Text
        let help_text_l = match left_crtd_t
            .get_game_object()
            .get_component_name::<HelpItemFixedText>("HelpItemFixedText") {
                Some(help_text) => help_text,
                None => panic!("Left CrtD HelpItemFixedText could not be found."),
            };
        help_text_l.set_data("MID_H_COMBAT_INFO_CrtD");

        // Set the primary CrtD value Text
        let left_crtd_value: String;
        if status.intersects(BattleInfoSideStatus::MaskNoAttack) {
            left_crtd_value = "--".to_string();
        } else {
            left_crtd_value = current.get_simple_power(true).to_string();
        }
        match left_crtd_t.find_child("Value/Value0") {
            Some(rect) => match rect
                .get_game_object()
                .get_component_name::<TextMeshProUGUI>("TextMeshProUGUI") {
                    Some(text) => text.set_text(left_crtd_value, true),
                    None => panic!("Left CrtD/Value/Value0 textbox could not be found."),
                },
            None => panic!("Left CrtD/Value/Value0 transform could not be found."),
        };

        // Set the chain CrtD value Text and match its active state with the standard Atk chain GameObject's state.
        let left_is_chain = match left_go_t.find_child("Info/Status/Atk/Value/Value1") {
            Some(rect) => rect.get_game_object().get_active(),
            None => panic!("Left Atk/Value/Value1 transform could not be found."),
        };
        let left_chain_crtd = match left_crtd_t.find_child("Value/Value1") {
            Some(rect) => rect,
            None => panic!("Left CrtD/Value/Value1 transform could not be found."),
        };

        // ChainAttacks cannot crit in vanilla Engage, so we set this text to the default of "--". Mods with
        // ChainAttacks that can crit should modify this code so their damage values are correctly printed here.
        match left_chain_crtd
            .get_game_object()
            .get_component_name::<TextMeshProUGUI>("TextMeshProUGUI") {
                Some(text) => text.set_text("--", true),
                None => panic!("Left CrtD/Value/Value1 textbox could not be found."),
            };
        if left_is_chain {
            left_chain_crtd.get_game_object().set_active(true);
        } else {
            left_chain_crtd.get_game_object().set_active(false);
        }


        // Repeat for the right side
        let right_crtd_t = match right_go_t.find_child("Info/Status/CrtD") {
            Some(rect) => rect,
            None => panic!("BattleInfoR/Info/Status/CrtD transform could not be found."),
        };

        // Set the Title Text
        match right_crtd_t.find_child("Title") {
            Some(rect) => match rect
                .get_game_object()
                .get_component_name::<TextMeshProUGUI>("TextMeshProUGUI") {
                    Some(text) => text.set_text("CrtD", true),
                    None => panic!("Right CrtD/Title textbox could not be found."),
                },
            None => panic!("Right CrtD/Title transform could not be found."),
        };
        
        // Setting Help Text
        let help_text_r = match right_crtd_t
            .get_game_object()
            .get_component_name::<HelpItemFixedText>("HelpItemFixedText") {
                Some(help_text) => help_text,
                None => panic!("Right CrtD HelpItemFixedText could not be found."),
            };
        help_text_r.set_data("MID_H_COMBAT_INFO_CrtD");

        // Set the primary CrtD value Text
        let right_crtd_value: String;
        if status.intersects(BattleInfoSideStatus::MaskNoAttack) {
            right_crtd_value = "--".to_string();
        } else if let Some(reverse) = current.get_reverse() {
            right_crtd_value = reverse.get_simple_power(true).to_string();
        } else {
            right_crtd_value = "--".to_string();
        }
        match right_crtd_t.find_child("Value/Value0") {
            Some(rect) => match rect
                .get_game_object()
                .get_component_name::<TextMeshProUGUI>("TextMeshProUGUI") {
                    Some(text) => text.set_text(right_crtd_value, true),
                    None => panic!("Right CrtD/Value/Value0 textbox could not be found."),
                },
            None => panic!("Right CrtD/Value/Value0 transform could not be found."),
        };

        // Set the chain CrtD value Text and match its active state with the standard Atk chain GameObject's state.
        let right_is_chain = match right_go_t.find_child("Info/Status/Atk/Value/Value1") {
            Some(rect) => rect.get_game_object().get_active(),
            None => panic!("Right Atk/Value/Value1 transform could not be found."),
        };
        let right_chain_crtd = match right_crtd_t.find_child("Value/Value1") {
            Some(rect) => rect,
            None => panic!("Right CrtD/Value/Value1 transform could not be found."),
        };

        // ChainAttacks cannot crit in vanilla Engage, so we set this text to the default of "--". Mods with
        // ChainAttacks that can crit should modify this code so their damage values are correctly printed here.
        match right_chain_crtd
            .get_game_object()
            .get_component_name::<TextMeshProUGUI>("TextMeshProUGUI") {
                Some(text) => text.set_text("--", true),
                None => panic!("Right CrtD/Value/Value1 textbox could not be found."),
            };
        if right_is_chain {
            right_chain_crtd.get_game_object().set_active(true);
        } else {
            right_chain_crtd.get_game_object().set_active(false);
        }
    } else {
        panic!("BattleInfoRoot(Clone)/BattleInfoL/Info/Status/CrtD GameObject could not be found.")
    }

    // // Old version, adjusting the text of BtlAtk. Kept for posterity.
    // let window_slice = this
    //     .fields
    //     .singles
    //     .fields
    //     .deref();

    // for i in window_slice {
    //     let param_setter = i.fields.map_battle_info_param_setter;
    //     let current = param_setter.fields.side;
    //     let status = current.fields.status.fields.sup.sup.value;

    //     // The window is left unchanged if the action is not an attack.
    //     if status.intersects(BattleInfoSideStatus::MaskNoAttack) {
    //         continue
    //     } else {
    //         let btl_atk_text = param_setter.fields.btl_atk.get_text().to_string();
    //         let crit_string = current.get_simple_power(true).to_string();
    //         let space = String::from(" ");
    //         let left = String::from("(");
    //         let right = String::from(")");
    //         let text_slice = &[left, crit_string, right, space, btl_atk_text];
    //         let new_text = text_slice.concat();

    //         InfoUtil::try_set_text_ugui_string(param_setter.fields.btl_atk, new_text);

    //         continue
    //     }
    // }

    // All WindowSingles have been updated.
    return
}