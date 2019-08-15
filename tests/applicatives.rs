use std::collections::LinkedList;

use gat::*;
use sugars::lkl;

#[test]
fn option_pure() {
    assert_eq!(Some(5), Option::pure(5));
    assert_eq!(Some("Str"), Option::pure("Str"));
}

#[test]
fn option_lift() {
    let case1 = Some(5);

    let expect2: Option<Box<dyn FnMut(u32) -> u32>> = None;
    assert_eq!(Some(6), case1.lift(Some(|x| x+1)));
    assert!(!case1.lift(expect2).is_some());
}

#[test]
fn result_pure() {
    let expected1: Result<u32, u32> = Ok(1u32);
    let expected2: Result<&str, u32> = Ok("str");
    assert_eq!(expected1, Result::pure(1u32));
    assert_eq!(expected2, Result::pure("str"));
}

#[test]
fn result_lift() {
    use sugars::boxed;
    let case1: Result<u32, &str> = Ok(1);
    let fs: Result<Box<dyn FnMut(_) -> _>, &str> = Ok(boxed!(|x| x+1));
    let fs2: Result<Box<dyn FnMut(u32) -> u32>, &str> = Err("err");

    assert_eq!(Ok(2), case1.lift(fs));
    assert_eq!(Err("err"), case1.lift(fs2));
}

#[test]
fn vec_pure() {
    assert_eq!(vec![1], Vec::pure(1));
    assert_eq!(vec!["str"], Vec::pure("str"));
}

#[test]
fn vec_lift() {
    let case1 = vec![1, 2];
    let f1 = |x| x + 1;
    let vf = vec![f1];

    assert_eq!(vec![2, 3], case1.lift(vf));
}

#[test]
fn lkl_pure() {
    assert_eq!(lkl![1], LinkedList::pure(1));
    assert_eq!(lkl!["str"], LinkedList::pure("str"));
}

#[test]
fn lkl_lift() {
    let case1 = lkl![1, 2];
    let lklf = lkl![|x| x+1];

    assert_eq!(lkl![2, 3], case1.lift(lklf));
}
