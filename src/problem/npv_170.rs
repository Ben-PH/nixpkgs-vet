use std::fmt;

use derive_new::new;

#[derive(Clone, new)]
pub struct ByNamePackegPrefixedWithNumber {
    #[new(into)]
    attribute_name: String,
}

impl fmt::Display for ByNamePackegPrefixedWithNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Self { attribute_name } = self;
        write!(
            f,
            r#"- pkgs.{attribute_name}: "Attribute names should not be number-prefixed. It is suggestet to `"`-wrap this name"#
        )
    }
}
