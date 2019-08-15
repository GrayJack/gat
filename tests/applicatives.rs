use gat::*;

#[test]
fn option_pure() {
    assert_eq!(Some(5), Applicative::pure(5));
    assert_eq!(Some("Str"), Applicative::pure("Str"));
}

#[test]
fn option_lift() {
    let case1 = Some(5);

    let expect2: Option<Box<dyn FnMut(u32) -> u32>> = None;
    assert_eq!(Some(6), case1.lift(Some(|x| x+1)));
    assert!(!case1.lift(expect2).is_some());
}
