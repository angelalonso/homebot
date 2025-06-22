pub mod action;
pub mod aux_funcs;
pub mod brain;
pub mod error;
#[cfg(any(feature = "test", feature = "live"))]
pub mod hw_live;
#[cfg(feature = "sim")]
pub mod hw_sim;
pub mod input;
pub mod loggin;
#[cfg(any(feature = "test", feature = "live"))]
pub mod output_live;
#[cfg(feature = "sim")]
pub mod output_sim;
pub mod reactionset;
#[cfg(any(feature = "test", feature = "live"))] // TODO: needed?
pub mod bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(improper_ctypes)]
    include!("bindings_live.rs");
}
#[cfg(feature = "sim")]
pub mod bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(improper_ctypes)]
    include!("bindings_sim.rs");
}
