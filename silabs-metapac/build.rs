use std::env;
#[cfg(feature = "rt")]
use std::path::PathBuf;

enum GetOneError {
    None,
    Multiple,
}

trait IteratorExt: Iterator {
    fn get_one(self) -> Result<Self::Item, GetOneError>;
}

impl<T: Iterator> IteratorExt for T {
    fn get_one(mut self) -> Result<Self::Item, GetOneError> {
        match self.next() {
            None => Err(GetOneError::None),
            Some(res) => match self.next() {
                Some(_) => Err(GetOneError::Multiple),
                None => Ok(res),
            },
        }
    }
}

fn main() {
    #[cfg(feature = "rt")]
    let crate_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());

    let chip_name = match env::vars()
        .map(|(a, _)| a)
        .filter(|x| x.starts_with("CARGO_FEATURE_EFR32") || x.starts_with("CARGO_FEATURE_EFM32"))
        .get_one()
    {
        Ok(x) => x,
        Err(GetOneError::None) => panic!("No silabs-metapac chip feature enabled (e.g. --features efr32mg26b211f2048im68)"),
        Err(GetOneError::Multiple) => panic!("Multiple silabs-metapac chip features enabled — pick one"),
    }
    .strip_prefix("CARGO_FEATURE_")
    .unwrap()
    .to_ascii_lowercase();

    #[cfg(feature = "rt")]
    println!(
        "cargo:rustc-link-search={}/src/chips/{}",
        crate_dir.display(),
        chip_name,
    );

    println!("cargo:rerun-if-changed=build.rs");
}
