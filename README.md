`#[diff_enum]` macro for Rust
=============================
[![crates.io][crate-badge]][crate]
[![documentation][doc-badge]][doc]
[![CI Status][travis-ci-badge]][travis-ci]

This is a small Rust library provides one attribute macro `#[diff_enum::common_fields]` to help defining
`enum` variants by their differences. It is useful when you need to handle data which are almost the
same, but different partially.

By the attribute macro, common fields among all variants and different fields for each variant can
be defined separately. Common fields are defined once. Additionally accessor methods for common fields
are automatically defined.

For example,

```rust
extern crate diff_enum;
use diff_enum::common_fields;

#[common_fields {
    user: String,
    name: String,
    stars: u32,
    issues: u32,
}]
#[derive(Debug)]
enum RemoteRepo {
    GitHub {
        language: String,
        pull_requests: u32,
    }
    GitLab {
        merge_requests: u32,
    }
}
```

is expanded to

```rust
#[derive(Debug)]
enum RemoteRepo {
    GitHub {
        language: String,
        pull_requests: u32,
        user: String,
        name: String,
        stars: u32,
        issues: u32,
    }
    GitLab {
        merge_requests: u32,
        user: String,
        name: String,
        stars: u32,
        issues: u32,
    }
}
```

Additionally, accessor functions are defined for common fields. For example,

```rust
let repo = RemoteRepo::GitHub {
    user: "rust-lang".to_string(),
    name: "rust".to_string(),
    language: "rust".to_string(),
    issues: 4536,
    pull_requests: 129,
    stars: 33679,
};

println!("User: {}", repo.user());
```



## Alternative

Without this crate, it's typical to separate the data into a struct with common fields and a enum
variants for differences.

For above `RemoteRepo` example,

```rust
enum RemoteRepoKind {
    GitHub {
        language: String,
        pull_requests: u32,
    }
    GitLab {
        merge_requests: u32,
    }
}
struct RemoteRepo {
    user: String,
    name: String,
    stars: u32,
    issues: u32,
    kind: RemoteRepoKind,
}
```

This solution has problems as follows:

- Fields are split into 2 parts for the reason of Rust enum. Essentially number of issues and number
  of pull requests are both properties of a GitHub repository. As natural data structure they should
  be in the same flat struct.
- Naming the inner enum is difficult. Here I used 'Kind' to separate parts. But is it appropriate?
  'Kind' is too generic name with weak meaning. The weak name comes from awkwardness of the data
  structure.



## Installation

Please add this crate to dependencies in your project's `Cargo.toml`.

```toml
[dependencies]
diff-enum = "0.1"
```



## Usage

At first, please load the crate.

```rust
extern crate diff_enum;
use diff_enum::common_fields;
```

And use `#[common_fields]` attribute macro for your enum definitions.

```
#[common_fields {
    common fields here...
}]
enum ...
```

or fully qualified name if you like

```
#[diff_enum::common_fields {
    common fields here...
}]
enum ...
```

Any attributes and comments can be put to the common fields as normal `enum` fields.

Accessor methods corresponding to common fields are defined. It is a useful helper to access common
fields without using pattern match.

For example,

```rust
#[common_fields { i: i32 }]
enum E { A, B{ b: bool } }
```

Generates an accessor method for `i` as follows:

```rust
impl E {
    fn i(&self) -> &i32 {
        match self {
            E::A{ref i, ..} => i,
            E::B{ref i, ..} => i,
        }
    }
}
```

The attribute macro causes compilation errors in the following cases.

- When no common field is put
- When fields in attribute argument is not form of `field: type`
- When `#[common_fields {...}]` is set to other than `enum` definitions
- When tuple style enum variant is used in `enum` definition



## License

Distributed under [the MIT License](./LICENSE.txt).


[crate]: https://crates.io/crates/diff-enum
[crate-badge]: https://img.shields.io/crates/v/diff-enum.svg
[doc-badge]: https://docs.rs/diff-enum/badge.svg
[doc]: https://docs.rs/diff-enum
[travis-ci-badge]: https://travis-ci.org/rhysd/diff-enum.svg?branch=master
[travis-ci]: https://travis-ci.org/rhysd/diff-enum
