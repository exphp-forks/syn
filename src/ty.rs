use super::*;
use crate::punctuated::Punctuated;
#[cfg(feature = "extra-traits")]
use crate::tt::TokenStreamHelper;
use proc_macro2::TokenStream;
#[cfg(feature = "extra-traits")]
use std::hash::{Hash, Hasher};

ast_enum_of_structs! {
    /// The possible types that a Rust value could have.
    ///
    /// *This type is available if Syn is built with the `"derive"` or `"full"`
    /// feature.*
    ///
    /// # Syntax tree enum
    ///
    /// This type is a [syntax tree enum].
    ///
    /// [syntax tree enum]: enum.Expr.html#syntax-tree-enums
    //
    // TODO: change syntax-tree-enum link to an intra rustdoc link, currently
    // blocked on https://github.com/rust-lang/rust/issues/62833
    pub enum Type #manual_extra_traits {
        /// A fixed size array type: `[T; n]`.
        Array(TypeArray),

        /// A bare function type: `fn(usize) -> bool`.
        BareFn(TypeBareFn),

        /// A type contained within invisible delimiters.
        Group(TypeGroup),

        /// An `impl Bound1 + Bound2 + Bound3` type where `Bound` is a trait or
        /// a lifetime.
        ImplTrait(TypeImplTrait),

        /// Indication that a type should be inferred by the compiler: `_`.
        Infer(TypeInfer),

        /// A macro in the type position.
        Macro(TypeMacro),

        /// The never type: `!`.
        Never(TypeNever),

        /// A parenthesized type equivalent to the inner type.
        Paren(TypeParen),

        /// A path like `std::slice::Iter`, optionally qualified with a
        /// self-type as in `<Vec<T> as SomeTrait>::Associated`.
        Path(TypePath),

        /// A raw pointer type: `*const T` or `*mut T`.
        Ptr(TypePtr),

        /// A reference type: `&'a T` or `&'a mut T`.
        Reference(TypeReference),

        /// A dynamically sized slice type: `[T]`.
        Slice(TypeSlice),

        /// A trait object type `Bound1 + Bound2 + Bound3` where `Bound` is a
        /// trait or a lifetime.
        TraitObject(TypeTraitObject),

        /// A tuple type: `(A, B, C, String)`.
        Tuple(TypeTuple),

        /// Tokens in type position not interpreted by Syn.
        Verbatim(TokenStream),

        #[doc(hidden)]
        __Nonexhaustive,
    }
}

ast_struct! {
    /// A fixed size array type: `[T; n]`.
    ///
    /// *This type is available if Syn is built with the `"derive"` or
    /// `"full"` feature.*
    pub struct TypeArray {
        pub bracket_token: token::Bracket,
        pub elem: Box<Type>,
        pub semi_token: Token![;],
        pub len: Expr,
    }
}

ast_struct! {
    /// A bare function type: `fn(usize) -> bool`.
    ///
    /// *This type is available if Syn is built with the `"derive"` or
    /// `"full"` feature.*
    pub struct TypeBareFn {
        pub lifetimes: Option<BoundLifetimes>,
        pub unsafety: Option<Token![unsafe]>,
        pub abi: Option<Abi>,
        pub fn_token: Token![fn],
        pub paren_token: token::Paren,
        pub inputs: Punctuated<BareFnArg, Token![,]>,
        pub variadic: Option<Token![...]>,
        pub output: ReturnType,
    }
}

ast_struct! {
    /// A type contained within invisible delimiters.
    ///
    /// *This type is available if Syn is built with the `"derive"` or
    /// `"full"` feature.*
    pub struct TypeGroup {
        pub group_token: token::Group,
        pub elem: Box<Type>,
    }
}

ast_struct! {
    /// An `impl Bound1 + Bound2 + Bound3` type where `Bound` is a trait or
    /// a lifetime.
    ///
    /// *This type is available if Syn is built with the `"derive"` or
    /// `"full"` feature.*
    pub struct TypeImplTrait {
        pub impl_token: Token![impl],
        pub bounds: Punctuated<TypeParamBound, Token![+]>,
    }
}

