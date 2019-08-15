use crate::hkt::{Bind, Rebind};

/// This trait is used for types that can be mapped over.
///
/// Any definition must satisfy:
/// Note: Assume `f` as a object of a `dyn Functor`.
/// 1. `func.fmap(id) == id` where `id = |x| x` (Identity function)
/// 2. `func.fmap(|x| f(g(x))) == f ∘ g` where `∘` is means the composition
/// of function `f` with function `g`
pub trait Functor: Bind + Rebind<<Self as Bind>::Type1> {
    /// Creates a a new `Functor` of type `B` from a `Functor` of type `Self` using
    /// the results of calling a function `f` on every value in `Functor` of type `Self`
    fn fmap<B, F>(self, f: F) -> <Self as Rebind<B>>::Res
    where
        Self: Rebind<B>,
        F: FnMut(Self::Type1) -> B;
}

/// It's a trait for `Functor`s with application, providing operations to:
/// * Embed pure expressions (`pure`)
/// * Sequence computation and combine their results (`lift`)
///
/// Any definition must satisfy:
/// 1. Identity: `v.lift(pure(id)) = v` where `id` is the identity function (|x| x)
/// 2. Homomorphism: `pure(x).lift(pure(f)) = pure(f(x))`
/// 3. Composition: `u.lift(v.lift(w.lift(pure(∘))) == v.lift(u.lift(w))` where `∘` is the composition of 2 functions
/// Interchange: `u.lift(pure(y)) == pure(|| y).lift(u)`
pub trait Applicative: Functor {
    /// Given a value of a type that the implementator type can hold
    /// return that implementator type holding it
    fn pure(value: Self::Type1) -> Self;

    /// Lift `self` using a function wrapped in a Functor `fs`
    fn lift<B, F>(self, fs: <Self as Rebind<F>>::Res) -> <Self as Rebind<B>>::Res
    where
        F: FnMut(Self::Type1) -> B,
        Self: Rebind<F> + Rebind<B> + Bind,
        <Self as Rebind<F>>::Res: Bind<Orig=Self::Orig,Type1=F> +
            Rebind<F>,
        Self::Orig: Rebind<F>;
}
