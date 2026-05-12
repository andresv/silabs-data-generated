#![no_std]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(clippy::all)]
#![allow(unused)]

#[cfg(not(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40",
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
)))]
compile_error!("a silabs-metapac chip feature must be enabled (e.g. --features efr32mg26b211f2048im68)");

pub mod common;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40",
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/acmp_v2.rs"]
pub mod acmp_v2;

#[cfg(any(
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/amuxcp_v2.rs"]
pub mod amuxcp_v2;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40",
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/buram_v0.rs"]
pub mod buram_v0;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40",
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/burtc_v1.rs"]
pub mod burtc_v1;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40"
))]
#[path = "registers/cmu_v3.rs"]
pub mod cmu_v3;

#[cfg(any(
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/cmu_v7.rs"]
pub mod cmu_v7;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40",
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/dcdc_v2.rs"]
pub mod dcdc_v2;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40"
))]
#[path = "registers/devinfo_v0_mg24.rs"]
pub mod devinfo_v0_mg24;

#[cfg(any(
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/devinfo_v0_mg26.rs"]
pub mod devinfo_v0_mg26;

#[cfg(any(
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/dmem_v2.rs"]
pub mod dmem_v2;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40"
))]
#[path = "registers/dmem_v2_ws.rs"]
pub mod dmem_v2_ws;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40",
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/dpll_v1.rs"]
pub mod dpll_v1;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40",
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/emu_v3.rs"]
pub mod emu_v3;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40",
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/eusart_v2.rs"]
pub mod eusart_v2;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40",
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/eusart_v2_lf.rs"]
pub mod eusart_v2_lf;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40",
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/fsrco_v0.rs"]
pub mod fsrco_v0;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40",
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/gpcrc_v0.rs"]
pub mod gpcrc_v0;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40"
))]
#[path = "registers/gpio_v3.rs"]
pub mod gpio_v3;

#[cfg(any(
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/gpio_v7.rs"]
pub mod gpio_v7;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40",
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/hfrco_v2.rs"]
pub mod hfrco_v2;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40",
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/hfrcoem_v2.rs"]
pub mod hfrcoem_v2;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40",
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/hfxo_v3.rs"]
pub mod hfxo_v3;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40",
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/hostmailbox_v0.rs"]
pub mod hostmailbox_v0;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40",
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/i2c_v0.rs"]
pub mod i2c_v0;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b610f1536im40",
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/iadc_v3.rs"]
pub mod iadc_v3;

#[cfg(any(
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68"
))]
#[path = "registers/iadc_v3_ha.rs"]
pub mod iadc_v3_ha;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40",
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/icache_v0.rs"]
pub mod icache_v0;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40",
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/keyscan_v1.rs"]
pub mod keyscan_v1;

#[cfg(any(
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/lcd_v3.rs"]
pub mod lcd_v3;

#[cfg(any(
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/lcdrf_v0.rs"]
pub mod lcdrf_v0;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40",
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/ldma_v0.rs"]
pub mod ldma_v0;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40"
))]
#[path = "registers/ldmaxbar_v3.rs"]
pub mod ldmaxbar_v3;

#[cfg(any(
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/ldmaxbar_v7.rs"]
pub mod ldmaxbar_v7;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40",
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/letimer_v1.rs"]
pub mod letimer_v1;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40",
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/lfrco_v2.rs"]
pub mod lfrco_v2;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40",
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/lfxo_v1.rs"]
pub mod lfxo_v1;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40"
))]
#[path = "registers/msc_v3.rs"]
pub mod msc_v3;

#[cfg(any(
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/msc_v9.rs"]
pub mod msc_v9;

#[cfg(any(
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48"
))]
#[path = "registers/mvp_v1.rs"]
pub mod mvp_v1;

#[cfg(any(
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b610f3200im48"
))]
#[path = "registers/mvp_v2.rs"]
pub mod mvp_v2;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40",
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/pcnt_v1.rs"]
pub mod pcnt_v1;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40"
))]
#[path = "registers/prs_v3.rs"]
pub mod prs_v3;

