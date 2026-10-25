#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DocumentAiWarehouseDocumentSchemaData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    document_is_folder: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    project_number: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    property_definitions: std::option::Option<std::vec::Vec<DocumentAiWarehouseDocumentSchema_PropertyDefinitions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DocumentAiWarehouseDocumentSchema_Timeouts>,
    dynamic: DocumentAiWarehouseDocumentSchemaDynamic,
}

struct DocumentAiWarehouseDocumentSchema_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DocumentAiWarehouseDocumentSchemaData_>,
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

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<DocumentAiWarehouseDocumentSchema_PropertyDefinitions>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<DocumentAiWarehouseDocumentSchema_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
    pub fn property_definitions(&self) -> terrars::ListRef<DocumentAiWarehouseDocumentSchema_PropertyDefinitionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.property_definitions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DocumentAiWarehouseDocumentSchema_TimeoutsRef {
        DocumentAiWarehouseDocumentSchema_TimeoutsRef::new(
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
            data: std::cell::RefCell::new(DocumentAiWarehouseDocumentSchemaData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
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
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
    pub fn property_definitions(&self) -> terrars::ListRef<DocumentAiWarehouseDocumentSchema_PropertyDefinitionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.property_definitions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DocumentAiWarehouseDocumentSchema_TimeoutsRef {
        DocumentAiWarehouseDocumentSchema_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct DocumentAiWarehouseDocumentSchemaDynamic {
    property_definitions: std
    ::option
    ::Option<terrars::DynamicBlock<DocumentAiWarehouseDocumentSchema_PropertyDefinitions>>,
}

#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchema_PropertyDefinitions_DateTimeTypeOptions {}

impl DocumentAiWarehouseDocumentSchema_PropertyDefinitions_DateTimeTypeOptions { }

impl terrars::ToListMappable for DocumentAiWarehouseDocumentSchema_PropertyDefinitions_DateTimeTypeOptions {
    type O = terrars::BlockAssignable<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_DateTimeTypeOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocumentAiWarehouseDocumentSchema_PropertyDefinitions_DateTimeTypeOptions {}

impl BuildDocumentAiWarehouseDocumentSchema_PropertyDefinitions_DateTimeTypeOptions {
    pub fn build(self) -> DocumentAiWarehouseDocumentSchema_PropertyDefinitions_DateTimeTypeOptions {
        DocumentAiWarehouseDocumentSchema_PropertyDefinitions_DateTimeTypeOptions {}
    }
}

#[allow(dead_code)]
pub struct DocumentAiWarehouseDocumentSchema_PropertyDefinitions_DateTimeTypeOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DocumentAiWarehouseDocumentSchema_PropertyDefinitions_DateTimeTypeOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DocumentAiWarehouseDocumentSchema_PropertyDefinitions_DateTimeTypeOptionsRef {
        DocumentAiWarehouseDocumentSchema_PropertyDefinitions_DateTimeTypeOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocumentAiWarehouseDocumentSchema_PropertyDefinitions_DateTimeTypeOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchema_PropertyDefinitions_EnumTypeOptions {
    possible_values: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validation_check_disabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DocumentAiWarehouseDocumentSchema_PropertyDefinitions_EnumTypeOptions {
    #[doc =
        "Set the field `validation_check_disabled`.\nMake sure the enum property value provided in the document is in the possile value list during document creation. The validation check runs by default."]
    pub fn set_validation_check_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.validation_check_disabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DocumentAiWarehouseDocumentSchema_PropertyDefinitions_EnumTypeOptions {
    type O = terrars::BlockAssignable<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_EnumTypeOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocumentAiWarehouseDocumentSchema_PropertyDefinitions_EnumTypeOptions {
    #[doc = "List of possible enum values."]
    pub possible_values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDocumentAiWarehouseDocumentSchema_PropertyDefinitions_EnumTypeOptions {
    pub fn build(self) -> DocumentAiWarehouseDocumentSchema_PropertyDefinitions_EnumTypeOptions {
        DocumentAiWarehouseDocumentSchema_PropertyDefinitions_EnumTypeOptions {
            possible_values: self.possible_values,
            validation_check_disabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DocumentAiWarehouseDocumentSchema_PropertyDefinitions_EnumTypeOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DocumentAiWarehouseDocumentSchema_PropertyDefinitions_EnumTypeOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DocumentAiWarehouseDocumentSchema_PropertyDefinitions_EnumTypeOptionsRef {
        DocumentAiWarehouseDocumentSchema_PropertyDefinitions_EnumTypeOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocumentAiWarehouseDocumentSchema_PropertyDefinitions_EnumTypeOptionsRef {
    #[allow(dead_code)]
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
pub struct DocumentAiWarehouseDocumentSchema_PropertyDefinitions_FloatTypeOptions {}

impl DocumentAiWarehouseDocumentSchema_PropertyDefinitions_FloatTypeOptions { }

impl terrars::ToListMappable for DocumentAiWarehouseDocumentSchema_PropertyDefinitions_FloatTypeOptions {
    type O = terrars::BlockAssignable<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_FloatTypeOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocumentAiWarehouseDocumentSchema_PropertyDefinitions_FloatTypeOptions {}

impl BuildDocumentAiWarehouseDocumentSchema_PropertyDefinitions_FloatTypeOptions {
    pub fn build(self) -> DocumentAiWarehouseDocumentSchema_PropertyDefinitions_FloatTypeOptions {
        DocumentAiWarehouseDocumentSchema_PropertyDefinitions_FloatTypeOptions {}
    }
}

#[allow(dead_code)]
pub struct DocumentAiWarehouseDocumentSchema_PropertyDefinitions_FloatTypeOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DocumentAiWarehouseDocumentSchema_PropertyDefinitions_FloatTypeOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DocumentAiWarehouseDocumentSchema_PropertyDefinitions_FloatTypeOptionsRef {
        DocumentAiWarehouseDocumentSchema_PropertyDefinitions_FloatTypeOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocumentAiWarehouseDocumentSchema_PropertyDefinitions_FloatTypeOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchema_PropertyDefinitions_IntegerTypeOptions {}

impl DocumentAiWarehouseDocumentSchema_PropertyDefinitions_IntegerTypeOptions { }

impl terrars::ToListMappable for DocumentAiWarehouseDocumentSchema_PropertyDefinitions_IntegerTypeOptions {
    type O = terrars::BlockAssignable<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_IntegerTypeOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocumentAiWarehouseDocumentSchema_PropertyDefinitions_IntegerTypeOptions {}

impl BuildDocumentAiWarehouseDocumentSchema_PropertyDefinitions_IntegerTypeOptions {
    pub fn build(self) -> DocumentAiWarehouseDocumentSchema_PropertyDefinitions_IntegerTypeOptions {
        DocumentAiWarehouseDocumentSchema_PropertyDefinitions_IntegerTypeOptions {}
    }
}

#[allow(dead_code)]
pub struct DocumentAiWarehouseDocumentSchema_PropertyDefinitions_IntegerTypeOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DocumentAiWarehouseDocumentSchema_PropertyDefinitions_IntegerTypeOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DocumentAiWarehouseDocumentSchema_PropertyDefinitions_IntegerTypeOptionsRef {
        DocumentAiWarehouseDocumentSchema_PropertyDefinitions_IntegerTypeOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocumentAiWarehouseDocumentSchema_PropertyDefinitions_IntegerTypeOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchema_PropertyDefinitions_MapTypeOptions {}

impl DocumentAiWarehouseDocumentSchema_PropertyDefinitions_MapTypeOptions { }

impl terrars::ToListMappable for DocumentAiWarehouseDocumentSchema_PropertyDefinitions_MapTypeOptions {
    type O = terrars::BlockAssignable<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_MapTypeOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocumentAiWarehouseDocumentSchema_PropertyDefinitions_MapTypeOptions {}

impl BuildDocumentAiWarehouseDocumentSchema_PropertyDefinitions_MapTypeOptions {
    pub fn build(self) -> DocumentAiWarehouseDocumentSchema_PropertyDefinitions_MapTypeOptions {
        DocumentAiWarehouseDocumentSchema_PropertyDefinitions_MapTypeOptions {}
    }
}

#[allow(dead_code)]
pub struct DocumentAiWarehouseDocumentSchema_PropertyDefinitions_MapTypeOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DocumentAiWarehouseDocumentSchema_PropertyDefinitions_MapTypeOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DocumentAiWarehouseDocumentSchema_PropertyDefinitions_MapTypeOptionsRef {
        DocumentAiWarehouseDocumentSchema_PropertyDefinitions_MapTypeOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocumentAiWarehouseDocumentSchema_PropertyDefinitions_MapTypeOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchema_PropertyDefinitions_SchemaSources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    processor_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DocumentAiWarehouseDocumentSchema_PropertyDefinitions_SchemaSources {
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

impl terrars::ToListMappable for DocumentAiWarehouseDocumentSchema_PropertyDefinitions_SchemaSources {
    type O = terrars::BlockAssignable<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_SchemaSources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocumentAiWarehouseDocumentSchema_PropertyDefinitions_SchemaSources {}

impl BuildDocumentAiWarehouseDocumentSchema_PropertyDefinitions_SchemaSources {
    pub fn build(self) -> DocumentAiWarehouseDocumentSchema_PropertyDefinitions_SchemaSources {
        DocumentAiWarehouseDocumentSchema_PropertyDefinitions_SchemaSources {
            name: core::default::Default::default(),
            processor_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DocumentAiWarehouseDocumentSchema_PropertyDefinitions_SchemaSourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DocumentAiWarehouseDocumentSchema_PropertyDefinitions_SchemaSourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DocumentAiWarehouseDocumentSchema_PropertyDefinitions_SchemaSourcesRef {
        DocumentAiWarehouseDocumentSchema_PropertyDefinitions_SchemaSourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocumentAiWarehouseDocumentSchema_PropertyDefinitions_SchemaSourcesRef {
    #[allow(dead_code)]
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
pub struct DocumentAiWarehouseDocumentSchema_PropertyDefinitions_TextTypeOptions {}

impl DocumentAiWarehouseDocumentSchema_PropertyDefinitions_TextTypeOptions { }

impl terrars::ToListMappable for DocumentAiWarehouseDocumentSchema_PropertyDefinitions_TextTypeOptions {
    type O = terrars::BlockAssignable<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_TextTypeOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocumentAiWarehouseDocumentSchema_PropertyDefinitions_TextTypeOptions {}

impl BuildDocumentAiWarehouseDocumentSchema_PropertyDefinitions_TextTypeOptions {
    pub fn build(self) -> DocumentAiWarehouseDocumentSchema_PropertyDefinitions_TextTypeOptions {
        DocumentAiWarehouseDocumentSchema_PropertyDefinitions_TextTypeOptions {}
    }
}

#[allow(dead_code)]
pub struct DocumentAiWarehouseDocumentSchema_PropertyDefinitions_TextTypeOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DocumentAiWarehouseDocumentSchema_PropertyDefinitions_TextTypeOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DocumentAiWarehouseDocumentSchema_PropertyDefinitions_TextTypeOptionsRef {
        DocumentAiWarehouseDocumentSchema_PropertyDefinitions_TextTypeOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocumentAiWarehouseDocumentSchema_PropertyDefinitions_TextTypeOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchema_PropertyDefinitions_TimestampTypeOptions {}

impl DocumentAiWarehouseDocumentSchema_PropertyDefinitions_TimestampTypeOptions { }

impl terrars::ToListMappable for DocumentAiWarehouseDocumentSchema_PropertyDefinitions_TimestampTypeOptions {
    type O = terrars::BlockAssignable<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_TimestampTypeOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocumentAiWarehouseDocumentSchema_PropertyDefinitions_TimestampTypeOptions {}

impl BuildDocumentAiWarehouseDocumentSchema_PropertyDefinitions_TimestampTypeOptions {
    pub fn build(self) -> DocumentAiWarehouseDocumentSchema_PropertyDefinitions_TimestampTypeOptions {
        DocumentAiWarehouseDocumentSchema_PropertyDefinitions_TimestampTypeOptions {}
    }
}

#[allow(dead_code)]
pub struct DocumentAiWarehouseDocumentSchema_PropertyDefinitions_TimestampTypeOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DocumentAiWarehouseDocumentSchema_PropertyDefinitions_TimestampTypeOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DocumentAiWarehouseDocumentSchema_PropertyDefinitions_TimestampTypeOptionsRef {
        DocumentAiWarehouseDocumentSchema_PropertyDefinitions_TimestampTypeOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocumentAiWarehouseDocumentSchema_PropertyDefinitions_TimestampTypeOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct DocumentAiWarehouseDocumentSchema_PropertyDefinitions_PropertyTypeOptions_PropertyDefinitionsDynamic {
    date_time_type_options: std
    ::option
    ::Option<terrars::DynamicBlock<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_DateTimeTypeOptions>>,
    enum_type_options: std
    ::option
    ::Option<terrars::DynamicBlock<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_EnumTypeOptions>>,
    float_type_options: std
    ::option
    ::Option<terrars::DynamicBlock<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_FloatTypeOptions>>,
    integer_type_options: std
    ::option
    ::Option<terrars::DynamicBlock<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_IntegerTypeOptions>>,
    map_type_options: std
    ::option
    ::Option<terrars::DynamicBlock<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_MapTypeOptions>>,
    schema_sources: std
    ::option
    ::Option<terrars::DynamicBlock<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_SchemaSources>>,
    text_type_options: std
    ::option
    ::Option<terrars::DynamicBlock<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_TextTypeOptions>>,
    timestamp_type_options: std
    ::option
    ::Option<terrars::DynamicBlock<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_TimestampTypeOptions>>,
}

#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchema_PropertyDefinitions_PropertyTypeOptions_PropertyDefinitions {
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
    ::Option<std::vec::Vec<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_DateTimeTypeOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enum_type_options: std
    ::option
    ::Option<std::vec::Vec<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_EnumTypeOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    float_type_options: std
    ::option
    ::Option<std::vec::Vec<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_FloatTypeOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    integer_type_options: std
    ::option
    ::Option<std::vec::Vec<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_IntegerTypeOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    map_type_options: std
    ::option
    ::Option<std::vec::Vec<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_MapTypeOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_sources: std
    ::option
    ::Option<std::vec::Vec<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_SchemaSources>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_type_options: std
    ::option
    ::Option<std::vec::Vec<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_TextTypeOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timestamp_type_options: std
    ::option
    ::Option<std::vec::Vec<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_TimestampTypeOptions>>,
    dynamic: DocumentAiWarehouseDocumentSchema_PropertyDefinitions_PropertyTypeOptions_PropertyDefinitionsDynamic,
}

impl DocumentAiWarehouseDocumentSchema_PropertyDefinitions_PropertyTypeOptions_PropertyDefinitions {
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
                        ::BlockAssignable<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_DateTimeTypeOptions>,
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
                        ::BlockAssignable<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_EnumTypeOptions>,
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
                        ::BlockAssignable<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_FloatTypeOptions>,
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
                        ::BlockAssignable<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_IntegerTypeOptions>,
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
                        terrars::BlockAssignable<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_MapTypeOptions>,
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
                        terrars::BlockAssignable<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_SchemaSources>,
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
                        ::BlockAssignable<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_TextTypeOptions>,
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
                        ::BlockAssignable<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_TimestampTypeOptions>,
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
::ToListMappable for DocumentAiWarehouseDocumentSchema_PropertyDefinitions_PropertyTypeOptions_PropertyDefinitions {
    type O =
        terrars
        ::BlockAssignable<
            DocumentAiWarehouseDocumentSchema_PropertyDefinitions_PropertyTypeOptions_PropertyDefinitions,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocumentAiWarehouseDocumentSchema_PropertyDefinitions_PropertyTypeOptions_PropertyDefinitions {
    #[doc = "The name of the metadata property."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDocumentAiWarehouseDocumentSchema_PropertyDefinitions_PropertyTypeOptions_PropertyDefinitions {
    pub fn build(
        self,
    ) -> DocumentAiWarehouseDocumentSchema_PropertyDefinitions_PropertyTypeOptions_PropertyDefinitions {
        DocumentAiWarehouseDocumentSchema_PropertyDefinitions_PropertyTypeOptions_PropertyDefinitions {
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

#[allow(dead_code)]
pub struct DocumentAiWarehouseDocumentSchema_PropertyDefinitions_PropertyTypeOptions_PropertyDefinitionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DocumentAiWarehouseDocumentSchema_PropertyDefinitions_PropertyTypeOptions_PropertyDefinitionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DocumentAiWarehouseDocumentSchema_PropertyDefinitions_PropertyTypeOptions_PropertyDefinitionsRef {
        DocumentAiWarehouseDocumentSchema_PropertyDefinitions_PropertyTypeOptions_PropertyDefinitionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocumentAiWarehouseDocumentSchema_PropertyDefinitions_PropertyTypeOptions_PropertyDefinitionsRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_DateTimeTypeOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.date_time_type_options", self.base))
    }

    #[doc = "Get a reference to the value of field `enum_type_options` after provisioning.\n"]
    pub fn enum_type_options(
        &self,
    ) -> terrars::ListRef<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_EnumTypeOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enum_type_options", self.base))
    }

    #[doc = "Get a reference to the value of field `float_type_options` after provisioning.\n"]
    pub fn float_type_options(
        &self,
    ) -> terrars::ListRef<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_FloatTypeOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.float_type_options", self.base))
    }

    #[doc = "Get a reference to the value of field `integer_type_options` after provisioning.\n"]
    pub fn integer_type_options(
        &self,
    ) -> terrars::ListRef<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_IntegerTypeOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.integer_type_options", self.base))
    }

    #[doc = "Get a reference to the value of field `map_type_options` after provisioning.\n"]
    pub fn map_type_options(
        &self,
    ) -> terrars::ListRef<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_MapTypeOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.map_type_options", self.base))
    }

    #[doc = "Get a reference to the value of field `schema_sources` after provisioning.\n"]
    pub fn schema_sources(
        &self,
    ) -> terrars::ListRef<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_SchemaSourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema_sources", self.base))
    }

    #[doc = "Get a reference to the value of field `text_type_options` after provisioning.\n"]
    pub fn text_type_options(
        &self,
    ) -> terrars::ListRef<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_TextTypeOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text_type_options", self.base))
    }

    #[doc = "Get a reference to the value of field `timestamp_type_options` after provisioning.\n"]
    pub fn timestamp_type_options(
        &self,
    ) -> terrars::ListRef<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_TimestampTypeOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timestamp_type_options", self.base))
    }
}

#[derive(Serialize, Default)]
struct DocumentAiWarehouseDocumentSchema_PropertyDefinitions_PropertyTypeOptionsDynamic {
    property_definitions: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_PropertyTypeOptions_PropertyDefinitions>,
    >,
}

#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchema_PropertyDefinitions_PropertyTypeOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    property_definitions: std
    ::option
    ::Option<
        std::vec::Vec<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_PropertyTypeOptions_PropertyDefinitions>,
    >,
    dynamic: DocumentAiWarehouseDocumentSchema_PropertyDefinitions_PropertyTypeOptionsDynamic,
}

impl DocumentAiWarehouseDocumentSchema_PropertyDefinitions_PropertyTypeOptions {
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
                            DocumentAiWarehouseDocumentSchema_PropertyDefinitions_PropertyTypeOptions_PropertyDefinitions,
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

impl terrars::ToListMappable for DocumentAiWarehouseDocumentSchema_PropertyDefinitions_PropertyTypeOptions {
    type O = terrars::BlockAssignable<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_PropertyTypeOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocumentAiWarehouseDocumentSchema_PropertyDefinitions_PropertyTypeOptions {}

impl BuildDocumentAiWarehouseDocumentSchema_PropertyDefinitions_PropertyTypeOptions {
    pub fn build(self) -> DocumentAiWarehouseDocumentSchema_PropertyDefinitions_PropertyTypeOptions {
        DocumentAiWarehouseDocumentSchema_PropertyDefinitions_PropertyTypeOptions {
            property_definitions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DocumentAiWarehouseDocumentSchema_PropertyDefinitions_PropertyTypeOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DocumentAiWarehouseDocumentSchema_PropertyDefinitions_PropertyTypeOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DocumentAiWarehouseDocumentSchema_PropertyDefinitions_PropertyTypeOptionsRef {
        DocumentAiWarehouseDocumentSchema_PropertyDefinitions_PropertyTypeOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocumentAiWarehouseDocumentSchema_PropertyDefinitions_PropertyTypeOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `property_definitions` after provisioning.\n"]
    pub fn property_definitions(
        &self,
    ) -> terrars
    ::ListRef<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_PropertyTypeOptions_PropertyDefinitionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.property_definitions", self.base))
    }
}

#[derive(Serialize, Default)]
struct DocumentAiWarehouseDocumentSchema_PropertyDefinitionsDynamic {
    date_time_type_options: std
    ::option
    ::Option<terrars::DynamicBlock<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_DateTimeTypeOptions>>,
    enum_type_options: std
    ::option
    ::Option<terrars::DynamicBlock<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_EnumTypeOptions>>,
    float_type_options: std
    ::option
    ::Option<terrars::DynamicBlock<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_FloatTypeOptions>>,
    integer_type_options: std
    ::option
    ::Option<terrars::DynamicBlock<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_IntegerTypeOptions>>,
    map_type_options: std
    ::option
    ::Option<terrars::DynamicBlock<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_MapTypeOptions>>,
    property_type_options: std
    ::option
    ::Option<terrars::DynamicBlock<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_PropertyTypeOptions>>,
    schema_sources: std
    ::option
    ::Option<terrars::DynamicBlock<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_SchemaSources>>,
    text_type_options: std
    ::option
    ::Option<terrars::DynamicBlock<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_TextTypeOptions>>,
    timestamp_type_options: std
    ::option
    ::Option<terrars::DynamicBlock<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_TimestampTypeOptions>>,
}

#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchema_PropertyDefinitions {
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
    ::Option<std::vec::Vec<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_DateTimeTypeOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enum_type_options: std
    ::option
    ::Option<std::vec::Vec<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_EnumTypeOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    float_type_options: std
    ::option
    ::Option<std::vec::Vec<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_FloatTypeOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    integer_type_options: std
    ::option
    ::Option<std::vec::Vec<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_IntegerTypeOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    map_type_options: std
    ::option
    ::Option<std::vec::Vec<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_MapTypeOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    property_type_options: std
    ::option
    ::Option<std::vec::Vec<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_PropertyTypeOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_sources: std
    ::option
    ::Option<std::vec::Vec<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_SchemaSources>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_type_options: std
    ::option
    ::Option<std::vec::Vec<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_TextTypeOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timestamp_type_options: std
    ::option
    ::Option<std::vec::Vec<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_TimestampTypeOptions>>,
    dynamic: DocumentAiWarehouseDocumentSchema_PropertyDefinitionsDynamic,
}

impl DocumentAiWarehouseDocumentSchema_PropertyDefinitions {
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
                        ::BlockAssignable<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_DateTimeTypeOptions>,
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
                        ::BlockAssignable<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_EnumTypeOptions>,
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
                        ::BlockAssignable<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_FloatTypeOptions>,
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
                        ::BlockAssignable<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_IntegerTypeOptions>,
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
                        terrars::BlockAssignable<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_MapTypeOptions>,
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
                        ::BlockAssignable<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_PropertyTypeOptions>,
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
                        terrars::BlockAssignable<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_SchemaSources>,
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
                        ::BlockAssignable<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_TextTypeOptions>,
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
                        ::BlockAssignable<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_TimestampTypeOptions>,
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

impl terrars::ToListMappable for DocumentAiWarehouseDocumentSchema_PropertyDefinitions {
    type O = terrars::BlockAssignable<DocumentAiWarehouseDocumentSchema_PropertyDefinitions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocumentAiWarehouseDocumentSchema_PropertyDefinitions {
    #[doc = "The name of the metadata property."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDocumentAiWarehouseDocumentSchema_PropertyDefinitions {
    pub fn build(self) -> DocumentAiWarehouseDocumentSchema_PropertyDefinitions {
        DocumentAiWarehouseDocumentSchema_PropertyDefinitions {
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

#[allow(dead_code)]
pub struct DocumentAiWarehouseDocumentSchema_PropertyDefinitionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DocumentAiWarehouseDocumentSchema_PropertyDefinitionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DocumentAiWarehouseDocumentSchema_PropertyDefinitionsRef {
        DocumentAiWarehouseDocumentSchema_PropertyDefinitionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocumentAiWarehouseDocumentSchema_PropertyDefinitionsRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_DateTimeTypeOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.date_time_type_options", self.base))
    }

    #[doc = "Get a reference to the value of field `enum_type_options` after provisioning.\n"]
    pub fn enum_type_options(
        &self,
    ) -> terrars::ListRef<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_EnumTypeOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enum_type_options", self.base))
    }

    #[doc = "Get a reference to the value of field `float_type_options` after provisioning.\n"]
    pub fn float_type_options(
        &self,
    ) -> terrars::ListRef<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_FloatTypeOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.float_type_options", self.base))
    }

    #[doc = "Get a reference to the value of field `integer_type_options` after provisioning.\n"]
    pub fn integer_type_options(
        &self,
    ) -> terrars::ListRef<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_IntegerTypeOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.integer_type_options", self.base))
    }

    #[doc = "Get a reference to the value of field `map_type_options` after provisioning.\n"]
    pub fn map_type_options(
        &self,
    ) -> terrars::ListRef<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_MapTypeOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.map_type_options", self.base))
    }

    #[doc = "Get a reference to the value of field `property_type_options` after provisioning.\n"]
    pub fn property_type_options(
        &self,
    ) -> terrars::ListRef<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_PropertyTypeOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.property_type_options", self.base))
    }

    #[doc = "Get a reference to the value of field `schema_sources` after provisioning.\n"]
    pub fn schema_sources(
        &self,
    ) -> terrars::ListRef<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_SchemaSourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema_sources", self.base))
    }

    #[doc = "Get a reference to the value of field `text_type_options` after provisioning.\n"]
    pub fn text_type_options(
        &self,
    ) -> terrars::ListRef<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_TextTypeOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text_type_options", self.base))
    }

    #[doc = "Get a reference to the value of field `timestamp_type_options` after provisioning.\n"]
    pub fn timestamp_type_options(
        &self,
    ) -> terrars::ListRef<DocumentAiWarehouseDocumentSchema_PropertyDefinitions_TimestampTypeOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timestamp_type_options", self.base))
    }
}

#[derive(Serialize)]
pub struct DocumentAiWarehouseDocumentSchema_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DocumentAiWarehouseDocumentSchema_Timeouts {
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

impl terrars::ToListMappable for DocumentAiWarehouseDocumentSchema_Timeouts {
    type O = terrars::BlockAssignable<DocumentAiWarehouseDocumentSchema_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocumentAiWarehouseDocumentSchema_Timeouts {}

impl BuildDocumentAiWarehouseDocumentSchema_Timeouts {
    pub fn build(self) -> DocumentAiWarehouseDocumentSchema_Timeouts {
        DocumentAiWarehouseDocumentSchema_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DocumentAiWarehouseDocumentSchema_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DocumentAiWarehouseDocumentSchema_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DocumentAiWarehouseDocumentSchema_TimeoutsRef {
        DocumentAiWarehouseDocumentSchema_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocumentAiWarehouseDocumentSchema_TimeoutsRef {
    #[allow(dead_code)]
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
