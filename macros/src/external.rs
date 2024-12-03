use crate::prelude::*;
use syn::Path;

pub struct Traits {
    pub base_parser: Path,
    pub once_parser: Path,
    pub mut_parser: Path,
    pub ref_parser: Path,
}

impl Traits {
    fn _new() -> Self {
        Self {
            base_parser: lavan_path!(parser::traits::Parser),
            once_parser: lavan_path!(parser::traits::IterativeParser),
            mut_parser: lavan_path!(parser::traits::IterativeParserMut),
            ref_parser: lavan_path!(parser::traits::IterativeParserRef),
        }
    }

    pub fn associated(ty: impl ToTokens) -> Self {
        Self {
            base_parser: lavan_path!(parser::traits::Parser),
            once_parser: lavan_path!(parser::traits::IterativeParser<#ty>),
            mut_parser: lavan_path!(parser::traits::IterativeParserMut<#ty>),
            ref_parser: lavan_path!(parser::traits::IterativeParserRef<#ty>),
        }
    }
}