#[cfg(any(
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/prs_v6.rs"]
pub mod prs_v6;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40",
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/radioaes_v1.rs"]
pub mod radioaes_v1;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40"
))]
#[path = "registers/scratchpad_v0.rs"]
pub mod scratchpad_v0;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40",
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/semailbox_ns_host_v1.rs"]
pub mod semailbox_ns_host_v1;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40",
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/semailbox_s_host_v1.rs"]
pub mod semailbox_s_host_v1;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b610f1536im40"
))]
#[path = "registers/smu_v3.rs"]
pub mod smu_v3;

#[cfg(any(
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48"
))]
#[path = "registers/smu_v3_mvp.rs"]
pub mod smu_v3_mvp;

#[cfg(any(
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/smu_v7.rs"]
pub mod smu_v7;

#[cfg(any(
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b610f3200im48"
))]
#[path = "registers/smu_v7_mvp.rs"]
pub mod smu_v7_mvp;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b610f1536im40"
))]
#[path = "registers/smu_ns_cfgns_v3.rs"]
pub mod smu_ns_cfgns_v3;

#[cfg(any(
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48"
))]
#[path = "registers/smu_ns_cfgns_v3_mvp.rs"]
pub mod smu_ns_cfgns_v3_mvp;

#[cfg(any(
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/smu_ns_cfgns_v7.rs"]
pub mod smu_ns_cfgns_v7;

#[cfg(any(
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b610f3200im48"
))]
#[path = "registers/smu_ns_cfgns_v7_mvp.rs"]
pub mod smu_ns_cfgns_v7_mvp;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b610f1536im40"
))]
#[path = "registers/smu_s_cfgns_v3.rs"]
pub mod smu_s_cfgns_v3;

#[cfg(any(
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48"
))]
#[path = "registers/smu_s_cfgns_v3_mvp.rs"]
pub mod smu_s_cfgns_v3_mvp;

#[cfg(any(
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/smu_s_cfgns_v7.rs"]
pub mod smu_s_cfgns_v7;

#[cfg(any(
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b610f3200im48"
))]
#[path = "registers/smu_s_cfgns_v7_mvp.rs"]
pub mod smu_s_cfgns_v7_mvp;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b610f1536im40"
))]
#[path = "registers/syscfg_v3.rs"]
pub mod syscfg_v3;

#[cfg(any(
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48"
))]
#[path = "registers/syscfg_v3_mvp.rs"]
pub mod syscfg_v3_mvp;

#[cfg(any(
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/syscfg_v9.rs"]
pub mod syscfg_v9;

#[cfg(any(
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b610f3200im48"
))]
#[path = "registers/syscfg_v9_mvp.rs"]
pub mod syscfg_v9_mvp;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40"
))]
#[path = "registers/syscfg_ns_cfgns_v3.rs"]
pub mod syscfg_ns_cfgns_v3;

#[cfg(any(
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/syscfg_ns_cfgns_v9.rs"]
pub mod syscfg_ns_cfgns_v9;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40"
))]
#[path = "registers/syscfg_s_cfgns_v3.rs"]
pub mod syscfg_s_cfgns_v3;

#[cfg(any(
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/syscfg_s_cfgns_v9.rs"]
pub mod syscfg_s_cfgns_v9;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40",
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/sysrtc_v1.rs"]
pub mod sysrtc_v1;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40",
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/timer_v1.rs"]
pub mod timer_v1;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40",
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/timer_v1_w.rs"]
pub mod timer_v1_w;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40",
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/ulfrco_v1.rs"]
pub mod ulfrco_v1;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40",
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/usart_v0.rs"]
pub mod usart_v0;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40",
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/vdac_v2.rs"]
pub mod vdac_v2;