ast_struct! {
    /// Indication that a type should be inferred by the compiler: `_`.
    ///
    /// *This type is available if Syn is built with the `"derive"` or
    /// `"full"` feature.*
    pub struct TypeInfer {
        pub underscore_token: Token![_],
    }
}

ast_struct! {
    /// A macro in the type position.
    ///
    /// *This type is available if Syn is built with the `"derive"` or
    /// `"full"` feature.*
    pub struct TypeMacro {
        pub mac: Macro,
    }
}

ast_struct! {
    /// The never type: `!`.
    ///
    /// *This type is available if Syn is built with the `"derive"` or
    /// `"full"` feature.*
    pub struct TypeNever {
        pub bang_token: Token![!],
    }
}

ast_struct! {
    /// A parenthesized type equivalent to the inner type.
    ///
    /// *This type is available if Syn is built with the `"derive"` or
    /// `"full"` feature.*
    pub struct TypeParen {
        pub paren_token: token::Paren,
        pub elem: Box<Type>,
    }
}

ast_struct! {
    /// A path like `std::slice::Iter`, optionally qualified with a
    /// self-type as in `<Vec<T> as SomeTrait>::Associated`.
    ///
    /// *This type is available if Syn is built with the `"derive"` or
    /// `"full"` feature.*
    pub struct TypePath {
        pub qself: Option<QSelf>,
        pub path: Path,
    }
}

ast_struct! {
    /// A raw pointer type: `*const T` or `*mut T`.
    ///
    /// *This type is available if Syn is built with the `"derive"` or
    /// `"full"` feature.*
    pub struct TypePtr {
        pub star_token: Token![*],
        pub const_token: Option<Token![const]>,
        pub mutability: Option<Token![mut]>,
        pub elem: Box<Type>,
    }
}

ast_struct! {
    /// A reference type: `&'a T` or `&'a mut T`.
    ///
    /// *This type is available if Syn is built with the `"derive"` or
    /// `"full"` feature.*
    pub struct TypeReference {
        pub and_token: Token![&],
        pub lifetime: Option<Lifetime>,
        pub mutability: Option<Token![mut]>,
        pub elem: Box<Type>,
    }
}

ast_struct! {
    /// A dynamically sized slice type: `[T]`.
    ///
    /// *This type is available if Syn is built with the `"derive"` or
    /// `"full"` feature.*
    pub struct TypeSlice {
        pub bracket_token: token::Bracket,
        pub elem: Box<Type>,
    }
}

ast_struct! {
    /// A trait object type `Bound1 + Bound2 + Bound3` where `Bound` is a
    /// trait or a lifetime.
    ///
    /// *This type is available if Syn is built with the `"derive"` or
    /// `"full"` feature.*
    pub struct TypeTraitObject {
        pub dyn_token: Option<Token![dyn]>,
        pub bounds: Punctuated<TypeParamBound, Token![+]>,
    }
}

ast_struct! {
    /// A tuple type: `(A, B, C, String)`.
    ///
    /// *This type is available if Syn is built with the `"derive"` or
    /// `"full"` feature.*
    pub struct TypeTuple {
        pub paren_token: token::Paren,
        pub elems: Punctuated<Type, Token![,]>,
    }
}

#[cfg(feature = "extra-traits")]
impl Eq for Type {}

