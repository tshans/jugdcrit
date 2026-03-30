use std::ops::Deref;

use unity::il2cpp::class::Il2CppClassData;
use unity::il2cpp::method::OptionalMethod;
use unity::il2cpp::object::Array;

use crate::engagelib::help::HelpItemBase;
use crate::engagelib::help::HelpItemFixedText;
use crate::engagelib::help::HelpItemList;
use crate::engagelib::help::SituationType;
use crate::engagelib::tmpro::Text;
use crate::engagelib::tmpro::TextMeshProUGUI;

use crate::unitylib::engine::RectTransform;
use crate::unitylib::engine::Transformation;
use crate::unitylib::engine::UEComponent;
use crate::unitylib::engine::UEObject;
use crate::unitylib::engine::Vector2;
use crate::unitylib::engine::Vector3;
use crate::unitylib::engine::Vector3Int;


// We intercept each HelpItemList before it's added to the HelpManager. If the correct situation
// applies, we create GameObjects to hold Critical Damage UI information and add the corresponding
// HelpItemBase data to the intercepted HelpItemList before passing it along to the HelpManager.
#[unity::hook("App", "HelpManager", "Add")]
pub fn help_manager_add_hook(
    list: &mut HelpItemList,
    method_info: OptionalMethod,
) {
    // We filter out unchanged help text, as the new UI changes only apply to BattleInfo situations.
    if list.get_situation_type() == SituationType::BattleInfo {
        // We check the intercepted HelpItemList for the new UI elements. If they already exist in
        // the list (x2), we pass the list to the original function with no changes.
        //
        // If the list does not contain the two new UI elements, we grab the index of an Atk GameObject
        // for later use. If no Atk GameObject exists in the list, we pass the list to the original
        // function with no changes.
        let mut count = 0;
        let mut index = -1;
        let mut atk_index: i32 = -1;
        let hibs = list.fields.list.fields.deref();
        for i in hibs {
            index += 1;
            let name = match i {
                Some(hib) => hib.get_game_object().get_name().unwrap().to_string(),
                None => continue, // If a HelpItemBase entry is NULL, we continue the for-loop.
            };

            if name == String::from("CrtD") {
                count += 1;
            } else if name == String::from("Atk") {
                // While atk_index will likely be overwritten because there should be 2 "Atk" GameObjects
                // (left and right), we only need one of them to find the BattleInfoRoot(Clone) GameObject.
                atk_index = index;
            }
        }

        if count == 2 {
            call_original!(list, method_info)
        } else if atk_index != -1 {
            let root_transform = hibs
                .get(atk_index as usize)
                .unwrap()
                .unwrap()
                .get_game_object()
                .get_transform::<RectTransform>()
                .get_root(); // Framework(Clone)

            // Here, we ensure that Framework(Clone)/GameUI/BattleInfoRoot(Clone)/BattleInfoRoot(Clone) exists.
            // If it does not exist, then the HelpItemList is for a UI configuration which doesn't need to be
            // modified with Critical Damage elements and we pass the list to the original function with no
            // changes.
            let battle_info_root = match root_transform
                .find_child("GameUI/BattleInfoRoot(Clone)/BattleInfoRoot(Clone)") {
                    Some(rect) => rect,
                    None => return call_original!(list, method_info),
                };

            // We now construct the CrtD GameObject hierarchy.
            let left_go_t = match battle_info_root.find("BattleInfoL") {
                Some(rect) => rect,
                None => panic!("BattleInfoL GameObject could not be found."),
            };

            let right_go_t = match battle_info_root.find("BattleInfoR") {
                Some(rect) => rect,
                None => panic!("BattleInfoR GameObject could not be found."),
            };

            // Shift the sides of the UI up to accomodate the new data. If the UI has already been moved, do nothing.
            let shift_ui = Vector3::new(0.0, 32.0, 0.0);
            let shift = Vector3::new(0.0, -96.0, 0.0);
            let sd_add = Vector2::new(0.0, 32.0); // SizeDelta, rectangular pixel size of a RectTransform

            if left_go_t.get_position().to_int() == Vector3Int::new(740, 194, 0) {
                match battle_info_root.find_child("BattleInfoL") {
                    Some(rect) => rect.translate_vec3(shift_ui),
                    None => panic!("BattleInfoL transform could not be found."),
                };
                match battle_info_root.find_child("BattleInfoR") {
                    Some(rect) => rect.translate_vec3(shift_ui),
                    None => panic!("BattleInfoR transform could not be found."),
                };
                match battle_info_root.find_child("WdwItemListL") {
                    Some(rect) => rect.translate_vec3(shift_ui),
                    None => panic!("WdwItemListL transform could not be found."),
                };
                match battle_info_root.find_child("WdwItemListR") {
                    Some(rect) => rect.translate_vec3(shift_ui),
                    None => panic!("WdwItemListR transform could not be found."),
                };
            }

            // Check if the BattleInfoL/Info/Status/CrtD GameObject has already been created.
            // If yes, then we simply add the help data to the HelpItemList and pass it on to the original function.
            // If no, then the objects must be initialized before adding the help data to the HelpItemList.
            if let Some(left_crtd_t) = left_go_t.find_child("Info/Status/CrtD") {

                // Grab Help Text for both sides.
                let help_text_l = match left_crtd_t
                    .get_game_object()
                    .get_component_name::<HelpItemFixedText>("HelpItemFixedText") {
                        Some(help_text) => help_text,
                        None => panic!("Left CrtD HelpItemFixedText could not be found."),
                    };
                help_text_l.set_data("MID_H_COMBAT_INFO_CrtD");

                let right_crtd_t = match right_go_t.find_child("Info/Status/CrtD") {
                    Some(rect) => rect,
                    None => panic!("Right Info/Status/CrtD transform could not be found."),
                };

                let help_text_r = match right_crtd_t
                    .get_game_object()
                    .get_component_name::<HelpItemFixedText>("HelpItemFixedText") {
                        Some(help_text) => help_text,
                        None => panic!("Right CrtD HelpItemFixedText could not be found."),
                    };
                help_text_r.set_data("MID_H_COMBAT_INFO_CrtD");
                
                // Add each HelpItemFixedText to the HelpItemList.
                let hib_l = help_text_l.cast_as_base();
                let hib_r = help_text_r.cast_as_base();

                // Create a new Array containing the original list and the new HelpItemBase objects.
                let mut hib_vec = list.fields.list.to_vec();
                hib_vec.push(hib_l);
                hib_vec.push(hib_r);
                let hib_slice = &mut hib_vec[..];
                let new_length = list.fields.list.len() + 2;
                let new_array = Array
                    ::<Option<&'static HelpItemBase>>
                    ::new_specific(HelpItemBase::class(), new_length).unwrap();
                new_array.copy_from_slice(hib_slice);

                list.fields.list = new_array;

                call_original!(list, method_info)
            } else {
                // Resize and reposition the background image to fit the new information.
                let left_wdw_t = match left_go_t.find_child("Info/Wdw") {
                    Some(rect) => rect,
                    None => panic!("BattleInfoL/Info/Wdw transform could not be found.")
                };
                let left_wdw_sd_change = left_wdw_t.get_size_delta().add(sd_add);
                left_wdw_t.set_size_delta(left_wdw_sd_change);
                left_wdw_t.translate_vec3(shift_ui.scale(-0.5));

                // Grab the Status RectTransform to set as a parent transform.
                let left_status_t = match left_go_t.find_child("Info/Status") {
                    Some(rect) => rect,
                    None => panic!("BattleInfoL/Info/Status transform could not be found."),
                };
                let left_atk_t = match left_status_t.find_child("Atk") {
                    Some(rect) => rect,
                    None => panic!("BattleInfoL/Info/Status/Atk transform could not be found."),
                };
                let left_atk_p = left_atk_t.get_position();

                // Create the CrtD GameObject by copying the Info/Status/Atk GameObject. We set
                // the name to CrtD so it can easily be accessed at a later time.
                let left_crtd_go = left_atk_t.get_game_object().copy();
                left_crtd_go.set_name("CrtD");
                let left_crtd_t = left_crtd_go.get_transform::<RectTransform>();
                let left_crtd_p = left_crtd_t.get_position();
                // Ensure CrtD GameObject has the same starting position as Atk GameObject via some vector algebra.
                left_crtd_t.translate_vec3(left_atk_p.add(left_crtd_p.scale(-1.0)));

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

                // Grab Help Text
                let help_text_l = match left_crtd_go
                    .get_component_name::<HelpItemFixedText>("HelpItemFixedText") {
                        Some(help_text) => help_text,
                        None => panic!("Left CrtD HelpItemFixedText could not be found."),
                    };
                help_text_l.set_data("MID_H_COMBAT_INFO_CrtD");
                
                // Destroy the unneeded Icon GameObject that was copied from Status/Info/Atk.
                match left_crtd_t.find_child("Icon") {
                    Some(rect) => rect.get_game_object().destroy(),
                    None => panic!("Left CrtD/Icon transform could not be found."),
                };

                // Set the position and hierarchy of CrtD GameObject
                left_crtd_t.translate_vec3(shift);
                left_crtd_t.set_parent(left_status_t);


                // Repeat for the right side
                // Resize and reposition the background image to fit the new information.
                let right_wdw_t = match right_go_t.find_child("Info/Wdw") {
                    Some(rect) => rect,
                    None => panic!("BattleInfoR/Info/Wdw transform could not be found.")
                };
                let right_wdw_sd_change = right_wdw_t.get_size_delta().add(sd_add);
                right_wdw_t.set_size_delta(right_wdw_sd_change);
                right_wdw_t.translate_vec3(shift_ui.scale(-0.5));

                // Grab the Status RectTransform to set as a parent transform.
                let right_status_t = match right_go_t.find_child("Info/Status") {
                    Some(rect) => rect,
                    None => panic!("BattleInfoR/Info/Status transform could not be found."),
                };
                let right_atk_t = match right_status_t.find_child("Atk") {
                    Some(rect) => rect,
                    None => panic!("BattleInfoR/Info/Status/Crit transform could not be found."),
                };
                let right_atk_p = right_atk_t.get_position();

                // Create the CrtD GameObject by copying the Atk GameObject
                let right_crtd_go = right_atk_t.get_game_object().copy();
                right_crtd_go.set_name("CrtD");
                let right_crtd_t = right_crtd_go.get_transform::<RectTransform>();
                let right_crtd_p = right_crtd_t.get_position();
                // Ensure CrtD GameObject has the same position as Atk GameObject via some vector algebra.
                right_crtd_t.translate_vec3(right_atk_p.add(right_crtd_p.scale(-1.0)));

                // Grab the Help Text
                let help_text_r = match right_crtd_go
                    .get_component_name::<HelpItemFixedText>("HelpItemFixedText") {
                        Some(help_text) => help_text,
                        None => panic!("Right CrtD HelpItemFixedText could not be found."),
                    };
                help_text_r.set_data("MID_H_COMBAT_INFO_CrtD");

                // Set the position and hierarchy of CrtD GameObject
                right_crtd_t.translate_vec3(shift);
                right_crtd_t.set_parent(right_status_t);

                
                // Add each HelpItemFixedText to the HelpItemList.
                let hib_l = help_text_l.cast_as_base();
                let hib_r = help_text_r.cast_as_base();

                // Create a new Array containing the original list and the new HelpItemBase objects.
                let mut hib_vec = list.fields.list.to_vec();
                hib_vec.push(hib_l);
                hib_vec.push(hib_r);
                let hib_slice = &mut hib_vec[..];
                let new_length = list.fields.list.len() + 2;
                let new_array = Array
                    ::<Option<&'static HelpItemBase>>
                    ::new_specific(HelpItemBase::class(), new_length).unwrap();
                new_array.copy_from_slice(hib_slice);
                
                list.fields.list = new_array;

                call_original!(list, method_info)
            }
        } else {
            call_original!(list, method_info)
        }
    } else {
        call_original!(list, method_info)
    }
}