//! As the purpose of this crate is to expose the `contract` macro, see [its
//! documentation](./attr.contract.html). You may also be interested in
//! checking out:
//!
//! * The README.md in source, most easily viewed at Github
//!     [here](https://github.com/ErichDonGubler/adhesion-rs)
//! * This crate's [example files](https://github.com/ErichDonGubler/adhesion-rs/tree/master/examples)
//! * This crate's [test suite](https://github.com/ErichDonGubler/adhesion-rs/tree/master/tests)
// #![deny(missing_docs)]
#![doc(html_root_url = "https://docs.rs/adhesion/0.5.0")]

extern crate proc_macro;

use {
    proc_macro::TokenStream,
    proc_macro2::TokenStream as TokenStream2,
    quote::{quote, ToTokens, TokenStreamExt},
    std::fmt::{
        Debug,
        Formatter,
        Result as FmtResult,
    },
    syn::{
        braced, parenthesized,
        parse::{Parse, ParseStream, Result as ParseResult},
        parse2,
        token::{Async, Const, Default as DefaultToken, Unsafe},
        Abi, Attribute, Block, FnArg, FnDecl, Ident, Item, MethodSig, Visibility,
    },
};

/// Converts one or more `fn` definitions inside to be contracted functions that
/// may have pre- and post-condition checks. The following blocks are valid inside
/// of a `fn` definition:
///
/// 1. `pre` -- runs once before `body`.
/// 2. `body` -- the main part of the function. This is the reason the function
///     exists!
/// 3. `post` -- runs once after `body`.
/// 5. `double_check` -- runs twice; after `pre`, and before `post`.
///
/// A `#[contract]` may be used on an `impl` block , which will be used by ALL `fn` definitions
/// inside. This block is particularly useful for structs, where invariants for all data members
/// may need to be maintained across method calls.
///
/// When every contract block is being utilized, the final order of the checks
/// inserted into the contract definition are as follows:
///
/// 1. `pre`
/// 2. `double_check` of the `contract!` block
/// 3. `double_check` of the `fn` definition
/// 4. `body`
/// 5. `double_check` of the `contract!` block
/// 6. `double_check` of the `fn` definition
/// 7. `post`
///
/// No blocks in this macro are required, nor is any specific order required.
///
/// It should be noted that conditional compilation is NOT handled by this
/// library, and that if conditional compilation is desired, [`cfg` statements](https://doc.rust-lang.org/beta/reference/attributes.html#conditional-compilation)
/// should be used like with any most other Rust code.
///
/// # Examples
///
/// ```
/// # use {
/// #     adhesion::contract,
/// #     galvanic_assert::assert_that,
/// # };
/// #
/// contract! {
///     fn asdf(asda: bool, stuff: u64) -> bool {
///         pre {
///             assert!(stuff < 30, "pre-condition violation");
///         }
///         body {
///             asda
///         }
///         post(return_value) {
///             assert!(return_value == &(stuff % 3 == 0), "post-condition violation");
///         }
///         double_check {
///             assert!(stuff > 5, "double_check violation");
///         }
///     }
/// }
///
/// # fn main () {
/// assert_that!(asdf(true, 7), panics); // post failure
/// assert_that!(asdf(true, 64), panics); // pre failure
/// assert_that!(asdf(false, 3), panics); // double_check failure
/// asdf(true, 6);
/// asdf(false, 7);
/// asdf(false, 11);
/// asdf(true, 24);
/// # }
/// ```
#[proc_macro]
pub fn contract(item: TokenStream) -> TokenStream {
    let parsed: ParsedContractItem = parse2(TokenStream2::from(item)).unwrap();
    TokenStream::from(quote! {
        #parsed
    })
}

fn parse_fn_decl(fn_token: syn::token::Fn, input: ParseStream) -> ParseResult<FnDecl> {
    let content;
    Ok(FnDecl {
        fn_token,
        generics: input.parse()?,
        paren_token: parenthesized!(content in input),
        inputs: content.parse_terminated(FnArg::parse)?,
        variadic: input.parse()?,
        output: input.parse()?,
    })
}

/// The fields of this `struct` are intended to mirror that of `ImplItemMethod`, except for the body.
#[derive(Clone)]
struct ImplMethodSig {
    attrs: Vec<Attribute>,
    vis: Visibility,
    defaultness: Option<DefaultToken>,
    sig: MethodSig,
}

impl Debug for ImplMethodSig {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", self.to_tokens(&mut TokenStream2::new()))
    }
}

