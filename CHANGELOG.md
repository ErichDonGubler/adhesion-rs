# Adhesion Changelog

All notable changes to this project will be documented in this file.

This format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html)
.

## [Unreleased]

### Changes

* *BREAKING*: The `invariant` block has now been renamed to `double_check` ([
    issue #24](https://github.com/ErichDonGubler/adhesion-rs/issues/24))
    . The [README](https://github.com/ErichDonGubler/adhesion-rs/blob/v0.4.0/README.md)
    has been updated to explain why this library now diverges from its original
    inspiration in D.

### Added

* Contracts [now accept more than one `fn` definition](https://github.com/ErichDonGubler/adhesion-rs/issues/17)
    , which is MUCH more ergonomic for public interfaces in structs and traits.
* A code of conduct (see `CONTRIBUTING.md`) adapted from the Contributor
    Covenant 1.4.

### Fixed

* Visibility modifiers (`pub` and its variants) were not accepted before this
    point. [Now they are](https://github.com/ErichDonGubler/adhesion-rs/issues/29)
    .

## [0.3.0] - 2017-08-15

This just in: generics are now supported in Adhesion! With the help of macros
drawn from the [`parse-generics-shim` crate](https://crates.io/crates/parse-generics-shim),
you can now write functions kind of like the ones you can find in the [new
tests](https://github.com/ErichDonGubler/adhesion-rs/blob/v0.3.0/tests/lib.rs#L158)
associated with this release:

```rust
contract! {
    fn add_together1<T: ::std::ops::Add>(left: T, right: T) -> <T as std::ops::Add>::Output {
        // ...
    }
}

contract! {
    fn add_together2<T>(left: T, right: T) -> T::Output where T: ::std::ops::Add {
        // ...
    }
}
```

There are some [limitations](https://docs.rs/parse-generics-shim/0.1.1/parse_generics_shim/index.html)
associated with the current handling of generics in macros, but they should
accommodate a majority of use cases until a better solution is provided by Rust
itself for generics parsing in macros.

### Acknowledgements

@DanielKeep and his `parse-generics-shim` crate has been essential in making
this release possible. Thank you so much!

## [0.2.0] - 2017-07-21

This particular release fills me with pride, because it's my second publish into
the `crates.io` ecosystem and my first attempt to really polish a crate. I've
learned WAY MORE about Rust macros by example than I planned, but the journey
to getting this crate somewhat usable has been a blast. I'm excited to tell you
about what's changed since 0.1.0!

### So...what's changed?

* Most of this release has been getting the `contract!` macro up to snuff with
    most usecases. Previously, the `contract!` macro was pretty stiff and
    restrictive with the `fn` declarations it accepted, and required you to
    specify every single block of a contract in order:

    ```rust
    contract! {
        fn do_something() -> () { // Yep, the `()` was necessary
            // So were each of these blocks, even if you didn't need them
            pre {}
            body {}
            post(result) {} // Param binding here non-negotiable
            invariant {}
        }
    }
    ```

    * Now, many more valid `fn` declarations are accepted, and every contract
        block is optional -- even the `post` block's parameter can be omitted!
        This makes things MUCH more usable, and hopefully the entire new suite
        of [tests](https://github.com/ErichDonGubler/adhesion-rs/tree/master/tests),
        [examples](https://github.com/ErichDonGubler/adhesion-rs/tree/master/examples),
        [README updates](https://github.com/ErichDonGubler/adhesion-rs/blob/master/README.md),
        and [new documentation](https://docs.rs/adhesion) speak for themselves!

    * The function we previously had to write everything out for:
        ```rust
        contract! {
            fn do_something() {} //
        }
        ```

    * One of the new examples called `square_root`, which was derived from the
        original inspiration of this library [here](https://tour.dlang.org/tour/en/gems/contract-programming):
        ```rust
        #[macro_use]
        extern crate adhesion;

        contract! {
            fn square_root(x: i64) -> i64 {
                pre {
                    assert!(x >= 0);
                }
                post(result) { // Look ma, `post` came before `body`!
                    assert!((result * result) <= x && (result + 1) * (result + 1) > x);
                }
                body {
                    (x as f64).sqrt() as i64
                }
            }
        }

        fn main() {
            assert!(square_root(0) == 0);
            assert!(square_root(1) == 1);
            assert!(square_root(25) == 5);
        }
        ```
* There's still two major limitations that I hope can be overcome with some
    development and learning on my part; hopefully they'll progress quickly:
    1. Inability to use function-specific generics (see issue [here](https://github.com/ErichDonGubler/adhesion-rs/issues/18))
    2. Inability to use more than a single function inside a single `contract!`
        block (issue [here](https://github.com/ErichDonGubler/adhesion-rs/issues/15))

Any help or guidance you can offer would be greatly appreciated! It's awesome
to me that this is what's left for me to consider Adhesion to have most of the
features I originally envisioned for it. Whoohoo!

### What's next?

I'll be focusing on overcoming the limitations mentioned previously, and
[figuring out if there's a way to get `invariant` blocks into `struct`
definitions](https://github.com/ErichDonGubler/adhesion-rs/issues/23). I'll
prioritize `struct` support and ergonomics for v0.3.0, which hopefully will
come over the course of the next month as time allows.

### Acknowledgements

* I've learned a lot about setting up CI and getting my docs and tests
    straight, and honestly `cargo` made it a breeze! I've appreciated more and
    more the tooling that comes with Rust, and am excited to keep developing
    Adhesion and making it something nice to use.
* Shout-out to the awesome [`skeptic` crate](https://github.com/brson/rust-skeptic)
    made by @brson, which lets you include tests in Markdown documents like the
    examples in the README written in this release.

[Unreleased]: https://github.com/erichdongubler/adhesion-rs/compare/v0.3.0...master
[0.3.0]: https://github.com/erichdongubler/adhesion-rs/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/erichdongubler/adhesion-rs/compare/c34f4006af894faa23b534fc2243720e4b7b5370...v0.2.0
