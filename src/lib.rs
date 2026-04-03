pub mod engagelib;
pub mod unitylib;
pub mod hooks;

const VERSION: &'static str = "1.0.0";

#[skyline::main(name = "judgcrit")]
pub fn main() {
    skyline::install_hook!(crate::hooks::aicalculations::calculate_indication_hook);
    skyline::install_hook!(crate::hooks::aicalculations::calculate_chain_indication_hook);
    skyline::install_hook!(crate::hooks::aicalculations::calculate_kill_probability_2_hook);
    skyline::install_hook!(crate::hooks::combatdetails::calc_attack_hit_hook);
    skyline::install_hook!(crate::hooks::combatui::combat_gauge_controller_setup_hook);
    skyline::install_hook!(crate::hooks::forecastui::set_battle_info_hook);
    skyline::install_hook!(crate::hooks::helpmanager::help_manager_add_hook);

    println!("judgcrit v{} successfully loaded.", VERSION);

    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();
        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => {
                match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Box<Any>",
                }
            },
        };
        let err_msg = format!(
            "judgcrit v{} has panicked at '{}' with the following message:\n{}\0",
            VERSION,
            location,
            msg
        );

        skyline::error::show_error(
            776,
            "judgcrit has panicked! Please open the details and send a screenshot to the developer, then close the game.\n\0",
            err_msg.as_str(),
        );
    }));
}