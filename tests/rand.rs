#![cfg(feature = "rand")]

use evalexpr::*;

fn assert_expr(expr: &str) {
    assert_eq!(eval::<i64, f64>(expr), Ok(Value::Boolean(true)))
}

#[test]
fn test_random() {
    for _ in 0..100 {
        // This has a probability of 1e-20 of failing
        assert_expr("random() != random()");
        assert_expr("0 <= random()");
        assert_expr("random() <= 1");
    }
}

#[test]
fn test_random_errors() {
    assert!(eval::<i64, f64>("random(9)").is_err());
    assert!(eval::<i64, f64>("random(\"a\", \"b\")").is_err());
}