#[cfg(any(
    feature = "efr32mg24a010f1024im40",
    feature = "efr32mg24a010f1024im48",
    feature = "efr32mg24a010f1536gm40",
    feature = "efr32mg24a010f1536gm48",
    feature = "efr32mg24a010f1536im40",
    feature = "efr32mg24a010f1536im48",
    feature = "efr32mg24a010f768im40",
    feature = "efr32mg24a010f768im48",
    feature = "efr32mg24a020f1024im40",
    feature = "efr32mg24a020f1024im48",
    feature = "efr32mg24a020f1536gm40",
    feature = "efr32mg24a020f1536gm48",
    feature = "efr32mg24a020f1536im40",
    feature = "efr32mg24a020f1536im48",
    feature = "efr32mg24a020f768im40",
    feature = "efr32mg24a021f1024im40",
    feature = "efr32mg24a110f1024im48",
    feature = "efr32mg24a110f1536gm48",
    feature = "efr32mg24a111f1536gm48",
    feature = "efr32mg24a120f1536gm48",
    feature = "efr32mg24a121f1536gm48",
    feature = "efr32mg24a410f1536im40",
    feature = "efr32mg24a410f1536im48",
    feature = "efr32mg24a420f1536im40",
    feature = "efr32mg24a420f1536im48",
    feature = "efr32mg24a610f1536im40",
    feature = "efr32mg24a620f1536im40",
    feature = "efr32mg24b010f1024im48",
    feature = "efr32mg24b010f1536im40",
    feature = "efr32mg24b010f1536im48",
    feature = "efr32mg24b020f1024im48",
    feature = "efr32mg24b020f1536im40",
    feature = "efr32mg24b020f1536im48",
    feature = "efr32mg24b110f1536gm48",
    feature = "efr32mg24b110f1536im48",
    feature = "efr32mg24b120f1536im48",
    feature = "efr32mg24b210f1536im40",
    feature = "efr32mg24b210f1536im48",
    feature = "efr32mg24b220f1536im48",
    feature = "efr32mg24b310f1536im48",
    feature = "efr32mg24b610f1536im40",
    feature = "efr32mg26b211f2048im68",
    feature = "efr32mg26b211f3200im48",
    feature = "efr32mg26b221f2048im68",
    feature = "efr32mg26b221f3200im48",
    feature = "efr32mg26b311f3200il136",
    feature = "efr32mg26b410f3200im48",
    feature = "efr32mg26b410f3200im68",
    feature = "efr32mg26b411f3200im48",
    feature = "efr32mg26b411f3200im68",
    feature = "efr32mg26b420f3200im48",
    feature = "efr32mg26b420f3200im68",
    feature = "efr32mg26b421f3200im48",
    feature = "efr32mg26b421f3200im68",
    feature = "efr32mg26b510f3200il136",
    feature = "efr32mg26b510f3200im48",
    feature = "efr32mg26b510f3200im68",
    feature = "efr32mg26b511f3200il136",
    feature = "efr32mg26b511f3200im48",
    feature = "efr32mg26b511f3200im68",
    feature = "efr32mg26b520f3200im48",
    feature = "efr32mg26b520f3200im68",
    feature = "efr32mg26b521f3200im48",
    feature = "efr32mg26b521f3200im68",
    feature = "efr32mg26b610f3200im48",
    feature = "efr32mg26b611f2048im48"
))]
#[path = "registers/wdog_v1.rs"]
pub mod wdog_v1;

#[cfg(feature = "efr32mg24a010f1024im40")]
pub mod chip {
    include!("chips/efr32mg24a010f1024im40/mod.rs");
}
#[cfg(feature = "efr32mg24a010f1024im40")]
pub use chip::*;

#[cfg(feature = "efr32mg24a010f1024im48")]
pub mod chip {
    include!("chips/efr32mg24a010f1024im48/mod.rs");
}
#[cfg(feature = "efr32mg24a010f1024im48")]
pub use chip::*;

#[cfg(feature = "efr32mg24a010f1536gm40")]
pub mod chip {
    include!("chips/efr32mg24a010f1536gm40/mod.rs");
}
#[cfg(feature = "efr32mg24a010f1536gm40")]
pub use chip::*;

