//! Contains code for selecting features

#![deny(unused_extern_crates)]
#![deny(clippy::missing_docs_in_private_items)]
#![allow(deprecated)]

use std::cmp::Ordering;
use std::io;
use std::str::FromStr;

/// This macro defines the [`RustTarget`] and [`RustFeatures`] types.
macro_rules! define_rust_targets {
    (
        Nightly => {$($nightly_feature:ident $(: #$issue:literal)?),* $(,)?} $(,)?
        $(
            $(#[$attrs:meta])*
            $variant:ident($minor:literal) => {$($feature:ident $(: #$pull:literal)?),* $(,)?},
        )*
        $(,)?
    ) => {
        /// Represents the version of the Rust language to target.
        ///
        /// To support a beta release, use the corresponding stable release.
        ///
        /// This enum will have more variants added as necessary.
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum RustTarget {
            /// Rust Nightly
            $(#[doc = concat!(
                "- [`", stringify!($nightly_feature), "`]",
                "(", $("https://github.com/rust-lang/rust/pull/", stringify!($issue),)* ")",
            )])*
            Nightly,
            $(
                #[doc = concat!("Rust 1.", stringify!($minor))]
                $(#[doc = concat!(
                    "- [`", stringify!($feature), "`]",
                    "(", $("https://github.com/rust-lang/rust/pull/", stringify!($pull),)* ")",
                )])*
                $(#[$attrs])*
                $variant,
            )*
        }

        impl RustTarget {
            const fn minor(self) -> Option<u64> {
                match self {
                    $( Self::$variant => Some($minor),)*
                    Self::Nightly => None
                }
            }

            const fn stable_releases() -> [(Self, u64); [$($minor,)*].len()] {
                [$((Self::$variant, $minor),)*]
            }
        }

        #[cfg(feature = "__cli")]
        /// Strings of allowed `RustTarget` values
        pub const RUST_TARGET_STRINGS: &[&str] = &[$(concat!("1.", stringify!($minor)),)*];

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

                $(if target >= RustTarget::$variant {
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
    Stable_1_77(77) => { offset_of: #106655 },
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
    #[deprecated]
    Stable_1_30(30) => {
        core_ffi_c_void: #53910,
        min_const_fn: #54835,
    },
    #[deprecated]
    Stable_1_28(28) => { repr_transparent: #51562 },
    #[deprecated]
    Stable_1_27(27) => { must_use_function: #48925 },
    #[deprecated]
    Stable_1_26(26) => { i128_and_u128: #49101 },
    #[deprecated]
    Stable_1_25(25) => { repr_align: #47006 },
    #[deprecated]
    Stable_1_21(21) => { builtin_clone_impls: #43690 },
    #[deprecated]
    Stable_1_20(20) => { associated_const: #42809 },
    #[deprecated]
    Stable_1_19(19) => { untagged_union: #42068 },
    #[deprecated]
    Stable_1_17(17) => { static_lifetime_elision: #39265 },
    #[deprecated]
    Stable_1_0(0) => {},
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

impl Default for RustTarget {
    fn default() -> Self {
        LATEST_STABLE_RUST
    }
}

impl PartialOrd for RustTarget {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RustTarget {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.minor(), other.minor()) {
            (Some(a), Some(b)) => a.cmp(&b),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => Ordering::Equal,
        }
    }
}

fn invalid_input<T>(input: &str, msg: impl std::fmt::Display) -> io::Result<T> {
    Err(io::Error::new(
        io::ErrorKind::InvalidInput,
        format!("\"{input}\" is not a valid Rust target, {msg}"),
    ))
}

impl FromStr for RustTarget {
    type Err = io::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input == "nightly" {
            return Ok(Self::Nightly);
        }

        let Some((major_str, tail)) = input.split_once('.') else {
            return invalid_input(input, "accepted values are of the form \"1.71\", \"1.71.1\" or \"nightly\"." );
        };

        if major_str != "1" {
            return invalid_input(
                input,
                "The largest major version of Rust released is \"1\"",
            );
        }

        // We ignore the patch version number as they only include backwards compatible bug fixes.
        let (minor, _patch) = match tail.split_once('.') {
            Some((minor_str, patch_str)) => {
                let Ok(minor) = minor_str.parse::<u64>() else {
                    return invalid_input(input, "the minor version number must be an unsigned 64-bit integer");
                };
                let Ok(patch) = patch_str.parse::<u64>() else {
                    return invalid_input(input, "the patch version number must be an unsigned 64-bit integer");
                };
                (minor, patch)
            }
            None => {
                let Ok(minor) = tail.parse::<u64>() else {
                    return invalid_input(input, "the minor version number must be an unsigned 64-bit integer");
                };
                (minor, 0)
            }
        };

        let Some(target) = Self::stable_releases()
            .iter()
            .filter(|(_, target_minor)| minor >= *target_minor)
            .max_by_key(|(_, target_minor)| target_minor)
            .map(|(target, _)| target)
            .cloned()
        else {
            return invalid_input(input, format!("the earliest Rust target supported by bindgen is {EARLIEST_STABLE_RUST}"));
        };

        Ok(target)
    }
}

impl std::fmt::Display for RustTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.minor() {
            Some(minor) => write!(f, "1.{}", minor),
            None => "nightly".fmt(f),
        }
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
        let f_1_0 = RustFeatures::from(RustTarget::Stable_1_0);
        assert!(
            !f_1_0.static_lifetime_elision &&
                !f_1_0.core_ffi_c_void &&
                !f_1_0.untagged_union &&
                !f_1_0.associated_const &&
                !f_1_0.builtin_clone_impls &&
                !f_1_0.repr_align &&
                !f_1_0.thiscall_abi &&
                !f_1_0.vectorcall_abi
        );
        let f_1_21 = RustFeatures::from(RustTarget::Stable_1_21);
        assert!(
            f_1_21.static_lifetime_elision &&
                !f_1_21.core_ffi_c_void &&
                f_1_21.untagged_union &&
                f_1_21.associated_const &&
                f_1_21.builtin_clone_impls &&
                !f_1_21.repr_align &&
                !f_1_21.thiscall_abi &&
                !f_1_21.vectorcall_abi
        );
        let features = RustFeatures::from(RustTarget::Stable_1_71);
        assert!(
            features.c_unwind_abi &&
                features.abi_efiapi &&
                !features.thiscall_abi
        );
        let f_nightly = RustFeatures::from(RustTarget::Nightly);
        assert!(
            f_nightly.static_lifetime_elision &&
                f_nightly.core_ffi_c_void &&
                f_nightly.untagged_union &&
                f_nightly.associated_const &&
                f_nightly.builtin_clone_impls &&
                f_nightly.maybe_uninit &&
                f_nightly.repr_align &&
                f_nightly.thiscall_abi &&
                f_nightly.vectorcall_abi
        );
    }

    fn test_target(target_str: &str, target: RustTarget) {
        assert_eq!(
            target,
            target_str.parse::<RustTarget>().unwrap(),
            "{target_str}"
        );
    }

    fn test_invalid_target(target_str: &str) {
        assert!(target_str.parse::<RustTarget>().is_err(), "{}", target_str);
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
    }
}
