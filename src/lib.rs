pub mod functors;
pub mod hkt;
pub mod groups;

pub use crate::{
    functors::Functor,
    hkt::{Bind, Rebind},
    groups::Monoid,
};

// Implementations
use std::{cmp::Ordering, collections::LinkedList};

// Group Monoids
impl Monoid for () {
    fn identity_value() -> Self {}
    fn associate(self, _other: &Self) -> Self {}
}

impl Monoid for Ordering {
    fn identity_value() -> Self {
        Ordering::Equal
    }
    fn associate(self, other: &Self) -> Self {
        use std::cmp::Ordering::*;
        match (self, other) {
            (Less, _) => Less,
            (Equal, y) => *y,
            (Greater, _) => Greater,
        }
    }
}

impl<T: Monoid + Clone> Monoid for Option<T> {
    #[inline]
    fn identity_value() -> Self {
        None
    }
    fn associate(self, other: &Self) -> Self {
        match (self, other) {
            (None, y) => y.clone(),
            (x, None) => x,
            (Some(ref x), Some(y)) => Some(Monoid::associate(x.clone(), y)),
        }
    }
}

impl<T: Clone> Monoid for Vec<T> {
    #[inline]
    fn identity_value() -> Self {
        Vec::new()
    }
    fn associate(mut self, other: &Self) -> Self {
        let mut o = other.clone();
        self.append(&mut o);
        self
    }
}

impl<T: Clone> Monoid for LinkedList<T> {
    #[inline]
    fn identity_value() -> Self {
        LinkedList::new()
    }
    fn associate(mut self, other: &Self) -> Self {
        let mut o = other.clone();
        self.append(&mut o);
        self
    }
}

// Functor Applicative and Monad
impl<T> Bind for Option<T> {
    type F = Option<T>;
    type A = T;
    type B = ();
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
    type B = E;
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
    type B = ();
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
    type B = ();
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
