use homebot::queue::add;


#[test]
fn it_works_indeed() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}
