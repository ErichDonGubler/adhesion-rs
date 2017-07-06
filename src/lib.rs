#[macro_export]
macro_rules! call_inner {
    ($arg: ident : $type: ty, $($rest: tt)*) => (($arg, call_inner!($($rest)*)));
    ($arg: ident : $type: ty) => ($arg);
}

#[macro_export]
macro_rules! contract {
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
        }
    ) => (
        fn $name($($args : $types),*) -> $return_type {
            if cfg!(debug_assertions) {
                $pre_body
            }

            let inner = |$($args),*| -> $return_type {
                $body
            };

            let $return_value = std::ops::Fn::call(&inner, call_inner!($($args : $types),*));
            // let return_value = inner call_inner!($($args : $types),*);

            if cfg!(debug_assertions) {
                $post_body
            }

            $return_value
        }
    );
}

