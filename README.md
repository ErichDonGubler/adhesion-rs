# Adhesion

[![Linux build status](https://travis-ci.org/ErichDonGubler/adhesion-rs.svg)](https://travis-ci.org/ErichDonGubler/adhesion-rs)
[![Windows build status](https://ci.appveyor.com/api/projects/status/github/ErichDonGubler/adhesion-rs?svg=true)](https://ci.appveyor.com/project/ErichDonGubler/adhesion-rs)
[![crates.io latest published version](https://img.shields.io/crates/v/adhesion.svg)](https://crates.io/crates/adhesion)
[![docs.rs latest published version](https://docs.rs/adhesion/badge.svg)](https://docs.rs/adhesion)

A set of macros for [design by contract](https://en.wikipedia.org/wiki/Design_by_contract)
in Rust. The design of this library was inspired by [D's contract programming
facilities](https://tour.dlang.org/tour/en/gems/contract-programming).
Here's a quick example:

```rust,skt-main
use std::i32;

contract! {
    fn add_one_to_odd(x: i32) -> i32 {
        post(y) {
            assert!(y - 1 == x, "reverse operation did not produce input");
        }
        body {
            x + 1
        }
        pre {
            assert!(x != i32::MAX, "cannot add one to input at max of number range");
            assert!(x % 2 != 0, "evens ain't appropriate here");
        }
    }
}

assert!(add_one_to_odd(3) == 4);
assert!(add_one_to_odd(5) == 6);
assert_that!(add_one_to_odd(2), panics);
assert_that!(add_one_to_odd(i32::MAX), panics);
```

In the above example, `pre` runs before `body`, and `post`, which has the
return value of this function bound to `y`, runs after. We can also define
checks with the `double_check` block, which will be checked before **and**
after `body` has run:

```rust,should_panic,skt-main
struct Counter {
    count: u32,
    max: u32
}

contract! {
    fn increment_counter(c: &mut Counter) {
        double_check {
            assert!(c.count <= c.max, "counter max has been exceeded");
        }
        body {
            c.count += 1;
        }
    }
}

let mut counter = Counter { count: 0, max: 3 };

macro_rules! assert_incremented_eq {
    ($e: expr) => ({
        increment_counter(&mut counter);
        assert!(counter.count == $e, format!("expected counter to be {}, got {}", $e, counter.count));
    })
}

assert_incremented_eq!(1);
assert_incremented_eq!(2);
assert_incremented_eq!(3);
assert_incremented_eq!(4); // panics!
```

Actually, the above example can use a top-level `double_check` block inside of
an `impl` block instead, so that invariants can be maintained for each method
without needing to duplicate code:

```rust,should_panic,skt-main
struct Counter {
    count: u32,
    max: u32
}

impl Counter {
    contract! {
        double_check {
            assert!(self.count <= self.max, "counter max has been exceeded");
        }

        fn increment(&mut self) {
            body {
                self.count.checked_add();
            }
        }
    }

}

let mut counter = Counter { count: 0, max: 3 };

macro_rules! assert_incremented_eq {
    ($e: expr) => ({
        counter.increment();
        assert!(counter.count == $e, format!("expected counter to be {}, got {}", $e, counter.count));
    })
}

assert_incremented_eq!(1);
assert_incremented_eq!(2);
assert_incremented_eq!(3);
assert_incremented_eq!(4); // panics!
```

Nifty, right? Check out [the docs](https://docs.rs/adhesion) if you want more
detail about this crate and what you can do with it.

## FAQ

### Why "Adhesion"?

This library is called "Adhesion" in reference to a particular type of contract
called a "contract of adhesion", also known as a "take-it-or-leave-it"
contract. Assertions in programming are definitely "take it or leave it" -- if
an assertion is failing, you either have to fix the conditions of the
assertion, or change the assertion itself. It sounded appropriate!

### Why has D's `invariant` been renamed to `double_check`?

After the v0.2.0 release, @eternaleye pointed out in this [Reddit thread](https://www.reddit.com/r/rust/comments/6ooinu/adhesionrs_v020_contract_programming_in_rust_with/dkjd3kc/)
that technically an "invariant" connotes a strong guarantee that must be
rigorously maintained between ALL operations in code. This sort of guarantee is
NOT provided by the behavior D's `invariant` block, as demonstrated by [the
link](http://hackingdistributed.com/2016/07/13/reentrancy-woes/) that
@eternaleye provided.

Semantics are important, especially in systems that attempt to introduce more
rigor to software development like design by contract. For this reason, the
combined pre- and post-check block that D calls `invariant` is called
`double_check` in this library.

## Licensing

This project is dual-licensed under your choice of the [MIT license](/LICENSE-MIT)
or the [Apache 2.0 license](/LICENSE-APACHE-2.0).

* Adhesion uses a modified version of components from the [rust-parse-generics](https://github.com/DanielKeep/rust-parse-generics)
    project. Both the original and modified versions here use the same dual
    license as this project.

## Contributors

* @ErichDonGubler, original author
* @dzamlo, for providing assistance with various important features.
* @DanielKeep, for his incredible help making it possible for generics to be
    parsed and used in macros generally, and for his mentoring during
    Adhesion's development of its features involving generics.
* @eternaleye, for bringing some security expertise to bear and motivating
    the `double_check` divergence from D's `invariant`.
