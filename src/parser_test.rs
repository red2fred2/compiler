#![cfg(test)]

use crate::ast::*;

// Functions

#[test]
fn test_b() {
    let x = ();
    assert_eq!(Box::new(x), b(x))
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
