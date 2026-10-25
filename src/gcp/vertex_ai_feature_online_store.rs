#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct VertexAiFeatureOnlineStoreData_ {
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
    force_destroy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bigtable: std::option::Option<std::vec::Vec<VertexAiFeatureOnlineStore_Bigtable>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dedicated_serving_endpoint: std::option::Option<std::vec::Vec<VertexAiFeatureOnlineStore_DedicatedServingEndpoint>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_spec: std::option::Option<std::vec::Vec<VertexAiFeatureOnlineStore_EncryptionSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    optimized: std::option::Option<std::vec::Vec<VertexAiFeatureOnlineStore_Optimized>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<VertexAiFeatureOnlineStore_Timeouts>,
    dynamic: VertexAiFeatureOnlineStoreDynamic,
}

struct VertexAiFeatureOnlineStore_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<VertexAiFeatureOnlineStoreData_>,
}

#[derive(Clone)]
pub struct VertexAiFeatureOnlineStore(std::rc::Rc<VertexAiFeatureOnlineStore_>);

impl VertexAiFeatureOnlineStore {
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

    #[doc =
        "Set the field `force_destroy`.\nIf set to true, any FeatureViews and Features for this FeatureOnlineStore will also be deleted."]
    pub fn set_force_destroy(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_destroy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nThe labels with user-defined metadata to organize your feature online stores.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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

    #[doc = "Set the field `region`.\nThe region of feature online store. eg us-central1"]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bigtable`.\n"]
    pub fn set_bigtable(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiFeatureOnlineStore_Bigtable>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().bigtable = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.bigtable = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dedicated_serving_endpoint`.\n"]
    pub fn set_dedicated_serving_endpoint(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiFeatureOnlineStore_DedicatedServingEndpoint>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dedicated_serving_endpoint = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dedicated_serving_endpoint = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `encryption_spec`.\n"]
    pub fn set_encryption_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiFeatureOnlineStore_EncryptionSpec>>,
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

    #[doc = "Set the field `optimized`.\n"]
    pub fn set_optimized(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiFeatureOnlineStore_Optimized>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().optimized = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.optimized = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<VertexAiFeatureOnlineStore_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp of when the feature online store was created in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `force_destroy` after provisioning.\nIf set to true, any FeatureViews and Features for this FeatureOnlineStore will also be deleted."]
    pub fn force_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe labels with user-defined metadata to organize your feature online stores.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Feature Online Store. This value may be up to 60 characters, and valid characters are [a-z0-9_]. The first character cannot be a number."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region of feature online store. eg us-central1"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe state of the Feature Online Store. See the possible states in [this link](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.featureOnlineStores#state)."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp of when the feature online store was last updated in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bigtable` after provisioning.\n"]
    pub fn bigtable(&self) -> terrars::ListRef<VertexAiFeatureOnlineStore_BigtableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigtable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dedicated_serving_endpoint` after provisioning.\n"]
    pub fn dedicated_serving_endpoint(&self) -> terrars::ListRef<VertexAiFeatureOnlineStore_DedicatedServingEndpointRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dedicated_serving_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_spec` after provisioning.\n"]
    pub fn encryption_spec(&self) -> terrars::ListRef<VertexAiFeatureOnlineStore_EncryptionSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `optimized` after provisioning.\n"]
    pub fn optimized(&self) -> terrars::ListRef<VertexAiFeatureOnlineStore_OptimizedRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.optimized", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiFeatureOnlineStore_TimeoutsRef {
        VertexAiFeatureOnlineStore_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for VertexAiFeatureOnlineStore {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for VertexAiFeatureOnlineStore { }

impl terrars::ToListMappable for VertexAiFeatureOnlineStore {
    type O = terrars::ListRef<VertexAiFeatureOnlineStoreRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for VertexAiFeatureOnlineStore_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_vertex_ai_feature_online_store".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVertexAiFeatureOnlineStore {
    pub tf_id: std::string::String,
    #[doc =
        "The resource name of the Feature Online Store. This value may be up to 60 characters, and valid characters are [a-z0-9_]. The first character cannot be a number."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiFeatureOnlineStore {
    pub fn build(self, stack: &mut terrars::Stack) -> VertexAiFeatureOnlineStore {
        let out = VertexAiFeatureOnlineStore(std::rc::Rc::new(VertexAiFeatureOnlineStore_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(VertexAiFeatureOnlineStoreData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                force_destroy: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                bigtable: core::default::Default::default(),
                dedicated_serving_endpoint: core::default::Default::default(),
                encryption_spec: core::default::Default::default(),
                optimized: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VertexAiFeatureOnlineStoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiFeatureOnlineStoreRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VertexAiFeatureOnlineStoreRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp of when the feature online store was created in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `force_destroy` after provisioning.\nIf set to true, any FeatureViews and Features for this FeatureOnlineStore will also be deleted."]
    pub fn force_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe labels with user-defined metadata to organize your feature online stores.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Feature Online Store. This value may be up to 60 characters, and valid characters are [a-z0-9_]. The first character cannot be a number."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region of feature online store. eg us-central1"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe state of the Feature Online Store. See the possible states in [this link](https://cloud.google.com/vertex-ai/docs/reference/rest/v1/projects.locations.featureOnlineStores#state)."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp of when the feature online store was last updated in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bigtable` after provisioning.\n"]
    pub fn bigtable(&self) -> terrars::ListRef<VertexAiFeatureOnlineStore_BigtableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigtable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dedicated_serving_endpoint` after provisioning.\n"]
    pub fn dedicated_serving_endpoint(&self) -> terrars::ListRef<VertexAiFeatureOnlineStore_DedicatedServingEndpointRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dedicated_serving_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_spec` after provisioning.\n"]
    pub fn encryption_spec(&self) -> terrars::ListRef<VertexAiFeatureOnlineStore_EncryptionSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `optimized` after provisioning.\n"]
    pub fn optimized(&self) -> terrars::ListRef<VertexAiFeatureOnlineStore_OptimizedRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.optimized", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiFeatureOnlineStore_TimeoutsRef {
        VertexAiFeatureOnlineStore_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct VertexAiFeatureOnlineStoreDynamic {
    bigtable: std::option::Option<terrars::DynamicBlock<VertexAiFeatureOnlineStore_Bigtable>>,
    dedicated_serving_endpoint: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiFeatureOnlineStore_DedicatedServingEndpoint>>,
    encryption_spec: std::option::Option<terrars::DynamicBlock<VertexAiFeatureOnlineStore_EncryptionSpec>>,
    optimized: std::option::Option<terrars::DynamicBlock<VertexAiFeatureOnlineStore_Optimized>>,
}

#[derive(Serialize)]
pub struct VertexAiFeatureOnlineStore_Bigtable_AutoScaling {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_utilization_target: std::option::Option<terrars::PrimField<f64>>,
    max_node_count: terrars::PrimField<f64>,
    min_node_count: terrars::PrimField<f64>,
}

impl VertexAiFeatureOnlineStore_Bigtable_AutoScaling {
    #[doc =
        "Set the field `cpu_utilization_target`.\nA percentage of the cluster's CPU capacity. Can be from 10% to 80%. When a cluster's CPU utilization exceeds the target that you have set, Bigtable immediately adds nodes to the cluster. When CPU utilization is substantially lower than the target, Bigtable removes nodes. If not set will default to 50%."]
    pub fn set_cpu_utilization_target(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu_utilization_target = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiFeatureOnlineStore_Bigtable_AutoScaling {
    type O = terrars::BlockAssignable<VertexAiFeatureOnlineStore_Bigtable_AutoScaling>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiFeatureOnlineStore_Bigtable_AutoScaling {
    #[doc =
        "The maximum number of nodes to scale up to. Must be greater than or equal to minNodeCount, and less than or equal to 10 times of 'minNodeCount'."]
    pub max_node_count: terrars::PrimField<f64>,
    #[doc = "The minimum number of nodes to scale down to. Must be greater than or equal to 1."]
    pub min_node_count: terrars::PrimField<f64>,
}

impl BuildVertexAiFeatureOnlineStore_Bigtable_AutoScaling {
    pub fn build(self) -> VertexAiFeatureOnlineStore_Bigtable_AutoScaling {
        VertexAiFeatureOnlineStore_Bigtable_AutoScaling {
            cpu_utilization_target: core::default::Default::default(),
            max_node_count: self.max_node_count,
            min_node_count: self.min_node_count,
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiFeatureOnlineStore_Bigtable_AutoScalingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiFeatureOnlineStore_Bigtable_AutoScalingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiFeatureOnlineStore_Bigtable_AutoScalingRef {
        VertexAiFeatureOnlineStore_Bigtable_AutoScalingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiFeatureOnlineStore_Bigtable_AutoScalingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cpu_utilization_target` after provisioning.\nA percentage of the cluster's CPU capacity. Can be from 10% to 80%. When a cluster's CPU utilization exceeds the target that you have set, Bigtable immediately adds nodes to the cluster. When CPU utilization is substantially lower than the target, Bigtable removes nodes. If not set will default to 50%."]
    pub fn cpu_utilization_target(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_utilization_target", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_node_count` after provisioning.\nThe maximum number of nodes to scale up to. Must be greater than or equal to minNodeCount, and less than or equal to 10 times of 'minNodeCount'."]
    pub fn max_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_node_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_node_count` after provisioning.\nThe minimum number of nodes to scale down to. Must be greater than or equal to 1."]
    pub fn min_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_node_count", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiFeatureOnlineStore_BigtableDynamic {
    auto_scaling: std::option::Option<terrars::DynamicBlock<VertexAiFeatureOnlineStore_Bigtable_AutoScaling>>,
}

#[derive(Serialize)]
pub struct VertexAiFeatureOnlineStore_Bigtable {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_direct_bigtable_access: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_scaling: std::option::Option<std::vec::Vec<VertexAiFeatureOnlineStore_Bigtable_AutoScaling>>,
    dynamic: VertexAiFeatureOnlineStore_BigtableDynamic,
}

impl VertexAiFeatureOnlineStore_Bigtable {
    #[doc =
        "Set the field `enable_direct_bigtable_access`.\nOptional. If true, enable direct access to the Bigtable instance."]
    pub fn set_enable_direct_bigtable_access(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_direct_bigtable_access = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone`.\nThe zone where the Bigtable instance will be created."]
    pub fn set_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auto_scaling`.\n"]
    pub fn set_auto_scaling(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiFeatureOnlineStore_Bigtable_AutoScaling>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.auto_scaling = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.auto_scaling = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiFeatureOnlineStore_Bigtable {
    type O = terrars::BlockAssignable<VertexAiFeatureOnlineStore_Bigtable>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiFeatureOnlineStore_Bigtable {}

impl BuildVertexAiFeatureOnlineStore_Bigtable {
    pub fn build(self) -> VertexAiFeatureOnlineStore_Bigtable {
        VertexAiFeatureOnlineStore_Bigtable {
            enable_direct_bigtable_access: core::default::Default::default(),
            zone: core::default::Default::default(),
            auto_scaling: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiFeatureOnlineStore_BigtableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiFeatureOnlineStore_BigtableRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiFeatureOnlineStore_BigtableRef {
        VertexAiFeatureOnlineStore_BigtableRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiFeatureOnlineStore_BigtableRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_direct_bigtable_access` after provisioning.\nOptional. If true, enable direct access to the Bigtable instance."]
    pub fn enable_direct_bigtable_access(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_direct_bigtable_access", self.base))
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nThe zone where the Bigtable instance will be created."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }

    #[doc = "Get a reference to the value of field `auto_scaling` after provisioning.\n"]
    pub fn auto_scaling(&self) -> terrars::ListRef<VertexAiFeatureOnlineStore_Bigtable_AutoScalingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_scaling", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiFeatureOnlineStore_DedicatedServingEndpoint_PrivateServiceConnectConfig {
    enable_private_service_connect: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_allowlist: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl VertexAiFeatureOnlineStore_DedicatedServingEndpoint_PrivateServiceConnectConfig {
    #[doc =
        "Set the field `project_allowlist`.\nA list of Projects from which the forwarding rule will target the service attachment."]
    pub fn set_project_allowlist(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.project_allowlist = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiFeatureOnlineStore_DedicatedServingEndpoint_PrivateServiceConnectConfig {
    type O =
        terrars::BlockAssignable<VertexAiFeatureOnlineStore_DedicatedServingEndpoint_PrivateServiceConnectConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiFeatureOnlineStore_DedicatedServingEndpoint_PrivateServiceConnectConfig {
    #[doc =
        "If set to true, customers will use private service connection to send request. Otherwise, the connection will set to public endpoint."]
    pub enable_private_service_connect: terrars::PrimField<bool>,
}

impl BuildVertexAiFeatureOnlineStore_DedicatedServingEndpoint_PrivateServiceConnectConfig {
    pub fn build(self) -> VertexAiFeatureOnlineStore_DedicatedServingEndpoint_PrivateServiceConnectConfig {
        VertexAiFeatureOnlineStore_DedicatedServingEndpoint_PrivateServiceConnectConfig {
            enable_private_service_connect: self.enable_private_service_connect,
            project_allowlist: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiFeatureOnlineStore_DedicatedServingEndpoint_PrivateServiceConnectConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiFeatureOnlineStore_DedicatedServingEndpoint_PrivateServiceConnectConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiFeatureOnlineStore_DedicatedServingEndpoint_PrivateServiceConnectConfigRef {
        VertexAiFeatureOnlineStore_DedicatedServingEndpoint_PrivateServiceConnectConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiFeatureOnlineStore_DedicatedServingEndpoint_PrivateServiceConnectConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_private_service_connect` after provisioning.\nIf set to true, customers will use private service connection to send request. Otherwise, the connection will set to public endpoint."]
    pub fn enable_private_service_connect(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_private_service_connect", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_allowlist` after provisioning.\nA list of Projects from which the forwarding rule will target the service attachment."]
    pub fn project_allowlist(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.project_allowlist", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiFeatureOnlineStore_DedicatedServingEndpointDynamic {
    private_service_connect_config: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiFeatureOnlineStore_DedicatedServingEndpoint_PrivateServiceConnectConfig>>,
}

#[derive(Serialize)]
pub struct VertexAiFeatureOnlineStore_DedicatedServingEndpoint {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_service_connect_config: std
    ::option
    ::Option<std::vec::Vec<VertexAiFeatureOnlineStore_DedicatedServingEndpoint_PrivateServiceConnectConfig>>,
    dynamic: VertexAiFeatureOnlineStore_DedicatedServingEndpointDynamic,
}

impl VertexAiFeatureOnlineStore_DedicatedServingEndpoint {
    #[doc = "Set the field `private_service_connect_config`.\n"]
    pub fn set_private_service_connect_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiFeatureOnlineStore_DedicatedServingEndpoint_PrivateServiceConnectConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.private_service_connect_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.private_service_connect_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiFeatureOnlineStore_DedicatedServingEndpoint {
    type O = terrars::BlockAssignable<VertexAiFeatureOnlineStore_DedicatedServingEndpoint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiFeatureOnlineStore_DedicatedServingEndpoint {}

impl BuildVertexAiFeatureOnlineStore_DedicatedServingEndpoint {
    pub fn build(self) -> VertexAiFeatureOnlineStore_DedicatedServingEndpoint {
        VertexAiFeatureOnlineStore_DedicatedServingEndpoint {
            private_service_connect_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiFeatureOnlineStore_DedicatedServingEndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiFeatureOnlineStore_DedicatedServingEndpointRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiFeatureOnlineStore_DedicatedServingEndpointRef {
        VertexAiFeatureOnlineStore_DedicatedServingEndpointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiFeatureOnlineStore_DedicatedServingEndpointRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `public_endpoint_domain_name` after provisioning.\nDomain name to use for this FeatureOnlineStore"]
    pub fn public_endpoint_domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_endpoint_domain_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_attachment` after provisioning.\nName of the service attachment resource. Applicable only if private service connect is enabled and after FeatureViewSync is created."]
    pub fn service_attachment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_attachment", self.base))
    }

    #[doc = "Get a reference to the value of field `private_service_connect_config` after provisioning.\n"]
    pub fn private_service_connect_config(
        &self,
    ) -> terrars::ListRef<VertexAiFeatureOnlineStore_DedicatedServingEndpoint_PrivateServiceConnectConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_service_connect_config", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiFeatureOnlineStore_EncryptionSpec {
    kms_key_name: terrars::PrimField<std::string::String>,
}

impl VertexAiFeatureOnlineStore_EncryptionSpec { }

impl terrars::ToListMappable for VertexAiFeatureOnlineStore_EncryptionSpec {
    type O = terrars::BlockAssignable<VertexAiFeatureOnlineStore_EncryptionSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiFeatureOnlineStore_EncryptionSpec {
    #[doc =
        "The Cloud KMS resource identifier of the customer managed encryption key used to protect a resource. Has the form: projects/my-project/locations/my-region/keyRings/my-kr/cryptoKeys/my-key. The key needs to be in the same region as where the compute resource is created."]
    pub kms_key_name: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiFeatureOnlineStore_EncryptionSpec {
    pub fn build(self) -> VertexAiFeatureOnlineStore_EncryptionSpec {
        VertexAiFeatureOnlineStore_EncryptionSpec { kms_key_name: self.kms_key_name }
    }
}

#[allow(dead_code)]
pub struct VertexAiFeatureOnlineStore_EncryptionSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiFeatureOnlineStore_EncryptionSpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiFeatureOnlineStore_EncryptionSpecRef {
        VertexAiFeatureOnlineStore_EncryptionSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiFeatureOnlineStore_EncryptionSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nThe Cloud KMS resource identifier of the customer managed encryption key used to protect a resource. Has the form: projects/my-project/locations/my-region/keyRings/my-kr/cryptoKeys/my-key. The key needs to be in the same region as where the compute resource is created."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiFeatureOnlineStore_Optimized {}

impl VertexAiFeatureOnlineStore_Optimized { }

impl terrars::ToListMappable for VertexAiFeatureOnlineStore_Optimized {
    type O = terrars::BlockAssignable<VertexAiFeatureOnlineStore_Optimized>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiFeatureOnlineStore_Optimized {}

impl BuildVertexAiFeatureOnlineStore_Optimized {
    pub fn build(self) -> VertexAiFeatureOnlineStore_Optimized {
        VertexAiFeatureOnlineStore_Optimized {}
    }
}

#[allow(dead_code)]
pub struct VertexAiFeatureOnlineStore_OptimizedRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiFeatureOnlineStore_OptimizedRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiFeatureOnlineStore_OptimizedRef {
        VertexAiFeatureOnlineStore_OptimizedRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiFeatureOnlineStore_OptimizedRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct VertexAiFeatureOnlineStore_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiFeatureOnlineStore_Timeouts {
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

impl terrars::ToListMappable for VertexAiFeatureOnlineStore_Timeouts {
    type O = terrars::BlockAssignable<VertexAiFeatureOnlineStore_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiFeatureOnlineStore_Timeouts {}

impl BuildVertexAiFeatureOnlineStore_Timeouts {
    pub fn build(self) -> VertexAiFeatureOnlineStore_Timeouts {
        VertexAiFeatureOnlineStore_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiFeatureOnlineStore_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiFeatureOnlineStore_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiFeatureOnlineStore_TimeoutsRef {
        VertexAiFeatureOnlineStore_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiFeatureOnlineStore_TimeoutsRef {
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
