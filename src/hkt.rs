//! Define traits to simulate Generic Associated Types

/// Implement to be able to rebind a type `B` to type `A`
pub trait Rebind<A> {
    type Res;
}

/// Implement to be able to maintain a a bind of type
pub trait Bind {
    type F;
    type A;
    type B;
}
