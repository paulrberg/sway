use sway_types::{Ident, Span};

use crate::{
    language::{ty::*, Visibility},
    transform, EqWithTypeEngine, PartialEqWithTypeEngine,
};

#[derive(Clone, Debug)]
pub struct TyConstantDeclaration {
    pub name: Ident,
    pub value: TyExpression,
    pub visibility: Visibility,
    pub attributes: transform::AttributesMap,
    pub span: Span,
}

impl EqWithTypeEngine for TyConstantDeclaration {}
impl PartialEqWithTypeEngine for TyConstantDeclaration {
    fn eq(&self, rhs: &Self, type_engine: &crate::TypeEngine) -> bool {
        self.name == rhs.name
            && self.value.eq(&rhs.value, type_engine)
            && self.visibility == rhs.visibility
            && self.attributes == rhs.attributes
            && self.span == rhs.span
    }
}
