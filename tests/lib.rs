#![cfg(test)]

extern crate diff_enum;

use diff_enum::common_fields;

#[test]
fn basic() {
    #[common_fields {
        x: i32,
    }]
    enum E {
        A { b: bool },
        B,
    }

    let e = E::A { b: true, x: 42 };
    assert_eq!(e.x(), &42);

    let e = E::B { x: 12 };
    assert_eq!(e.x(), &12);
}