#[cfg(feature = "efr32mg24a010f1536gm48")]
pub mod chip {
    include!("chips/efr32mg24a010f1536gm48/mod.rs");
}
#[cfg(feature = "efr32mg24a010f1536gm48")]
pub use chip::*;

#[cfg(feature = "efr32mg24a010f1536im40")]
pub mod chip {
    include!("chips/efr32mg24a010f1536im40/mod.rs");
}
#[cfg(feature = "efr32mg24a010f1536im40")]
pub use chip::*;

#[cfg(feature = "efr32mg24a010f1536im48")]
pub mod chip {
    include!("chips/efr32mg24a010f1536im48/mod.rs");
}
#[cfg(feature = "efr32mg24a010f1536im48")]
pub use chip::*;

#[cfg(feature = "efr32mg24a010f768im40")]
pub mod chip {
    include!("chips/efr32mg24a010f768im40/mod.rs");
}
#[cfg(feature = "efr32mg24a010f768im40")]
pub use chip::*;

#[cfg(feature = "efr32mg24a010f768im48")]
pub mod chip {
    include!("chips/efr32mg24a010f768im48/mod.rs");
}
#[cfg(feature = "efr32mg24a010f768im48")]
pub use chip::*;

#[cfg(feature = "efr32mg24a020f1024im40")]
pub mod chip {
    include!("chips/efr32mg24a020f1024im40/mod.rs");
}
#[cfg(feature = "efr32mg24a020f1024im40")]
pub use chip::*;

#[cfg(feature = "efr32mg24a020f1024im48")]
pub mod chip {
    include!("chips/efr32mg24a020f1024im48/mod.rs");
}
#[cfg(feature = "efr32mg24a020f1024im48")]
pub use chip::*;

#[cfg(feature = "efr32mg24a020f1536gm40")]
pub mod chip {
    include!("chips/efr32mg24a020f1536gm40/mod.rs");
}
#[cfg(feature = "efr32mg24a020f1536gm40")]
pub use chip::*;

#[cfg(feature = "efr32mg24a020f1536gm48")]
pub mod chip {
    include!("chips/efr32mg24a020f1536gm48/mod.rs");
}
#[cfg(feature = "efr32mg24a020f1536gm48")]
pub use chip::*;

#[cfg(feature = "efr32mg24a020f1536im40")]
pub mod chip {
    include!("chips/efr32mg24a020f1536im40/mod.rs");
}
#[cfg(feature = "efr32mg24a020f1536im40")]
pub use chip::*;

#[cfg(feature = "efr32mg24a020f1536im48")]
pub mod chip {
    include!("chips/efr32mg24a020f1536im48/mod.rs");
}
#[cfg(feature = "efr32mg24a020f1536im48")]
pub use chip::*;

#[cfg(feature = "efr32mg24a020f768im40")]
pub mod chip {
    include!("chips/efr32mg24a020f768im40/mod.rs");
}
#[cfg(feature = "efr32mg24a020f768im40")]
pub use chip::*;

#[cfg(feature = "efr32mg24a021f1024im40")]
pub mod chip {
    include!("chips/efr32mg24a021f1024im40/mod.rs");
}
#[cfg(feature = "efr32mg24a021f1024im40")]
pub use chip::*;

#[cfg(feature = "efr32mg24a110f1024im48")]
pub mod chip {
    include!("chips/efr32mg24a110f1024im48/mod.rs");
}
#[cfg(feature = "efr32mg24a110f1024im48")]
pub use chip::*;

#[cfg(feature = "efr32mg24a110f1536gm48")]
pub mod chip {
    include!("chips/efr32mg24a110f1536gm48/mod.rs");
}
#[cfg(feature = "efr32mg24a110f1536gm48")]
pub use chip::*;

