// -- Common to all Modes
// ----------------------------------------------------------------
pub mod homebot_action;
pub mod homebot_aux_funcs;
pub mod homebot_brain;
pub mod homebot_input;
pub mod homebot_reactionset;
pub mod loggin;

// -- Live and Test Mode
// ----------------------------------------------------------------
#[cfg(any(feature = "test", feature = "live"))]
pub mod env;
#[cfg(any(feature = "test", feature = "live"))]
pub mod live_hw;
#[cfg(any(feature = "test", feature = "live"))]
pub mod live_bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(improper_ctypes)]
    include!("live_bindings.rs");
}

// -- Live Mode Only
// ----------------------------------------------------------------
#[cfg(feature = "live")]
pub mod live_output;

// -- Test Mode Only
// ----------------------------------------------------------------
#[cfg(feature = "test")]
pub mod test_output;

// -- Sim Mode Only
// ----------------------------------------------------------------
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
pub mod sim_hw;