impl Parse for ImplMethodSig {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let mut attrs = input.call(Attribute::parse_outer)?;
        let vis = input.parse()?;
        let defaultness = input.parse()?;
        let sig = {
            let constness = input.parse()?;
            let unsafety = input.parse()?;
            let asyncness = input.parse()?;
            let abi = input.parse()?;
            let fn_token = input.parse()?;
            let ident = input.parse()?;
            let decl = parse_fn_decl(fn_token, input)?;
            MethodSig {
                constness,
                unsafety,
                asyncness,
                abi,
                ident,
                decl,
            }
        };
        attrs.extend(input.call(Attribute::parse_inner)?);
        Ok(ImplMethodSig {
            attrs,
            vis,
            defaultness,
            sig,
        })
    }
}

impl ToTokens for ImplMethodSig {
    fn to_tokens(&self, stream: &mut TokenStream2) {
        let Self {
            attrs,
            vis,
            defaultness,
            sig,
        } = self;
        stream.append_all({
            quote! {
                #(#attrs)*
                #vis
                #defaultness
                #sig
            }
        })
    }
}

/// The fields of this `struct` are intended to mirror that of ItemFn, except for the body.
#[derive(Clone)]
struct FnSig {
    attrs: Vec<Attribute>,
    vis: Visibility,
    constness: Option<Const>,
    unsafety: Option<Unsafe>,
    asyncness: Option<Async>,
    abi: Option<Abi>,
    ident: Ident,
    decl: Box<FnDecl>,
}

impl Debug for FnSig {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", self.to_tokens(&mut TokenStream2::new()))
    }
}

impl Parse for FnSig {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let mut attrs = input.call(Attribute::parse_outer)?;

        let vis = input.parse()?;
        let constness = input.parse()?;
        let unsafety = input.parse()?;
        let asyncness = input.parse()?;
        let abi = input.parse()?;
        let fn_token = input.parse()?;
        let ident = input.parse()?;
        let decl = Box::new(parse_fn_decl(fn_token, input)?);

        attrs.extend(input.call(Attribute::parse_inner)?);
        Ok(FnSig {
            attrs,
            vis,
            constness,
            unsafety,
            asyncness,
            abi,
            ident,
            decl,
        })
    }
}

impl ToTokens for FnSig {
    fn to_tokens(&self, stream: &mut TokenStream2) {
        let FnSig {
            attrs,
            vis,
            constness,
            unsafety,
            asyncness,
            abi,
            ident,
            decl,
        } = self;

        let FnDecl {
            fn_token,
            generics,
            paren_token: _,
            inputs,
            variadic,
            output,
        } = decl.as_ref();

        stream.append_all(quote! {
            #(#attrs)*
            #vis
            #constness
            #unsafety
            #asyncness
            #abi
            #fn_token
            #ident
            #generics
            (#inputs)
            #variadic
            #output
        });
    }
}

/// The base model for all contracts that Adhesion uses.
#[derive(Clone)]
struct Contract {
    inner_attributes: Vec<Attribute>,
    /// The core logic of the function.
    body: Box<Block>,
    /// Pre-condition checks for this function.
    pre: Vec<Block>,
    /// Post-condition checks for this function.
    post: Vec<(Option<Ident>, Block)>,
    /// Pre- and post-condition checks for this function.
    double_check: Vec<Block>,
}

impl Parse for Contract {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let inner_attributes = input.call(Attribute::parse_inner)?;
        // TODO: Implement contract parsing
        let mut body = None;
        let mut pre = Vec::new();
        let mut post = Vec::new();
        let mut double_check = Vec::new();

        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            match ident.to_string().as_ref() {
                "pre" => {
                    pre.push(input.parse()?);
                }
                "post" => {
                    let mut ret_val_ident = None;
                    let _ = (|| {
                        let paren_contents;
                        parenthesized!(paren_contents in input);
                        ret_val_ident = paren_contents.parse()?;
                        Ok(())
                    })();
                    post.push((ret_val_ident, input.parse()?));
                }
                "double_check" => double_check.push(input.parse()?),
                "body" => {
                    if body.is_some() {
                        panic!("cannot have more than one `body` block");
                    } else {
                        body = Some(input.parse()?);
                    }
                }
                i => panic!("unrecognized contract group {:?}", i),
            }
        }

        Ok(Contract {
            inner_attributes,
            body: body.unwrap_or_else(|| parse2(quote!({})).unwrap()),
            pre,
            post,
            double_check,
        })
    }
}

impl Contract {
    fn parse_from_fn_body(input: ParseStream) -> ParseResult<Contract> {
        let body;
        braced!(body in input);
        body.parse()
    }
}

