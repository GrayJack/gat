//! A module for groups

/// A `Monoid` trait is implemented for types that is possible to have
/// a associative binary operation and an identity element.
///
/// So, in orther to implement this trait, a type should follow these rules:
/// 1. `a ⊗ b ⊗ c == (a ⊗ b) ⊗ c == a ⊗ (b ⊗ c)` where `a`, `b` and `c` of the
/// same type and `⊗` a associative binary operation
/// 2. `a ⊗ ε == ε ⊗ a == a` where `ε` is the identity element of the type
///
/// **Note:** The identity element is implemented as function cause not all types that
/// are `Monoid`s in the Rust standard library have a `const fn` constructor, like `Vec`
/// and `LinkedList`. If they became `const fn` (Vec is already in the process to become),
/// it will be updated to use associated constant.
pub trait Monoid: Sized {
    const IDENTITY: Self;

    /// A function that return the identity value/element of the `Monoid`
    fn identity_value() -> Self {
        Self::IDENTITY
    }

    /// A `Monoid` method that represents a associative binary operation
    fn associate(self, other: &Self) -> Self;

    /// A function that folds over using the `associate`function and the `identity_value`
    #[inline]
    fn mfold(xs: &[Self]) -> Self {
        xs.iter().fold(Self::identity_value(), Self::associate)
    }
}
