#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DocumentAiSchemaVersionData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    schema_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema: std::option::Option<std::vec::Vec<DocumentAiSchemaVersion_Schema>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DocumentAiSchemaVersion_Timeouts>,
    dynamic: DocumentAiSchemaVersionDynamic,
}

struct DocumentAiSchemaVersion_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DocumentAiSchemaVersionData_>,
}

#[derive(Clone)]
pub struct DocumentAiSchemaVersion(std::rc::Rc<DocumentAiSchemaVersion_>);

impl DocumentAiSchemaVersion {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nThe Google Cloud labels for the SchemaVersion.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schema`.\n"]
    pub fn set_schema(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DocumentAiSchemaVersion_Schema>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schema = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schema = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DocumentAiSchemaVersion_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the SchemaVersion was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe user-defined name of the SchemaVersion."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe Google Cloud labels for the SchemaVersion.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the SchemaVersion."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `schema_id` after provisioning.\nThe schema id that this schema version belongs to."]
    pub fn schema_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> terrars::ListRef<DocumentAiSchemaVersion_SchemaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DocumentAiSchemaVersion_TimeoutsRef {
        DocumentAiSchemaVersion_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DocumentAiSchemaVersion {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DocumentAiSchemaVersion { }

impl terrars::ToListMappable for DocumentAiSchemaVersion {
    type O = terrars::ListRef<DocumentAiSchemaVersionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DocumentAiSchemaVersion_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_document_ai_schema_version".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDocumentAiSchemaVersion {
    pub tf_id: std::string::String,
    #[doc = "The user-defined name of the SchemaVersion."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc = "The location of the resource."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The schema id that this schema version belongs to."]
    pub schema_id: terrars::PrimField<std::string::String>,
}

impl BuildDocumentAiSchemaVersion {
    pub fn build(self, stack: &mut terrars::Stack) -> DocumentAiSchemaVersion {
        let out = DocumentAiSchemaVersion(std::rc::Rc::new(DocumentAiSchemaVersion_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DocumentAiSchemaVersionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                schema_id: self.schema_id,
                schema: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DocumentAiSchemaVersionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DocumentAiSchemaVersionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DocumentAiSchemaVersionRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time when the SchemaVersion was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe user-defined name of the SchemaVersion."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe Google Cloud labels for the SchemaVersion.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the resource."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the SchemaVersion."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `schema_id` after provisioning.\nThe schema id that this schema version belongs to."]
    pub fn schema_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> terrars::ListRef<DocumentAiSchemaVersion_SchemaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DocumentAiSchemaVersion_TimeoutsRef {
        DocumentAiSchemaVersion_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DocumentAiSchemaVersionDynamic {
    schema: std::option::Option<terrars::DynamicBlock<DocumentAiSchemaVersion_Schema>>,
}

#[derive(Serialize)]
pub struct DocumentAiSchemaVersion_Schema_EntityTypes_EnumValues {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DocumentAiSchemaVersion_Schema_EntityTypes_EnumValues {
    #[doc = "Set the field `values`.\nThe individual values that this enum values type can include."]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DocumentAiSchemaVersion_Schema_EntityTypes_EnumValues {
    type O = terrars::BlockAssignable<DocumentAiSchemaVersion_Schema_EntityTypes_EnumValues>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocumentAiSchemaVersion_Schema_EntityTypes_EnumValues {}

impl BuildDocumentAiSchemaVersion_Schema_EntityTypes_EnumValues {
    pub fn build(self) -> DocumentAiSchemaVersion_Schema_EntityTypes_EnumValues {
        DocumentAiSchemaVersion_Schema_EntityTypes_EnumValues { values: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DocumentAiSchemaVersion_Schema_EntityTypes_EnumValuesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DocumentAiSchemaVersion_Schema_EntityTypes_EnumValuesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DocumentAiSchemaVersion_Schema_EntityTypes_EnumValuesRef {
        DocumentAiSchemaVersion_Schema_EntityTypes_EnumValuesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocumentAiSchemaVersion_Schema_EntityTypes_EnumValuesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `values` after provisioning.\nThe individual values that this enum values type can include."]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DocumentAiSchemaVersion_Schema_EntityTypes_Properties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    occurrence_type: std::option::Option<terrars::PrimField<std::string::String>>,
    value_type: terrars::PrimField<std::string::String>,
}

impl DocumentAiSchemaVersion_Schema_EntityTypes_Properties {
    #[doc = "Set the field `display_name`.\nUser defined name for the property."]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `method`.\nSpecifies how the entity's value is obtained. Possible values: [\"EXTRACT\", \"DERIVE\", \"RELAXED_EXTRACT\"]"]
    pub fn set_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.method = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `occurrence_type`.\nOccurrence type limits the number of instances an entity type appears in the document. Possible values: [\"OPTIONAL_ONCE\", \"OPTIONAL_MULTIPLE\", \"REQUIRED_ONCE\", \"REQUIRED_MULTIPLE\"]"]
    pub fn set_occurrence_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.occurrence_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DocumentAiSchemaVersion_Schema_EntityTypes_Properties {
    type O = terrars::BlockAssignable<DocumentAiSchemaVersion_Schema_EntityTypes_Properties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocumentAiSchemaVersion_Schema_EntityTypes_Properties {
    #[doc = "The name of the property. Follows the same guidelines as the EntityType name."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "A reference to the value type of the property. This type is subject to the same conventions as the 'Entity.base_types' field."]
    pub value_type: terrars::PrimField<std::string::String>,
}

impl BuildDocumentAiSchemaVersion_Schema_EntityTypes_Properties {
    pub fn build(self) -> DocumentAiSchemaVersion_Schema_EntityTypes_Properties {
        DocumentAiSchemaVersion_Schema_EntityTypes_Properties {
            display_name: core::default::Default::default(),
            method: core::default::Default::default(),
            name: self.name,
            occurrence_type: core::default::Default::default(),
            value_type: self.value_type,
        }
    }
}

#[allow(dead_code)]
pub struct DocumentAiSchemaVersion_Schema_EntityTypes_PropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DocumentAiSchemaVersion_Schema_EntityTypes_PropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DocumentAiSchemaVersion_Schema_EntityTypes_PropertiesRef {
        DocumentAiSchemaVersion_Schema_EntityTypes_PropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocumentAiSchemaVersion_Schema_EntityTypes_PropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nUser defined name for the property."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `method` after provisioning.\nSpecifies how the entity's value is obtained. Possible values: [\"EXTRACT\", \"DERIVE\", \"RELAXED_EXTRACT\"]"]
    pub fn method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the property. Follows the same guidelines as the EntityType name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `occurrence_type` after provisioning.\nOccurrence type limits the number of instances an entity type appears in the document. Possible values: [\"OPTIONAL_ONCE\", \"OPTIONAL_MULTIPLE\", \"REQUIRED_ONCE\", \"REQUIRED_MULTIPLE\"]"]
    pub fn occurrence_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.occurrence_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value_type` after provisioning.\nA reference to the value type of the property. This type is subject to the same conventions as the 'Entity.base_types' field."]
    pub fn value_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct DocumentAiSchemaVersion_Schema_EntityTypesDynamic {
    enum_values: std::option::Option<terrars::DynamicBlock<DocumentAiSchemaVersion_Schema_EntityTypes_EnumValues>>,
    properties: std::option::Option<terrars::DynamicBlock<DocumentAiSchemaVersion_Schema_EntityTypes_Properties>>,
}

#[derive(Serialize)]
pub struct DocumentAiSchemaVersion_Schema_EntityTypes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enum_values: std::option::Option<std::vec::Vec<DocumentAiSchemaVersion_Schema_EntityTypes_EnumValues>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<std::vec::Vec<DocumentAiSchemaVersion_Schema_EntityTypes_Properties>>,
    dynamic: DocumentAiSchemaVersion_Schema_EntityTypesDynamic,
}

