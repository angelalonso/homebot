pub mod loggin;
pub mod sim_action;
pub mod sim_input;
pub mod sim_reactionset;

#[cfg(any(feature = "sim", feature = "test"))]
pub mod sim_brain;

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
