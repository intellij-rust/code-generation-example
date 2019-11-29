include!(concat!(env!("OUT_DIR"), "/hello.rs"));
include!(concat!(env!("GENERATED_ENV"), "/hello2.rs"));

#[cfg(not(has_generated_feature))]
mod disabled;
#[cfg(has_generated_feature)]
mod enabled;

#[cfg(not(has_generated_feature))]
pub use disabled::function_under_cfg;
#[cfg(has_generated_feature)]
pub use enabled::function_under_cfg;
