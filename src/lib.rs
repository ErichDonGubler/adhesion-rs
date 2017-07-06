#[macro_export]
macro_rules! contract {
    (@call_inner $arg: ident : $type: ty, $($rest: tt)*) => (($arg, contract!(@call_inner $($rest)*)));
    (@call_inner $arg: ident : $type: ty) => ($arg);
    (
        fn
        $name: ident
        (
            $($args: tt : $types: ty),*
        )
        ->
        $return_type: ty
        {
            pre $pre_body: block
            body $body: block
            post ($return_value: ident) $post_body: block
            invariant $invariant_block: block
        }
    ) => (
        fn $name($($args : $types),*) -> $return_type {
            $pre_body

            $invariant_block

            let inner = |$($args),*| -> $return_type {
                $body
            };

            let $return_value = std::ops::Fn::call(&inner, contract!(@call_inner $($args : $types),*));

            $invariant_block

            $post_body

            $return_value
        }
    );
}

