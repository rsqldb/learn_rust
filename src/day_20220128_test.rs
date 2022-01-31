#[derive(PartialEq, Debug)]
pub struct Rectangle {
    pub length: u32,
    pub width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, target: &Rectangle) -> bool {
        if self.length > target.length && self.width > target.width {
            true
        } else {
            false
        }
    }
}

// cargo test to run tests
#[cfg(test)] //cfg unit tests, only build while `cargo test`, ignored while cargo build
mod tests {
    use super::*;
    #[test]
    fn larger_can_hold_smaller() {
        let rec1 = Rectangle { width: 10, length: 20};
        let rec2 = Rectangle { width: 1, length: 2 };
        //Rust can test private function
        assert!(rec1.can_hold(& rec2), "Given rectangle can't hold rec");
    }

    #[test]
    fn rectangle_compare() {
        assert_eq!(Rectangle {width: 10, length: 10}, Rectangle {width: 10, length: 10});
        assert_ne!(Rectangle {width: 10, length: 10}, Rectangle {width: 1, length: 10});
    }

    #[test]
    #[should_panic(expected = "panic")] // expected is for contain string from panic
    fn test_panic() {
        panic!("panic!")
    }

    // Use result for testing
    #[test]
    #[ignore] //- if want to ignore the test
    fn test_with_result() -> Result<(), String> {
        if 2 + 2 == 3 {
            Ok(())
        } else {
            Err(String::from("failed "))
        }
    }

    //cargo test parameters
    //`cart test --help`
    //`cart test -- --help` // parameter can be used
    // Tests are run concurrently, so they should be independent with each other.
    //`cart test -- --test-threads=1` using one thread for testing
    // println won't show by default for succeeded tests, we can use `cargo test -- --show-output`

    //test one unit test - `cargo test test_with_result`
    //test multiple tests with start string - `cargo test test_` will run all tests start with test_

    //Add #[ignore] to ignore a test
    //To run #[ignore] tests using `cargo test -- --ignored`

    // Unit tests - with the same file, need #[cfg(test)] to specify unit tests
    // Integration tests - in a separate `tests` folder, don't need #[cfg(test)]

    // Binary crate can't have integration tests
}