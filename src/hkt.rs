//! Define traits to simulate Generic Associated Types

/// Implement to be able to rebind a type `B` to type `A`
pub trait Rebind<A> {
    type Res;
}

/// Implement to be able to maintain a a bind of type
pub trait Bind {
    type F;
    type A;
}

impl<T> Bind for Option<T> {
    type F = Option<T>;
    type A = T;
}

impl<A, B> Rebind<A> for Option<B> {
    type Res = Option<A>;
}

impl<T, E> Bind for Result<T, E> {
    type F = Result<T, E>;
    type A = T;
}

impl<T, E, A> Rebind<A> for Result<T, E> {
    type Res = Result<A, E>;
}
