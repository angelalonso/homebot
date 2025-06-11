// -- Common
// ----------------------------------------------------------------
pub mod action;
pub mod aux_funcs;
pub mod brain;
// #[cfg(any(feature = "test", feature = "live"))]
pub mod env;
pub mod error;
#[cfg(any(feature = "test", feature = "live"))]
pub mod hw_arduino;
#[cfg(feature = "sim")]
pub mod hw_webots;
pub mod input;
pub mod loggin;
pub mod reactionset;
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
#[cfg(any(feature = "test", feature = "live"))]
pub mod live_output;

// -- Test Mode Only
// ----------------------------------------------------------------

// -- Sim Mode Only
// ----------------------------------------------------------------
//#[cfg(feature = "sim")]
//pub mod sim_env;
#[cfg(feature = "sim")]
pub mod sim_hw;
#[cfg(feature = "sim")]
pub mod sim_bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(improper_ctypes)]
    include!("sim_bindings.rs");
}
#[cfg(feature = "sim")]
pub mod sim_output;
