use std::collections::HashMap;

use simplexpr::SimplExpr;

use crate::{
    error::AstResult,
    parser::{
        ast::{Ast, AstIterator, Span},
        from_ast::FromAst,
    },
    spanned,
    value::{AttrName, VarName},
};

use super::{widget_definition::WidgetDefinition, widget_use::WidgetUse};

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Unknown widget referenced: {1}")]
    UnknownWidget(Span, String),

    #[error("Missing attribute `{arg_name}` in use of widget `{widget_name}`")]
    MissingAttr { widget_name: String, arg_name: AttrName, arg_list_span: Span, use_span: Span },
}

pub fn validate(defs: &HashMap<String, WidgetDefinition>, content: &WidgetUse) -> Result<(), ValidationError> {
    if let Some(def) = defs.get(&content.name) {
        for expected in def.expected_args.iter() {
            if !content.attrs.attrs.contains_key(expected) {
                return Err(ValidationError::MissingAttr {
                    widget_name: def.name.to_string(),
                    arg_name: expected.clone(),
                    arg_list_span: def.args_span,
                    use_span: content.span,
                });
            }
        }
    } else {
        return Err(ValidationError::UnknownWidget(content.span, content.name.to_string()));
    }
    Ok(())
}
