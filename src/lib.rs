#[doc(hidden)]
#[macro_export]
macro_rules! contract_processed {
    (
        $(#[$attribute: meta])*
        fn $name: ident $args: tt $( -> $return_type: ty)* {
            $(#![$inner_attribute: meta])*
            pre $pre_body: block
            body $body: block
            post ($return_value: ident) $post_body: block
            invariant $invariant_block: block
        }
    ) => (
        $(#[$attribute])*
        fn $name $args $( -> $return_type)* {
            $(#![$inner_attribute])*
            $pre_body

            $invariant_block

            let $return_value = {
                $body
            };

            $invariant_block

            $post_body

            $return_value
        }
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! contract_processing {
    (
        (pre {}, body $body: tt, post $return_value: tt $post: tt, invariant $invariant: tt, $(#![$inner_attribute: meta])*)
        $(#[$attribute: meta])*
        fn $name: ident $args: tt $( -> $return_type: ty)* {
            pre $pre: block
            $($tail: tt)*
        }
    ) => {
        contract_processing! {
           (pre $pre, body $body, post $return_value $post, invariant $invariant, $(#![$inner_attribute])*)
            $(#[$attribute])*
            fn $name $args $( -> $return_type)* {
                $($tail)*
            }
        }
    };
    (
        (pre $pre: tt, body {}, post $return_value: tt $post: tt, invariant $invariant: tt, $(#![$inner_attribute: meta])*)
        $(#[$attribute: meta])*
        fn $name: ident $args: tt $( -> $return_type: ty)* {
            body $body: tt
            $($tail: tt)*
        }
    ) => {
        contract_processing! {
            (pre $pre, body $body, post $return_value $post, invariant $invariant, $(#![$inner_attribute])*)
            $(#[$attribute])*
            fn $name $args $( -> $return_type)* {
                $($tail)*
            }
        }
    };
    (
        (pre $pre: tt, body $body: tt, post $old_return_value: tt {}, invariant $invariant: tt, $(#![$inner_attribute: meta])*)
        $(#[$attribute: meta])*
        fn $name: ident $args: tt $( -> $return_type: ty)* {
            post ($return_value: ident) $post: tt
            $($tail: tt)*
        }
    ) => {
        contract_processing! {
            (pre $pre, body $body, post ($return_value) $post, invariant $invariant, $(#![$inner_attribute])*)
            $(#[$attribute])*
            fn $name $args $( -> $return_type)* {
                $($tail)*
            }
        }
    };
    (
        (pre $pre: tt, body $body: tt, post $return_value: tt {}, invariant $invariant: tt, $(#![$inner_attribute: meta])*)
        $(#[$attribute: meta])*
        fn $name: ident ($($args: tt : $types: ty),*)$( -> $return_type: ty)* {
            post $post: tt
            $($tail: tt)*
        }
    ) => {
        contract_processing! {
            (pre $pre, body $body, post $return_value $post, invariant $invariant, $(#![$inner_attribute])*)
            $(#[$attribute])*
            fn $name($($args : $types),*)$( -> $return_type)* {
                $($tail)*
            }
        }
    };
    (
        (pre $pre: tt, body $body: tt, post $return_value: tt $post: tt, invariant {}, $(#![$inner_attribute: meta])*)
        $(#[$attribute: meta])*
        fn $name: ident $args: tt $( -> $return_type: ty)* {
            invariant $invariant: tt
            $($tail: tt)*
        }
    ) => {
        contract_processing! {
            (pre $pre, body $body, post $return_value $post, invariant $invariant, $(#![$inner_attribute])*)
            $(#[$attribute])*
            fn $name $args $( -> $return_type)* {
                $($tail)*
            }
        }
    };
    (
        (pre $pre: tt, body $body: tt, post $return_value: tt $post: tt, invariant $invariant: tt, $(#![$inner_attribute: meta])*)
        $(#[$attribute: meta])*
        fn $name: ident $args: tt $( -> $return_type: ty)* {
            $($tail: tt)*
        }
    ) => {
        contract_processed! {
            $(#[$attribute])*
            fn $name $args $( -> $return_type)* {
                $(#![$inner_attribute])*
                pre $pre
                body $body
                post $return_value $post
                invariant $invariant
            }
        }
    };
}

/// Converts a `fn` definition inside to be a contracted function, complete with invariant, pre-, and post-conditions.
///
/// No blocks in this macro are required, nor is any specific order required.
///
/// # Examples
///
/// ```
/// # #[macro_use]
/// # extern crate adhesion;
/// #
/// # macro_rules! assert_panic {
/// #     ($e: expr) => {
/// #         let result = ::std::panic::catch_unwind(|| $e);
/// #         assert!(result.is_err(), concat!("expression \"", stringify!($e), "\" failed to panic"));
/// #     }
/// # }
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
///         invariant {
///             assert!(stuff > 5, "invariant violation");
///         }
///     }
/// }
///
/// assert_panic!(asdf(true, 7)); // post failure
/// assert_panic!(asdf(true, 64)); // pre failure
/// assert_panic!(asdf(false, 3)); // invariant failure
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
        fn $name: ident $args: tt $( -> $return_type: ty)* {
            $(#![$inner_attribute: meta])+
            $($block_name: ident $(($param: ident))*  { $($block_content: tt)* })*
        }
    ) => {
        contract_processing! {
            (pre {}, body {}, post (def) {}, invariant {}, $(#![$inner_attribute])+)
            $(#[$attribute])*
            fn $name $args $( -> $return_type)* {
                $($block_name $(($param))* { $($block_content)* })*
            }
        }
    };
    (
        $(#[$attribute: meta])*
        fn $name: ident $args: tt $( -> $return_type: ty)* {
            $($block_name: ident $(($param: ident))*  { $($block_content: tt)* })*
        }
    ) => {
        contract_processing! {
            (pre {}, body {}, post (def) {}, invariant {},)
            $(#[$attribute])*
            fn $name $args $( -> $return_type)* {
                $($block_name $(($param))* { $($block_content)* })*
            }
        }
    };
}