#[cfg(feature = "extra-traits")]
impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Type::Array(this), Type::Array(other)) => this == other,
            (Type::BareFn(this), Type::BareFn(other)) => this == other,
            (Type::Group(this), Type::Group(other)) => this == other,
            (Type::ImplTrait(this), Type::ImplTrait(other)) => this == other,
            (Type::Infer(this), Type::Infer(other)) => this == other,
            (Type::Macro(this), Type::Macro(other)) => this == other,
            (Type::Never(this), Type::Never(other)) => this == other,
            (Type::Paren(this), Type::Paren(other)) => this == other,
            (Type::Path(this), Type::Path(other)) => this == other,
            (Type::Ptr(this), Type::Ptr(other)) => this == other,
            (Type::Reference(this), Type::Reference(other)) => this == other,
            (Type::Slice(this), Type::Slice(other)) => this == other,
            (Type::TraitObject(this), Type::TraitObject(other)) => this == other,
            (Type::Tuple(this), Type::Tuple(other)) => this == other,
            (Type::Verbatim(this), Type::Verbatim(other)) => {
                TokenStreamHelper(this) == TokenStreamHelper(other)
            }
            _ => false,
        }
    }
}

#[cfg(feature = "extra-traits")]
impl Hash for Type {
    fn hash<H>(&self, hash: &mut H)
    where
        H: Hasher,
    {
        match self {
            Type::Array(ty) => {
                hash.write_u8(0);
                ty.hash(hash);
            }
            Type::BareFn(ty) => {
                hash.write_u8(1);
                ty.hash(hash);
            }
            Type::Group(ty) => {
                hash.write_u8(2);
                ty.hash(hash);
            }
            Type::ImplTrait(ty) => {
                hash.write_u8(3);
                ty.hash(hash);
            }
            Type::Infer(ty) => {
                hash.write_u8(4);
                ty.hash(hash);
            }
            Type::Macro(ty) => {
                hash.write_u8(5);
                ty.hash(hash);
            }
            Type::Never(ty) => {
                hash.write_u8(6);
                ty.hash(hash);
            }
            Type::Paren(ty) => {
                hash.write_u8(7);
                ty.hash(hash);
            }
            Type::Path(ty) => {
                hash.write_u8(8);
                ty.hash(hash);
            }
            Type::Ptr(ty) => {
                hash.write_u8(9);
                ty.hash(hash);
            }
            Type::Reference(ty) => {
                hash.write_u8(10);
                ty.hash(hash);
            }
            Type::Slice(ty) => {
                hash.write_u8(11);
                ty.hash(hash);
            }
            Type::TraitObject(ty) => {
                hash.write_u8(12);
                ty.hash(hash);
            }
            Type::Tuple(ty) => {
                hash.write_u8(13);
                ty.hash(hash);
            }
            Type::Verbatim(ty) => {
                hash.write_u8(14);
                TokenStreamHelper(ty).hash(hash);
            }
            Type::__Nonexhaustive => unreachable!(),
        }
    }
}

ast_struct! {
    /// The binary interface of a function: `extern "C"`.
    ///
    /// *This type is available if Syn is built with the `"derive"` or `"full"`
    /// feature.*
    pub struct Abi {
        pub extern_token: Token![extern],
        pub name: Option<LitStr>,
    }
}

ast_struct! {
    /// An argument in a function type: the `usize` in `fn(usize) -> bool`.
    ///
    /// *This type is available if Syn is built with the `"derive"` or `"full"`
    /// feature.*
    pub struct BareFnArg {
        pub attrs: Vec<Attribute>,
        pub name: Option<(Ident, Token![:])>,
        pub ty: Type,
    }
}

ast_enum! {
    /// Return type of a function signature.
    ///
    /// *This type is available if Syn is built with the `"derive"` or `"full"`
    /// feature.*
    pub enum ReturnType {
        /// Return type is not specified.
        ///
        /// Functions default to `()` and closures default to type inference.
        Default,
        /// A particular type is returned.
        Type(Token![->], Box<Type>),
    }
}

#[cfg(feature = "parsing")]
pub mod parsing {
    use super::*;

    use crate::ext::IdentExt;
    use crate::parse::{Parse, ParseStream, Result};
    use crate::path;

    impl Parse for Type {
        fn parse(input: ParseStream) -> Result<Self> {
            ambig_ty(input, true)
        }
    }

