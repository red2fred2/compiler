#![cfg(test)]

use crate::ast::*;

// Functions

#[test]
fn test_b() {
    let x = ();
    assert_eq!(Box::new(x), b(x))
}

#[test]
fn test_fmt_body() {
    let vec = vec![1, 2, 3];
    let expected = "{\n    1\n    2\n    3\n}";
    assert_eq!(fmt_body(&vec), expected);
}

#[test]
fn test_fmt_list() {
    let vec = vec![1, 2, 3];
    let expected = "(1, 2, 3)";
    assert_eq!(fmt_list(&vec), expected);
}

// Methods

#[test]
fn test_expression_new_int() {
    let test = Expression::new_int("420");
    let expected = Expression::IntegerLiteral(420);

    assert_eq!(test, expected)
}

#[test]
fn test_expression_new_string() {
    let test = Expression::new_string("\"test\"");
    let expected = Expression::StringLiteral("test".to_string());

    assert_eq!(test, expected)
}

#[test]
fn test_location_new_from_id() {
    let id = Id {
        name: "test".to_string(),
    };
    let test = Location::new_from_id(id);
    let expected = Location {
        name: "test".to_string(),
    };

    assert_eq!(test, expected)
}
