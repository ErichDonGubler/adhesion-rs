#[macro_export]
macro_rules! contract_processed {
    (
        fn $name: ident ($($args: tt : $types: ty),*) -> $return_type: ty {
            pre $pre_body: block
            body $body: block
            post ($return_value: ident) $post_body: block
            invariant $invariant_block: block
        }
    ) => (
        fn $name($($args : $types),*) -> $return_type {
            $pre_body

            $invariant_block

            fn inner($($args : $types),*) -> $return_type {
                $body
            }

            let $return_value = inner($($args),*);

            $invariant_block

            $post_body

            $return_value
        }
    );
}

#[macro_export]
macro_rules! contract_processing {
    (
        (pre {}, body $body: tt, post $return_value: tt $post: tt, invariant $invariant: tt)
        fn $name: ident ($($args: tt : $types: ty),*) -> $return_type: ty {
            pre $pre: block
            $($tail: tt)*
        }
    ) => {
        contract_processing! {
            (pre $pre, body $body, post $return_value $post, invariant $invariant)
            fn $name($($args : $types),*) -> $return_type {
                $($tail)*
            }
        }
    };
    (
        (pre $pre: tt, body {}, post $return_value: tt $post: tt, invariant $invariant: tt)
        fn $name: ident ($($args: tt : $types: ty),*) -> $return_type: ty {
            body $body: tt
            $($tail: tt)*
        }
    ) => {
        contract_processing! {
            (pre $pre, body $body, post $return_value $post, invariant $invariant)
            fn $name($($args : $types),*) -> $return_type {
                $($tail)*
            }
        }
    };
    (
        (pre $pre: tt, body $body: tt, post (def) {}, invariant $invariant: tt)
        fn $name: ident ($($args: tt : $types: ty),*) -> $return_type: ty {
            post ($return_value: ident) $post: tt
            $($tail: tt)*
        }
    ) => {
        contract_processing! {
            (pre $pre, body $body, post ($return_value) $post, invariant $invariant)
            fn $name($($args : $types),*) -> $return_type {
                $($tail)*
            }
        }
    };
    (
        (pre $pre: tt, body $body: tt, post $return_value: tt $post: tt, invariant {})
        fn $name: ident ($($args: tt : $types: ty),*) -> $return_type: ty {
            invariant $invariant: tt
            $($tail: tt)*
        }
    ) => {
        contract_processing! {
            (pre $pre, body $body, post $return_value $post, invariant $invariant)
            fn $name($($args : $types),*) -> $return_type {
                $($tail)*
            }
        }
    };
    (
        (pre $pre: tt, body $body: tt, post $return_value: tt $post: tt, invariant $invariant: tt)
        fn $name: ident ($($args: tt : $types: ty),*) -> $return_type: ty {
            $($tail: tt)*
        }
    ) => {
        contract_processed! {
            fn $name($($args : $types),*) -> $return_type {
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
        fn $name: ident ($($args: tt : $types: ty),*) -> $return_type: ty {
            $($block_name: ident $(($return_value: ident))*  { $($block_content: tt)* })*
        }
    ) => {
        contract_processing! {
            (pre {}, body {}, post (def) {}, invariant {})
            fn $name($($args : $types),*) -> $return_type {
                $($block_name $(($return_value))* { $($block_content)* })*
            }
        }
    };
}

