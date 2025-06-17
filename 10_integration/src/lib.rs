// -- Common
// ----------------------------------------------------------------
pub mod action;
pub mod aux_funcs;
pub mod brain;
// #[cfg(any(feature = "test", feature = "live"))]
pub mod env;
pub mod error;
#[cfg(any(feature = "test", feature = "live"))]
pub mod hw_live;
pub mod input;
pub mod loggin;
pub mod reactionset;
#[cfg(any(feature = "test", feature = "live"))] // TODO: needed?
pub mod bindings_live {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(improper_ctypes)]
    include!("bindings_live.rs");
}

// -- Live Mode Only
// ----------------------------------------------------------------
#[cfg(any(feature = "test", feature = "live"))]
pub mod output_live;

// -- Test Mode Only
// ----------------------------------------------------------------

// -- Sim Mode Only
// ----------------------------------------------------------------
//#[cfg(feature = "sim")]
//pub mod sim_env;
#[cfg(feature = "sim")]
pub mod hw_sim;
#[cfg(feature = "sim")]
pub mod bindings_sim {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(improper_ctypes)]
    include!("bindings_sim.rs");
}
#[cfg(feature = "sim")]
pub mod output_sim;
