use gat::*;

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