    impl Type {
        /// In some positions, types may not contain the `+` character, to
        /// disambiguate them. For example in the expression `1 as T`, T may not
        /// contain a `+` character.
        ///
        /// This parser does not allow a `+`, while the default parser does.
        pub fn without_plus(input: ParseStream) -> Result<Self> {
            ambig_ty(input, false)
        }
    }

    fn ambig_ty(input: ParseStream, allow_plus: bool) -> Result<Type> {
        if input.peek(token::Group) {
            return input.parse().map(Type::Group);
        }

        let mut lifetimes = None::<BoundLifetimes>;
        let mut lookahead = input.lookahead1();
        if lookahead.peek(Token![for]) {
            lifetimes = input.parse()?;
            lookahead = input.lookahead1();
            if !lookahead.peek(Ident)
                && !lookahead.peek(Token![fn])
                && !lookahead.peek(Token![unsafe])
                && !lookahead.peek(Token![extern])
                && !lookahead.peek(Token![super])
                && !lookahead.peek(Token![self])
                && !lookahead.peek(Token![Self])
                && !lookahead.peek(Token![crate])
            {
                return Err(lookahead.error());
            }
        }

        if lookahead.peek(token::Paren) {
            let content;
            let paren_token = parenthesized!(content in input);
            if content.is_empty() {
                return Ok(Type::Tuple(TypeTuple {
                    paren_token,
                    elems: Punctuated::new(),
                }));
            }
            if content.peek(Lifetime) {
                return Ok(Type::Paren(TypeParen {
                    paren_token,
                    elem: Box::new(Type::TraitObject(content.parse()?)),
                }));
            }
            if content.peek(Token![?]) {
                return Ok(Type::TraitObject(TypeTraitObject {
                    dyn_token: None,
                    bounds: {
                        let mut bounds = Punctuated::new();
                        bounds.push_value(TypeParamBound::Trait(TraitBound {
                            paren_token: Some(paren_token),
                            ..content.parse()?
                        }));
                        while let Some(plus) = input.parse()? {
                            bounds.push_punct(plus);
                            bounds.push_value(input.parse()?);
                        }
                        bounds
                    },
                }));
            }
            let mut first: Type = content.parse()?;
            if content.peek(Token![,]) {
                return Ok(Type::Tuple(TypeTuple {
                    paren_token,
                    elems: {
                        let mut elems = Punctuated::new();
                        elems.push_value(first);
                        elems.push_punct(content.parse()?);
                        let rest: Punctuated<Type, Token![,]> =
                            content.parse_terminated(Parse::parse)?;
                        elems.extend(rest);
                        elems
                    },
                }));
            }
            if allow_plus && input.peek(Token![+]) {
                loop {
                    let first = match first {
                        Type::Path(TypePath { qself: None, path }) => {
                            TypeParamBound::Trait(TraitBound {
                                paren_token: Some(paren_token),
                                modifier: TraitBoundModifier::None,
                                lifetimes: None,
                                path,
                            })
                        }
                        Type::TraitObject(TypeTraitObject {
                            dyn_token: None,
                            bounds,
                        }) => {
                            if bounds.len() > 1 || bounds.trailing_punct() {
                                first = Type::TraitObject(TypeTraitObject {
                                    dyn_token: None,
                                    bounds,
                                });
                                break;
                            }
                            match bounds.into_iter().next().unwrap() {
                                TypeParamBound::Trait(trait_bound) => {
                                    TypeParamBound::Trait(TraitBound {
                                        paren_token: Some(paren_token),
                                        ..trait_bound
                                    })
                                }
                                other => other,
                            }
                        }
                        _ => break,
                    };
                    return Ok(Type::TraitObject(TypeTraitObject {
                        dyn_token: None,
                        bounds: {
                            let mut bounds = Punctuated::new();
                            bounds.push_value(first);
                            while let Some(plus) = input.parse()? {
                                bounds.push_punct(plus);
                                bounds.push_value(input.parse()?);
                            }
                            bounds
                        },
                    }));
                }
            }
            Ok(Type::Paren(TypeParen {
                paren_token,
                elem: Box::new(first),
            }))
        } else if lookahead.peek(Token![fn])
            || lookahead.peek(Token![unsafe])
            || lookahead.peek(Token![extern]) && !input.peek2(Token![::])
        {
            let mut bare_fn: TypeBareFn = input.parse()?;
            bare_fn.lifetimes = lifetimes;
            Ok(Type::BareFn(bare_fn))
        } else if lookahead.peek(Ident)
            || input.peek(Token![super])
            || input.peek(Token![self])
            || input.peek(Token![Self])
            || input.peek(Token![crate])
            || input.peek(Token![extern])
            || lookahead.peek(Token![::])
            || lookahead.peek(Token![<])
        {
            if input.peek(Token![dyn ]) {
                let mut trait_object: TypeTraitObject = input.parse()?;
                if lifetimes.is_some() {
                    match trait_object.bounds.iter_mut().next().unwrap() {
                        TypeParamBound::Trait(trait_bound) => {
                            trait_bound.lifetimes = lifetimes;
                        }
                        TypeParamBound::Lifetime(_) => unreachable!(),
                    }
                }
                return Ok(Type::TraitObject(trait_object));
            }

            let ty: TypePath = input.parse()?;
            if ty.qself.is_some() {
                return Ok(Type::Path(ty));
            }

            if input.peek(Token![!]) && !input.peek(Token![!=]) {
                let mut contains_arguments = false;
                for segment in &ty.path.segments {
                    match segment.arguments {
                        PathArguments::None => {}
                        PathArguments::AngleBracketed(_) | PathArguments::Parenthesized(_) => {
                            contains_arguments = true;
                        }
                    }
                }

                if !contains_arguments {
                    let bang_token: Token![!] = input.parse()?;
                    let (delimiter, tokens) = mac::parse_delimiter(input)?;
                    return Ok(Type::Macro(TypeMacro {
                        mac: Macro {
                            path: ty.path,
                            bang_token,
                            delimiter,
                            tokens,
                        },
                    }));
                }
            }

            if lifetimes.is_some() || allow_plus && input.peek(Token![+]) {
                let mut bounds = Punctuated::new();
                bounds.push_value(TypeParamBound::Trait(TraitBound {
                    paren_token: None,
                    modifier: TraitBoundModifier::None,
                    lifetimes,
                    path: ty.path,
                }));
                if allow_plus {
                    while input.peek(Token![+]) {
                        bounds.push_punct(input.parse()?);
                        if input.peek(Token![>]) {
                            break;
                        }
                        bounds.push_value(input.parse()?);
                    }
                }
                return Ok(Type::TraitObject(TypeTraitObject {
                    dyn_token: None,
                    bounds,
                }));
            }

            Ok(Type::Path(ty))
        } else if lookahead.peek(token::Bracket) {
            let content;
            let bracket_token = bracketed!(content in input);
            let elem: Type = content.parse()?;
            if content.peek(Token![;]) {
                Ok(Type::Array(TypeArray {
                    bracket_token,
                    elem: Box::new(elem),
                    semi_token: content.parse()?,
                    len: content.parse()?,
                }))
            } else {
                Ok(Type::Slice(TypeSlice {
                    bracket_token,
                    elem: Box::new(elem),
                }))
            }
        } else if lookahead.peek(Token![*]) {
            input.parse().map(Type::Ptr)
        } else if lookahead.peek(Token![&]) {
            input.parse().map(Type::Reference)
        } else if lookahead.peek(Token![!]) && !input.peek(Token![=]) {
            input.parse().map(Type::Never)
        } else if lookahead.peek(Token![impl]) {
            input.parse().map(Type::ImplTrait)
        } else if lookahead.peek(Token![_]) {
            input.parse().map(Type::Infer)
        } else if lookahead.peek(Lifetime) {
            input.parse().map(Type::TraitObject)
        } else {
            Err(lookahead.error())
        }
    }