#[cfg(feature = "efr32mg24a111f1536gm48")]
pub mod chip {
    include!("chips/efr32mg24a111f1536gm48/mod.rs");
}
#[cfg(feature = "efr32mg24a111f1536gm48")]
pub use chip::*;

#[cfg(feature = "efr32mg24a120f1536gm48")]
pub mod chip {
    include!("chips/efr32mg24a120f1536gm48/mod.rs");
}
#[cfg(feature = "efr32mg24a120f1536gm48")]
pub use chip::*;

#[cfg(feature = "efr32mg24a121f1536gm48")]
pub mod chip {
    include!("chips/efr32mg24a121f1536gm48/mod.rs");
}
#[cfg(feature = "efr32mg24a121f1536gm48")]
pub use chip::*;

#[cfg(feature = "efr32mg24a410f1536im40")]
pub mod chip {
    include!("chips/efr32mg24a410f1536im40/mod.rs");
}
#[cfg(feature = "efr32mg24a410f1536im40")]
pub use chip::*;

#[cfg(feature = "efr32mg24a410f1536im48")]
pub mod chip {
    include!("chips/efr32mg24a410f1536im48/mod.rs");
}
#[cfg(feature = "efr32mg24a410f1536im48")]
pub use chip::*;

#[cfg(feature = "efr32mg24a420f1536im40")]
pub mod chip {
    include!("chips/efr32mg24a420f1536im40/mod.rs");
}
#[cfg(feature = "efr32mg24a420f1536im40")]
pub use chip::*;

#[cfg(feature = "efr32mg24a420f1536im48")]
pub mod chip {
    include!("chips/efr32mg24a420f1536im48/mod.rs");
}
#[cfg(feature = "efr32mg24a420f1536im48")]
pub use chip::*;

#[cfg(feature = "efr32mg24a610f1536im40")]
pub mod chip {
    include!("chips/efr32mg24a610f1536im40/mod.rs");
}
#[cfg(feature = "efr32mg24a610f1536im40")]
pub use chip::*;

#[cfg(feature = "efr32mg24a620f1536im40")]
pub mod chip {
    include!("chips/efr32mg24a620f1536im40/mod.rs");
}
#[cfg(feature = "efr32mg24a620f1536im40")]
pub use chip::*;

#[cfg(feature = "efr32mg24b010f1024im48")]
pub mod chip {
    include!("chips/efr32mg24b010f1024im48/mod.rs");
}
#[cfg(feature = "efr32mg24b010f1024im48")]
pub use chip::*;

#[cfg(feature = "efr32mg24b010f1536im40")]
pub mod chip {
    include!("chips/efr32mg24b010f1536im40/mod.rs");
}
#[cfg(feature = "efr32mg24b010f1536im40")]
pub use chip::*;

#[cfg(feature = "efr32mg24b010f1536im48")]
pub mod chip {
    include!("chips/efr32mg24b010f1536im48/mod.rs");
}
#[cfg(feature = "efr32mg24b010f1536im48")]
pub use chip::*;

#[cfg(feature = "efr32mg24b020f1024im48")]
pub mod chip {
    include!("chips/efr32mg24b020f1024im48/mod.rs");
}
#[cfg(feature = "efr32mg24b020f1024im48")]
pub use chip::*;

#[cfg(feature = "efr32mg24b020f1536im40")]
pub mod chip {
    include!("chips/efr32mg24b020f1536im40/mod.rs");
}
#[cfg(feature = "efr32mg24b020f1536im40")]
pub use chip::*;

#[cfg(feature = "efr32mg24b020f1536im48")]
pub mod chip {
    include!("chips/efr32mg24b020f1536im48/mod.rs");
}
#[cfg(feature = "efr32mg24b020f1536im48")]
pub use chip::*;

#[cfg(feature = "efr32mg24b110f1536gm48")]
pub mod chip {
    include!("chips/efr32mg24b110f1536gm48/mod.rs");
}
#[cfg(feature = "efr32mg24b110f1536gm48")]
pub use chip::*;