#[derive(Clone)]
struct FullContract<'c, 'idc> {
    contract: &'c Contract,
    impl_double_check: Option<&'idc [Block]>,
}

impl<'c, 'idc> Debug for FullContract<'c, 'idc> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", self.to_tokens(&mut TokenStream2::new()))
    }
}

impl<'c, 'idc> ToTokens for FullContract<'c, 'idc> {
    fn to_tokens(&self, stream: &mut TokenStream2) {
        let Self {
            contract:
                Contract {
                    inner_attributes,
                    pre,
                    post,
                    double_check,
                    body,
                },
            impl_double_check,
        } = self;

        let post_assigns = post.iter().map(|(i, _)| quote!(let #i = &ret;));
        let post_blocks = post.iter().map(|(_, b)| b);
        let impl_double_check = impl_double_check.unwrap_or(&[]);

        stream.append_all(quote! {
            #(#inner_attributes)*
            #(#pre)*
            #(#impl_double_check)*
            #(#double_check)*
            let ret = #body;
            #(#impl_double_check)*
            #(#double_check)*
            #(
                {
                    #post_assigns
                    #post_blocks
                }
            )*
            ret
        })
    }
}

/// Represents a "contractualized" `impl` block.
#[derive(Clone)]
struct ContractImplBlock {
    double_check: Vec<Block>,
    items: Vec<Item>,
    contracted_methods: Vec<ContractImplMethod>,
}

impl Debug for ContractImplBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", self.to_tokens(&mut TokenStream2::new()))
    }
}

impl ToTokens for ContractImplBlock {
    fn to_tokens(&self, stream: &mut TokenStream2) {
        let ContractImplBlock {
            items,
            contracted_methods,
            double_check,
        } = self;

        let contracted_methods = contracted_methods.iter().map(
            |ContractImplMethod {
                 signature,
                 contract,
             }| {
                let contract = FullContract {
                    contract,
                    impl_double_check: Some(double_check.as_slice()),
                };
                quote! {
                    #signature { #contract }
                }
            },
        );

        stream.append_all(quote! {
            #(#items)*
            #(#contracted_methods)*
        })
    }
}

/// Represents a "contractualized" free function.
#[derive(Clone)]
struct ContractFn {
    signature: FnSig,
    contract: Contract,
}

impl Debug for ContractFn {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", self.to_tokens(&mut TokenStream2::new()))
    }
}

impl Parse for ContractFn {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        Ok(ContractFn {
            signature: input.parse()?,
            contract: Contract::parse_from_fn_body(input)?,
        })
    }
}

impl ToTokens for ContractFn {
    fn to_tokens(&self, stream: &mut TokenStream2) {
        let Self {
            signature,
            contract,
        } = self;

        let contract = FullContract {
            contract,
            impl_double_check: None,
        };

        stream.append_all(quote! {
            #signature { #contract }
        });
    }
}

/// Represents a "contractualized" function.
#[derive(Clone)]
struct ContractImplMethod {
    signature: ImplMethodSig,
    contract: Contract,
}

impl Debug for ContractImplMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", self.to_tokens(&mut TokenStream2::new()))
    }
}

impl Parse for ContractImplMethod {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let signature: ImplMethodSig = input.parse()?;
        let contract = Contract::parse_from_fn_body(input)?;
        if !input.is_empty() {
            println!("contract: {:?}", ContractImplMethod {
                signature: signature.clone(),
                contract: contract.clone(),
            });
            println!("remaining input: {}", input);
        }
        assert!(input.is_empty());

        Ok(ContractImplMethod {
            signature,
            contract,
        })
    }
}

impl ToTokens for ContractImplMethod {
    fn to_tokens(&self, stream: &mut TokenStream2) {
        stream.append_all({
            let ContractImplMethod {
                signature,
                contract,
            } = self;

            let contract = FullContract {
                contract,
                impl_double_check: None,
            };
            quote! {
                #signature { #contract }
            }
        })
    }
}

#[derive(Debug)]
enum ParsedContractItem {
    ImplBlock(ContractImplBlock),
    ImplMethod(ContractImplMethod),
    Fn(ContractFn),
}

impl Parse for ParsedContractItem {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        use self::ParsedContractItem::*;

        Ok(ImplMethod(input.parse()?))
    }
}

impl ToTokens for ParsedContractItem {
    fn to_tokens(&self, stream: &mut TokenStream2) {
        use self::ParsedContractItem::*;

        stream.append_all(match self {
            ImplBlock(b) => quote!(#b),
            ImplMethod(m) => quote!(#m),
            Fn(f) => quote!(#f),
        })
    }
}
