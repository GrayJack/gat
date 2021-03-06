use crate::{rebd, hkt::{Bind, Rebind}};

/// This trait is used for types that can be mapped over.
///
/// Any definition must satisfy:
/// Note: Assume `f` as a object of a `dyn Functor`.
/// 1. `func.fmap(id) == id` where `id = |x| x` (Identity function)
/// 2. `func.fmap(|x| f(g(x))) == f ∘ g` where `∘` is means the composition
/// of function `f` with function `g`
pub trait Functor: Bind + Rebind<<Self as Bind>::Type1> + Sized {
    /// Creates a a new `Functor` of type `B` from a `Functor` of type `Self` using
    /// the results of calling a function `f` on every value in `Functor` of type `Self`
    fn fmap<B, F>(self, f: F) -> rebd!(Self => B)
    where
        Self: Rebind<B>,
        F: FnMut(Self::Type1) -> B;
}

/// It's a trait for `Functor`s with application, providing operations to:
/// * Embed pure expressions (`pure`)
/// * Sequence computation and combine their results (`lift`)
///
/// Any definition must satisfy:
/// 1. Identity: `v.lift(pure(id)) = v` where `id` is the identity function (`|x| x`)
/// 2. Homomorphism: `pure(x).lift(pure(f)) = pure(f(x))`
/// 3. Composition: `u.lift(v.lift(w.lift(pure(∘))) == v.lift(u.lift(w))` where `∘` is the composition of 2 functions
/// 4. Interchange: `u.lift(pure(y)) == pure(|| y).lift(u)`
pub trait Applicative: Functor {
    const PURE: Self;

    /// Wraps a ordinary `value` into a `Functor` value
    fn pure(_value: Self::Type1) -> Self {
        Self::PURE
    }

    /// Lift `self` using a function wrapped in a Functor `fs`
    fn lift<B, F>(self, fs: rebd!(Self => F)) -> rebd!(Self => B)
    where
        F: FnMut(Self::Type1) -> B,
        Self: Rebind<F> + Rebind<B> + Bind,
        <Self as Rebind<F>>::Res: Bind<Orig=Self::Orig,Type1=F> +
            Rebind<F>,
        Self::Orig: Rebind<F>;
}

/// It's a trait for `Applicative` that can binf a wrapped value to a wrapped result
///
/// Any definition must satisfy:
/// 1. `m.bind(pure) == m`
pub trait Monad: Applicative {
    /// Bind monadic value to the monadic action
    fn bind<B, F>(self, f: F) -> rebd!(Self => B)
    where
        Self: Rebind<F> + Rebind<B>,
        F: FnMut(Self::Type1) -> rebd!(Self => B);
}
