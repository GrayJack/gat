use std::{cmp::Ordering::{self, *}, collections::LinkedList};

use gat::*;
use sugars::lkl;

#[test]
fn unit_associate() {
    let x = ();
    let y = x.associate(&());

    assert_eq!((), y);
}

#[test]
fn unit_mfold() {
    let x = [(), (), ()];

    let test = Monoid::mfold(&x);

    assert_eq!((), test);
}

#[test]
fn ordering_associate() {
    let case1 = Equal;
    let case2 = Less;
    let case3 = Greater;


    assert_eq!(Equal, case1.associate(&Equal));
    assert_eq!(Less, case1.associate(&Less));
    assert_eq!(Greater, case1.associate(&Greater));

    assert_eq!(Less, case2.associate(&Equal));
    assert_eq!(Less, case2.associate(&Less));
    assert_eq!(Less, case2.associate(&Greater));

    assert_eq!(Greater, case3.associate(&Equal));
    assert_eq!(Greater, case3.associate(&Less));
    assert_eq!(Greater, case3.associate(&Greater));
}

#[test]
fn ordering_mfold() {
    let case1 = [Equal, Equal, Equal];
    let case2 = [Equal, Equal, Less];
    let case3 = [Equal, Greater, Equal];

    assert_eq!(Equal, Monoid::mfold(&case1));
    assert_eq!(Less, Monoid::mfold(&case2));
    assert_eq!(Greater, Monoid::mfold(&case3));
}

#[test]
fn option_associate() {
    let case1: Option<Ordering> = None;
    let case2 = Some(Equal);

    assert_eq!(Some(Equal), case1.associate(&Some(Equal)));
    assert_eq!(Some(Less), case2.associate(&Some(Less)));
}

#[test]
fn option_mfold() {
    let case1: [Option<Ordering>; 3] = [None, None, None];
    let case2 = [Some(Less), Some(Equal), None];

    assert_eq!(None, Monoid::mfold(&case1));
    assert_eq!(Some(Less), Monoid::mfold(&case2));
}

#[test]
fn vec_associate() {
    let case1: Vec<u16> = Vec::new();
    let case2 = vec![1, 2];

    assert_eq!(vec![1], case1.associate(&vec![1]));
    assert_eq!(vec![1, 2, 3, 4], case2.associate(&vec![3, 4]));
}

#[test]
fn vec_mfold() {
    let case1 = [vec![1], vec![2], vec![3]];
    let case2: [Vec<u8>; 3] = [vec![], vec![], vec![]];

    assert!(Monoid::mfold(&case2).is_empty());
    assert_eq!(vec![1, 2, 3], Monoid::mfold(&case1));
}

#[test]
fn linked_list_associate() {
    let case1: LinkedList<u16> = LinkedList::new();
    let case2 = lkl![1, 2];

    assert_eq!(lkl![1], case1.associate(&lkl![1]));
    assert_eq!(lkl![1, 2, 3, 4], case2.associate(&lkl![3, 4]));
}

#[test]
fn linked_list_mfold() {
    let case1 = [lkl![1], lkl![2], lkl![3]];
    let case2: [LinkedList<u8>; 3] = [lkl![], lkl![], lkl![]];

    assert!(Monoid::mfold(&case2).is_empty());
    assert_eq!(lkl![1, 2, 3], Monoid::mfold(&case1));
}
