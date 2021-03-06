//! Define traits to simulate Generic Associated Types

/// A empty struct representing a 'unapplied' type
pub struct ForAll;

/// Implement to be able to rebind a type `B` to type `A`
pub trait Rebind<A> {
    type Res;
}

/// Implement to be able to maintain a a bind of type
pub trait Bind {
    type Orig;
    type Type1;
    type Type2;
}