    impl Parse for TypeSlice {
        fn parse(input: ParseStream) -> Result<Self> {
            let content;
            Ok(TypeSlice {
                bracket_token: bracketed!(content in input),
                elem: content.parse()?,
            })
        }
    }

    impl Parse for TypeArray {
        fn parse(input: ParseStream) -> Result<Self> {
            let content;
            Ok(TypeArray {
                bracket_token: bracketed!(content in input),
                elem: content.parse()?,
                semi_token: content.parse()?,
                len: content.parse()?,
            })
        }
    }

    impl Parse for TypePtr {
        fn parse(input: ParseStream) -> Result<Self> {
            let star_token: Token![*] = input.parse()?;

            let lookahead = input.lookahead1();
            let (const_token, mutability) = if lookahead.peek(Token![const]) {
                (Some(input.parse()?), None)
            } else if lookahead.peek(Token![mut]) {
                (None, Some(input.parse()?))
            } else {
                return Err(lookahead.error());
            };

            Ok(TypePtr {
                star_token,
                const_token,
                mutability,
                elem: Box::new(input.call(Type::without_plus)?),
            })
        }
    }

    impl Parse for TypeReference {
        fn parse(input: ParseStream) -> Result<Self> {
            Ok(TypeReference {
                and_token: input.parse()?,
                lifetime: input.parse()?,
                mutability: input.parse()?,
                // & binds tighter than +, so we don't allow + here.
                elem: Box::new(input.call(Type::without_plus)?),
            })
        }
    }

