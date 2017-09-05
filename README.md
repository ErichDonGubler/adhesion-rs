# Adhesion

[![Linux build status](https://travis-ci.org/ErichDonGubler/adhesion-rs.svg)](https://travis-ci.org/ErichDonGubler/adhesion-rs)
[![Windows build status](https://ci.appveyor.com/api/projects/status/github/ErichDonGubler/adhesion-rs?svg=true)](https://ci.appveyor.com/project/ErichDonGubler/adhesion-rs)
[![crates.io latest published version](https://img.shields.io/crates/v/adhesion.svg)](https://crates.io/crates/adhesion)
[![docs.rs latest published version](https://docs.rs/adhesion/badge.svg)](https://docs.rs/adhesion)

A set of macros for [design by contact](https://en.wikipedia.org/wiki/Design_by_contract)
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
invariants with the `invariant` block, which will be checked before **and**
after `body` has run:

```rust,should_panic,skt-main
struct Counter {
    count: u32,
	max: u32
}

contract! {
    fn increment_counter(c: &mut Counter) -> () { // Unfortunately, the () return type is necessary for now (see issue #12)
        invariant {
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

When every contract block is being utilized, the order of the checks inserted
into the contract definition are as follows:

1. `pre`
2. `invariant`
3. `body`
4. `invariant`
5. `post`

More examples can be found in:
* The [examples directory](/examples)
* The [test suite](/tests/lib.rs) for this library

## Current Limitations

The only major known limitation at the time of writing is the inability to us
e more than a single function inside a single `contract!` block (issue [here](https://github.com/ErichDonGubler/adhesion-rs/issues/15)
). This limitation is planned to be lifted.

## Why "Adhesion"?

This library is called "Adhesion" in reference to a particular type of contract
called a "contract of adhesion", also known as a "take-it-or-leave-it"
contract. Assertions in programming are definitely "take it or leave it" -- if
an assertion is failing, you either have to fix the conditions of the
assertion, or change the assertion itself. It sounded appropriate!

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