#[cfg(feature = "efr32mg24b110f1536im48")]
pub mod chip {
    include!("chips/efr32mg24b110f1536im48/mod.rs");
}
#[cfg(feature = "efr32mg24b110f1536im48")]
pub use chip::*;

#[cfg(feature = "efr32mg24b120f1536im48")]
pub mod chip {
    include!("chips/efr32mg24b120f1536im48/mod.rs");
}
#[cfg(feature = "efr32mg24b120f1536im48")]
pub use chip::*;

#[cfg(feature = "efr32mg24b210f1536im40")]
pub mod chip {
    include!("chips/efr32mg24b210f1536im40/mod.rs");
}
#[cfg(feature = "efr32mg24b210f1536im40")]
pub use chip::*;

#[cfg(feature = "efr32mg24b210f1536im48")]
pub mod chip {
    include!("chips/efr32mg24b210f1536im48/mod.rs");
}
#[cfg(feature = "efr32mg24b210f1536im48")]
pub use chip::*;

#[cfg(feature = "efr32mg24b220f1536im48")]
pub mod chip {
    include!("chips/efr32mg24b220f1536im48/mod.rs");
}
#[cfg(feature = "efr32mg24b220f1536im48")]
pub use chip::*;

#[cfg(feature = "efr32mg24b310f1536im48")]
pub mod chip {
    include!("chips/efr32mg24b310f1536im48/mod.rs");
}
#[cfg(feature = "efr32mg24b310f1536im48")]
pub use chip::*;

#[cfg(feature = "efr32mg24b610f1536im40")]
pub mod chip {
    include!("chips/efr32mg24b610f1536im40/mod.rs");
}
#[cfg(feature = "efr32mg24b610f1536im40")]
pub use chip::*;

#[cfg(feature = "efr32mg26b211f2048im68")]
pub mod chip {
    include!("chips/efr32mg26b211f2048im68/mod.rs");
}
#[cfg(feature = "efr32mg26b211f2048im68")]
pub use chip::*;

#[cfg(feature = "efr32mg26b211f3200im48")]
pub mod chip {
    include!("chips/efr32mg26b211f3200im48/mod.rs");
}
#[cfg(feature = "efr32mg26b211f3200im48")]
pub use chip::*;

#[cfg(feature = "efr32mg26b221f2048im68")]
pub mod chip {
    include!("chips/efr32mg26b221f2048im68/mod.rs");
}
#[cfg(feature = "efr32mg26b221f2048im68")]
pub use chip::*;

#[cfg(feature = "efr32mg26b221f3200im48")]
pub mod chip {
    include!("chips/efr32mg26b221f3200im48/mod.rs");
}
#[cfg(feature = "efr32mg26b221f3200im48")]
pub use chip::*;

#[cfg(feature = "efr32mg26b311f3200il136")]
pub mod chip {
    include!("chips/efr32mg26b311f3200il136/mod.rs");
}
#[cfg(feature = "efr32mg26b311f3200il136")]
pub use chip::*;

#[cfg(feature = "efr32mg26b410f3200im48")]
pub mod chip {
    include!("chips/efr32mg26b410f3200im48/mod.rs");
}
#[cfg(feature = "efr32mg26b410f3200im48")]
pub use chip::*;

#[cfg(feature = "efr32mg26b410f3200im68")]
pub mod chip {
    include!("chips/efr32mg26b410f3200im68/mod.rs");
}
#[cfg(feature = "efr32mg26b410f3200im68")]
pub use chip::*;

#[cfg(feature = "efr32mg26b411f3200im48")]
pub mod chip {
    include!("chips/efr32mg26b411f3200im48/mod.rs");
}
#[cfg(feature = "efr32mg26b411f3200im48")]
pub use chip::*;

#[cfg(feature = "efr32mg26b411f3200im68")]
pub mod chip {
    include!("chips/efr32mg26b411f3200im68/mod.rs");
}
#[cfg(feature = "efr32mg26b411f3200im68")]
pub use chip::*;

