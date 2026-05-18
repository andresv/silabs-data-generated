#![no_std]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(clippy::all)]
#![allow(unused)]

pub mod common;

#[cfg(feature = "pac")]
include!(env!("SILABS_METAPAC_PAC_PATH"));

#[cfg(feature = "metadata")]
pub mod metadata {
    include!("metadata.rs");
    include!(env!("SILABS_METAPAC_METADATA_PATH"));
}
