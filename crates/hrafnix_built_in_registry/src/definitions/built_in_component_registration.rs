use crate::{BuiltInComponent, BuiltInComponentDefinition};
use hrafnix_expression_engine::prelude::ExpressionEngine;
use hrafnix_message::message::{Message, MessageCategory};

/// Constructs a runtime instance of a built-in component.
pub type BuiltInComponentFactory = fn() -> Box<dyn BuiltInComponent>;

/// Connects a component's static definition to its runtime factory.
#[derive(Debug, Clone, Copy)]
pub struct BuiltInComponentRegistration {
    /// Static metadata for the registered component.
    definition: &'static BuiltInComponentDefinition,
    /// Factory for creating runtime component behavior.
    factory: BuiltInComponentFactory,
}

impl BuiltInComponentRegistration {
    /// Creates a component registration.
    #[must_use]
    pub const fn new(
        definition: &'static BuiltInComponentDefinition,
        factory: BuiltInComponentFactory,
    ) -> Self {
        Self {
            definition,
            factory,
        }
    }

    /// Returns the component's static definition.
    #[must_use]
    pub const fn definition(&self) -> &'static BuiltInComponentDefinition {
        self.definition
    }

    /// Instantiates the component with an isolated expression engine.
    ///
    /// # Errors
    ///
    /// Returns an error when the factory produces a different component version, a component
    /// function has an invalid name, or component functions have conflicting names.
    pub fn instantiate(&self) -> Result<BuiltInComponentInstance, Message> {
        let component = (self.factory)();
        let actual_definition = component.definition();
        if actual_definition.id() != self.definition.id()
            || actual_definition.version() != self.definition.version()
        {
            return Err(Message::error(
                MessageCategory::Component,
                "built_in_registry_factory_definition_mismatch",
            ));
        }

        let mut engine = ExpressionEngine::new();
        for function in component.functions() {
            let name = function.name().clone();
            if engine.functions().get(name.as_str()).is_some() {
                return Err(Message::error_with_param(
                    MessageCategory::Component,
                    "built_in_registry_function_name_conflict",
                    "name",
                    name,
                ));
            }
            engine.register_function(function)?;
        }

        Ok(BuiltInComponentInstance { component, engine })
    }
}

/// A runtime component and its isolated expression engine.
#[derive(Debug)]
pub struct BuiltInComponentInstance {
    /// Component-specific runtime behavior.
    component: Box<dyn BuiltInComponent>,
    /// Engine containing the default and component-specific functions.
    engine: ExpressionEngine,
}

impl BuiltInComponentInstance {
    /// Returns the component behavior.
    #[must_use]
    pub fn component(&self) -> &dyn BuiltInComponent {
        self.component.as_ref()
    }

    /// Returns the component's static definition.
    #[must_use]
    pub fn definition(&self) -> &'static BuiltInComponentDefinition {
        self.component.definition()
    }

    /// Returns the component's expression engine.
    #[must_use]
    pub const fn engine(&self) -> &ExpressionEngine {
        &self.engine
    }

    /// Returns the component's mutable expression engine.
    #[must_use]
    pub const fn engine_mut(&mut self) -> &mut ExpressionEngine {
        &mut self.engine
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::signal::gain_v1::definition::GAIN_V1;
    use crate::definitions::built_in_component_definition::built_in_component_definition;
    use crate::definitions::icon_definition::icon_definition;
    use hrafnix_expression_engine::prelude::{ArgumentCount, ComputedItem, FunctionDefinition};
    use hrafnix_keys::{ConstStoreKey, store_key};

    static OTHER_DEFINITION: BuiltInComponentDefinition = built_in_component_definition!(
        "other",
        2,
        "Other",
        hrafnix_datastore::const_parameter_object!([]),
        hrafnix_datastore::const_variable_object!([]),
        icon_definition!("<svg></svg>", (32, 32)),
        [],
    );

    #[derive(Debug)]
    struct TestComponent {
        definition: &'static BuiltInComponentDefinition,
        functions: Vec<ConstStoreKey>,
    }

    impl BuiltInComponent for TestComponent {
        fn definition(&self) -> &'static BuiltInComponentDefinition {
            self.definition
        }

        fn functions(&self) -> Vec<FunctionDefinition> {
            self.functions
                .iter()
                .map(|name| {
                    FunctionDefinition::new(*name, "", ArgumentCount::Exact { count: 0 }, |_| {
                        Ok(ComputedItem::Float(1.0))
                    })
                })
                .collect()
        }
    }

    #[test]
    fn factories_must_return_the_registered_component_version() {
        let registration = BuiltInComponentRegistration::new(&GAIN_V1, || {
            Box::new(TestComponent {
                definition: &OTHER_DEFINITION,
                functions: Vec::new(),
            })
        });
        let error = registration.instantiate().unwrap_err();
        assert_eq!(error.category(), MessageCategory::Component);
        assert_eq!(
            error.translate_data().message_key().as_str(),
            "built_in_registry_factory_definition_mismatch"
        );
    }

    #[test]
    fn factories_cannot_replace_default_functions() {
        let registration = BuiltInComponentRegistration::new(&GAIN_V1, || {
            Box::new(TestComponent {
                definition: &GAIN_V1,
                functions: vec![store_key!("sin")],
            })
        });
        let error = registration.instantiate().unwrap_err();
        assert_eq!(error.category(), MessageCategory::Component);
        assert_eq!(
            error.translate_data().message_key().as_str(),
            "built_in_registry_function_name_conflict"
        );
        assert_eq!(
            error
                .translate_data()
                .message_params()
                .get("name")
                .unwrap()
                .as_str(),
            "sin"
        );
    }

    #[test]
    fn factories_cannot_register_duplicate_custom_functions() {
        let registration = BuiltInComponentRegistration::new(&GAIN_V1, || {
            Box::new(TestComponent {
                definition: &GAIN_V1,
                functions: vec![store_key!("local"), store_key!("local")],
            })
        });
        let error = registration.instantiate().unwrap_err();
        assert_eq!(
            error.translate_data().message_key().as_str(),
            "built_in_registry_function_name_conflict"
        );
        assert_eq!(
            error
                .translate_data()
                .message_params()
                .get("name")
                .unwrap()
                .as_str(),
            "local"
        );
    }
}
