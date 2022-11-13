#![allow(unused)]

fn answer() -> i32 {
    42
}

#[test]
fn check_answer_validity() {
    assert_eq!(answer(), 42);
}