#[cfg(feature = "efr32mg26b420f3200im48")]
pub mod chip {
    include!("chips/efr32mg26b420f3200im48/mod.rs");
}
#[cfg(feature = "efr32mg26b420f3200im48")]
pub use chip::*;

#[cfg(feature = "efr32mg26b420f3200im68")]
pub mod chip {
    include!("chips/efr32mg26b420f3200im68/mod.rs");
}
#[cfg(feature = "efr32mg26b420f3200im68")]
pub use chip::*;

#[cfg(feature = "efr32mg26b421f3200im48")]
pub mod chip {
    include!("chips/efr32mg26b421f3200im48/mod.rs");
}
#[cfg(feature = "efr32mg26b421f3200im48")]
pub use chip::*;

#[cfg(feature = "efr32mg26b421f3200im68")]
pub mod chip {
    include!("chips/efr32mg26b421f3200im68/mod.rs");
}
#[cfg(feature = "efr32mg26b421f3200im68")]
pub use chip::*;

#[cfg(feature = "efr32mg26b510f3200il136")]
pub mod chip {
    include!("chips/efr32mg26b510f3200il136/mod.rs");
}
#[cfg(feature = "efr32mg26b510f3200il136")]
pub use chip::*;

#[cfg(feature = "efr32mg26b510f3200im48")]
pub mod chip {
    include!("chips/efr32mg26b510f3200im48/mod.rs");
}
#[cfg(feature = "efr32mg26b510f3200im48")]
pub use chip::*;

#[cfg(feature = "efr32mg26b510f3200im68")]
pub mod chip {
    include!("chips/efr32mg26b510f3200im68/mod.rs");
}
#[cfg(feature = "efr32mg26b510f3200im68")]
pub use chip::*;

#[cfg(feature = "efr32mg26b511f3200il136")]
pub mod chip {
    include!("chips/efr32mg26b511f3200il136/mod.rs");
}
#[cfg(feature = "efr32mg26b511f3200il136")]
pub use chip::*;

#[cfg(feature = "efr32mg26b511f3200im48")]
pub mod chip {
    include!("chips/efr32mg26b511f3200im48/mod.rs");
}
#[cfg(feature = "efr32mg26b511f3200im48")]
pub use chip::*;

#[cfg(feature = "efr32mg26b511f3200im68")]
pub mod chip {
    include!("chips/efr32mg26b511f3200im68/mod.rs");
}
#[cfg(feature = "efr32mg26b511f3200im68")]
pub use chip::*;

#[cfg(feature = "efr32mg26b520f3200im48")]
pub mod chip {
    include!("chips/efr32mg26b520f3200im48/mod.rs");
}
#[cfg(feature = "efr32mg26b520f3200im48")]
pub use chip::*;

#[cfg(feature = "efr32mg26b520f3200im68")]
pub mod chip {
    include!("chips/efr32mg26b520f3200im68/mod.rs");
}
#[cfg(feature = "efr32mg26b520f3200im68")]
pub use chip::*;

#[cfg(feature = "efr32mg26b521f3200im48")]
pub mod chip {
    include!("chips/efr32mg26b521f3200im48/mod.rs");
}
#[cfg(feature = "efr32mg26b521f3200im48")]
pub use chip::*;

#[cfg(feature = "efr32mg26b521f3200im68")]
pub mod chip {
    include!("chips/efr32mg26b521f3200im68/mod.rs");
}
#[cfg(feature = "efr32mg26b521f3200im68")]
pub use chip::*;

#[cfg(feature = "efr32mg26b610f3200im48")]
pub mod chip {
    include!("chips/efr32mg26b610f3200im48/mod.rs");
}
#[cfg(feature = "efr32mg26b610f3200im48")]
pub use chip::*;

#[cfg(feature = "efr32mg26b611f2048im48")]
pub mod chip {
    include!("chips/efr32mg26b611f2048im48/mod.rs");
}
#[cfg(feature = "efr32mg26b611f2048im48")]
pub use chip::*;

