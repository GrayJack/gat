pub mod functors;
pub mod hkt;

pub use crate::{
    functors::Functor,
    hkt::{Bind, Rebind},
};

// Implementations
use std::collections::LinkedList;

impl<T> Bind for Option<T> {
    type F = Option<T>;
    type A = T;
}

impl<A, B> Rebind<A> for Option<B> {
    type Res = Option<A>;
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

impl<T, E> Bind for Result<T, E> {
    type F = Result<T, E>;
    type A = T;
}

impl<T, E, A> Rebind<A> for Result<T, E> {
    type Res = Result<A, E>;
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

impl<T> Bind for Vec<T> {
    type F = Vec<T>;
    type A = T;
}

impl<A, T> Rebind<A> for Vec<T> {
    type Res = Vec<A>;
}

impl<T> Functor for Vec<T> {
    type Item = T;

    fn fmap<B, F: FnMut(Self::Item) -> B>(self, f: F) -> <Self as Rebind<B>>::Res {
        self.into_iter().map(f).collect()
    }
}

impl<T> Bind for LinkedList<T> {
    type F = LinkedList<T>;
    type A = T;
}

impl<A, T> Rebind<A> for LinkedList<T> {
    type Res = LinkedList<A>;
}

impl<T> Functor for LinkedList<T> {
    type Item = T;

    fn fmap<B, F: FnMut(Self::Item) -> B>(self, f: F) -> <Self as Rebind<B>>::Res {
        self.into_iter().map(f).collect()
    }
}
