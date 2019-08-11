use crate::hkt::{Bind, Rebind};

/// This trait is used for types that can be mapped over.
///
/// There is two laws that a implementator of this trait should satisfy:
/// Note: Assume `f` as a object of a `dyn Functor`.
/// 1. `func.fmap(id) == id` where `id = |x| x` (Identity function)
/// 2. `func.fmap(|x| f(g(x))) == f ∘ g` where `∘` is means the composition
/// of function `f` with function `g`
pub trait Functor: Bind + Rebind<<Self as Unbind>::A> {
    type Item;

    /// Creates a a new `Functor` of type `B` from a `Functor` of type `Self` using
    /// the results of calling a function `f` on every value in `Functor` of type `Self`
    fn fmap<B, F>(self, f: F) -> <Self as Rebind<B>>::Res
    where
        Self: Rebind<B>,
        F: FnMut(Self::Item) -> B;
}

impl<T> Functor for Option<T> {
    type Item = T;

    fn fmap<B, F: FnMut(Self::Item) -> B>(self, mut f: F) -> <Self as Rebind<B>>::Res {
        match self {
            Some(value) => Some(f(value)),
            None => None,
        }
    }
}

impl<T, E> Functor for Result<T, E> {
    type Item = T;

    fn fmap<B, F: FnMut(Self::Item) -> B>(self, mut f: F) -> <Self as Rebind<B>>::Res {
        match self {
            Ok(value) => Ok(f(value)),
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn optional_id() {
        let id = |x| x;
        let test1 = Nothing;
        let test2 = Some(10_u16);

        assert_eq!(Nothing, test1.fmap(id));
        assert_eq!(Some(10_u16), test2.fmap(id));
    }

    #[test]
    fn optional_compose() {
        let f1 = |x| x+2;
        let f2 = |x| x*2;
        let test1 = Nothing;
        let test2 = Some(10_u16);

        assert_eq!(Nothing, test1.fmap(|x| f1(f2(x))));
        assert_eq!(Some(24), test2.fmap(|x| f1(f2(x))));
    }
}