    impl Parse for TypeBareFn {
        fn parse(input: ParseStream) -> Result<Self> {
            let args;
            let allow_variadic;
            Ok(TypeBareFn {
                lifetimes: input.parse()?,
                unsafety: input.parse()?,
                abi: input.parse()?,
                fn_token: input.parse()?,
                paren_token: parenthesized!(args in input),
                inputs: {
                    let mut inputs = Punctuated::new();
                    while !args.is_empty() && !args.peek(Token![...]) {
                        inputs.push_value(args.parse()?);
                        if args.is_empty() {
                            break;
                        }
                        inputs.push_punct(args.parse()?);
                    }
                    allow_variadic = inputs.empty_or_trailing();
                    inputs
                },
                variadic: {
                    if allow_variadic && args.peek(Token![...]) {
                        Some(args.parse()?)
                    } else {
                        None
                    }
                },
                output: input.call(ReturnType::without_plus)?,
            })
        }
    }

    impl Parse for TypeNever {
        fn parse(input: ParseStream) -> Result<Self> {
            Ok(TypeNever {
                bang_token: input.parse()?,
            })
        }
    }

    impl Parse for TypeInfer {
        fn parse(input: ParseStream) -> Result<Self> {
            Ok(TypeInfer {
                underscore_token: input.parse()?,
            })
        }
    }

    impl Parse for TypeTuple {
        fn parse(input: ParseStream) -> Result<Self> {
            let content;
            Ok(TypeTuple {
                paren_token: parenthesized!(content in input),
                elems: content.parse_terminated(Type::parse)?,
            })
        }
    }

    impl Parse for TypeMacro {
        fn parse(input: ParseStream) -> Result<Self> {
            Ok(TypeMacro {
                mac: input.parse()?,
            })
        }
    }

    impl Parse for TypePath {
        fn parse(input: ParseStream) -> Result<Self> {
            let (qself, mut path) = path::parsing::qpath(input, false)?;

            if path.segments.last().unwrap().arguments.is_empty() && input.peek(token::Paren) {
                let args: ParenthesizedGenericArguments = input.parse()?;
                let parenthesized = PathArguments::Parenthesized(args);
                path.segments.last_mut().unwrap().arguments = parenthesized;
            }

            Ok(TypePath { qself, path })
        }
    }

    impl ReturnType {
        pub fn without_plus(input: ParseStream) -> Result<Self> {
            Self::parse(input, false)
        }

