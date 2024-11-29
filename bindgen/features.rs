//! Contains code for selecting features

#![deny(unused_extern_crates)]
#![deny(clippy::missing_docs_in_private_items)]
#![allow(deprecated)]

use std::str::FromStr;
use std::{fmt, io};

/// Represents the version of the Rust language to target.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct RustTarget(Version);

impl RustTarget {
    /// Create a new [`RustTarget`] for a stable release of Rust.
    pub fn stable(minor: u64, patch: u64) -> Result<Self, InvalidRustTarget> {
        let target = Self(Version::Stable(minor, patch));

        if target < EARLIEST_STABLE_RUST {
            return Err(InvalidRustTarget::TooEarly);
        }

        Ok(target)
    }

    const fn minor(&self) -> Option<u64> {
        match self.0 {
            Version::Nightly => None,
            Version::Stable(minor, _) => Some(minor),
        }
    }

    const fn is_compatible(&self, other: &Self) -> bool {
        match (self.0, other.0) {
            (Version::Stable(minor, _), Version::Stable(other_minor, _)) => {
                // We ignore the patch version number as they only include backwards compatible bug
                // fixes.
                minor >= other_minor
            }
            (_, Version::Nightly) => false,
            (Version::Nightly, _) => true,
        }
    }
}

impl Default for RustTarget {
    fn default() -> Self {
        LATEST_STABLE_RUST
    }
}

impl fmt::Display for RustTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Version::Stable(minor, patch) => write!(f, "1.{minor}.{patch}"),
            Version::Nightly => "nightly".fmt(f),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Version {
    Stable(u64, u64),
    Nightly,
}

pub enum InvalidRustTarget {
    TooEarly,
}

impl fmt::Display for InvalidRustTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooEarly => write!(f, "the earliest Rust version supported by bindgen is {EARLIEST_STABLE_RUST}"),
        }
    }
}

/// This macro defines the [`RustTarget`] and [`RustFeatures`] types.
macro_rules! define_rust_targets {
    (
        Nightly => {$($nightly_feature:ident $(: #$issue:literal)?),* $(,)?} $(,)?
        $(
            $variant:ident($minor:literal) => {$($feature:ident $(: #$pull:literal)?),* $(,)?},
        )*
        $(,)?
    ) => {

        impl RustTarget {
            /// The nightly version of Rust, which introduces the following features:"
            $(#[doc = concat!(
                "- [`", stringify!($nightly_feature), "`]",
                "(", $("https://github.com/rust-lang/rust/pull/", stringify!($issue),)* ")",
            )])*
            #[deprecated = "The use of this constant is deprecated, please use `RustTarget::nightly` instead."]
            pub const Nightly: Self = Self::nightly();

            /// The nightly version of Rust, which introduces the following features:"
            $(#[doc = concat!(
                "- [`", stringify!($nightly_feature), "`]",
                "(", $("https://github.com/rust-lang/rust/pull/", stringify!($issue),)* ")",
            )])*
            pub const fn nightly() -> Self {
                Self(Version::Nightly)
            }

            $(
                #[doc = concat!("Version 1.", stringify!($minor), " of Rust, which introduced the following features:")]
                $(#[doc = concat!(
                    "- [`", stringify!($feature), "`]",
                    "(", $("https://github.com/rust-lang/rust/pull/", stringify!($pull),)* ")",
                )])*
                #[deprecated = "The use of this constant is deprecated, please use `RustTarget::stable` instead."]
                pub const $variant: Self = Self(Version::Stable($minor, 0));
            )*

            const fn stable_releases() -> [(Self, u64); [$($minor,)*].len()] {
                [$((Self::$variant, $minor),)*]
            }
        }

        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
        pub(crate) struct RustFeatures {
            $($(pub(crate) $feature: bool,)*)*
            $(pub(crate) $nightly_feature: bool,)*
        }

        impl From<RustTarget> for RustFeatures {
            fn from(target: RustTarget) -> Self {
                if target == RustTarget::Nightly {
                    return Self {
                        $($($feature: true,)*)*
                        $($nightly_feature: true,)*
                    };
                }

                let mut features = Self {
                    $($($feature: false,)*)*
                    $($nightly_feature: false,)*
                };

                $(if target.is_compatible(&RustTarget::$variant) {
                    $(features.$feature = true;)*
                })*

                features
            }
        }
    };
}

