// -- Common to all Modes
pub mod loggin;
pub mod homebot_action;
pub mod homebot_input;
pub mod homebot_reactionset;

// This cfg is here for reference ;) 
#[cfg(any(feature = "sim", feature = "test"))]
pub mod homebot_brain;

// -- Test Mode
#[cfg(feature = "test")]
pub mod test_env;
#[cfg(feature = "test")]
pub mod test_output;
#[cfg(feature = "test")]
pub mod test_bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(improper_ctypes)]
    include!("test_bindings.rs");
}
#[cfg(feature = "test")]
pub mod test_nowebots;

// -- Sim Mode
#[cfg(feature = "sim")]
pub mod sim_env;
#[cfg(feature = "sim")]
pub mod sim_output;
#[cfg(feature = "sim")]
pub mod sim_bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(improper_ctypes)]
    include!("sim_bindings.rs");
}
#[cfg(feature = "sim")]
pub mod sim_webots;

// -- Live Mode
#[cfg(feature = "live")]
pub mod live_env;
#[cfg(feature = "live")]
pub mod live_output;
#[cfg(feature = "live")]
pub mod live_bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(improper_ctypes)]
    include!("live_bindings.rs");
}
#[cfg(feature = "live")]
pub mod live_nowebots;