        pub fn parse(input: ParseStream, allow_plus: bool) -> Result<Self> {
            if input.peek(Token![->]) {
                let arrow = input.parse()?;
                let ty = ambig_ty(input, allow_plus)?;
                Ok(ReturnType::Type(arrow, Box::new(ty)))
            } else {
                Ok(ReturnType::Default)
            }
        }
    }

    impl Parse for ReturnType {
        fn parse(input: ParseStream) -> Result<Self> {
            Self::parse(input, true)
        }
    }

    impl Parse for TypeTraitObject {
        fn parse(input: ParseStream) -> Result<Self> {
            Self::parse(input, true)
        }
    }

    fn at_least_one_type(bounds: &Punctuated<TypeParamBound, Token![+]>) -> bool {
        for bound in bounds {
            if let TypeParamBound::Trait(_) = *bound {
                return true;
            }
        }
        false
    }

    impl TypeTraitObject {
        pub fn without_plus(input: ParseStream) -> Result<Self> {
            Self::parse(input, false)
        }

        // Only allow multiple trait references if allow_plus is true.
        pub fn parse(input: ParseStream, allow_plus: bool) -> Result<Self> {
            Ok(TypeTraitObject {
                dyn_token: input.parse()?,
                bounds: {
                    let mut bounds = Punctuated::new();
                    if allow_plus {
                        loop {
                            bounds.push_value(input.parse()?);
                            if !input.peek(Token![+]) {
                                break;
                            }
                            bounds.push_punct(input.parse()?);
                            if input.peek(Token![>]) {
                                break;
                            }
                        }
                    } else {
                        bounds.push_value(input.parse()?);
                    }
                    // Just lifetimes like `'a + 'b` is not a TraitObject.
                    if !at_least_one_type(&bounds) {
                        return Err(input.error("expected at least one type"));
                    }
                    bounds
                },
            })
        }
    }

    impl Parse for TypeImplTrait {
        fn parse(input: ParseStream) -> Result<Self> {
            Ok(TypeImplTrait {
                impl_token: input.parse()?,
                // NOTE: rust-lang/rust#34511 includes discussion about whether
                // or not + should be allowed in ImplTrait directly without ().
                bounds: {
                    let mut bounds = Punctuated::new();
                    loop {
                        bounds.push_value(input.parse()?);
                        if !input.peek(Token![+]) {
                            break;
                        }
                        bounds.push_punct(input.parse()?);
                    }
                    bounds
                },
            })
        }
    }

    impl Parse for TypeGroup {
        fn parse(input: ParseStream) -> Result<Self> {
            let group = crate::group::parse_group(input)?;
            Ok(TypeGroup {
                group_token: group.token,
                elem: group.content.parse()?,
            })
        }
    }

    impl Parse for TypeParen {
        fn parse(input: ParseStream) -> Result<Self> {
            Self::parse(input, false)
        }
    }

    impl TypeParen {
        fn parse(input: ParseStream, allow_plus: bool) -> Result<Self> {
            let content;
            Ok(TypeParen {
                paren_token: parenthesized!(content in input),
                elem: Box::new(ambig_ty(&content, allow_plus)?),
            })
        }
    }

    impl Parse for BareFnArg {
        fn parse(input: ParseStream) -> Result<Self> {
            Ok(BareFnArg {
                attrs: input.call(Attribute::parse_outer)?,
                name: {
                    if (input.peek(Ident) || input.peek(Token![_]))
                        && input.peek2(Token![:])
                        && !input.peek2(Token![::])
                    {
                        let name = input.call(Ident::parse_any)?;
                        let colon: Token![:] = input.parse()?;
                        Some((name, colon))
                    } else {
                        None
                    }
                },
                ty: input.parse()?,
            })
        }
    }

    impl Parse for Abi {
        fn parse(input: ParseStream) -> Result<Self> {
            Ok(Abi {
                extern_token: input.parse()?,
                name: input.parse()?,
            })
        }
    }

