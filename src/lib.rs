pub mod functors;
pub mod hkt;
pub mod groups;

pub use crate::{
    functors::{Functor, Applicative},
    hkt::{Bind, Rebind, ForAll},
    groups::Monoid,
};

// Implementations
use std::{cmp::Ordering, collections::LinkedList};

// Group Monoids
impl Monoid for () {
    #[inline]
    fn identity_value() -> Self {}
    #[inline]
    fn associate(self, _other: &Self) -> Self {}
}

impl Monoid for Ordering {
    #[inline]
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
    type Orig = Option<ForAll>;
    type Type1 = T;
    type Type2 = ();
}

impl<A, B> Rebind<A> for Option<B> {
    type Res = Option<A>;
}

impl<T> Functor for Option<T> {
    fn fmap<B, F: FnMut(Self::Type1) -> B>(self, mut f: F) -> <Self as Rebind<B>>::Res {
        match self {
            Some(value) => Some(f(value)),
            None => None,
        }
    }
}

impl<T> Applicative for Option<T> {
    fn pure(s: Self::Type1) -> Self {
        Some(s)
    }

    fn lift<B, F>(self, f: <Self as Rebind<F>>::Res) -> <Self as Rebind<B>>::Res
    where
        F: FnMut(Self::Type1) -> B
    {
        match f {
            Some(func) => self.fmap(func),
            None => None,
        }
    }
}

impl<T, E> Bind for Result<T, E> {
    type Orig = Result<ForAll, E>;
    type Type1 = T;
    type Type2 = E;
}

impl<T, E, A> Rebind<A> for Result<T, E> {
    type Res = Result<A, E>;
}

impl<T, E> Functor for Result<T, E> {
    #[inline]
    fn fmap<B, F: FnMut(Self::Type1) -> B>(self, mut f: F) -> <Self as Rebind<B>>::Res {
        match self {
            Ok(value) => Ok(f(value)),
            Err(err) => Err(err),
        }
    }
}

impl<T> Bind for Vec<T> {
    type Orig = Vec<ForAll>;
    type Type1 = T;
    type Type2 = ();
}

impl<A, T> Rebind<A> for Vec<T> {
    type Res = Vec<A>;
}

impl<T> Functor for Vec<T> {
    #[inline]
    fn fmap<B, F: FnMut(Self::Type1) -> B>(self, f: F) -> <Self as Rebind<B>>::Res {
        self.into_iter().map(f).collect()
    }
}

impl<T> Bind for LinkedList<T> {
    type Orig = LinkedList<ForAll>;
    type Type1 = T;
    type Type2 = ();
}

impl<A, T> Rebind<A> for LinkedList<T> {
    type Res = LinkedList<A>;
}

impl<T> Functor for LinkedList<T> {
    #[inline]
    fn fmap<B, F: FnMut(Self::Type1) -> B>(self, f: F) -> <Self as Rebind<B>>::Res {
        self.into_iter().map(f).collect()
    }
}