impl DocumentAiSchemaVersion_Schema_EntityTypes {
    #[doc =
        "Set the field `base_types`.\nThe entity type that this type is derived from. For now, one and only one should be set."]
    pub fn set_base_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.base_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nUser defined name for the type."]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enum_values`.\n"]
    pub fn set_enum_values(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DocumentAiSchemaVersion_Schema_EntityTypes_EnumValues>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.enum_values = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.enum_values = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DocumentAiSchemaVersion_Schema_EntityTypes_Properties>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.properties = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DocumentAiSchemaVersion_Schema_EntityTypes {
    type O = terrars::BlockAssignable<DocumentAiSchemaVersion_Schema_EntityTypes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocumentAiSchemaVersion_Schema_EntityTypes {
    #[doc = "Name of the type. It must be unique within the schema file and cannot be a \"Common Type\"."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDocumentAiSchemaVersion_Schema_EntityTypes {
    pub fn build(self) -> DocumentAiSchemaVersion_Schema_EntityTypes {
        DocumentAiSchemaVersion_Schema_EntityTypes {
            base_types: core::default::Default::default(),
            display_name: core::default::Default::default(),
            name: self.name,
            enum_values: core::default::Default::default(),
            properties: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DocumentAiSchemaVersion_Schema_EntityTypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DocumentAiSchemaVersion_Schema_EntityTypesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DocumentAiSchemaVersion_Schema_EntityTypesRef {
        DocumentAiSchemaVersion_Schema_EntityTypesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocumentAiSchemaVersion_Schema_EntityTypesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `base_types` after provisioning.\nThe entity type that this type is derived from. For now, one and only one should be set."]
    pub fn base_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.base_types", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser defined name for the type."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the type. It must be unique within the schema file and cannot be a \"Common Type\"."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `enum_values` after provisioning.\n"]
    pub fn enum_values(&self) -> terrars::ListRef<DocumentAiSchemaVersion_Schema_EntityTypes_EnumValuesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enum_values", self.base))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> terrars::ListRef<DocumentAiSchemaVersion_Schema_EntityTypes_PropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
}

#[derive(Serialize)]
pub struct DocumentAiSchemaVersion_Schema_Metadata {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    document_allow_multiple_labels: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    document_splitter: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefixed_naming_on_properties: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skip_naming_validation: std::option::Option<terrars::PrimField<bool>>,
}

impl DocumentAiSchemaVersion_Schema_Metadata {
    #[doc =
        "Set the field `document_allow_multiple_labels`.\nIf true, on a given page, there can be multiple 'document' annotations covering it."]
    pub fn set_document_allow_multiple_labels(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.document_allow_multiple_labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `document_splitter`.\nIf true, a 'document' entity type can be applied to subdocument (splitting). Otherwise, it can only be applied to the entire document (classification)."]
    pub fn set_document_splitter(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.document_splitter = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `prefixed_naming_on_properties`.\nIf set, all the nested entities must be prefixed with the parents."]
    pub fn set_prefixed_naming_on_properties(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.prefixed_naming_on_properties = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `skip_naming_validation`.\nIf set, this will skip the naming format validation in the schema. So the string values in 'DocumentSchema.EntityType.name' and 'DocumentSchema.EntityType.Property.name' will not be checked."]
    pub fn set_skip_naming_validation(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.skip_naming_validation = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DocumentAiSchemaVersion_Schema_Metadata {
    type O = terrars::BlockAssignable<DocumentAiSchemaVersion_Schema_Metadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocumentAiSchemaVersion_Schema_Metadata {}

impl BuildDocumentAiSchemaVersion_Schema_Metadata {
    pub fn build(self) -> DocumentAiSchemaVersion_Schema_Metadata {
        DocumentAiSchemaVersion_Schema_Metadata {
            document_allow_multiple_labels: core::default::Default::default(),
            document_splitter: core::default::Default::default(),
            prefixed_naming_on_properties: core::default::Default::default(),
            skip_naming_validation: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DocumentAiSchemaVersion_Schema_MetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DocumentAiSchemaVersion_Schema_MetadataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DocumentAiSchemaVersion_Schema_MetadataRef {
        DocumentAiSchemaVersion_Schema_MetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocumentAiSchemaVersion_Schema_MetadataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `document_allow_multiple_labels` after provisioning.\nIf true, on a given page, there can be multiple 'document' annotations covering it."]
    pub fn document_allow_multiple_labels(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.document_allow_multiple_labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `document_splitter` after provisioning.\nIf true, a 'document' entity type can be applied to subdocument (splitting). Otherwise, it can only be applied to the entire document (classification)."]
    pub fn document_splitter(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.document_splitter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prefixed_naming_on_properties` after provisioning.\nIf set, all the nested entities must be prefixed with the parents."]
    pub fn prefixed_naming_on_properties(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefixed_naming_on_properties", self.base))
    }

    #[doc =
        "Get a reference to the value of field `skip_naming_validation` after provisioning.\nIf set, this will skip the naming format validation in the schema. So the string values in 'DocumentSchema.EntityType.name' and 'DocumentSchema.EntityType.Property.name' will not be checked."]
    pub fn skip_naming_validation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_naming_validation", self.base))
    }
}

#[derive(Serialize, Default)]
struct DocumentAiSchemaVersion_SchemaDynamic {
    entity_types: std::option::Option<terrars::DynamicBlock<DocumentAiSchemaVersion_Schema_EntityTypes>>,
    metadata: std::option::Option<terrars::DynamicBlock<DocumentAiSchemaVersion_Schema_Metadata>>,
}

#[derive(Serialize)]
pub struct DocumentAiSchemaVersion_Schema {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    document_prompt: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entity_types: std::option::Option<std::vec::Vec<DocumentAiSchemaVersion_Schema_EntityTypes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<std::vec::Vec<DocumentAiSchemaVersion_Schema_Metadata>>,
    dynamic: DocumentAiSchemaVersion_SchemaDynamic,
}

impl DocumentAiSchemaVersion_Schema {
    #[doc = "Set the field `description`.\nDescription of the schema."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nDisplay name to show users."]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `document_prompt`.\nDocument-level prompt provided by the user. This custom text is injected into the AI model's prompt to provide extra, document-wide guidance for processing."]
    pub fn set_document_prompt(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.document_prompt = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `entity_types`.\n"]
    pub fn set_entity_types(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DocumentAiSchemaVersion_Schema_EntityTypes>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.entity_types = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.entity_types = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DocumentAiSchemaVersion_Schema_Metadata>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metadata = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metadata = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DocumentAiSchemaVersion_Schema {
    type O = terrars::BlockAssignable<DocumentAiSchemaVersion_Schema>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocumentAiSchemaVersion_Schema {}

impl BuildDocumentAiSchemaVersion_Schema {
    pub fn build(self) -> DocumentAiSchemaVersion_Schema {
        DocumentAiSchemaVersion_Schema {
            description: core::default::Default::default(),
            display_name: core::default::Default::default(),
            document_prompt: core::default::Default::default(),
            entity_types: core::default::Default::default(),
            metadata: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DocumentAiSchemaVersion_SchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DocumentAiSchemaVersion_SchemaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DocumentAiSchemaVersion_SchemaRef {
        DocumentAiSchemaVersion_SchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocumentAiSchemaVersion_SchemaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the schema."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name to show users."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `document_prompt` after provisioning.\nDocument-level prompt provided by the user. This custom text is injected into the AI model's prompt to provide extra, document-wide guidance for processing."]
    pub fn document_prompt(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.document_prompt", self.base))
    }

    #[doc = "Get a reference to the value of field `entity_types` after provisioning.\n"]
    pub fn entity_types(&self) -> terrars::ListRef<DocumentAiSchemaVersion_Schema_EntityTypesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entity_types", self.base))
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> terrars::ListRef<DocumentAiSchemaVersion_Schema_MetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
}

#[derive(Serialize)]
pub struct DocumentAiSchemaVersion_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DocumentAiSchemaVersion_Timeouts {
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

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DocumentAiSchemaVersion_Timeouts {
    type O = terrars::BlockAssignable<DocumentAiSchemaVersion_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDocumentAiSchemaVersion_Timeouts {}

impl BuildDocumentAiSchemaVersion_Timeouts {
    pub fn build(self) -> DocumentAiSchemaVersion_Timeouts {
        DocumentAiSchemaVersion_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DocumentAiSchemaVersion_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DocumentAiSchemaVersion_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DocumentAiSchemaVersion_TimeoutsRef {
        DocumentAiSchemaVersion_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DocumentAiSchemaVersion_TimeoutsRef {
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

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
