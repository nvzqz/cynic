use crate::{type_system::ids::InterfaceDefinitionId, AstLookup};

use super::{directives::Directive, fields::FieldDefinition, ReadContext, TypeSystemId};

#[derive(Clone, Copy)]
pub struct InterfaceDefinition<'a>(ReadContext<'a, InterfaceDefinitionId>);

impl<'a> InterfaceDefinition<'a> {
    pub fn name(&self) -> &'a str {
        self.0
            .document
            .lookup(self.0.document.lookup(self.0.id).name)
    }

    pub fn description(&self) -> Option<&'a str> {
        self.0
            .document
            .lookup(self.0.id)
            .description
            .map(|id| self.0.document.lookup(id))
    }

    pub fn implements_interfaces(&self) -> impl ExactSizeIterator<Item = &'a str> + 'a {
        self.0
            .document
            .lookup(self.0.id)
            .implements
            .iter()
            .map(|id| self.0.document.lookup(*id))
    }

    pub fn fields(&self) -> impl ExactSizeIterator<Item = FieldDefinition<'a>> + 'a {
        self.0
            .document
            .lookup(self.0.id)
            .fields
            .iter()
            .map(|id| self.0.document.read(id))
    }

    pub fn directives(&self) -> impl ExactSizeIterator<Item = Directive<'a>> + 'a {
        self.0
            .document
            .lookup(self.0.id)
            .directives
            .iter()
            .map(|id| self.0.document.read(id))
    }
}

impl TypeSystemId for InterfaceDefinitionId {
    type Reader<'a> = InterfaceDefinition<'a>;
}

impl<'a> From<ReadContext<'a, InterfaceDefinitionId>> for InterfaceDefinition<'a> {
    fn from(value: ReadContext<'a, InterfaceDefinitionId>) -> Self {
        Self(value)
    }
}