// NOTE: When adding or removing features here, make sure to add the stabilization PR
// number for the feature if it has been stabilized or the tracking issue number if the feature is
// not stable.
define_rust_targets! {
    Nightly => {
        vectorcall_abi: #124485,
        ptr_metadata: #81513,
        layout_for_ptr: #69835,
    },
    Stable_1_77(77) => {
        offset_of: #106655,
        literal_cstr: #117472,  // Edition 2021+ only
    },
    Stable_1_73(73) => { thiscall_abi: #42202 },
    Stable_1_71(71) => { c_unwind_abi: #106075 },
    Stable_1_68(68) => { abi_efiapi: #105795 },
    Stable_1_64(64) => { core_ffi_c: #94503 },
    Stable_1_51(51) => { raw_ref_macros: #80886 },
    Stable_1_59(59) => { const_cstr: #54745 },
    Stable_1_47(47) => { larger_arrays: #74060 },
    Stable_1_43(43) => { associated_constants: #68952 },
    Stable_1_40(40) => { non_exhaustive: #44109 },
    Stable_1_36(36) => { maybe_uninit: #60445 },
    Stable_1_33(33) => { repr_packed_n: #57049 },
}

/// Latest stable release of Rust that is supported by bindgen
pub const LATEST_STABLE_RUST: RustTarget = {
    // FIXME: replace all this code by
    // ```
    // RustTarget::stable_releases()
    //     .into_iter()
    //     .max_by_key(|(_, m)| m)
    //     .map(|(t, _)| t)
    //     .unwrap()
    // ```
    // once those operations can be used in constants.
    let targets = RustTarget::stable_releases();

    let mut i = 0;
    let mut latest_target = None;
    let mut latest_minor = 0;

    while i < targets.len() {
        let (target, minor) = targets[i];

        if latest_minor < minor {
            latest_minor = minor;
            latest_target = Some(target);
        }

        i += 1;
    }

    match latest_target {
        Some(target) => target,
        None => unreachable!(),
    }
};

/// Earliest stable release of Rust that is supported by bindgen
pub const EARLIEST_STABLE_RUST: RustTarget = {
    // FIXME: replace all this code by
    // ```
    // RustTarget::stable_releases()
    //     .into_iter()
    //     .min_by_key(|(_, m)| m)
    //     .map(|(t, _)| t)
    //     .unwrap_or(LATEST_STABLE_RUST)
    // ```
    // once those operations can be used in constants.
    let targets = RustTarget::stable_releases();

    let mut i = 0;
    let mut earliest_target = None;
    let Some(mut earliest_minor) = LATEST_STABLE_RUST.minor() else {
        unreachable!()
    };

    while i < targets.len() {
        let (target, minor) = targets[i];

        if earliest_minor > minor {
            earliest_minor = minor;
            earliest_target = Some(target);
        }

        i += 1;
    }

    match earliest_target {
        Some(target) => target,
        None => unreachable!(),
    }
};

fn invalid_input(input: &str, msg: impl fmt::Display) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidInput,
        format!("\"{input}\" is not a valid Rust target, {msg}"),
    )
}

impl FromStr for RustTarget {
    type Err = io::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input == "nightly" {
            return Ok(Self::Nightly);
        }

        let Some((major_str, tail)) = input.split_once('.') else {
            return Err(invalid_input(input, "accepted values are of the form \"1.71\", \"1.71.1\" or \"nightly\"." ) );
        };

        if major_str != "1" {
            return Err(invalid_input(
                input,
                "The largest major version of Rust released is \"1\"",
            ));
        }

        let (minor, patch) = match tail.split_once('.') {
            Some((minor_str, patch_str)) => {
                let Ok(minor) = minor_str.parse::<u64>() else {
                    return Err(invalid_input(input, "the minor version number must be an unsigned 64-bit integer"));
                };
                let Ok(patch) = patch_str.parse::<u64>() else {
                    return Err(invalid_input(input, "the patch version number must be an unsigned 64-bit integer"));
                };
                (minor, patch)
            }
            None => {
                let Ok(minor) = tail.parse::<u64>() else {
                    return Err(invalid_input(input, "the minor version number must be an unsigned 64-bit integer"));
                };
                (minor, 0)
            }
        };

        Self::stable(minor, patch).map_err(|err| invalid_input(input, err))
    }
}

impl Default for RustFeatures {
    fn default() -> Self {
        RustTarget::default().into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn target_features() {
        let features = RustFeatures::from(RustTarget::Stable_1_71);
        assert!(
            features.c_unwind_abi &&
                features.abi_efiapi &&
                !features.thiscall_abi
        );
        let f_nightly = RustFeatures::from(RustTarget::Nightly);
        assert!(
            f_nightly.maybe_uninit &&
                f_nightly.thiscall_abi &&
                f_nightly.vectorcall_abi
        );
    }

    fn test_target(input: &str, expected: RustTarget) {
        // Two targets are equivalent if they enable the same set of features
        let expected = RustFeatures::from(expected);
        let found = RustFeatures::from(input.parse::<RustTarget>().unwrap());
        assert_eq!(
            expected,
            found,
            "target {input} enables features:\n{found:#?}\nand should enable features:\n{expected:#?}"
        );
    }

    fn test_invalid_target(input: &str) {
        assert!(
            input.parse::<RustTarget>().is_err(),
            "{} should be an invalid target",
            input
        );
    }

    #[test]
    fn valid_targets() {
        test_target("1.71", RustTarget::Stable_1_71);
        test_target("1.71.0", RustTarget::Stable_1_71);
        test_target("1.71.1", RustTarget::Stable_1_71);
        test_target("1.72", RustTarget::Stable_1_71);
        test_target("1.73", RustTarget::Stable_1_73);
        test_target("1.18446744073709551615", LATEST_STABLE_RUST);
        test_target("nightly", RustTarget::Nightly);
    }

    #[test]
    fn invalid_targets() {
        test_invalid_target("2.0");
        test_invalid_target("1.cat");
        test_invalid_target("1.0.cat");
        test_invalid_target("1.18446744073709551616");
        test_invalid_target("1.0.18446744073709551616");
        test_invalid_target("1.-1.0");
        test_invalid_target("1.0.-1");
        test_invalid_target("beta");
        test_invalid_target("1.0.0");
        test_invalid_target("1.32.0");
    }
}
