#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct VertexAiIndexData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    index_update_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_spec: std::option::Option<std::vec::Vec<VertexAiIndex_EncryptionSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<std::vec::Vec<VertexAiIndex_Metadata>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<VertexAiIndex_Timeouts>,
    dynamic: VertexAiIndexDynamic,
}

struct VertexAiIndex_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<VertexAiIndexData_>,
}

#[derive(Clone)]
pub struct VertexAiIndex(std::rc::Rc<VertexAiIndex_>);

impl VertexAiIndex {
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

    #[doc = "Set the field `description`.\nThe description of the Index."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `index_update_method`.\nThe update method to use with this Index. The value must be the followings. If not set, BATCH_UPDATE will be used by default.\n* BATCH_UPDATE: user can call indexes.patch with files on Cloud Storage of datapoints to update.\n* STREAM_UPDATE: user can call indexes.upsertDatapoints/DeleteDatapoints to update the Index and the updates will be applied in corresponding DeployedIndexes in nearly real-time."]
    pub fn set_index_update_method(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().index_update_method = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nThe labels with user-defined metadata to organize your Indexes.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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

    #[doc = "Set the field `region`.\nThe region of the index. eg us-central1"]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_spec`.\n"]
    pub fn set_encryption_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiIndex_EncryptionSpec>>,
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

    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(self, v: impl std::convert::Into<terrars::BlockAssignable<VertexAiIndex_Metadata>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().metadata = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.metadata = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<VertexAiIndex_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp of when the Index was created in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deployed_indexes` after provisioning.\nThe pointers to DeployedIndexes created from this Index. An Index can be only deleted if all its DeployedIndexes had been undeployed first."]
    pub fn deployed_indexes(&self) -> terrars::ListRef<VertexAiIndex_DeployedIndexesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployed_indexes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the Index."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the Index. The name can be up to 128 characters long and can consist of any UTF-8 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nUsed to perform consistent read-modify-write updates."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `index_stats` after provisioning.\nStats of the index resource."]
    pub fn index_stats(&self) -> terrars::ListRef<VertexAiIndex_IndexStatsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.index_stats", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `index_update_method` after provisioning.\nThe update method to use with this Index. The value must be the followings. If not set, BATCH_UPDATE will be used by default.\n* BATCH_UPDATE: user can call indexes.patch with files on Cloud Storage of datapoints to update.\n* STREAM_UPDATE: user can call indexes.upsertDatapoints/DeleteDatapoints to update the Index and the updates will be applied in corresponding DeployedIndexes in nearly real-time."]
    pub fn index_update_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_update_method", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe labels with user-defined metadata to organize your Indexes.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `metadata_schema_uri` after provisioning.\nPoints to a YAML file stored on Google Cloud Storage describing additional information about the Index, that is specific to it. Unset if the Index does not have any additional information."]
    pub fn metadata_schema_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata_schema_uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Index."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region of the index. eg us-central1"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp of when the Index was last updated in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_spec` after provisioning.\n"]
    pub fn encryption_spec(&self) -> terrars::ListRef<VertexAiIndex_EncryptionSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> terrars::ListRef<VertexAiIndex_MetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiIndex_TimeoutsRef {
        VertexAiIndex_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for VertexAiIndex {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for VertexAiIndex { }

impl terrars::ToListMappable for VertexAiIndex {
    type O = terrars::ListRef<VertexAiIndexRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for VertexAiIndex_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_vertex_ai_index".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVertexAiIndex {
    pub tf_id: std::string::String,
    #[doc =
        "The display name of the Index. The name can be up to 128 characters long and can consist of any UTF-8 characters."]
    pub display_name: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiIndex {
    pub fn build(self, stack: &mut terrars::Stack) -> VertexAiIndex {
        let out = VertexAiIndex(std::rc::Rc::new(VertexAiIndex_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(VertexAiIndexData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                index_update_method: core::default::Default::default(),
                labels: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                encryption_spec: core::default::Default::default(),
                metadata: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VertexAiIndexRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiIndexRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VertexAiIndexRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp of when the Index was created in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deployed_indexes` after provisioning.\nThe pointers to DeployedIndexes created from this Index. An Index can be only deleted if all its DeployedIndexes had been undeployed first."]
    pub fn deployed_indexes(&self) -> terrars::ListRef<VertexAiIndex_DeployedIndexesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployed_indexes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the Index."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the Index. The name can be up to 128 characters long and can consist of any UTF-8 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nUsed to perform consistent read-modify-write updates."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `index_stats` after provisioning.\nStats of the index resource."]
    pub fn index_stats(&self) -> terrars::ListRef<VertexAiIndex_IndexStatsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.index_stats", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `index_update_method` after provisioning.\nThe update method to use with this Index. The value must be the followings. If not set, BATCH_UPDATE will be used by default.\n* BATCH_UPDATE: user can call indexes.patch with files on Cloud Storage of datapoints to update.\n* STREAM_UPDATE: user can call indexes.upsertDatapoints/DeleteDatapoints to update the Index and the updates will be applied in corresponding DeployedIndexes in nearly real-time."]
    pub fn index_update_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_update_method", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe labels with user-defined metadata to organize your Indexes.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `metadata_schema_uri` after provisioning.\nPoints to a YAML file stored on Google Cloud Storage describing additional information about the Index, that is specific to it. Unset if the Index does not have any additional information."]
    pub fn metadata_schema_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata_schema_uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Index."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region of the index. eg us-central1"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp of when the Index was last updated in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_spec` after provisioning.\n"]
    pub fn encryption_spec(&self) -> terrars::ListRef<VertexAiIndex_EncryptionSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> terrars::ListRef<VertexAiIndex_MetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiIndex_TimeoutsRef {
        VertexAiIndex_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct VertexAiIndexDynamic {
    encryption_spec: std::option::Option<terrars::DynamicBlock<VertexAiIndex_EncryptionSpec>>,
    metadata: std::option::Option<terrars::DynamicBlock<VertexAiIndex_Metadata>>,
}

#[derive(Serialize)]
pub struct VertexAiIndex_DeployedIndexes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployed_index_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    index_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiIndex_DeployedIndexes {
    #[doc = "Set the field `deployed_index_id`.\n"]
    pub fn set_deployed_index_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.deployed_index_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `index_endpoint`.\n"]
    pub fn set_index_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.index_endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiIndex_DeployedIndexes {
    type O = terrars::BlockAssignable<VertexAiIndex_DeployedIndexes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiIndex_DeployedIndexes {}

impl BuildVertexAiIndex_DeployedIndexes {
    pub fn build(self) -> VertexAiIndex_DeployedIndexes {
        VertexAiIndex_DeployedIndexes {
            deployed_index_id: core::default::Default::default(),
            index_endpoint: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiIndex_DeployedIndexesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiIndex_DeployedIndexesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiIndex_DeployedIndexesRef {
        VertexAiIndex_DeployedIndexesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiIndex_DeployedIndexesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `deployed_index_id` after provisioning.\n"]
    pub fn deployed_index_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployed_index_id", self.base))
    }

    #[doc = "Get a reference to the value of field `index_endpoint` after provisioning.\n"]
    pub fn index_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiIndex_IndexStats {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shards_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vectors_count: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiIndex_IndexStats {
    #[doc = "Set the field `shards_count`.\n"]
    pub fn set_shards_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.shards_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vectors_count`.\n"]
    pub fn set_vectors_count(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vectors_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiIndex_IndexStats {
    type O = terrars::BlockAssignable<VertexAiIndex_IndexStats>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiIndex_IndexStats {}

impl BuildVertexAiIndex_IndexStats {
    pub fn build(self) -> VertexAiIndex_IndexStats {
        VertexAiIndex_IndexStats {
            shards_count: core::default::Default::default(),
            vectors_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiIndex_IndexStatsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiIndex_IndexStatsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiIndex_IndexStatsRef {
        VertexAiIndex_IndexStatsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiIndex_IndexStatsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `shards_count` after provisioning.\n"]
    pub fn shards_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shards_count", self.base))
    }

    #[doc = "Get a reference to the value of field `vectors_count` after provisioning.\n"]
    pub fn vectors_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vectors_count", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiIndex_EncryptionSpec {
    kms_key_name: terrars::PrimField<std::string::String>,
}

impl VertexAiIndex_EncryptionSpec { }

impl terrars::ToListMappable for VertexAiIndex_EncryptionSpec {
    type O = terrars::BlockAssignable<VertexAiIndex_EncryptionSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiIndex_EncryptionSpec {
    #[doc =
        "Required. The Cloud KMS resource identifier of the customer managed encryption key used to protect a resource. Has the form: 'projects/my-project/locations/my-region/keyRings/my-kr/cryptoKeys/my-key'. The key needs to be in the same region as where the compute resource is created."]
    pub kms_key_name: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiIndex_EncryptionSpec {
    pub fn build(self) -> VertexAiIndex_EncryptionSpec {
        VertexAiIndex_EncryptionSpec { kms_key_name: self.kms_key_name }
    }
}

#[allow(dead_code)]
pub struct VertexAiIndex_EncryptionSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiIndex_EncryptionSpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiIndex_EncryptionSpecRef {
        VertexAiIndex_EncryptionSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiIndex_EncryptionSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nRequired. The Cloud KMS resource identifier of the customer managed encryption key used to protect a resource. Has the form: 'projects/my-project/locations/my-region/keyRings/my-kr/cryptoKeys/my-key'. The key needs to be in the same region as where the compute resource is created."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiIndex_Metadata_Config_AlgorithmConfig_BruteForceConfig {}

impl VertexAiIndex_Metadata_Config_AlgorithmConfig_BruteForceConfig { }

impl terrars::ToListMappable for VertexAiIndex_Metadata_Config_AlgorithmConfig_BruteForceConfig {
    type O = terrars::BlockAssignable<VertexAiIndex_Metadata_Config_AlgorithmConfig_BruteForceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiIndex_Metadata_Config_AlgorithmConfig_BruteForceConfig {}

impl BuildVertexAiIndex_Metadata_Config_AlgorithmConfig_BruteForceConfig {
    pub fn build(self) -> VertexAiIndex_Metadata_Config_AlgorithmConfig_BruteForceConfig {
        VertexAiIndex_Metadata_Config_AlgorithmConfig_BruteForceConfig {}
    }
}

#[allow(dead_code)]
pub struct VertexAiIndex_Metadata_Config_AlgorithmConfig_BruteForceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiIndex_Metadata_Config_AlgorithmConfig_BruteForceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiIndex_Metadata_Config_AlgorithmConfig_BruteForceConfigRef {
        VertexAiIndex_Metadata_Config_AlgorithmConfig_BruteForceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiIndex_Metadata_Config_AlgorithmConfig_BruteForceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct VertexAiIndex_Metadata_Config_AlgorithmConfig_TreeAhConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    leaf_node_embedding_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    leaf_nodes_to_search_percent: std::option::Option<terrars::PrimField<f64>>,
}

impl VertexAiIndex_Metadata_Config_AlgorithmConfig_TreeAhConfig {
    #[doc =
        "Set the field `leaf_node_embedding_count`.\nNumber of embeddings on each leaf node. The default value is 1000 if not set."]
    pub fn set_leaf_node_embedding_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.leaf_node_embedding_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `leaf_nodes_to_search_percent`.\nThe default percentage of leaf nodes that any query may be searched. Must be in\nrange 1-100, inclusive. The default value is 10 (means 10%) if not set."]
    pub fn set_leaf_nodes_to_search_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.leaf_nodes_to_search_percent = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiIndex_Metadata_Config_AlgorithmConfig_TreeAhConfig {
    type O = terrars::BlockAssignable<VertexAiIndex_Metadata_Config_AlgorithmConfig_TreeAhConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiIndex_Metadata_Config_AlgorithmConfig_TreeAhConfig {}

impl BuildVertexAiIndex_Metadata_Config_AlgorithmConfig_TreeAhConfig {
    pub fn build(self) -> VertexAiIndex_Metadata_Config_AlgorithmConfig_TreeAhConfig {
        VertexAiIndex_Metadata_Config_AlgorithmConfig_TreeAhConfig {
            leaf_node_embedding_count: core::default::Default::default(),
            leaf_nodes_to_search_percent: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiIndex_Metadata_Config_AlgorithmConfig_TreeAhConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiIndex_Metadata_Config_AlgorithmConfig_TreeAhConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiIndex_Metadata_Config_AlgorithmConfig_TreeAhConfigRef {
        VertexAiIndex_Metadata_Config_AlgorithmConfig_TreeAhConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiIndex_Metadata_Config_AlgorithmConfig_TreeAhConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `leaf_node_embedding_count` after provisioning.\nNumber of embeddings on each leaf node. The default value is 1000 if not set."]
    pub fn leaf_node_embedding_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.leaf_node_embedding_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `leaf_nodes_to_search_percent` after provisioning.\nThe default percentage of leaf nodes that any query may be searched. Must be in\nrange 1-100, inclusive. The default value is 10 (means 10%) if not set."]
    pub fn leaf_nodes_to_search_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.leaf_nodes_to_search_percent", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiIndex_Metadata_Config_AlgorithmConfigDynamic {
    brute_force_config: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiIndex_Metadata_Config_AlgorithmConfig_BruteForceConfig>>,
    tree_ah_config: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiIndex_Metadata_Config_AlgorithmConfig_TreeAhConfig>>,
}

#[derive(Serialize)]
pub struct VertexAiIndex_Metadata_Config_AlgorithmConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    brute_force_config: std
    ::option
    ::Option<std::vec::Vec<VertexAiIndex_Metadata_Config_AlgorithmConfig_BruteForceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tree_ah_config: std::option::Option<std::vec::Vec<VertexAiIndex_Metadata_Config_AlgorithmConfig_TreeAhConfig>>,
    dynamic: VertexAiIndex_Metadata_Config_AlgorithmConfigDynamic,
}

impl VertexAiIndex_Metadata_Config_AlgorithmConfig {
    #[doc = "Set the field `brute_force_config`.\n"]
    pub fn set_brute_force_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<VertexAiIndex_Metadata_Config_AlgorithmConfig_BruteForceConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.brute_force_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.brute_force_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tree_ah_config`.\n"]
    pub fn set_tree_ah_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<VertexAiIndex_Metadata_Config_AlgorithmConfig_TreeAhConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tree_ah_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tree_ah_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiIndex_Metadata_Config_AlgorithmConfig {
    type O = terrars::BlockAssignable<VertexAiIndex_Metadata_Config_AlgorithmConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiIndex_Metadata_Config_AlgorithmConfig {}

impl BuildVertexAiIndex_Metadata_Config_AlgorithmConfig {
    pub fn build(self) -> VertexAiIndex_Metadata_Config_AlgorithmConfig {
        VertexAiIndex_Metadata_Config_AlgorithmConfig {
            brute_force_config: core::default::Default::default(),
            tree_ah_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiIndex_Metadata_Config_AlgorithmConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiIndex_Metadata_Config_AlgorithmConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiIndex_Metadata_Config_AlgorithmConfigRef {
        VertexAiIndex_Metadata_Config_AlgorithmConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiIndex_Metadata_Config_AlgorithmConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `brute_force_config` after provisioning.\n"]
    pub fn brute_force_config(
        &self,
    ) -> terrars::ListRef<VertexAiIndex_Metadata_Config_AlgorithmConfig_BruteForceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.brute_force_config", self.base))
    }

    #[doc = "Get a reference to the value of field `tree_ah_config` after provisioning.\n"]
    pub fn tree_ah_config(&self) -> terrars::ListRef<VertexAiIndex_Metadata_Config_AlgorithmConfig_TreeAhConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tree_ah_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiIndex_Metadata_ConfigDynamic {
    algorithm_config: std::option::Option<terrars::DynamicBlock<VertexAiIndex_Metadata_Config_AlgorithmConfig>>,
}

#[derive(Serialize)]
pub struct VertexAiIndex_Metadata_Config {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    approximate_neighbors_count: std::option::Option<terrars::PrimField<f64>>,
    dimensions: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    distance_measure_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    feature_norm_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shard_size: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    algorithm_config: std::option::Option<std::vec::Vec<VertexAiIndex_Metadata_Config_AlgorithmConfig>>,
    dynamic: VertexAiIndex_Metadata_ConfigDynamic,
}

impl VertexAiIndex_Metadata_Config {
    #[doc =
        "Set the field `approximate_neighbors_count`.\nThe default number of neighbors to find via approximate search before exact reordering is\nperformed. Exact reordering is a procedure where results returned by an\napproximate search algorithm are reordered via a more expensive distance computation.\nRequired if tree-AH algorithm is used."]
    pub fn set_approximate_neighbors_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.approximate_neighbors_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `distance_measure_type`.\nThe distance measure used in nearest neighbor search. The value must be one of the followings:\n* SQUARED_L2_DISTANCE: Euclidean (L_2) Distance\n* L1_DISTANCE: Manhattan (L_1) Distance\n* COSINE_DISTANCE: Cosine Distance. Defined as 1 - cosine similarity.\n* DOT_PRODUCT_DISTANCE: Dot Product Distance. Defined as a negative of the dot product"]
    pub fn set_distance_measure_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.distance_measure_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `feature_norm_type`.\nType of normalization to be carried out on each vector. The value must be one of the followings:\n* UNIT_L2_NORM: Unit L2 normalization type\n* NONE: No normalization type is specified."]
    pub fn set_feature_norm_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.feature_norm_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `shard_size`.\nIndex data is split into equal parts to be processed. These are called \"shards\".\nThe shard size must be specified when creating an index. The value must be one of the followings:\n* SHARD_SIZE_SMALL: Small (2GB)\n* SHARD_SIZE_MEDIUM: Medium (20GB)\n* SHARD_SIZE_LARGE: Large (50GB)"]
    pub fn set_shard_size(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.shard_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `algorithm_config`.\n"]
    pub fn set_algorithm_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiIndex_Metadata_Config_AlgorithmConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.algorithm_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.algorithm_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiIndex_Metadata_Config {
    type O = terrars::BlockAssignable<VertexAiIndex_Metadata_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiIndex_Metadata_Config {
    #[doc = "The number of dimensions of the input vectors."]
    pub dimensions: terrars::PrimField<f64>,
}

impl BuildVertexAiIndex_Metadata_Config {
    pub fn build(self) -> VertexAiIndex_Metadata_Config {
        VertexAiIndex_Metadata_Config {
            approximate_neighbors_count: core::default::Default::default(),
            dimensions: self.dimensions,
            distance_measure_type: core::default::Default::default(),
            feature_norm_type: core::default::Default::default(),
            shard_size: core::default::Default::default(),
            algorithm_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiIndex_Metadata_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiIndex_Metadata_ConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiIndex_Metadata_ConfigRef {
        VertexAiIndex_Metadata_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiIndex_Metadata_ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `approximate_neighbors_count` after provisioning.\nThe default number of neighbors to find via approximate search before exact reordering is\nperformed. Exact reordering is a procedure where results returned by an\napproximate search algorithm are reordered via a more expensive distance computation.\nRequired if tree-AH algorithm is used."]
    pub fn approximate_neighbors_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.approximate_neighbors_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dimensions` after provisioning.\nThe number of dimensions of the input vectors."]
    pub fn dimensions(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dimensions", self.base))
    }

    #[doc =
        "Get a reference to the value of field `distance_measure_type` after provisioning.\nThe distance measure used in nearest neighbor search. The value must be one of the followings:\n* SQUARED_L2_DISTANCE: Euclidean (L_2) Distance\n* L1_DISTANCE: Manhattan (L_1) Distance\n* COSINE_DISTANCE: Cosine Distance. Defined as 1 - cosine similarity.\n* DOT_PRODUCT_DISTANCE: Dot Product Distance. Defined as a negative of the dot product"]
    pub fn distance_measure_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.distance_measure_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `feature_norm_type` after provisioning.\nType of normalization to be carried out on each vector. The value must be one of the followings:\n* UNIT_L2_NORM: Unit L2 normalization type\n* NONE: No normalization type is specified."]
    pub fn feature_norm_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.feature_norm_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `shard_size` after provisioning.\nIndex data is split into equal parts to be processed. These are called \"shards\".\nThe shard size must be specified when creating an index. The value must be one of the followings:\n* SHARD_SIZE_SMALL: Small (2GB)\n* SHARD_SIZE_MEDIUM: Medium (20GB)\n* SHARD_SIZE_LARGE: Large (50GB)"]
    pub fn shard_size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shard_size", self.base))
    }

    #[doc = "Get a reference to the value of field `algorithm_config` after provisioning.\n"]
    pub fn algorithm_config(&self) -> terrars::ListRef<VertexAiIndex_Metadata_Config_AlgorithmConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.algorithm_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiIndex_MetadataDynamic {
    config: std::option::Option<terrars::DynamicBlock<VertexAiIndex_Metadata_Config>>,
}

#[derive(Serialize)]
pub struct VertexAiIndex_Metadata {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contents_delta_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_complete_overwrite: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config: std::option::Option<std::vec::Vec<VertexAiIndex_Metadata_Config>>,
    dynamic: VertexAiIndex_MetadataDynamic,
}

impl VertexAiIndex_Metadata {
    #[doc =
        "Set the field `contents_delta_uri`.\nAllows inserting, updating  or deleting the contents of the Matching Engine Index.\nThe string must be a valid Cloud Storage directory path. If this\nfield is set when calling IndexService.UpdateIndex, then no other\nIndex field can be also updated as part of the same call.\nThe expected structure and format of the files this URI points to is\ndescribed at https://cloud.google.com/vertex-ai/docs/matching-engine/using-matching-engine#input-data-format"]
    pub fn set_contents_delta_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.contents_delta_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `is_complete_overwrite`.\nIf this field is set together with contentsDeltaUri when calling IndexService.UpdateIndex,\nthen existing content of the Index will be replaced by the data from the contentsDeltaUri."]
    pub fn set_is_complete_overwrite(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_complete_overwrite = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `config`.\n"]
    pub fn set_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiIndex_Metadata_Config>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiIndex_Metadata {
    type O = terrars::BlockAssignable<VertexAiIndex_Metadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiIndex_Metadata {}

impl BuildVertexAiIndex_Metadata {
    pub fn build(self) -> VertexAiIndex_Metadata {
        VertexAiIndex_Metadata {
            contents_delta_uri: core::default::Default::default(),
            is_complete_overwrite: core::default::Default::default(),
            config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiIndex_MetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiIndex_MetadataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiIndex_MetadataRef {
        VertexAiIndex_MetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiIndex_MetadataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `contents_delta_uri` after provisioning.\nAllows inserting, updating  or deleting the contents of the Matching Engine Index.\nThe string must be a valid Cloud Storage directory path. If this\nfield is set when calling IndexService.UpdateIndex, then no other\nIndex field can be also updated as part of the same call.\nThe expected structure and format of the files this URI points to is\ndescribed at https://cloud.google.com/vertex-ai/docs/matching-engine/using-matching-engine#input-data-format"]
    pub fn contents_delta_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contents_delta_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_complete_overwrite` after provisioning.\nIf this field is set together with contentsDeltaUri when calling IndexService.UpdateIndex,\nthen existing content of the Index will be replaced by the data from the contentsDeltaUri."]
    pub fn is_complete_overwrite(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_complete_overwrite", self.base))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<VertexAiIndex_Metadata_ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiIndex_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiIndex_Timeouts {
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

impl terrars::ToListMappable for VertexAiIndex_Timeouts {
    type O = terrars::BlockAssignable<VertexAiIndex_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiIndex_Timeouts {}

impl BuildVertexAiIndex_Timeouts {
    pub fn build(self) -> VertexAiIndex_Timeouts {
        VertexAiIndex_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiIndex_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiIndex_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiIndex_TimeoutsRef {
        VertexAiIndex_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiIndex_TimeoutsRef {
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
