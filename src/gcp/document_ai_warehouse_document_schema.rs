use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DocumentAiWarehouseDocumentSchemaData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::ResourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    provisioner: std::vec::Vec<terrars::Provisioner>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    document_is_folder: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    project_number: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    property_definitions: std::option::Option<std::vec::Vec<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DocumentAiWarehouseDocumentSchemaTimeoutsEl>,
    dynamic: DocumentAiWarehouseDocumentSchemaDynamic,
}

struct DocumentAiWarehouseDocumentSchema_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DocumentAiWarehouseDocumentSchemaData>,
}

#[derive(Clone)]
pub struct DocumentAiWarehouseDocumentSchema(std::rc::Rc<DocumentAiWarehouseDocumentSchema_>);

impl DocumentAiWarehouseDocumentSchema {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
        self
    }

    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }

    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }

    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            std::option::Option::Some(terrars::IgnoreChanges::All(terrars::IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl std::string::ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                std::option::Option::Some(i) => match i {
                    terrars::IgnoreChanges::All(_) => {
                        true
                    },
                    terrars::IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                std::option::Option::None => true,
            } {
                d.lifecycle.ignore_changes =
                    std::option::Option::Some(terrars::IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl terrars::Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = r" Add a provisioner (ex: `local-exec`, `remote-exec`, `file`) to this resource."]
    #[doc = r" Provisioners run in the order they're added."]
    pub fn add_provisioner(self, v: impl std::convert::Into<terrars::Provisioner>) -> Self {
        self.0.data.borrow_mut().provisioner.push(v.into());
        self
    }

    pub fn add_lifecycle_precondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.precondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    pub fn add_lifecycle_postcondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.postcondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    #[doc = "Set the field `document_is_folder`.\nTells whether the document is a folder or a typical document."]
    pub fn set_document_is_folder(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().document_is_folder = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `property_definitions`.\n"]
    pub fn set_property_definitions(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().property_definitions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.property_definitions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DocumentAiWarehouseDocumentSchemaTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nName of the schema given by the user."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `document_is_folder` after provisioning.\nTells whether the document is a folder or a typical document."]
    pub fn document_is_folder(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.document_is_folder", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the document schema."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project_number` after provisioning.\nThe unique identifier of the project."]
    pub fn project_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_number", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `property_definitions` after provisioning.\n"]
    pub fn property_definitions(&self) -> terrars::ListRef<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.property_definitions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DocumentAiWarehouseDocumentSchemaTimeoutsElRef {
        DocumentAiWarehouseDocumentSchemaTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DocumentAiWarehouseDocumentSchema {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DocumentAiWarehouseDocumentSchema { }

impl terrars::ToListMappable for DocumentAiWarehouseDocumentSchema {
    type O = terrars::ListRef<DocumentAiWarehouseDocumentSchemaRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DocumentAiWarehouseDocumentSchema_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_document_ai_warehouse_document_schema".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDocumentAiWarehouseDocumentSchema {
    pub tf_id: std::string::String,
    #[doc = "Name of the schema given by the user."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc = "The location of the resource."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The unique identifier of the project."]
    pub project_number: terrars::PrimField<std::string::String>,
}

impl BuildDocumentAiWarehouseDocumentSchema {
    pub fn build(self, stack: &mut terrars::Stack) -> DocumentAiWarehouseDocumentSchema {
        let out = DocumentAiWarehouseDocumentSchema(std::rc::Rc::new(DocumentAiWarehouseDocumentSchema_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DocumentAiWarehouseDocumentSchemaData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                display_name: self.display_name,
                document_is_folder: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                project_number: self.project_number,
                property_definitions: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DocumentAiWarehouseDocumentSchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DocumentAiWarehouseDocumentSchemaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DocumentAiWarehouseDocumentSchemaRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nName of the schema given by the user."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `document_is_folder` after provisioning.\nTells whether the document is a folder or a typical document."]
    pub fn document_is_folder(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.document_is_folder", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the document schema."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project_number` after provisioning.\nThe unique identifier of the project."]
    pub fn project_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_number", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `property_definitions` after provisioning.\n"]
    pub fn property_definitions(&self) -> terrars::ListRef<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.property_definitions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DocumentAiWarehouseDocumentSchemaTimeoutsElRef {
        DocumentAiWarehouseDocumentSchemaTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElDateTimeTypeOptionsEl {}

impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElDateTimeTypeOptionsEl { }

impl terrars::ToListMappable for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElDateTimeTypeOptionsEl {
    type O = terrars::BlockAssignable<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElDateTimeTypeOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElDateTimeTypeOptionsEl {}

impl BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElDateTimeTypeOptionsEl {
    pub fn build(self) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElDateTimeTypeOptionsEl {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElDateTimeTypeOptionsEl {}
    }
}

pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElDateTimeTypeOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElDateTimeTypeOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElDateTimeTypeOptionsElRef {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElDateTimeTypeOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElDateTimeTypeOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElEnumTypeOptionsEl {
    possible_values: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validation_check_disabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElEnumTypeOptionsEl {
    #[doc =
        "Set the field `validation_check_disabled`.\nMake sure the enum property value provided in the document is in the possile value list during document creation. The validation check runs by default."]
    pub fn set_validation_check_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.validation_check_disabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElEnumTypeOptionsEl {
    type O = terrars::BlockAssignable<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElEnumTypeOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElEnumTypeOptionsEl {
    #[doc = "List of possible enum values."]
    pub possible_values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElEnumTypeOptionsEl {
    pub fn build(self) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElEnumTypeOptionsEl {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElEnumTypeOptionsEl {
            possible_values: self.possible_values,
            validation_check_disabled: core::default::Default::default(),
        }
    }
}

pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElEnumTypeOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElEnumTypeOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElEnumTypeOptionsElRef {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElEnumTypeOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElEnumTypeOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `possible_values` after provisioning.\nList of possible enum values."]
    pub fn possible_values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.possible_values", self.base))
    }

    #[doc =
        "Get a reference to the value of field `validation_check_disabled` after provisioning.\nMake sure the enum property value provided in the document is in the possile value list during document creation. The validation check runs by default."]
    pub fn validation_check_disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validation_check_disabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElFloatTypeOptionsEl {}

impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElFloatTypeOptionsEl { }

impl terrars::ToListMappable for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElFloatTypeOptionsEl {
    type O = terrars::BlockAssignable<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElFloatTypeOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElFloatTypeOptionsEl {}

impl BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElFloatTypeOptionsEl {
    pub fn build(self) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElFloatTypeOptionsEl {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElFloatTypeOptionsEl {}
    }
}

pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElFloatTypeOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElFloatTypeOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElFloatTypeOptionsElRef {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElFloatTypeOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElFloatTypeOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElIntegerTypeOptionsEl {}

impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElIntegerTypeOptionsEl { }

impl terrars::ToListMappable for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElIntegerTypeOptionsEl {
    type O = terrars::BlockAssignable<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElIntegerTypeOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElIntegerTypeOptionsEl {}

impl BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElIntegerTypeOptionsEl {
    pub fn build(self) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElIntegerTypeOptionsEl {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElIntegerTypeOptionsEl {}
    }
}

pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElIntegerTypeOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElIntegerTypeOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElIntegerTypeOptionsElRef {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElIntegerTypeOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElIntegerTypeOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElMapTypeOptionsEl {}

impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElMapTypeOptionsEl { }

impl terrars::ToListMappable for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElMapTypeOptionsEl {
    type O = terrars::BlockAssignable<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElMapTypeOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElMapTypeOptionsEl {}

impl BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElMapTypeOptionsEl {
    pub fn build(self) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElMapTypeOptionsEl {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElMapTypeOptionsEl {}
    }
}

pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElMapTypeOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElMapTypeOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElMapTypeOptionsElRef {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElMapTypeOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElMapTypeOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElDateTimeTypeOptionsEl {}

impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElDateTimeTypeOptionsEl {

}

impl terrars
::ToListMappable for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElDateTimeTypeOptionsEl {
    type O =
        terrars
        ::BlockAssignable<
            DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElDateTimeTypeOptionsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElDateTimeTypeOptionsEl {}

impl BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElDateTimeTypeOptionsEl {
    pub fn build(
        self,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElDateTimeTypeOptionsEl {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElDateTimeTypeOptionsEl {}
    }
}

pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElDateTimeTypeOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElDateTimeTypeOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElDateTimeTypeOptionsElRef {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElDateTimeTypeOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElDateTimeTypeOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElEnumTypeOptionsEl {
    possible_values: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validation_check_disabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElEnumTypeOptionsEl {
    #[doc =
        "Set the field `validation_check_disabled`.\nMake sure the enum property value provided in the document is in the possile value list during document creation. The validation check runs by default."]
    pub fn set_validation_check_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.validation_check_disabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElEnumTypeOptionsEl {
    type O =
        terrars
        ::BlockAssignable<
            DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElEnumTypeOptionsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElEnumTypeOptionsEl {
    #[doc = "List of possible enum values."]
    pub possible_values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElEnumTypeOptionsEl {
    pub fn build(
        self,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElEnumTypeOptionsEl {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElEnumTypeOptionsEl {
            possible_values: self.possible_values,
            validation_check_disabled: core::default::Default::default(),
        }
    }
}

pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElEnumTypeOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElEnumTypeOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElEnumTypeOptionsElRef {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElEnumTypeOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElEnumTypeOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `possible_values` after provisioning.\nList of possible enum values."]
    pub fn possible_values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.possible_values", self.base))
    }

    #[doc =
        "Get a reference to the value of field `validation_check_disabled` after provisioning.\nMake sure the enum property value provided in the document is in the possile value list during document creation. The validation check runs by default."]
    pub fn validation_check_disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validation_check_disabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElFloatTypeOptionsEl {}

impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElFloatTypeOptionsEl {

}

impl terrars
::ToListMappable for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElFloatTypeOptionsEl {
    type O =
        terrars
        ::BlockAssignable<
            DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElFloatTypeOptionsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElFloatTypeOptionsEl {}

impl BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElFloatTypeOptionsEl {
    pub fn build(
        self,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElFloatTypeOptionsEl {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElFloatTypeOptionsEl {}
    }
}

pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElFloatTypeOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElFloatTypeOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElFloatTypeOptionsElRef {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElFloatTypeOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElFloatTypeOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElIntegerTypeOptionsEl {}

impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElIntegerTypeOptionsEl {

}

impl terrars
::ToListMappable for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElIntegerTypeOptionsEl {
    type O =
        terrars
        ::BlockAssignable<
            DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElIntegerTypeOptionsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElIntegerTypeOptionsEl {}

impl BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElIntegerTypeOptionsEl {
    pub fn build(
        self,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElIntegerTypeOptionsEl {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElIntegerTypeOptionsEl {}
    }
}

pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElIntegerTypeOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElIntegerTypeOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElIntegerTypeOptionsElRef {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElIntegerTypeOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElIntegerTypeOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElMapTypeOptionsEl {}

impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElMapTypeOptionsEl {

}

impl terrars
::ToListMappable for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElMapTypeOptionsEl {
    type O =
        terrars
        ::BlockAssignable<
            DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElMapTypeOptionsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElMapTypeOptionsEl {}

impl BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElMapTypeOptionsEl {
    pub fn build(
        self,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElMapTypeOptionsEl {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElMapTypeOptionsEl {}
    }
}

pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElMapTypeOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElMapTypeOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElMapTypeOptionsElRef {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElMapTypeOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElMapTypeOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElSchemaSourcesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    processor_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElSchemaSourcesEl {
    #[doc = "Set the field `name`.\nThe schema name in the source."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `processor_type`.\nThe Doc AI processor type name."]
    pub fn set_processor_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.processor_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElSchemaSourcesEl {
    type O =
        terrars
        ::BlockAssignable<
            DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElSchemaSourcesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElSchemaSourcesEl {}

impl BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElSchemaSourcesEl {
    pub fn build(
        self,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElSchemaSourcesEl {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElSchemaSourcesEl {
            name: core::default::Default::default(),
            processor_type: core::default::Default::default(),
        }
    }
}

pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElSchemaSourcesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElSchemaSourcesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElSchemaSourcesElRef {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElSchemaSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElSchemaSourcesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe schema name in the source."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `processor_type` after provisioning.\nThe Doc AI processor type name."]
    pub fn processor_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.processor_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTextTypeOptionsEl {}

impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTextTypeOptionsEl {

}

impl terrars
::ToListMappable for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTextTypeOptionsEl {
    type O =
        terrars
        ::BlockAssignable<
            DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTextTypeOptionsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTextTypeOptionsEl {}

impl BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTextTypeOptionsEl {
    pub fn build(
        self,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTextTypeOptionsEl {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTextTypeOptionsEl {}
    }
}

pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTextTypeOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTextTypeOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTextTypeOptionsElRef {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTextTypeOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTextTypeOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTimestampTypeOptionsEl {}

impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTimestampTypeOptionsEl {

}

impl terrars
::ToListMappable for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTimestampTypeOptionsEl {
    type O =
        terrars
        ::BlockAssignable<
            DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTimestampTypeOptionsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTimestampTypeOptionsEl {}

impl BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTimestampTypeOptionsEl {
    pub fn build(
        self,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTimestampTypeOptionsEl {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTimestampTypeOptionsEl {}
    }
}

pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTimestampTypeOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTimestampTypeOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTimestampTypeOptionsElRef {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTimestampTypeOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTimestampTypeOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElDynamic {
    date_time_type_options: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElDateTimeTypeOptionsEl,
        >,
    >,
    enum_type_options: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElEnumTypeOptionsEl,
        >,
    >,
    float_type_options: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElFloatTypeOptionsEl,
        >,
    >,
    integer_type_options: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElIntegerTypeOptionsEl,
        >,
    >,
    map_type_options: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElMapTypeOptionsEl,
        >,
    >,
    schema_sources: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElSchemaSourcesEl,
        >,
    >,
    text_type_options: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTextTypeOptionsEl,
        >,
    >,
    timestamp_type_options: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTimestampTypeOptionsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_filterable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_metadata: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_repeatable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_required: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_searchable: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retrieval_importance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_time_type_options: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElDateTimeTypeOptionsEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enum_type_options: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElEnumTypeOptionsEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    float_type_options: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElFloatTypeOptionsEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    integer_type_options: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElIntegerTypeOptionsEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    map_type_options: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElMapTypeOptionsEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_sources: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElSchemaSourcesEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_type_options: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTextTypeOptionsEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timestamp_type_options: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTimestampTypeOptionsEl,
        >,
    >,
    dynamic: DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElDynamic,
}

impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsEl {
    #[doc = "Set the field `display_name`.\nThe display-name for the property, used for front-end."]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `is_filterable`.\nWhether the property can be filtered. If this is a sub-property, all the parent properties must be marked filterable."]
    pub fn set_is_filterable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_filterable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_metadata`.\nWhether the property is user supplied metadata."]
    pub fn set_is_metadata(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_repeatable`.\nWhether the property can have multiple values."]
    pub fn set_is_repeatable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_repeatable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_required`.\nWhether the property is mandatory."]
    pub fn set_is_required(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_required = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_searchable`.\nIndicates that the property should be included in a global search."]
    pub fn set_is_searchable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_searchable = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `retrieval_importance`.\nStores the retrieval importance. Possible values: [\"HIGHEST\", \"HIGHER\", \"HIGH\", \"MEDIUM\", \"LOW\", \"LOWEST\"]"]
    pub fn set_retrieval_importance(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.retrieval_importance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `date_time_type_options`.\n"]
    pub fn set_date_time_type_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElDateTimeTypeOptionsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.date_time_type_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.date_time_type_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `enum_type_options`.\n"]
    pub fn set_enum_type_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElEnumTypeOptionsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.enum_type_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.enum_type_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `float_type_options`.\n"]
    pub fn set_float_type_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElFloatTypeOptionsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.float_type_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.float_type_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `integer_type_options`.\n"]
    pub fn set_integer_type_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElIntegerTypeOptionsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.integer_type_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.integer_type_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `map_type_options`.\n"]
    pub fn set_map_type_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElMapTypeOptionsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.map_type_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.map_type_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `schema_sources`.\n"]
    pub fn set_schema_sources(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElSchemaSourcesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.schema_sources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.schema_sources = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `text_type_options`.\n"]
    pub fn set_text_type_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTextTypeOptionsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.text_type_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.text_type_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timestamp_type_options`.\n"]
    pub fn set_timestamp_type_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTimestampTypeOptionsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.timestamp_type_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.timestamp_type_options = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsEl {
    type O =
        terrars
        ::BlockAssignable<
            DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsEl {
    #[doc = "The name of the metadata property."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsEl {
    pub fn build(
        self,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsEl {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsEl {
            display_name: core::default::Default::default(),
            is_filterable: core::default::Default::default(),
            is_metadata: core::default::Default::default(),
            is_repeatable: core::default::Default::default(),
            is_required: core::default::Default::default(),
            is_searchable: core::default::Default::default(),
            name: self.name,
            retrieval_importance: core::default::Default::default(),
            date_time_type_options: core::default::Default::default(),
            enum_type_options: core::default::Default::default(),
            float_type_options: core::default::Default::default(),
            integer_type_options: core::default::Default::default(),
            map_type_options: core::default::Default::default(),
            schema_sources: core::default::Default::default(),
            text_type_options: core::default::Default::default(),
            timestamp_type_options: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElRef {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display-name for the property, used for front-end."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_filterable` after provisioning.\nWhether the property can be filtered. If this is a sub-property, all the parent properties must be marked filterable."]
    pub fn is_filterable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_filterable", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_metadata` after provisioning.\nWhether the property is user supplied metadata."]
    pub fn is_metadata(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_metadata", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_repeatable` after provisioning.\nWhether the property can have multiple values."]
    pub fn is_repeatable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_repeatable", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_required` after provisioning.\nWhether the property is mandatory."]
    pub fn is_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_required", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_searchable` after provisioning.\nIndicates that the property should be included in a global search."]
    pub fn is_searchable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_searchable", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the metadata property."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retrieval_importance` after provisioning.\nStores the retrieval importance. Possible values: [\"HIGHEST\", \"HIGHER\", \"HIGH\", \"MEDIUM\", \"LOW\", \"LOWEST\"]"]
    pub fn retrieval_importance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retrieval_importance", self.base))
    }

    #[doc = "Get a reference to the value of field `date_time_type_options` after provisioning.\n"]
    pub fn date_time_type_options(
        &self,
    ) -> terrars
    ::ListRef<
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElDateTimeTypeOptionsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.date_time_type_options", self.base))
    }

    #[doc = "Get a reference to the value of field `enum_type_options` after provisioning.\n"]
    pub fn enum_type_options(
        &self,
    ) -> terrars
    ::ListRef<
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElEnumTypeOptionsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enum_type_options", self.base))
    }

    #[doc = "Get a reference to the value of field `float_type_options` after provisioning.\n"]
    pub fn float_type_options(
        &self,
    ) -> terrars
    ::ListRef<
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElFloatTypeOptionsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.float_type_options", self.base))
    }

    #[doc = "Get a reference to the value of field `integer_type_options` after provisioning.\n"]
    pub fn integer_type_options(
        &self,
    ) -> terrars
    ::ListRef<
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElIntegerTypeOptionsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.integer_type_options", self.base))
    }

    #[doc = "Get a reference to the value of field `map_type_options` after provisioning.\n"]
    pub fn map_type_options(
        &self,
    ) -> terrars
    ::ListRef<
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElMapTypeOptionsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.map_type_options", self.base))
    }

    #[doc = "Get a reference to the value of field `schema_sources` after provisioning.\n"]
    pub fn schema_sources(
        &self,
    ) -> terrars
    ::ListRef<
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElSchemaSourcesElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema_sources", self.base))
    }

    #[doc = "Get a reference to the value of field `text_type_options` after provisioning.\n"]
    pub fn text_type_options(
        &self,
    ) -> terrars
    ::ListRef<
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTextTypeOptionsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text_type_options", self.base))
    }

    #[doc = "Get a reference to the value of field `timestamp_type_options` after provisioning.\n"]
    pub fn timestamp_type_options(
        &self,
    ) -> terrars
    ::ListRef<
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElTimestampTypeOptionsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timestamp_type_options", self.base))
    }
}

#[derive(Serialize, Default)]
struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElDynamic {
    property_definitions: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    property_definitions: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsEl>,
    >,
    dynamic: DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElDynamic,
}

impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsEl {
    #[doc = "Set the field `property_definitions`.\n"]
    pub fn set_property_definitions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.property_definitions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.property_definitions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsEl {
    type O = terrars::BlockAssignable<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsEl {}

impl BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsEl {
    pub fn build(self) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsEl {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsEl {
            property_definitions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElRef {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `property_definitions` after provisioning.\n"]
    pub fn property_definitions(
        &self,
    ) -> terrars
    ::ListRef<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElPropertyDefinitionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.property_definitions", self.base))
    }
}

#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElSchemaSourcesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    processor_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElSchemaSourcesEl {
    #[doc = "Set the field `name`.\nThe schema name in the source."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `processor_type`.\nThe Doc AI processor type name."]
    pub fn set_processor_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.processor_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElSchemaSourcesEl {
    type O = terrars::BlockAssignable<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElSchemaSourcesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElSchemaSourcesEl {}

impl BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElSchemaSourcesEl {
    pub fn build(self) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElSchemaSourcesEl {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElSchemaSourcesEl {
            name: core::default::Default::default(),
            processor_type: core::default::Default::default(),
        }
    }
}

pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElSchemaSourcesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElSchemaSourcesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElSchemaSourcesElRef {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElSchemaSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElSchemaSourcesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe schema name in the source."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `processor_type` after provisioning.\nThe Doc AI processor type name."]
    pub fn processor_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.processor_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTextTypeOptionsEl {}

impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTextTypeOptionsEl { }

impl terrars::ToListMappable for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTextTypeOptionsEl {
    type O = terrars::BlockAssignable<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTextTypeOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTextTypeOptionsEl {}

impl BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTextTypeOptionsEl {
    pub fn build(self) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTextTypeOptionsEl {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTextTypeOptionsEl {}
    }
}

pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTextTypeOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTextTypeOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTextTypeOptionsElRef {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTextTypeOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTextTypeOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTimestampTypeOptionsEl {}

impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTimestampTypeOptionsEl { }

impl terrars::ToListMappable for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTimestampTypeOptionsEl {
    type O = terrars::BlockAssignable<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTimestampTypeOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTimestampTypeOptionsEl {}

impl BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTimestampTypeOptionsEl {
    pub fn build(self) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTimestampTypeOptionsEl {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTimestampTypeOptionsEl {}
    }
}

pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTimestampTypeOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTimestampTypeOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTimestampTypeOptionsElRef {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTimestampTypeOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTimestampTypeOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElDynamic {
    date_time_type_options: std
    ::option
    ::Option<terrars::DynamicBlock<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElDateTimeTypeOptionsEl>>,
    enum_type_options: std
    ::option
    ::Option<terrars::DynamicBlock<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElEnumTypeOptionsEl>>,
    float_type_options: std
    ::option
    ::Option<terrars::DynamicBlock<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElFloatTypeOptionsEl>>,
    integer_type_options: std
    ::option
    ::Option<terrars::DynamicBlock<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElIntegerTypeOptionsEl>>,
    map_type_options: std
    ::option
    ::Option<terrars::DynamicBlock<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElMapTypeOptionsEl>>,
    property_type_options: std
    ::option
    ::Option<terrars::DynamicBlock<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsEl>>,
    schema_sources: std
    ::option
    ::Option<terrars::DynamicBlock<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElSchemaSourcesEl>>,
    text_type_options: std
    ::option
    ::Option<terrars::DynamicBlock<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTextTypeOptionsEl>>,
    timestamp_type_options: std
    ::option
    ::Option<terrars::DynamicBlock<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTimestampTypeOptionsEl>>,
}

#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_filterable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_metadata: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_repeatable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_required: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_searchable: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retrieval_importance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_time_type_options: std
    ::option
    ::Option<std::vec::Vec<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElDateTimeTypeOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enum_type_options: std
    ::option
    ::Option<std::vec::Vec<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElEnumTypeOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    float_type_options: std
    ::option
    ::Option<std::vec::Vec<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElFloatTypeOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    integer_type_options: std
    ::option
    ::Option<std::vec::Vec<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElIntegerTypeOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    map_type_options: std
    ::option
    ::Option<std::vec::Vec<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElMapTypeOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    property_type_options: std
    ::option
    ::Option<std::vec::Vec<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_sources: std
    ::option
    ::Option<std::vec::Vec<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElSchemaSourcesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_type_options: std
    ::option
    ::Option<std::vec::Vec<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTextTypeOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timestamp_type_options: std
    ::option
    ::Option<std::vec::Vec<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTimestampTypeOptionsEl>>,
    dynamic: DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElDynamic,
}

impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsEl {
    #[doc = "Set the field `display_name`.\nThe display-name for the property, used for front-end."]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `is_filterable`.\nWhether the property can be filtered. If this is a sub-property, all the parent properties must be marked filterable."]
    pub fn set_is_filterable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_filterable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_metadata`.\nWhether the property is user supplied metadata."]
    pub fn set_is_metadata(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_repeatable`.\nWhether the property can have multiple values."]
    pub fn set_is_repeatable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_repeatable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_required`.\nWhether the property is mandatory."]
    pub fn set_is_required(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_required = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_searchable`.\nIndicates that the property should be included in a global search."]
    pub fn set_is_searchable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_searchable = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `retrieval_importance`.\nStores the retrieval importance. Possible values: [\"HIGHEST\", \"HIGHER\", \"HIGH\", \"MEDIUM\", \"LOW\", \"LOWEST\"]"]
    pub fn set_retrieval_importance(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.retrieval_importance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `date_time_type_options`.\n"]
    pub fn set_date_time_type_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElDateTimeTypeOptionsEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.date_time_type_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.date_time_type_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `enum_type_options`.\n"]
    pub fn set_enum_type_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElEnumTypeOptionsEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.enum_type_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.enum_type_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `float_type_options`.\n"]
    pub fn set_float_type_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElFloatTypeOptionsEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.float_type_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.float_type_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `integer_type_options`.\n"]
    pub fn set_integer_type_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElIntegerTypeOptionsEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.integer_type_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.integer_type_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `map_type_options`.\n"]
    pub fn set_map_type_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElMapTypeOptionsEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.map_type_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.map_type_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `property_type_options`.\n"]
    pub fn set_property_type_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.property_type_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.property_type_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `schema_sources`.\n"]
    pub fn set_schema_sources(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElSchemaSourcesEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.schema_sources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.schema_sources = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `text_type_options`.\n"]
    pub fn set_text_type_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTextTypeOptionsEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.text_type_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.text_type_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timestamp_type_options`.\n"]
    pub fn set_timestamp_type_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTimestampTypeOptionsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.timestamp_type_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.timestamp_type_options = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsEl {
    type O = terrars::BlockAssignable<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsEl {
    #[doc = "The name of the metadata property."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDocumentAiWarehouseDocumentSchemaPropertyDefinitionsEl {
    pub fn build(self) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsEl {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsEl {
            display_name: core::default::Default::default(),
            is_filterable: core::default::Default::default(),
            is_metadata: core::default::Default::default(),
            is_repeatable: core::default::Default::default(),
            is_required: core::default::Default::default(),
            is_searchable: core::default::Default::default(),
            name: self.name,
            retrieval_importance: core::default::Default::default(),
            date_time_type_options: core::default::Default::default(),
            enum_type_options: core::default::Default::default(),
            float_type_options: core::default::Default::default(),
            integer_type_options: core::default::Default::default(),
            map_type_options: core::default::Default::default(),
            property_type_options: core::default::Default::default(),
            schema_sources: core::default::Default::default(),
            text_type_options: core::default::Default::default(),
            timestamp_type_options: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElRef {
        DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display-name for the property, used for front-end."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_filterable` after provisioning.\nWhether the property can be filtered. If this is a sub-property, all the parent properties must be marked filterable."]
    pub fn is_filterable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_filterable", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_metadata` after provisioning.\nWhether the property is user supplied metadata."]
    pub fn is_metadata(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_metadata", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_repeatable` after provisioning.\nWhether the property can have multiple values."]
    pub fn is_repeatable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_repeatable", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_required` after provisioning.\nWhether the property is mandatory."]
    pub fn is_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_required", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_searchable` after provisioning.\nIndicates that the property should be included in a global search."]
    pub fn is_searchable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_searchable", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the metadata property."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retrieval_importance` after provisioning.\nStores the retrieval importance. Possible values: [\"HIGHEST\", \"HIGHER\", \"HIGH\", \"MEDIUM\", \"LOW\", \"LOWEST\"]"]
    pub fn retrieval_importance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retrieval_importance", self.base))
    }

    #[doc = "Get a reference to the value of field `date_time_type_options` after provisioning.\n"]
    pub fn date_time_type_options(
        &self,
    ) -> terrars::ListRef<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElDateTimeTypeOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.date_time_type_options", self.base))
    }

    #[doc = "Get a reference to the value of field `enum_type_options` after provisioning.\n"]
    pub fn enum_type_options(
        &self,
    ) -> terrars::ListRef<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElEnumTypeOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enum_type_options", self.base))
    }

    #[doc = "Get a reference to the value of field `float_type_options` after provisioning.\n"]
    pub fn float_type_options(
        &self,
    ) -> terrars::ListRef<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElFloatTypeOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.float_type_options", self.base))
    }

    #[doc = "Get a reference to the value of field `integer_type_options` after provisioning.\n"]
    pub fn integer_type_options(
        &self,
    ) -> terrars::ListRef<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElIntegerTypeOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.integer_type_options", self.base))
    }

    #[doc = "Get a reference to the value of field `map_type_options` after provisioning.\n"]
    pub fn map_type_options(
        &self,
    ) -> terrars::ListRef<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElMapTypeOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.map_type_options", self.base))
    }

    #[doc = "Get a reference to the value of field `property_type_options` after provisioning.\n"]
    pub fn property_type_options(
        &self,
    ) -> terrars::ListRef<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElPropertyTypeOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.property_type_options", self.base))
    }

    #[doc = "Get a reference to the value of field `schema_sources` after provisioning.\n"]
    pub fn schema_sources(
        &self,
    ) -> terrars::ListRef<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElSchemaSourcesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema_sources", self.base))
    }

    #[doc = "Get a reference to the value of field `text_type_options` after provisioning.\n"]
    pub fn text_type_options(
        &self,
    ) -> terrars::ListRef<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTextTypeOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text_type_options", self.base))
    }

    #[doc = "Get a reference to the value of field `timestamp_type_options` after provisioning.\n"]
    pub fn timestamp_type_options(
        &self,
    ) -> terrars::ListRef<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsElTimestampTypeOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timestamp_type_options", self.base))
    }
}

#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchemaTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DocumentAiWarehouseDocumentSchemaTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DocumentAiWarehouseDocumentSchemaTimeoutsEl {
    type O = terrars::BlockAssignable<DocumentAiWarehouseDocumentSchemaTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocumentAiWarehouseDocumentSchemaTimeoutsEl {}

impl BuildDocumentAiWarehouseDocumentSchemaTimeoutsEl {
    pub fn build(self) -> DocumentAiWarehouseDocumentSchemaTimeoutsEl {
        DocumentAiWarehouseDocumentSchemaTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct DocumentAiWarehouseDocumentSchemaTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DocumentAiWarehouseDocumentSchemaTimeoutsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DocumentAiWarehouseDocumentSchemaTimeoutsElRef {
        DocumentAiWarehouseDocumentSchemaTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocumentAiWarehouseDocumentSchemaTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}

#[derive(Serialize, Default)]
struct DocumentAiWarehouseDocumentSchemaDynamic {
    property_definitions: std
    ::option
    ::Option<terrars::DynamicBlock<DocumentAiWarehouseDocumentSchemaPropertyDefinitionsEl>>,
}
