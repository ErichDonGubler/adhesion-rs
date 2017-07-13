#[doc(hidden)]
#[macro_export]
macro_rules! contract_processed {
    (
        $(#[$attribute: meta])*
        fn $name: ident ($($args: tt : $types: ty),*)$( -> $return_type: ty)* {
            $(#![$inner_attribute: meta])*
            pre $pre_body: block
            body $body: block
            post ($return_value: ident) $post_body: block
            invariant $invariant_block: block
        }
    ) => (
        $(#[$attribute])*
        fn $name($($args : $types),*)$( -> $return_type)* {
            $(#![$inner_attribute])*
            $pre_body

            $invariant_block

            fn inner($($args : $types),*)$( -> $return_type)* {
                $body
            }

            let $return_value = inner($($args),*);

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
        fn $name: ident ($($args: tt : $types: ty),*)$( -> $return_type: ty)* {
            pre $pre: block
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
        (pre $pre: tt, body {}, post $return_value: tt $post: tt, invariant $invariant: tt, $(#![$inner_attribute: meta])*)
        $(#[$attribute: meta])*
        fn $name: ident ($($args: tt : $types: ty),*)$( -> $return_type: ty)* {
            body $body: tt
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
        (pre $pre: tt, body $body: tt, post (def) {}, invariant $invariant: tt, $(#![$inner_attribute: meta])*)
        $(#[$attribute: meta])*
        fn $name: ident ($($args: tt : $types: ty),*)$( -> $return_type: ty)* {
            post ($return_value: ident) $post: tt
            $($tail: tt)*
        }
    ) => {
        contract_processing! {
            (pre $pre, body $body, post ($return_value) $post, invariant $invariant, $(#![$inner_attribute])*)
            $(#[$attribute])*
            fn $name($($args : $types),*)$( -> $return_type)* {
                $($tail)*
            }
        }
    };
    (
        (pre $pre: tt, body $body: tt, post $return_value: tt $post: tt, invariant {}, $(#![$inner_attribute: meta])*)
        $(#[$attribute: meta])*
        fn $name: ident ($($args: tt : $types: ty),*)$( -> $return_type: ty)* {
            invariant $invariant: tt
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
        (pre $pre: tt, body $body: tt, post $return_value: tt $post: tt, invariant $invariant: tt, $(#![$inner_attribute: meta])*)
        $(#[$attribute: meta])*
        fn $name: ident ($($args: tt : $types: ty),*)$( -> $return_type: ty)* {
            $($tail: tt)*
        }
    ) => {
        contract_processed! {
            $(#[$attribute])*
            fn $name($($args : $types),*)$( -> $return_type)* {
                $(#![$inner_attribute])*
                pre $pre
                body $body
                post $return_value $post
                invariant $invariant
            }
        }
    };
}

#[macro_export]
macro_rules! contract {
    (
        $(#[$attribute: meta])*
        fn $name: ident ($($args: tt : $types: ty),*)$( -> $return_type: ty)* {
            $(#![$inner_attribute: meta])+
            $($block_name: ident $(($param: ident))*  { $($block_content: tt)* })*
        }
    ) => {
        contract_processing! {
            (pre {}, body {}, post (def) {}, invariant {}, $(#![$inner_attribute])+)
            $(#[$attribute])*
            fn $name($($args : $types),*)$( -> $return_type)* {
                $($block_name $(($param))* { $($block_content)* })*
            }
        }
    };
    (
        $(#[$attribute: meta])*
        fn $name: ident ($($args: tt : $types: ty),*)$( -> $return_type: ty)* {
            $($block_name: ident $(($param: ident))*  { $($block_content: tt)* })*
        }
    ) => {
        contract_processing! {
            (pre {}, body {}, post (def) {}, invariant {},)
            $(#[$attribute])*
            fn $name($($args : $types),*)$( -> $return_type)* {
                $($block_name $(($param))* { $($block_content)* })*
            }
        }
    };
}

