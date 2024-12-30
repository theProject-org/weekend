#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]
#[macro_use]
extern crate plus;

#[cfg(feature = "jwt")]
mod jwt;

#[cfg(feature = "oauth")]
mod oauth;

#[cfg(feature = "cookie")]
mod cookie;

#[cfg(feature = "bearer")]
mod bearer;

//  core authentication module
mod authentication;
pub use authentication::*;
