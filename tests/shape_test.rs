mod common;

use learn_rust::day_20220128_test::Rectangle;
use crate::common::set_up;

#[test]
fn test_shape() {
    let rec1 = Rectangle{ width: 10, length: 20};
    let rec2 = Rectangle{ width: 1, length: 19};
    set_up();
    assert!(rec1.can_hold(&rec2))
}
// Each file under tests folder will create a separate crate, which is different with src folder
// run a specific test: `cargo test fn_name`
// run a test file: `cargo test --test shape_test`