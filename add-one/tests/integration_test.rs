extern crate add_one;

#[test]
fn it_adds_two() {
    assert_eq!(3, add_one::add_one(2));
}