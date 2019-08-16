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
