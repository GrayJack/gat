use std::collections::LinkedList;

use gat::*;

#[test]
fn option_bind() {
    let case1: Option<u32> = None;
    let case2 = Some(10);
    let func = |x| Some(x+1);

    assert!(!case1.bind(Option::pure).is_some());
    assert_eq!(Some(10), case2.bind(Option::pure));
    assert_eq!(Some(11), case2.bind(func));
}

#[test]
fn result_bind() {
    let case1: Result<u32, &str> = Err("str");
    let case2: Result<u32, &str> = Ok(10);
    let func = |x| Ok(x+1);

    assert_eq!(Err("str"), case1.bind(Result::pure));
    assert_eq!(Ok(10), case2.bind(Result::pure));
    assert_eq!(Err("str"), case1.bind(func));
    assert_eq!(Ok(11), case2.bind(func));
}

#[test]
fn vec_bind() {
    let case1: Vec<u32> = Vec::new();
    let case2 = vec![1, 2, 3];
    let case3 = vec![1, 2, 3];
    let func = |x| vec![x+1];

    assert!(case1.bind(Vec::pure).is_empty());
    assert_eq!(vec![1, 2, 3], case2.bind(Vec::pure));
    assert_eq!(vec![2, 3, 4], case3.bind(func));
}

#[test]
fn linked_list_bind() {
    use sugars::lkl;
    let case1: LinkedList<u32> = LinkedList::new();
    let case2 = lkl![1, 2, 3];
    let case3 = lkl![1, 2, 3];
    let func = |x| lkl![x+1];

    assert!(case1.bind(LinkedList::pure).is_empty());
    assert_eq!(lkl![1, 2, 3], case2.bind(LinkedList::pure));
    assert_eq!(lkl![2, 3, 4], case3.bind(func));
}
