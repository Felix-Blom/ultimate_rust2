use testing::{splish, sploosh};

// - that `sploosh(splish(-1, 0), splish(1, 1), splish(3, 2))` returns the value `4`
#[test]
fn test_sploosh_two() {
    assert_eq!(sploosh(splish(-1, 0), splish(1, 1), splish(3, 2)), 4);
}

// Challenge: Create a benchmark that measures the speed of sploosh(8, 9, 10)
// - Speed up the implementation of sploosh(8, 9, 10) without breaking the other tests.
// - Hint: See Cargo.toml to get you started
