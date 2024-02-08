use testing::{splish, sploosh};

#[test]
fn integration_test() {
    // - that `sploosh(splish(-1, 0), splish(1, 1), splish(3, 2))` returns the value `4`
    assert!(sploosh(splish(-1, 0), splish(1, 1), splish(3, 2)) == 4);
}
