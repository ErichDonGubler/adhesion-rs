/*
Copyright ⓒ 2016 rust-custom-derive contributors.
Modifications copyright ⓒ 2017 adhesion-rs contributors.
Licensed under the MIT license (see LICENSE or <http://opensource.org
/licenses/MIT>) or the Apache License, Version 2.0 (see LICENSE of
<http://www.apache.org/licenses/LICENSE-2.0>), at your option. All
files in the project carrying such notice may not be copied, modified,
or distributed except according to those terms.
*/
#[doc(hidden)]
#[macro_export]
macro_rules! parse_generics_shim_util {
    (
        @callback
        ($cb_name:ident ! ($($cb_arg:tt)*)),
        $($tail:tt)*
    ) => {
        $cb_name! { $($cb_arg)* $($tail)* }
    };

    (
        @callback
        ($cb_name:ident ! [$($cb_arg:tt)*]),
        $($tail:tt)*
    ) => {
        $cb_name! { $($cb_arg)* $($tail)* }
    };

    (
        @callback
        ($cb_name:ident ! {$($cb_arg:tt)*}),
        $($tail:tt)*
    ) => {
        $cb_name! { $($cb_arg)* $($tail)* }
    };
}

mod parse_constr;
mod parse_generics_shim;
mod parse_where_shim;
