#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct VertexAiFeatureOnlineStoreFeatureviewData_ {
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
    feature_online_store: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    big_query_source: std::option::Option<std::vec::Vec<VertexAiFeatureOnlineStoreFeatureview_BigQuerySource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    feature_registry_source: std
    ::option
    ::Option<std::vec::Vec<VertexAiFeatureOnlineStoreFeatureview_FeatureRegistrySource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sync_config: std::option::Option<std::vec::Vec<VertexAiFeatureOnlineStoreFeatureview_SyncConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<VertexAiFeatureOnlineStoreFeatureview_Timeouts>,
    dynamic: VertexAiFeatureOnlineStoreFeatureviewDynamic,
}

struct VertexAiFeatureOnlineStoreFeatureview_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<VertexAiFeatureOnlineStoreFeatureviewData_>,
}

#[derive(Clone)]
pub struct VertexAiFeatureOnlineStoreFeatureview(std::rc::Rc<VertexAiFeatureOnlineStoreFeatureview_>);

impl VertexAiFeatureOnlineStoreFeatureview {
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
        "Set the field `labels`.\nA set of key/value label pairs to assign to this FeatureView.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `name`.\nName of the FeatureView. This value may be up to 60 characters, and valid characters are [a-z0-9_]. The first character cannot be a number."]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nThe region for the resource. It should be the same as the featureonlinestore region."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `big_query_source`.\n"]
    pub fn set_big_query_source(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiFeatureOnlineStoreFeatureview_BigQuerySource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().big_query_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.big_query_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `feature_registry_source`.\n"]
    pub fn set_feature_registry_source(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<VertexAiFeatureOnlineStoreFeatureview_FeatureRegistrySource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().feature_registry_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.feature_registry_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sync_config`.\n"]
    pub fn set_sync_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiFeatureOnlineStoreFeatureview_SyncConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sync_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sync_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<VertexAiFeatureOnlineStoreFeatureview_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp of when the featureOnlinestore was created in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
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
        "Get a reference to the value of field `feature_online_store` after provisioning.\nThe name of the FeatureOnlineStore to use for the featureview."]
    pub fn feature_online_store(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.feature_online_store", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to this FeatureView.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the FeatureView. This value may be up to 60 characters, and valid characters are [a-z0-9_]. The first character cannot be a number."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region for the resource. It should be the same as the featureonlinestore region."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp of when the featureOnlinestore was last updated in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `big_query_source` after provisioning.\n"]
    pub fn big_query_source(&self) -> terrars::ListRef<VertexAiFeatureOnlineStoreFeatureview_BigQuerySourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.big_query_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `feature_registry_source` after provisioning.\n"]
    pub fn feature_registry_source(
        &self,
    ) -> terrars::ListRef<VertexAiFeatureOnlineStoreFeatureview_FeatureRegistrySourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.feature_registry_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sync_config` after provisioning.\n"]
    pub fn sync_config(&self) -> terrars::ListRef<VertexAiFeatureOnlineStoreFeatureview_SyncConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sync_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiFeatureOnlineStoreFeatureview_TimeoutsRef {
        VertexAiFeatureOnlineStoreFeatureview_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for VertexAiFeatureOnlineStoreFeatureview {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for VertexAiFeatureOnlineStoreFeatureview { }

impl terrars::ToListMappable for VertexAiFeatureOnlineStoreFeatureview {
    type O = terrars::ListRef<VertexAiFeatureOnlineStoreFeatureviewRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for VertexAiFeatureOnlineStoreFeatureview_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_vertex_ai_feature_online_store_featureview".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVertexAiFeatureOnlineStoreFeatureview {
    pub tf_id: std::string::String,
    #[doc = "The name of the FeatureOnlineStore to use for the featureview."]
    pub feature_online_store: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiFeatureOnlineStoreFeatureview {
    pub fn build(self, stack: &mut terrars::Stack) -> VertexAiFeatureOnlineStoreFeatureview {
        let out = VertexAiFeatureOnlineStoreFeatureview(std::rc::Rc::new(VertexAiFeatureOnlineStoreFeatureview_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(VertexAiFeatureOnlineStoreFeatureviewData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                feature_online_store: self.feature_online_store,
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                big_query_source: core::default::Default::default(),
                feature_registry_source: core::default::Default::default(),
                sync_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VertexAiFeatureOnlineStoreFeatureviewRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiFeatureOnlineStoreFeatureviewRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VertexAiFeatureOnlineStoreFeatureviewRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp of when the featureOnlinestore was created in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
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
        "Get a reference to the value of field `feature_online_store` after provisioning.\nThe name of the FeatureOnlineStore to use for the featureview."]
    pub fn feature_online_store(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.feature_online_store", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to this FeatureView.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the FeatureView. This value may be up to 60 characters, and valid characters are [a-z0-9_]. The first character cannot be a number."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region for the resource. It should be the same as the featureonlinestore region."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp of when the featureOnlinestore was last updated in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `big_query_source` after provisioning.\n"]
    pub fn big_query_source(&self) -> terrars::ListRef<VertexAiFeatureOnlineStoreFeatureview_BigQuerySourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.big_query_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `feature_registry_source` after provisioning.\n"]
    pub fn feature_registry_source(
        &self,
    ) -> terrars::ListRef<VertexAiFeatureOnlineStoreFeatureview_FeatureRegistrySourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.feature_registry_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sync_config` after provisioning.\n"]
    pub fn sync_config(&self) -> terrars::ListRef<VertexAiFeatureOnlineStoreFeatureview_SyncConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sync_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiFeatureOnlineStoreFeatureview_TimeoutsRef {
        VertexAiFeatureOnlineStoreFeatureview_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct VertexAiFeatureOnlineStoreFeatureviewDynamic {
    big_query_source: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiFeatureOnlineStoreFeatureview_BigQuerySource>>,
    feature_registry_source: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiFeatureOnlineStoreFeatureview_FeatureRegistrySource>>,
    sync_config: std::option::Option<terrars::DynamicBlock<VertexAiFeatureOnlineStoreFeatureview_SyncConfig>>,
}

#[derive(Serialize)]
pub struct VertexAiFeatureOnlineStoreFeatureview_BigQuerySource {
    entity_id_columns: terrars::ListField<terrars::PrimField<std::string::String>>,
    uri: terrars::PrimField<std::string::String>,
}

impl VertexAiFeatureOnlineStoreFeatureview_BigQuerySource { }

impl terrars::ToListMappable for VertexAiFeatureOnlineStoreFeatureview_BigQuerySource {
    type O = terrars::BlockAssignable<VertexAiFeatureOnlineStoreFeatureview_BigQuerySource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiFeatureOnlineStoreFeatureview_BigQuerySource {
    #[doc = "Columns to construct entityId / row keys. Start by supporting 1 only."]
    pub entity_id_columns: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = "The BigQuery view URI that will be materialized on each sync trigger based on FeatureView.SyncConfig."]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiFeatureOnlineStoreFeatureview_BigQuerySource {
    pub fn build(self) -> VertexAiFeatureOnlineStoreFeatureview_BigQuerySource {
        VertexAiFeatureOnlineStoreFeatureview_BigQuerySource {
            entity_id_columns: self.entity_id_columns,
            uri: self.uri,
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiFeatureOnlineStoreFeatureview_BigQuerySourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiFeatureOnlineStoreFeatureview_BigQuerySourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiFeatureOnlineStoreFeatureview_BigQuerySourceRef {
        VertexAiFeatureOnlineStoreFeatureview_BigQuerySourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiFeatureOnlineStoreFeatureview_BigQuerySourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `entity_id_columns` after provisioning.\nColumns to construct entityId / row keys. Start by supporting 1 only."]
    pub fn entity_id_columns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entity_id_columns", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uri` after provisioning.\nThe BigQuery view URI that will be materialized on each sync trigger based on FeatureView.SyncConfig."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiFeatureOnlineStoreFeatureview_FeatureRegistrySource_FeatureGroups {
    feature_group_id: terrars::PrimField<std::string::String>,
    feature_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl VertexAiFeatureOnlineStoreFeatureview_FeatureRegistrySource_FeatureGroups { }

impl terrars::ToListMappable for VertexAiFeatureOnlineStoreFeatureview_FeatureRegistrySource_FeatureGroups {
    type O = terrars::BlockAssignable<VertexAiFeatureOnlineStoreFeatureview_FeatureRegistrySource_FeatureGroups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiFeatureOnlineStoreFeatureview_FeatureRegistrySource_FeatureGroups {
    #[doc = "Identifier of the feature group."]
    pub feature_group_id: terrars::PrimField<std::string::String>,
    #[doc = "Identifiers of features under the feature group."]
    pub feature_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildVertexAiFeatureOnlineStoreFeatureview_FeatureRegistrySource_FeatureGroups {
    pub fn build(self) -> VertexAiFeatureOnlineStoreFeatureview_FeatureRegistrySource_FeatureGroups {
        VertexAiFeatureOnlineStoreFeatureview_FeatureRegistrySource_FeatureGroups {
            feature_group_id: self.feature_group_id,
            feature_ids: self.feature_ids,
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiFeatureOnlineStoreFeatureview_FeatureRegistrySource_FeatureGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiFeatureOnlineStoreFeatureview_FeatureRegistrySource_FeatureGroupsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiFeatureOnlineStoreFeatureview_FeatureRegistrySource_FeatureGroupsRef {
        VertexAiFeatureOnlineStoreFeatureview_FeatureRegistrySource_FeatureGroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiFeatureOnlineStoreFeatureview_FeatureRegistrySource_FeatureGroupsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `feature_group_id` after provisioning.\nIdentifier of the feature group."]
    pub fn feature_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.feature_group_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `feature_ids` after provisioning.\nIdentifiers of features under the feature group."]
    pub fn feature_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.feature_ids", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiFeatureOnlineStoreFeatureview_FeatureRegistrySourceDynamic {
    feature_groups: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiFeatureOnlineStoreFeatureview_FeatureRegistrySource_FeatureGroups>>,
}

#[derive(Serialize)]
pub struct VertexAiFeatureOnlineStoreFeatureview_FeatureRegistrySource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_number: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    feature_groups: std
    ::option
    ::Option<std::vec::Vec<VertexAiFeatureOnlineStoreFeatureview_FeatureRegistrySource_FeatureGroups>>,
    dynamic: VertexAiFeatureOnlineStoreFeatureview_FeatureRegistrySourceDynamic,
}

impl VertexAiFeatureOnlineStoreFeatureview_FeatureRegistrySource {
    #[doc = "Set the field `project_number`.\nThe project number of the parent project of the feature Groups."]
    pub fn set_project_number(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_number = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `feature_groups`.\n"]
    pub fn set_feature_groups(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<VertexAiFeatureOnlineStoreFeatureview_FeatureRegistrySource_FeatureGroups>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.feature_groups = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.feature_groups = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiFeatureOnlineStoreFeatureview_FeatureRegistrySource {
    type O = terrars::BlockAssignable<VertexAiFeatureOnlineStoreFeatureview_FeatureRegistrySource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiFeatureOnlineStoreFeatureview_FeatureRegistrySource {}

impl BuildVertexAiFeatureOnlineStoreFeatureview_FeatureRegistrySource {
    pub fn build(self) -> VertexAiFeatureOnlineStoreFeatureview_FeatureRegistrySource {
        VertexAiFeatureOnlineStoreFeatureview_FeatureRegistrySource {
            project_number: core::default::Default::default(),
            feature_groups: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiFeatureOnlineStoreFeatureview_FeatureRegistrySourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiFeatureOnlineStoreFeatureview_FeatureRegistrySourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiFeatureOnlineStoreFeatureview_FeatureRegistrySourceRef {
        VertexAiFeatureOnlineStoreFeatureview_FeatureRegistrySourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiFeatureOnlineStoreFeatureview_FeatureRegistrySourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `project_number` after provisioning.\nThe project number of the parent project of the feature Groups."]
    pub fn project_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_number", self.base))
    }

    #[doc = "Get a reference to the value of field `feature_groups` after provisioning.\n"]
    pub fn feature_groups(
        &self,
    ) -> terrars::ListRef<VertexAiFeatureOnlineStoreFeatureview_FeatureRegistrySource_FeatureGroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.feature_groups", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiFeatureOnlineStoreFeatureview_SyncConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    continuous: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cron: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiFeatureOnlineStoreFeatureview_SyncConfig {
    #[doc = "Set the field `continuous`.\nIf true, syncs the FeatureView in a continuous manner to Online Store."]
    pub fn set_continuous(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.continuous = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cron`.\nCron schedule (https://en.wikipedia.org/wiki/Cron) to launch scheduled runs.\nTo explicitly set a timezone to the cron tab, apply a prefix in the cron tab: \"CRON_TZ=${IANA_TIME_ZONE}\" or \"TZ=${IANA_TIME_ZONE}\"."]
    pub fn set_cron(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cron = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiFeatureOnlineStoreFeatureview_SyncConfig {
    type O = terrars::BlockAssignable<VertexAiFeatureOnlineStoreFeatureview_SyncConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiFeatureOnlineStoreFeatureview_SyncConfig {}

impl BuildVertexAiFeatureOnlineStoreFeatureview_SyncConfig {
    pub fn build(self) -> VertexAiFeatureOnlineStoreFeatureview_SyncConfig {
        VertexAiFeatureOnlineStoreFeatureview_SyncConfig {
            continuous: core::default::Default::default(),
            cron: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiFeatureOnlineStoreFeatureview_SyncConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiFeatureOnlineStoreFeatureview_SyncConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiFeatureOnlineStoreFeatureview_SyncConfigRef {
        VertexAiFeatureOnlineStoreFeatureview_SyncConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiFeatureOnlineStoreFeatureview_SyncConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `continuous` after provisioning.\nIf true, syncs the FeatureView in a continuous manner to Online Store."]
    pub fn continuous(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.continuous", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cron` after provisioning.\nCron schedule (https://en.wikipedia.org/wiki/Cron) to launch scheduled runs.\nTo explicitly set a timezone to the cron tab, apply a prefix in the cron tab: \"CRON_TZ=${IANA_TIME_ZONE}\" or \"TZ=${IANA_TIME_ZONE}\"."]
    pub fn cron(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cron", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiFeatureOnlineStoreFeatureview_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiFeatureOnlineStoreFeatureview_Timeouts {
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

impl terrars::ToListMappable for VertexAiFeatureOnlineStoreFeatureview_Timeouts {
    type O = terrars::BlockAssignable<VertexAiFeatureOnlineStoreFeatureview_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiFeatureOnlineStoreFeatureview_Timeouts {}

impl BuildVertexAiFeatureOnlineStoreFeatureview_Timeouts {
    pub fn build(self) -> VertexAiFeatureOnlineStoreFeatureview_Timeouts {
        VertexAiFeatureOnlineStoreFeatureview_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiFeatureOnlineStoreFeatureview_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiFeatureOnlineStoreFeatureview_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiFeatureOnlineStoreFeatureview_TimeoutsRef {
        VertexAiFeatureOnlineStoreFeatureview_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiFeatureOnlineStoreFeatureview_TimeoutsRef {
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
