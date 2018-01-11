//! As the purpose of this crate is to expose the `contract` macro, see [its
//! documentation](./macro.contract.html). You may also be interested in
//! checking out:
//!
//! * The README.md in source, most easily viewed at Github
//!     [here](https://github.com/ErichDonGubler/adhesion-rs)
//! * This crate's [example files](https://github.com/ErichDonGubler/adhesion-rs/tree/master/examples)
//! * This crate's [test suite](https://github.com/ErichDonGubler/adhesion-rs/tree/master/tests)
#![deny(missing_docs)]

mod parse_generics_shim_util;

/// Converts one or more `fn` definitions inside to be contracted functions that
/// may have pre- and post-condition checks. The following blocks are valid:
///
/// 1. `pre` -- runs once before `body`.
/// 2. `body` -- the main part of the function. This is the reason the function
///     exists!
/// 3. `post` -- runs once after `body`.
/// 5. `double_check` -- runs twice; after `pre`, and before `post`.
///
/// When every contract block is being utilized, the final order of the checks
/// inserted into the contract definition are as follows:
///
/// 1. `pre`
/// 2. `double_check`
/// 3. `body`
/// 4. `double_check`
/// 5. `post`
///
/// No blocks in this macro are required, nor is any specific order required.
///
/// # Examples
///
/// ```
/// # #[macro_use]
/// # extern crate adhesion;
/// # #[macro_use]
/// # extern crate galvanic_assert;
/// #
/// # fn main () {
/// contract! {
///     fn asdf(asda: bool, stuff: u64) -> bool {
///         pre {
///             assert!(stuff < 30, "pre-condition violation");
///         }
///         body {
///             asda
///         }
///         post(return_value) {
///             assert!(return_value == (stuff % 3 == 0), "post-condition violation");
///         }
///         double_check {
///             assert!(stuff > 5, "double_check violation");
///         }
///     }
/// }
///
/// assert_that!(asdf(true, 7), panics); // post failure
/// assert_that!(asdf(true, 64), panics); // pre failure
/// assert_that!(asdf(false, 3), panics); // double_check failure
/// asdf(true, 6);
/// asdf(false, 7);
/// asdf(false, 11);
/// asdf(true, 24);
/// # }
/// ```
#[macro_export]
macro_rules! contract {
    (
        $(#[$attribute: meta])*
        $(pub$(($access_modifier: ident))*)* fn $fn_name: ident $($tail: tt)*
    ) => {
        parse_generics_shim! {
            { constr },
            then contract!(@after_bracket_generics, $(#[$attribute])* $(pub$(($access_modifier))*)* fn $fn_name,),
            $($tail)*
        }
    };
    (
        @after_bracket_generics,
        $(#[$attribute: meta])* $(pub$(($access_modifier: ident))*)* fn $fn_name: ident,
        {
            constr: [$($constr: tt)*],
        },
        $args: tt $( -> $return_type: ty)* where $($tail: tt)*
    ) => {
        parse_where_shim! {
            { clause, preds },
            then contract!(
                @after_where_generics,
                $(#[$attribute])* $(pub$(($access_modifier))*)* fn $fn_name,
                {
                    constr: [$($constr)*],
                },
                $args $( -> $return_type)*,
            ),
            where $($tail)*
        }
    };
    (
        @after_bracket_generics,
        $(#[$attribute: meta])* $(pub$(($access_modifier: ident))*)* fn $fn_name: ident,
        {
            constr: [$($constr: tt)*],
        },
        $args: tt $( -> $return_type: ty)*
        {
            $($block: tt)*
        }
        $($tail: tt)*
    ) => {
        contract! {
            @after_where_generics,
            $(#[$attribute])* $(pub$(($access_modifier))*)* fn $fn_name,
            {
                constr: [$($constr)*],
            },
            $args $( -> $return_type)*,
            {
                clause: [],
                preds: [],
            },
            {
                $($block)*
            }
            $($tail)*
        }
    };
    (
        @after_where_generics,
        $(#[$attribute: meta])* $(pub$(($access_modifier: ident))*)* fn $fn_name: ident,
        {
            constr: [$($constr: tt)*],
        },
        $args: tt $( -> $return_type: ty)*,
        {
            clause: [$($where_clause: tt)*],
            preds: $preds: tt,
        },
        {
            $($block: tt)*
        }
        $($tail: tt)*
    ) => {
        $(#[$attribute])* $(pub$(($access_modifier))*)* fn $fn_name <$($constr)*> $args $( -> $return_type )* $($where_clause)* {
            contract_body! {
                (pre {}, body {}, post (def) {}, double_check {})
                $($block)*
            }
        }
        contract!{ @repeat $($tail)* }
    };
    (
        @repeat
    ) => {};
    (
        @repeat
        $($tail: tt)+
    ) => {
        contract!{ $($tail)+ }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! contract_body {
    (
        ($($blocks: tt)*)
        #![$inner_attribute: meta]
        $($tail: tt)*
    ) => {
        contract_body! {
            @processing_blocks
            ($($blocks)*, #![$inner_attribute])
            $($tail)*
        }
    };
    (
        ($($blocks: tt)*)
        $($tail: tt)*
    ) => {
        contract_body! {
            @processing_blocks
            ($($blocks)*)
            $($tail)*
        }
    };
    (
        @processing_blocks
        (pre {}, body $body: tt, post ($return_value: ident) $post: tt, double_check $double_check: tt $(, #![$inner_attribute: meta])*)
        pre $pre: tt
        $($tail: tt)*
    ) => {
        contract_body! {
            @processing_blocks
            (pre $pre, body $body, post ($return_value) $post, double_check $double_check $(, #![$inner_attribute])*)
            $($tail)*
        }
    };
    (
        @processing_blocks
        (pre $pre: tt, body {}, post ($return_value: ident) $post: tt, double_check $double_check: tt $(, #![$inner_attribute: meta])*)
        body $body: tt
        $($tail: tt)*
    ) => {
        contract_body! {
            @processing_blocks
            (pre $pre, body $body, post ($return_value) $post, double_check $double_check $(, #![$inner_attribute])*)
            $($tail)*
        }
    };
    (
        @processing_blocks
        (pre $pre: tt, body $body: tt, post ($old_return_value: ident) {}, double_check $double_check: tt $(, #![$inner_attribute: meta])*)
        post ($return_value: ident) $post: tt
        $($tail: tt)*
    ) => {
        contract_body! {
            @processing_blocks
            (pre $pre, body $body, post ($return_value) $post, double_check $double_check $(, #![$inner_attribute])*)
            $($tail)*
        }
    };
    (
        @processing_blocks
        (pre $pre: tt, body $body: tt, post ($return_value: ident) {}, double_check $double_check: tt $(, #![$inner_attribute: meta])*)
        post $post: tt
        $($tail: tt)*
    ) => {
        contract_body! {
            @processing_blocks
            (pre $pre, body $body, post ($return_value) $post, double_check $double_check $(, #![$inner_attribute])*)
            $($tail)*
        }
    };
    (
        @processing_blocks
        (pre $pre: tt, body $body: tt, post ($return_value: ident) $post: tt, double_check {} $(, #![$inner_attribute: meta])*)
        double_check $double_check: tt
        $($tail: tt)*
    ) => {
        contract_body! {
            @processing_blocks
            (pre $pre, body $body, post ($return_value) $post, double_check $double_check $(, #![$inner_attribute])*)
            $($tail)*
        }
    };
    (
        @processing_blocks
        (pre $pre: tt, body $body: tt, post ($return_value: ident) $post: tt, double_check $double_check: tt $(, #![$inner_attribute: meta])*)
    ) => {
        {
            $(#![$inner_attribute])*

            $pre

            $double_check

            let $return_value = $body;

            $double_check

            $post

            $return_value
        }
    };
}
