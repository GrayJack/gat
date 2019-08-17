pub mod functors;
pub mod groups;
pub mod hkt;

pub use crate::{
    functors::{Applicative, Functor, Monad},
    groups::Monoid,
    hkt::{Bind, ForAll, Rebind},
};

/// Convenience macro to write **almost** idiomatic Rust to `Rebind` for a type
#[macro_export]
macro_rules! rebd {
    ($t1:ty => $t2:ty) => {
        <$t1 as Rebind<$t2>>::Res
    };
}

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
    #[inline]
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
    fn fmap<B, F>(self, mut f: F) -> rebd!(Self => B)
    where
        F: FnMut(Self::Type1) -> B,
    {
        match self {
            Some(value) => Some(f(value)),
            None => None,
        }
    }
}

impl<T> Applicative for Option<T> {
    #[inline]
    fn pure(s: Self::Type1) -> Self {
        Some(s)
    }

    #[inline]
    fn lift<B, F>(self, fs: rebd!(Self => F)) -> rebd!(Self => B)
    where
        F: FnMut(Self::Type1) -> B,
    {
        match fs {
            Some(func) => self.fmap(func),
            None => None,
        }
    }
}

impl<T> Monad for Option<T> {
    fn bind<B, F>(self, mut f: F) -> rebd!(Self => B)
    where
        F: FnMut(Self::Type1) -> rebd!(Self => B)
    {
        match self {
            Some(value) => f(value),
            None => None,
        }
    }
}

impl<T, E> Bind for Result<T, E> {
    type Orig = Result<ForAll, ForAll>;
    type Type1 = T;
    type Type2 = E;
}

impl<T, E, A> Rebind<A> for Result<T, E> {
    type Res = Result<A, E>;
}

impl<T, E> Functor for Result<T, E> {
    #[inline]
    fn fmap<B, F>(self, mut f: F) -> rebd!(Self => B)
    where
        F: FnMut(Self::Type1) -> B,
    {
        match self {
            Ok(value) => Ok(f(value)),
            Err(err) => Err(err),
        }
    }
}

impl<T, E> Applicative for Result<T, E> {
    #[inline]
    fn pure(value: Self::Type1) -> Self {
        Ok(value)
    }

    #[inline]
    fn lift<B, F>(self, fs: rebd!(Self => F)) -> rebd!(Self => B)
    where
        F: FnMut(Self::Type1) -> B,
    {
        match fs {
            Ok(f) => self.fmap(f),
            Err(e) => Err(e),
        }
    }
}

impl<T, E> Monad for Result<T, E> {
    fn bind<B, F>(self, mut f: F) -> rebd!(Self => B)
    where
        F: FnMut(Self::Type1) -> rebd!(Self => B)
    {
        match self {
            Ok(value) => f(value),
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
    fn fmap<B, F>(self, f: F) -> rebd!(Self => B)
    where
        F: FnMut(Self::Type1) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

impl<T: Clone> Applicative for Vec<T> {
    #[inline]
    fn pure(value: Self::Type1) -> Self {
        vec![value]
    }

    fn lift<B, F>(self, mut fs: rebd!(Self => F)) -> rebd!(Self => B)
    where
        F: FnMut(Self::Type1) -> B,
    {
        use sugars::cvec;
        cvec![f(x); f <- &mut fs, x <- self.clone()]
    }
}

impl<T: Clone> Monad for Vec<T> {
    fn bind<B, F>(self, mut f: F) -> rebd!(Self => B)
    where
        F: FnMut(Self::Type1) -> rebd!(Self => B)
    {
        use sugars::cvec;
        cvec![y; x <- self.clone(), y <- f(x)]
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
    fn fmap<B, F>(self, f: F) -> rebd!(Self => B)
    where
        F: FnMut(Self::Type1) -> B,
    {
        self.into_iter().map(f).collect()
    }
}

impl<T: Clone> Applicative for LinkedList<T> {
    #[inline]
    fn pure(value: Self::Type1) -> Self {
        use sugars::lkl;
        lkl![value]
    }

    fn lift<B, F>(self, fs: rebd!(Self => F)) -> rebd!(Self => B)
    where
        F: FnMut(Self::Type1) -> B,
    {
        let mut lkl = LinkedList::new();
        for mut f in fs {
            for x in self.clone() {
                lkl.push_back(f(x));
            }
        }
        lkl
    }
}

impl<T: Clone> Monad for LinkedList<T> {
    fn bind<B, F>(self, mut f: F) -> rebd!(Self => B)
    where
        F: FnMut(Self::Type1) -> rebd!(Self => B)
    {
        let mut lkl = LinkedList::new();
        for x in self {
            for value in f(x) {
                lkl.push_back(value);
            }
        }
        lkl
    }
}
