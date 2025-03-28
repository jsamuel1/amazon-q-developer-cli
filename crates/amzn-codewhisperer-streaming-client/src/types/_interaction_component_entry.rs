// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Interaction component with an identifier
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InteractionComponentEntry {
    /// Identifier that can uniquely identify the interaction component within stream response. This
    /// field is optional.
    pub interaction_component_id: ::std::option::Option<::std::string::String>,
    /// Interaction component
    pub interaction_component: crate::types::InteractionComponent,
}
impl InteractionComponentEntry {
    /// Identifier that can uniquely identify the interaction component within stream response. This
    /// field is optional.
    pub fn interaction_component_id(&self) -> ::std::option::Option<&str> {
        self.interaction_component_id.as_deref()
    }

    /// Interaction component
    pub fn interaction_component(&self) -> &crate::types::InteractionComponent {
        &self.interaction_component
    }
}
impl InteractionComponentEntry {
    /// Creates a new builder-style object to manufacture
    /// [`InteractionComponentEntry`](crate::types::InteractionComponentEntry).
    pub fn builder() -> crate::types::builders::InteractionComponentEntryBuilder {
        crate::types::builders::InteractionComponentEntryBuilder::default()
    }
}

/// A builder for [`InteractionComponentEntry`](crate::types::InteractionComponentEntry).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct InteractionComponentEntryBuilder {
    pub(crate) interaction_component_id: ::std::option::Option<::std::string::String>,
    pub(crate) interaction_component: ::std::option::Option<crate::types::InteractionComponent>,
}
impl InteractionComponentEntryBuilder {
    /// Identifier that can uniquely identify the interaction component within stream response. This
    /// field is optional.
    pub fn interaction_component_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.interaction_component_id = ::std::option::Option::Some(input.into());
        self
    }

    /// Identifier that can uniquely identify the interaction component within stream response. This
    /// field is optional.
    pub fn set_interaction_component_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.interaction_component_id = input;
        self
    }

    /// Identifier that can uniquely identify the interaction component within stream response. This
    /// field is optional.
    pub fn get_interaction_component_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.interaction_component_id
    }

    /// Interaction component
    /// This field is required.
    pub fn interaction_component(mut self, input: crate::types::InteractionComponent) -> Self {
        self.interaction_component = ::std::option::Option::Some(input);
        self
    }

    /// Interaction component
    pub fn set_interaction_component(
        mut self,
        input: ::std::option::Option<crate::types::InteractionComponent>,
    ) -> Self {
        self.interaction_component = input;
        self
    }

    /// Interaction component
    pub fn get_interaction_component(&self) -> &::std::option::Option<crate::types::InteractionComponent> {
        &self.interaction_component
    }

    /// Consumes the builder and constructs a
    /// [`InteractionComponentEntry`](crate::types::InteractionComponentEntry). This method will
    /// fail if any of the following fields are not set:
    /// - [`interaction_component`](crate::types::builders::InteractionComponentEntryBuilder::interaction_component)
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::types::InteractionComponentEntry, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::types::InteractionComponentEntry {
            interaction_component_id: self.interaction_component_id,
            interaction_component: self.interaction_component.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "interaction_component",
                    "interaction_component was not specified but it is required when building InteractionComponentEntry",
                )
            })?,
        })
    }
}
