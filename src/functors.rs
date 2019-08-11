use crate::hkt::{Bind, Rebind};

/// This trait is used for types that can be mapped over.
///
/// There is two laws that a implementator of this trait should satisfy:
/// Note: Assume `f` as a object of a `dyn Functor`.
/// 1. `func.fmap(id) == id` where `id = |x| x` (Identity function)
/// 2. `func.fmap(|x| f(g(x))) == f ∘ g` where `∘` is means the composition
/// of function `f` with function `g`
pub trait Functor: Bind + Rebind<<Self as Bind>::A> {
    type Item;

    /// Creates a a new `Functor` of type `B` from a `Functor` of type `Self` using
    /// the results of calling a function `f` on every value in `Functor` of type `Self`
    fn fmap<B, F>(self, f: F) -> <Self as Rebind<B>>::Res
    where
        Self: Rebind<B>,
        F: FnMut(Self::Item) -> B;
}
