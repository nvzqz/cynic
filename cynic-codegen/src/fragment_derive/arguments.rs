use proc_macro2::TokenStream;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Expr, Ident, Result, Token,
};

pub fn arguments_from_field_attrs(attrs: &[syn::Attribute]) -> Result<Vec<FieldArgument>> {
    for attr in attrs {
        if attr.path.is_ident("arguments") {
            let parsed: CynicArguments = attr.parse_args()?;
            return Ok(parsed.arguments.into_iter().collect());
        }
    }
    Ok(vec![])
}

/// Implements syn::Parse to parse out arguments from the arguments
/// attribute.
#[derive(PartialEq, Debug)]
struct CynicArguments {
    arguments: Punctuated<FieldArgument, Token![,]>,
}

impl Parse for CynicArguments {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(CynicArguments {
            arguments: Punctuated::parse_terminated(input)?,
        })
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct FieldArgument {
    pub argument_name: Ident,
    pub expr: syn::Expr,
}

impl Parse for FieldArgument {
    fn parse(input: ParseStream) -> Result<Self> {
        let argument_name = input.parse()?;
        input.parse::<Token![=]>()?;
        let expr: Expr = input.parse()?;

        Ok(FieldArgument {
            argument_name,
            expr, //expr.try_into()?,
        })
    }
}

impl quote::ToTokens for FieldArgument {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        use quote::{quote, TokenStreamExt};

        let argument_name = &self.argument_name;
        let expr = &self.expr;

        tokens.append_all(quote! {
            #argument_name: #expr
        });
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_parsing_string_literal() {
        let parsed: CynicArguments = parse_quote! { x = "abcd" };

        let arguments = parsed.arguments.iter().collect::<Vec<_>>();

        assert_eq!(arguments.len(), 1);
        assert_eq!(arguments[0].argument_name.to_string(), "x".to_string());
    }

    #[test]
    fn test_parsing_multiple_arg_expressions() {
        let parsed: CynicArguments = parse_quote! { x = 1, y = args.test };

        let arguments = parsed.arguments.iter().collect::<Vec<_>>();

        assert_eq!(arguments.len(), 2);
        assert_eq!(arguments[0].argument_name.to_string(), "x".to_string());
        /*
        TODO: Re-instate

        assert_eq!(
            arguments[0].expr,
            ArgumentExpression::Literal(parse_quote! { 1 })
        );
        assert_eq!(arguments[1].argument_name.to_string(), "y".to_string());
        assert_matches!(arguments[1].expr, ArgumentExpression::FieldAccess(_));
        */
    }
}
