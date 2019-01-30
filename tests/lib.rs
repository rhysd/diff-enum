#![cfg(test)]

extern crate diff_enum;

use diff_enum::common_fields;

#[test]
fn basic() {
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
