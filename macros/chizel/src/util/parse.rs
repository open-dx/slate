use syn::Ident;
use syn::Attribute;

/// TODO
pub fn get_attr_ident<'attr>(attr: &'attr Attribute) -> Option<&'attr Ident> {
    match attr.path().get_ident() {
        Some(ident) => Some(ident),
        None => None,
    }
}
