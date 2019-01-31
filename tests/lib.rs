#![cfg(test)]

// Separated from src/lib.rs since proc macro cannot be used at the same place
// where it is defined.

extern crate diff_enum;

use diff_enum::common_fields;

#[test]
fn public_enum() {
    #[common_fields {
        x: i32,
    }]
    pub enum E {
        A { b: bool },
        B,
    }

    let e = E::A { b: true, x: 42 };
    assert_eq!(e.x(), &42);

    let e = E::B { x: 12 };
    assert_eq!(e.x(), &12);
}

#[test]
fn private_enum() {
    #[common_fields {
        x: i32,
    }]
    enum E {
        A,
        B,
    }

    let e = E::A { x: 42 };
    assert_eq!(e.x(), &42);

    let e = E::B { x: 12 };
    assert_eq!(e.x(), &12);
}

#[test]
fn contain_comment() {
    #[common_fields {
        // This is comment
        /* this is comment */ x: i32, // This is comment
        // This is comment
    }]
    pub enum E {
        A { b: bool },
        B,
    }

    let e = E::A { b: true, x: 42 };
    assert_eq!(e.x(), &42);

    let e = E::B { x: 12 };
    assert_eq!(e.x(), &12);
}

#[test]
fn contain_doc_comment() {
    #[common_fields {
        /// This is comment
        x: i32,
    }]
    pub enum E {
        A { b: bool },
        B,
    }

    let e = E::A { b: true, x: 42 };
    assert_eq!(e.x(), &42);

    let e = E::B { x: 12 };
    assert_eq!(e.x(), &12);
}

#[test]
fn contain_attribute() {
    #[common_fields {
        #[doc(hidden)]
        x: i32,
    }]
    pub enum E {
        A { b: bool },
        B,
    }

    let e = E::A { b: true, x: 42 };
    assert_eq!(e.x(), &42);

    let e = E::B { x: 12 };
    assert_eq!(e.x(), &12);
}

#[test]
fn derive_enum() {
    #[common_fields {
        #[doc(hidden)]
        x: i32,
    }]
    #[derive(Debug)]
    enum E {
        A { b: bool },
        B,
    }

    let s = format!("{:?}", E::A { b: true, x: 42 });
    assert_eq!(&s, "A { b: true, x: 42 }");

    let s = format!("{:?}", E::B { x: 12 });
    assert_eq!(&s, "B { x: 12 }");
}

#[test]
fn avoid_accessor_dead_code_warning() {
    #[common_fields {
        x: i32,
    }]
    #[deny(dead_code)]
    enum E {
        A,
        B,
    };

    // Use E
    E::A { x: 42 };
}
