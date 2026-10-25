#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct VectorSearchCollectionData_ {
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
    collection_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_schema: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_spec: std::option::Option<std::vec::Vec<VectorSearchCollection_EncryptionSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<VectorSearchCollection_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vector_schema: std::option::Option<std::vec::Vec<VectorSearchCollection_VectorSchema>>,
    dynamic: VectorSearchCollectionDynamic,
}

struct VectorSearchCollection_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<VectorSearchCollectionData_>,
}

#[derive(Clone)]
pub struct VectorSearchCollection(std::rc::Rc<VectorSearchCollection_>);

impl VectorSearchCollection {
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
        "Set the field `data_schema`.\nJSON Schema for data.\nField names must contain only alphanumeric characters,\nunderscores, and hyphens."]
    pub fn set_data_schema(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().data_schema = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nUser-specified description of the collection"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nUser-specified display name of the collection"]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nLabels as key value pairs.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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

    #[doc = "Set the field `encryption_spec`.\n"]
    pub fn set_encryption_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VectorSearchCollection_EncryptionSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<VectorSearchCollection_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vector_schema`.\n"]
    pub fn set_vector_schema(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VectorSearchCollection_VectorSchema>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vector_schema = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vector_schema = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `collection_id` after provisioning.\nID of the Collection to create.\nThe id must be 1-63 characters long, and comply with\n[RFC1035](https://www.ietf.org/rfc/rfc1035.txt).\nSpecifically, it must be 1-63 characters long and match the regular\nexpression '[a-z](?:[-a-z0-9]{0,61}[a-z0-9])?'."]
    pub fn collection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collection_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n[Output only] Create time stamp"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_schema` after provisioning.\nJSON Schema for data.\nField names must contain only alphanumeric characters,\nunderscores, and hyphens."]
    pub fn data_schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_schema", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nUser-specified description of the collection"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nUser-specified display name of the collection"]
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
        "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. name of resource"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n[Output only] Update time stamp"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_spec` after provisioning.\n"]
    pub fn encryption_spec(&self) -> terrars::ListRef<VectorSearchCollection_EncryptionSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VectorSearchCollection_TimeoutsRef {
        VectorSearchCollection_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for VectorSearchCollection {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for VectorSearchCollection { }

impl terrars::ToListMappable for VectorSearchCollection {
    type O = terrars::ListRef<VectorSearchCollectionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for VectorSearchCollection_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_vector_search_collection".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVectorSearchCollection {
    pub tf_id: std::string::String,
    #[doc =
        "ID of the Collection to create.\nThe id must be 1-63 characters long, and comply with\n[RFC1035](https://www.ietf.org/rfc/rfc1035.txt).\nSpecifically, it must be 1-63 characters long and match the regular\nexpression '[a-z](?:[-a-z0-9]{0,61}[a-z0-9])?'."]
    pub collection_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildVectorSearchCollection {
    pub fn build(self, stack: &mut terrars::Stack) -> VectorSearchCollection {
        let out = VectorSearchCollection(std::rc::Rc::new(VectorSearchCollection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(VectorSearchCollectionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                collection_id: self.collection_id,
                data_schema: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                encryption_spec: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vector_schema: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VectorSearchCollectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VectorSearchCollectionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VectorSearchCollectionRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `collection_id` after provisioning.\nID of the Collection to create.\nThe id must be 1-63 characters long, and comply with\n[RFC1035](https://www.ietf.org/rfc/rfc1035.txt).\nSpecifically, it must be 1-63 characters long and match the regular\nexpression '[a-z](?:[-a-z0-9]{0,61}[a-z0-9])?'."]
    pub fn collection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collection_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n[Output only] Create time stamp"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_schema` after provisioning.\nJSON Schema for data.\nField names must contain only alphanumeric characters,\nunderscores, and hyphens."]
    pub fn data_schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_schema", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nUser-specified description of the collection"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nUser-specified display name of the collection"]
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
        "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. name of resource"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n[Output only] Update time stamp"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_spec` after provisioning.\n"]
    pub fn encryption_spec(&self) -> terrars::ListRef<VectorSearchCollection_EncryptionSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VectorSearchCollection_TimeoutsRef {
        VectorSearchCollection_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct VectorSearchCollectionDynamic {
    encryption_spec: std::option::Option<terrars::DynamicBlock<VectorSearchCollection_EncryptionSpec>>,
    vector_schema: std::option::Option<terrars::DynamicBlock<VectorSearchCollection_VectorSchema>>,
}

#[derive(Serialize)]
pub struct VectorSearchCollection_EncryptionSpec {
    crypto_key_name: terrars::PrimField<std::string::String>,
}

impl VectorSearchCollection_EncryptionSpec { }

impl terrars::ToListMappable for VectorSearchCollection_EncryptionSpec {
    type O = terrars::BlockAssignable<VectorSearchCollection_EncryptionSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVectorSearchCollection_EncryptionSpec {
    #[doc =
        "Resource name of the Cloud KMS key used to protect the resource.\n\nThe Cloud KMS key must be in the same region as the resource. It must have\nthe format\n'projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}'."]
    pub crypto_key_name: terrars::PrimField<std::string::String>,
}

impl BuildVectorSearchCollection_EncryptionSpec {
    pub fn build(self) -> VectorSearchCollection_EncryptionSpec {
        VectorSearchCollection_EncryptionSpec { crypto_key_name: self.crypto_key_name }
    }
}

#[allow(dead_code)]
pub struct VectorSearchCollection_EncryptionSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VectorSearchCollection_EncryptionSpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VectorSearchCollection_EncryptionSpecRef {
        VectorSearchCollection_EncryptionSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VectorSearchCollection_EncryptionSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `crypto_key_name` after provisioning.\nResource name of the Cloud KMS key used to protect the resource.\n\nThe Cloud KMS key must be in the same region as the resource. It must have\nthe format\n'projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}'."]
    pub fn crypto_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.crypto_key_name", self.base))
    }
}

#[derive(Serialize)]
pub struct VectorSearchCollection_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VectorSearchCollection_Timeouts {
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

impl terrars::ToListMappable for VectorSearchCollection_Timeouts {
    type O = terrars::BlockAssignable<VectorSearchCollection_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVectorSearchCollection_Timeouts {}

impl BuildVectorSearchCollection_Timeouts {
    pub fn build(self) -> VectorSearchCollection_Timeouts {
        VectorSearchCollection_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VectorSearchCollection_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VectorSearchCollection_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VectorSearchCollection_TimeoutsRef {
        VectorSearchCollection_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VectorSearchCollection_TimeoutsRef {
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

#[derive(Serialize)]
pub struct VectorSearchCollection_VectorSchema_DenseVector_VertexEmbeddingConfig {
    model_id: terrars::PrimField<std::string::String>,
    task_type: terrars::PrimField<std::string::String>,
    text_template: terrars::PrimField<std::string::String>,
}

impl VectorSearchCollection_VectorSchema_DenseVector_VertexEmbeddingConfig { }

impl terrars::ToListMappable for VectorSearchCollection_VectorSchema_DenseVector_VertexEmbeddingConfig {
    type O = terrars::BlockAssignable<VectorSearchCollection_VectorSchema_DenseVector_VertexEmbeddingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVectorSearchCollection_VectorSchema_DenseVector_VertexEmbeddingConfig {
    #[doc =
        "Required: ID of the embedding model to use. See\nhttps://cloud.google.com/vertex-ai/generative-ai/docs/learn/models#embeddings-models\nfor the list of supported models."]
    pub model_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Possible values:\nRETRIEVAL_QUERY\nRETRIEVAL_DOCUMENT\nSEMANTIC_SIMILARITY\nCLASSIFICATION\nCLUSTERING\nQUESTION_ANSWERING\nFACT_VERIFICATION\nCODE_RETRIEVAL_QUERY"]
    pub task_type: terrars::PrimField<std::string::String>,
    #[doc =
        "Required: Text template for the input to the model. The template must\ncontain one or more references to fields in the DataObject, e.g.:\n\"Movie Title: {title} ---- Movie Plot: {plot}\"."]
    pub text_template: terrars::PrimField<std::string::String>,
}

impl BuildVectorSearchCollection_VectorSchema_DenseVector_VertexEmbeddingConfig {
    pub fn build(self) -> VectorSearchCollection_VectorSchema_DenseVector_VertexEmbeddingConfig {
        VectorSearchCollection_VectorSchema_DenseVector_VertexEmbeddingConfig {
            model_id: self.model_id,
            task_type: self.task_type,
            text_template: self.text_template,
        }
    }
}

#[allow(dead_code)]
pub struct VectorSearchCollection_VectorSchema_DenseVector_VertexEmbeddingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VectorSearchCollection_VectorSchema_DenseVector_VertexEmbeddingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VectorSearchCollection_VectorSchema_DenseVector_VertexEmbeddingConfigRef {
        VectorSearchCollection_VectorSchema_DenseVector_VertexEmbeddingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VectorSearchCollection_VectorSchema_DenseVector_VertexEmbeddingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `model_id` after provisioning.\nRequired: ID of the embedding model to use. See\nhttps://cloud.google.com/vertex-ai/generative-ai/docs/learn/models#embeddings-models\nfor the list of supported models."]
    pub fn model_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `task_type` after provisioning.\nPossible values:\nRETRIEVAL_QUERY\nRETRIEVAL_DOCUMENT\nSEMANTIC_SIMILARITY\nCLASSIFICATION\nCLUSTERING\nQUESTION_ANSWERING\nFACT_VERIFICATION\nCODE_RETRIEVAL_QUERY"]
    pub fn task_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `text_template` after provisioning.\nRequired: Text template for the input to the model. The template must\ncontain one or more references to fields in the DataObject, e.g.:\n\"Movie Title: {title} ---- Movie Plot: {plot}\"."]
    pub fn text_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text_template", self.base))
    }
}

#[derive(Serialize, Default)]
struct VectorSearchCollection_VectorSchema_DenseVectorDynamic {
    vertex_embedding_config: std
    ::option
    ::Option<terrars::DynamicBlock<VectorSearchCollection_VectorSchema_DenseVector_VertexEmbeddingConfig>>,
}

#[derive(Serialize)]
pub struct VectorSearchCollection_VectorSchema_DenseVector {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimensions: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vertex_embedding_config: std
    ::option
    ::Option<std::vec::Vec<VectorSearchCollection_VectorSchema_DenseVector_VertexEmbeddingConfig>>,
    dynamic: VectorSearchCollection_VectorSchema_DenseVectorDynamic,
}

impl VectorSearchCollection_VectorSchema_DenseVector {
    #[doc = "Set the field `dimensions`.\nDimensionality of the vector field."]
    pub fn set_dimensions(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.dimensions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vertex_embedding_config`.\n"]
    pub fn set_vertex_embedding_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<VectorSearchCollection_VectorSchema_DenseVector_VertexEmbeddingConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vertex_embedding_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vertex_embedding_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VectorSearchCollection_VectorSchema_DenseVector {
    type O = terrars::BlockAssignable<VectorSearchCollection_VectorSchema_DenseVector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVectorSearchCollection_VectorSchema_DenseVector {}

impl BuildVectorSearchCollection_VectorSchema_DenseVector {
    pub fn build(self) -> VectorSearchCollection_VectorSchema_DenseVector {
        VectorSearchCollection_VectorSchema_DenseVector {
            dimensions: core::default::Default::default(),
            vertex_embedding_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VectorSearchCollection_VectorSchema_DenseVectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VectorSearchCollection_VectorSchema_DenseVectorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VectorSearchCollection_VectorSchema_DenseVectorRef {
        VectorSearchCollection_VectorSchema_DenseVectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VectorSearchCollection_VectorSchema_DenseVectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dimensions` after provisioning.\nDimensionality of the vector field."]
    pub fn dimensions(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dimensions", self.base))
    }

    #[doc = "Get a reference to the value of field `vertex_embedding_config` after provisioning.\n"]
    pub fn vertex_embedding_config(
        &self,
    ) -> terrars::ListRef<VectorSearchCollection_VectorSchema_DenseVector_VertexEmbeddingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vertex_embedding_config", self.base))
    }
}

#[derive(Serialize)]
pub struct VectorSearchCollection_VectorSchema_SparseVector {}

impl VectorSearchCollection_VectorSchema_SparseVector { }

impl terrars::ToListMappable for VectorSearchCollection_VectorSchema_SparseVector {
    type O = terrars::BlockAssignable<VectorSearchCollection_VectorSchema_SparseVector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVectorSearchCollection_VectorSchema_SparseVector {}

impl BuildVectorSearchCollection_VectorSchema_SparseVector {
    pub fn build(self) -> VectorSearchCollection_VectorSchema_SparseVector {
        VectorSearchCollection_VectorSchema_SparseVector {}
    }
}

#[allow(dead_code)]
pub struct VectorSearchCollection_VectorSchema_SparseVectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VectorSearchCollection_VectorSchema_SparseVectorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VectorSearchCollection_VectorSchema_SparseVectorRef {
        VectorSearchCollection_VectorSchema_SparseVectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VectorSearchCollection_VectorSchema_SparseVectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct VectorSearchCollection_VectorSchemaDynamic {
    dense_vector: std::option::Option<terrars::DynamicBlock<VectorSearchCollection_VectorSchema_DenseVector>>,
    sparse_vector: std::option::Option<terrars::DynamicBlock<VectorSearchCollection_VectorSchema_SparseVector>>,
}

#[derive(Serialize)]
pub struct VectorSearchCollection_VectorSchema {
    field_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dense_vector: std::option::Option<std::vec::Vec<VectorSearchCollection_VectorSchema_DenseVector>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sparse_vector: std::option::Option<std::vec::Vec<VectorSearchCollection_VectorSchema_SparseVector>>,
    dynamic: VectorSearchCollection_VectorSchemaDynamic,
}

impl VectorSearchCollection_VectorSchema {
    #[doc = "Set the field `dense_vector`.\n"]
    pub fn set_dense_vector(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VectorSearchCollection_VectorSchema_DenseVector>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dense_vector = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dense_vector = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sparse_vector`.\n"]
    pub fn set_sparse_vector(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VectorSearchCollection_VectorSchema_SparseVector>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sparse_vector = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sparse_vector = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VectorSearchCollection_VectorSchema {
    type O = terrars::BlockAssignable<VectorSearchCollection_VectorSchema>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVectorSearchCollection_VectorSchema {
    #[doc = ""]
    pub field_name: terrars::PrimField<std::string::String>,
}

impl BuildVectorSearchCollection_VectorSchema {
    pub fn build(self) -> VectorSearchCollection_VectorSchema {
        VectorSearchCollection_VectorSchema {
            field_name: self.field_name,
            dense_vector: core::default::Default::default(),
            sparse_vector: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VectorSearchCollection_VectorSchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VectorSearchCollection_VectorSchemaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VectorSearchCollection_VectorSchemaRef {
        VectorSearchCollection_VectorSchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VectorSearchCollection_VectorSchemaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `field_name` after provisioning.\n"]
    pub fn field_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field_name", self.base))
    }

    #[doc = "Get a reference to the value of field `dense_vector` after provisioning.\n"]
    pub fn dense_vector(&self) -> terrars::ListRef<VectorSearchCollection_VectorSchema_DenseVectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dense_vector", self.base))
    }

    #[doc = "Get a reference to the value of field `sparse_vector` after provisioning.\n"]
    pub fn sparse_vector(&self) -> terrars::ListRef<VectorSearchCollection_VectorSchema_SparseVectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sparse_vector", self.base))
    }
}
