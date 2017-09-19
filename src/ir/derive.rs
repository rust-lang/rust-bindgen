//! Traits for determining whether we can derive traits for a thing or not.
//!
//! These traits tend to come in pairs:
//!
//! 1. A "trivial" version, whose implementations aren't allowed to recursively
//! look at other types or the results of fix point analyses.
//!
//! 2. A "normal" version, whose implementations simply query the results of a
//! fix point analysis.
//!
//! The former is used by the analyses when creating the results queried by the
//! second.

use super::context::BindgenContext;

/// A trait that encapsulates the logic for whether or not we can derive `Debug`
/// for a given thing.
pub trait CanDeriveDebug {
    /// Return `true` if `Debug` can be derived for this thing, `false`
    /// otherwise.
    fn can_derive_debug(&self, ctx: &BindgenContext) -> bool;
}

/// A trait that encapsulates the logic for whether or not we can trivially
/// derive `Debug` without looking at any other types or the results of a fix
/// point analysis. This is a helper trait for the fix point analysis.
pub trait CanTriviallyDeriveDebug {
    /// Return `true` if `Debug` can trivially be derived for this thing,
    /// `false` otherwise.
    fn can_trivially_derive_debug(&self) -> bool;
}

/// A trait that encapsulates the logic for whether or not we can derive `Copy`
/// for a given thing.
pub trait CanDeriveCopy<'a> {
    /// Return `true` if `Copy` can be derived for this thing, `false`
    /// otherwise.
    fn can_derive_copy(&'a self, ctx: &'a BindgenContext) -> bool;
}

/// A trait that encapsulates the logic for whether or not we can trivially
/// derive `Copy` without looking at any other types or results of fix point
/// analsyses. This is a helper trait for fix point analysis.
pub trait CanTriviallyDeriveCopy {
    /// Return `true` if `Copy` can be trivially derived for this thing, `false`
    /// otherwise.
    fn can_trivially_derive_copy(&self) -> bool;
}

/// A trait that encapsulates the logic for whether or not we can derive
/// `Default` for a given thing.
pub trait CanDeriveDefault {
    /// Return `true` if `Default` can be derived for this thing, `false`
    /// otherwise.
    fn can_derive_default(&self, ctx: &BindgenContext) -> bool;
}

/// A trait that encapsulates the logic for whether or not we can trivially
/// derive `Default` without looking at any other types or results of fix point
/// analyses. This is a helper trait for the fix point analysis.
pub trait CanTriviallyDeriveDefault {
    /// Return `true` if `Default` can trivially derived for this thing, `false`
    /// otherwise.
    fn can_trivially_derive_default(&self) -> bool;
}

/// A trait that encapsulates the logic for whether or not we can derive `Hash`
/// for a given thing.
pub trait CanDeriveHash {
    /// Return `true` if `Hash` can be derived for this thing, `false`
    /// otherwise.
    fn can_derive_hash(&self, ctx: &BindgenContext) -> bool;
}

/// A trait that encapsulates the logic for whether or not we can derive
/// `PartialEq` for a given thing.
pub trait CanDerivePartialEq {
    /// Return `true` if `PartialEq` can be derived for this thing, `false`
    /// otherwise.
    fn can_derive_partialeq(&self, ctx: &BindgenContext) -> bool;
}

/// A trait that encapsulates the logic for whether or not we can derive
/// `PartialOrd` for a given thing.
pub trait CanDerivePartialOrd {
    /// Return `true` if `PartialOrd` can be derived for this thing, `false`
    /// otherwise.
    fn can_derive_partialord(&self, ctx: &BindgenContext) -> bool;
}

/// A trait that encapsulates the logic for whether or not we can derive `Eq`
/// for a given thing.
pub trait CanDeriveEq {
    /// Return `true` if `Eq` can be derived for this thing, `false` otherwise.
    fn can_derive_eq(&self, ctx: &BindgenContext) -> bool;
}

/// A trait that encapsulates the logic for whether or not we can derive `Ord`
/// for a given thing.
pub trait CanDeriveOrd {
    /// Return `true` if `Ord` can be derived for this thing, `false` otherwise.
    fn can_derive_ord(&self, ctx: &BindgenContext) -> bool;
}

/// A trait that encapsulates the logic for whether or not we can derive `Hash`
/// without looking at any other types or the results of any fix point
/// analyses. This is a helper trait for the fix point analysis.
pub trait CanTriviallyDeriveHash {
    /// Return `true` if `Hash` can trivially be derived for this thing, `false`
    /// otherwise.
    fn can_trivially_derive_hash(&self) -> bool;
}

/// A trait that encapsulates the logic for whether or not we can trivially
/// derive `PartialEq` or `PartialOrd` without looking at any other types or
/// results of fix point analyses. This is a helper for the fix point analysis.
pub trait CanTriviallyDerivePartialEqOrPartialOrd {
    /// Return `true` if `PartialEq` or `PartialOrd` can trivially be derived
    /// for this thing, `false` otherwise.
    fn can_trivially_derive_partialeq_or_partialord(&self) -> bool;
}
