#[cfg(test)]
mod tests {
    use std::collections::LinkedList;
    use gat::*;

    #[test]
    fn optional_id() {
        let id = |x| x;
        let test1 = None;
        let test2 = Some(10_u16);

        assert_eq!(None, test1.fmap(id));
        assert_eq!(Some(10_u16), test2.fmap(id));
    }

    #[test]
    fn optional_compose() {
        let f1 = |x| x + 2;
        let f2 = |x| x * 2;
        let test1 = None;
        let test2 = Some(10_u16);

        assert_eq!(None, test1.fmap(|x| f1(f2(x))));
        assert_eq!(Some(22), test2.fmap(|x| f1(f2(x))));
    }

    #[test]
    fn optional_other() {
        let test1: Option<u16> = None;
        let test2 = Some(10_u16);

        assert_eq!(None, test1.fmap(u32::from));
        assert_eq!(Some(10_u32), test2.fmap(u32::from));
    }

    #[test]
    fn result_id() {
        let id = |x| x;
        let test1 = Err("a error");
        let test2: Result<u16, String> = Ok(10_u16);

        assert_eq!(Err("a error"), test1.fmap(id));
        assert_eq!(Ok(10_u16), test2.fmap(id));
    }

    #[test]
    fn result_compose() {
        let f1 = |x| x + 2;
        let f2 = |x| x * 2;
        let test1 = Err("erro");
        let test2: Result<u16, &str> = Ok(10_u16);

        assert_eq!(Err("erro"), test1.fmap(|x| f1(f2(x))));
        assert_eq!(Ok(22), test2.fmap(|x| f1(f2(x))));
    }

    #[test]
    fn result_other() {
        let test1: Result<u16, &str> = Err("erro");
        let test2: Result<u16, &str> = Ok(10_u16);

        assert_eq!(Err("erro"), test1.fmap(u32::from));
        assert_eq!(Ok(10_u32), test2.fmap(u32::from));
    }

    #[test]
    fn vec_id() {
        let id = |x| x;
        let test1: Vec<u16> = vec![];
        let test2 = vec![0_u16, 1, 2, 3];

        assert!(test1.fmap(id).is_empty());
        assert_eq!(vec![0_u16, 1, 2, 3], test2.fmap(id));
    }

    #[test]
    fn vec_compose() {
        let f1 = |x| x + 2;
        let f2 = |x| x * 2;
        let test1: Vec<u16> = vec![];
        let test2 = vec![0_u16, 1, 2, 3];

        assert!(test1.fmap(|x| f1(f2(x))).is_empty());
        assert_eq!(vec![2, 4, 6, 8], test2.fmap(|x| f1(f2(x))));
    }

    #[test]
    fn vec_other() {
        let test1: Vec<u16> = vec![];
        let test2 = vec![0_u16, 1, 2, 3];

        assert!(test1.fmap(u32::from).is_empty());
        assert_eq!(vec![0_u32, 1, 2, 3], test2.fmap(u32::from));
    }

    // #[test]
    // fn linked_list_id() {
    //     let id = |x| x;
    //     let test1 = LinkedList::new();
    //     let test2 = ;
    //
    //     assert!(test1.fmap(id).is_empty());
    //     assert_eq!( , test2.fmap(id));
    // }
    //
    // #[test]
    // fn linked_list_compose() {
    //     let f1 = |x| x + 2;
    //     let f2 = |x| x * 2;
    //     let test1 = LinkedList::new();
    //     let test2 = vec![0_u16, 1, 2, 3];
    //
    //     assert!(test1.fmap(|x| f1(f2(x))).is_empty());
    //     assert_eq!(vec![2, 4, 6, 8], test2.fmap(|x| f1(f2(x))));
    // }
    //
    // #[test]
    // fn linked_list_other() {
    //     let test1: Vec<u16> = vec![];
    //     let test2 = vec![0_u16, 1, 2, 3];
    //
    //     assert!(test1.fmap(u32::from).is_empty());
    //     assert_eq!(vec![0_u32, 1, 2, 3], test2.fmap(u32::from));
    // }
}