    impl Parse for Option<Abi> {
        fn parse(input: ParseStream) -> Result<Self> {
            if input.peek(Token![extern]) {
                input.parse().map(Some)
            } else {
                Ok(None)
            }
        }
    }
}

#[cfg(feature = "printing")]
mod printing {
    use super::*;

    use proc_macro2::TokenStream;
    use quote::{ToTokens, TokenStreamExt};

    use crate::attr::FilterAttrs;
    use crate::print::TokensOrDefault;

    impl ToTokens for TypeSlice {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.bracket_token.surround(tokens, |tokens| {
                self.elem.to_tokens(tokens);
            });
        }
    }

    impl ToTokens for TypeArray {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.bracket_token.surround(tokens, |tokens| {
                self.elem.to_tokens(tokens);
                self.semi_token.to_tokens(tokens);
                self.len.to_tokens(tokens);
            });
        }
    }

    impl ToTokens for TypePtr {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.star_token.to_tokens(tokens);
            match &self.mutability {
                Some(tok) => tok.to_tokens(tokens),
                None => {
                    TokensOrDefault(&self.const_token).to_tokens(tokens);
                }
            }
            self.elem.to_tokens(tokens);
        }
    }

    impl ToTokens for TypeReference {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.and_token.to_tokens(tokens);
            self.lifetime.to_tokens(tokens);
            self.mutability.to_tokens(tokens);
            self.elem.to_tokens(tokens);
        }
    }

    impl ToTokens for TypeBareFn {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.lifetimes.to_tokens(tokens);
            self.unsafety.to_tokens(tokens);
            self.abi.to_tokens(tokens);
            self.fn_token.to_tokens(tokens);
            self.paren_token.surround(tokens, |tokens| {
                self.inputs.to_tokens(tokens);
                if let Some(variadic) = &self.variadic {
                    if !self.inputs.empty_or_trailing() {
                        let span = variadic.spans[0];
                        Token![,](span).to_tokens(tokens);
                    }
                    variadic.to_tokens(tokens);
                }
            });
            self.output.to_tokens(tokens);
        }
    }

    impl ToTokens for TypeNever {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.bang_token.to_tokens(tokens);
        }
    }

    impl ToTokens for TypeTuple {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.paren_token.surround(tokens, |tokens| {
                self.elems.to_tokens(tokens);
            });
        }
    }

    impl ToTokens for TypePath {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            private::print_path(tokens, &self.qself, &self.path);
        }
    }

    impl ToTokens for TypeTraitObject {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.dyn_token.to_tokens(tokens);
            self.bounds.to_tokens(tokens);
        }
    }

    impl ToTokens for TypeImplTrait {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.impl_token.to_tokens(tokens);
            self.bounds.to_tokens(tokens);
        }
    }

    impl ToTokens for TypeGroup {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.group_token.surround(tokens, |tokens| {
                self.elem.to_tokens(tokens);
            });
        }
    }

    impl ToTokens for TypeParen {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.paren_token.surround(tokens, |tokens| {
                self.elem.to_tokens(tokens);
            });
        }
    }

    impl ToTokens for TypeInfer {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.underscore_token.to_tokens(tokens);
        }
    }

    impl ToTokens for TypeMacro {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.mac.to_tokens(tokens);
        }
    }

    impl ToTokens for ReturnType {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            match self {
                ReturnType::Default => {}
                ReturnType::Type(arrow, ty) => {
                    arrow.to_tokens(tokens);
                    ty.to_tokens(tokens);
                }
            }
        }
    }

    impl ToTokens for BareFnArg {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(self.attrs.outer());
            if let Some((name, colon)) = &self.name {
                name.to_tokens(tokens);
                colon.to_tokens(tokens);
            }
            self.ty.to_tokens(tokens);
        }
    }

    impl ToTokens for Abi {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.extern_token.to_tokens(tokens);
            self.name.to_tokens(tokens);
        }
    }
}
