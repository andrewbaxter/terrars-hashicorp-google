#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct GkeHubFleetData_ {
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
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_cluster_config: std::option::Option<std::vec::Vec<GkeHubFleet_DefaultClusterConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<GkeHubFleet_Timeouts>,
    dynamic: GkeHubFleetDynamic,
}

struct GkeHubFleet_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<GkeHubFleetData_>,
}

#[derive(Clone)]
pub struct GkeHubFleet(std::rc::Rc<GkeHubFleet_>);

impl GkeHubFleet {
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
        "Set the field `display_name`.\nA user-assigned display name of the Fleet. When present, it must be between 4 to 30 characters.\nAllowed characters are: lowercase and uppercase letters, numbers, hyphen, single-quote, double-quote, space, and exclamation point."]
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
        "Set the field `labels`.\nLabels for this Fleet.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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

    #[doc = "Set the field `default_cluster_config`.\n"]
    pub fn set_default_cluster_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeHubFleet_DefaultClusterConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_cluster_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_cluster_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<GkeHubFleet_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time the fleet was created, in RFC3339 text format."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_time` after provisioning.\nThe time the fleet was deleted, in RFC3339 text format."]
    pub fn delete_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nA user-assigned display name of the Fleet. When present, it must be between 4 to 30 characters.\nAllowed characters are: lowercase and uppercase letters, numbers, hyphen, single-quote, double-quote, space, and exclamation point."]
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
        "Get a reference to the value of field `labels` after provisioning.\nLabels for this Fleet.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of the fleet resource."]
    pub fn state(&self) -> terrars::ListRef<GkeHubFleet_StateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nGoogle-generated UUID for this resource. This is unique across all\nFleet resources. If a Fleet resource is deleted and another\nresource with the same name is created, it gets a different uid."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time the fleet was last updated, in RFC3339 text format."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_cluster_config` after provisioning.\n"]
    pub fn default_cluster_config(&self) -> terrars::ListRef<GkeHubFleet_DefaultClusterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_cluster_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeHubFleet_TimeoutsRef {
        GkeHubFleet_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for GkeHubFleet {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for GkeHubFleet { }

impl terrars::ToListMappable for GkeHubFleet {
    type O = terrars::ListRef<GkeHubFleetRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for GkeHubFleet_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_gke_hub_fleet".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGkeHubFleet {
    pub tf_id: std::string::String,
}

impl BuildGkeHubFleet {
    pub fn build(self, stack: &mut terrars::Stack) -> GkeHubFleet {
        let out = GkeHubFleet(std::rc::Rc::new(GkeHubFleet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(GkeHubFleetData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                project: core::default::Default::default(),
                default_cluster_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GkeHubFleetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFleetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GkeHubFleetRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time the fleet was created, in RFC3339 text format."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_time` after provisioning.\nThe time the fleet was deleted, in RFC3339 text format."]
    pub fn delete_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nA user-assigned display name of the Fleet. When present, it must be between 4 to 30 characters.\nAllowed characters are: lowercase and uppercase letters, numbers, hyphen, single-quote, double-quote, space, and exclamation point."]
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
        "Get a reference to the value of field `labels` after provisioning.\nLabels for this Fleet.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of the fleet resource."]
    pub fn state(&self) -> terrars::ListRef<GkeHubFleet_StateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nGoogle-generated UUID for this resource. This is unique across all\nFleet resources. If a Fleet resource is deleted and another\nresource with the same name is created, it gets a different uid."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time the fleet was last updated, in RFC3339 text format."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_cluster_config` after provisioning.\n"]
    pub fn default_cluster_config(&self) -> terrars::ListRef<GkeHubFleet_DefaultClusterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_cluster_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeHubFleet_TimeoutsRef {
        GkeHubFleet_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct GkeHubFleetDynamic {
    default_cluster_config: std::option::Option<terrars::DynamicBlock<GkeHubFleet_DefaultClusterConfig>>,
}

#[derive(Serialize)]
pub struct GkeHubFleet_State {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeHubFleet_State {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.code = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeHubFleet_State {
    type O = terrars::BlockAssignable<GkeHubFleet_State>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFleet_State {}

impl BuildGkeHubFleet_State {
    pub fn build(self) -> GkeHubFleet_State {
        GkeHubFleet_State { code: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct GkeHubFleet_StateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFleet_StateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeHubFleet_StateRef {
        GkeHubFleet_StateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFleet_StateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubFleet_DefaultClusterConfig_BinaryAuthorizationConfig_PolicyBindings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeHubFleet_DefaultClusterConfig_BinaryAuthorizationConfig_PolicyBindings {
    #[doc =
        "Set the field `name`.\nThe relative resource name of the binauthz platform policy to audit. GKE\nplatform policies have the following format:\n'projects/{project_number}/platforms/gke/policies/{policy_id}'."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeHubFleet_DefaultClusterConfig_BinaryAuthorizationConfig_PolicyBindings {
    type O = terrars::BlockAssignable<GkeHubFleet_DefaultClusterConfig_BinaryAuthorizationConfig_PolicyBindings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFleet_DefaultClusterConfig_BinaryAuthorizationConfig_PolicyBindings {}

impl BuildGkeHubFleet_DefaultClusterConfig_BinaryAuthorizationConfig_PolicyBindings {
    pub fn build(self) -> GkeHubFleet_DefaultClusterConfig_BinaryAuthorizationConfig_PolicyBindings {
        GkeHubFleet_DefaultClusterConfig_BinaryAuthorizationConfig_PolicyBindings {
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeHubFleet_DefaultClusterConfig_BinaryAuthorizationConfig_PolicyBindingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFleet_DefaultClusterConfig_BinaryAuthorizationConfig_PolicyBindingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFleet_DefaultClusterConfig_BinaryAuthorizationConfig_PolicyBindingsRef {
        GkeHubFleet_DefaultClusterConfig_BinaryAuthorizationConfig_PolicyBindingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFleet_DefaultClusterConfig_BinaryAuthorizationConfig_PolicyBindingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe relative resource name of the binauthz platform policy to audit. GKE\nplatform policies have the following format:\n'projects/{project_number}/platforms/gke/policies/{policy_id}'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeHubFleet_DefaultClusterConfig_BinaryAuthorizationConfigDynamic {
    policy_bindings: std
    ::option
    ::Option<terrars::DynamicBlock<GkeHubFleet_DefaultClusterConfig_BinaryAuthorizationConfig_PolicyBindings>>,
}

#[derive(Serialize)]
pub struct GkeHubFleet_DefaultClusterConfig_BinaryAuthorizationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluation_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_bindings: std
    ::option
    ::Option<std::vec::Vec<GkeHubFleet_DefaultClusterConfig_BinaryAuthorizationConfig_PolicyBindings>>,
    dynamic: GkeHubFleet_DefaultClusterConfig_BinaryAuthorizationConfigDynamic,
}

impl GkeHubFleet_DefaultClusterConfig_BinaryAuthorizationConfig {
    #[doc =
        "Set the field `evaluation_mode`.\nMode of operation for binauthz policy evaluation. Possible values: [\"DISABLED\", \"POLICY_BINDINGS\"]"]
    pub fn set_evaluation_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.evaluation_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `policy_bindings`.\n"]
    pub fn set_policy_bindings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<GkeHubFleet_DefaultClusterConfig_BinaryAuthorizationConfig_PolicyBindings>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.policy_bindings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.policy_bindings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeHubFleet_DefaultClusterConfig_BinaryAuthorizationConfig {
    type O = terrars::BlockAssignable<GkeHubFleet_DefaultClusterConfig_BinaryAuthorizationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFleet_DefaultClusterConfig_BinaryAuthorizationConfig {}

impl BuildGkeHubFleet_DefaultClusterConfig_BinaryAuthorizationConfig {
    pub fn build(self) -> GkeHubFleet_DefaultClusterConfig_BinaryAuthorizationConfig {
        GkeHubFleet_DefaultClusterConfig_BinaryAuthorizationConfig {
            evaluation_mode: core::default::Default::default(),
            policy_bindings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeHubFleet_DefaultClusterConfig_BinaryAuthorizationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFleet_DefaultClusterConfig_BinaryAuthorizationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFleet_DefaultClusterConfig_BinaryAuthorizationConfigRef {
        GkeHubFleet_DefaultClusterConfig_BinaryAuthorizationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFleet_DefaultClusterConfig_BinaryAuthorizationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `evaluation_mode` after provisioning.\nMode of operation for binauthz policy evaluation. Possible values: [\"DISABLED\", \"POLICY_BINDINGS\"]"]
    pub fn evaluation_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluation_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `policy_bindings` after provisioning.\n"]
    pub fn policy_bindings(
        &self,
    ) -> terrars::ListRef<GkeHubFleet_DefaultClusterConfig_BinaryAuthorizationConfig_PolicyBindingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy_bindings", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubFleet_DefaultClusterConfig_CompliancePostureConfig_ComplianceStandards {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    standard: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeHubFleet_DefaultClusterConfig_CompliancePostureConfig_ComplianceStandards {
    #[doc = "Set the field `standard`.\nName of the compliance standard."]
    pub fn set_standard(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.standard = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeHubFleet_DefaultClusterConfig_CompliancePostureConfig_ComplianceStandards {
    type O = terrars::BlockAssignable<GkeHubFleet_DefaultClusterConfig_CompliancePostureConfig_ComplianceStandards>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFleet_DefaultClusterConfig_CompliancePostureConfig_ComplianceStandards {}

impl BuildGkeHubFleet_DefaultClusterConfig_CompliancePostureConfig_ComplianceStandards {
    pub fn build(self) -> GkeHubFleet_DefaultClusterConfig_CompliancePostureConfig_ComplianceStandards {
        GkeHubFleet_DefaultClusterConfig_CompliancePostureConfig_ComplianceStandards {
            standard: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeHubFleet_DefaultClusterConfig_CompliancePostureConfig_ComplianceStandardsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFleet_DefaultClusterConfig_CompliancePostureConfig_ComplianceStandardsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFleet_DefaultClusterConfig_CompliancePostureConfig_ComplianceStandardsRef {
        GkeHubFleet_DefaultClusterConfig_CompliancePostureConfig_ComplianceStandardsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFleet_DefaultClusterConfig_CompliancePostureConfig_ComplianceStandardsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `standard` after provisioning.\nName of the compliance standard."]
    pub fn standard(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.standard", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeHubFleet_DefaultClusterConfig_CompliancePostureConfigDynamic {
    compliance_standards: std
    ::option
    ::Option<terrars::DynamicBlock<GkeHubFleet_DefaultClusterConfig_CompliancePostureConfig_ComplianceStandards>>,
}

#[derive(Serialize)]
pub struct GkeHubFleet_DefaultClusterConfig_CompliancePostureConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compliance_standards: std
    ::option
    ::Option<std::vec::Vec<GkeHubFleet_DefaultClusterConfig_CompliancePostureConfig_ComplianceStandards>>,
    dynamic: GkeHubFleet_DefaultClusterConfig_CompliancePostureConfigDynamic,
}

impl GkeHubFleet_DefaultClusterConfig_CompliancePostureConfig {
    #[doc =
        "Set the field `mode`.\nSets which mode to use for Compliance Posture features. Possible values: [\"DISABLED\", \"ENABLED\"]"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compliance_standards`.\n"]
    pub fn set_compliance_standards(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GkeHubFleet_DefaultClusterConfig_CompliancePostureConfig_ComplianceStandards,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.compliance_standards = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.compliance_standards = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeHubFleet_DefaultClusterConfig_CompliancePostureConfig {
    type O = terrars::BlockAssignable<GkeHubFleet_DefaultClusterConfig_CompliancePostureConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFleet_DefaultClusterConfig_CompliancePostureConfig {}

impl BuildGkeHubFleet_DefaultClusterConfig_CompliancePostureConfig {
    pub fn build(self) -> GkeHubFleet_DefaultClusterConfig_CompliancePostureConfig {
        GkeHubFleet_DefaultClusterConfig_CompliancePostureConfig {
            mode: core::default::Default::default(),
            compliance_standards: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeHubFleet_DefaultClusterConfig_CompliancePostureConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFleet_DefaultClusterConfig_CompliancePostureConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFleet_DefaultClusterConfig_CompliancePostureConfigRef {
        GkeHubFleet_DefaultClusterConfig_CompliancePostureConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFleet_DefaultClusterConfig_CompliancePostureConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nSets which mode to use for Compliance Posture features. Possible values: [\"DISABLED\", \"ENABLED\"]"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `compliance_standards` after provisioning.\n"]
    pub fn compliance_standards(
        &self,
    ) -> terrars::ListRef<GkeHubFleet_DefaultClusterConfig_CompliancePostureConfig_ComplianceStandardsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.compliance_standards", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubFleet_DefaultClusterConfig_SecurityPostureConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vulnerability_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeHubFleet_DefaultClusterConfig_SecurityPostureConfig {
    #[doc =
        "Set the field `mode`.\nSets which mode to use for Security Posture features. Possible values: [\"DISABLED\", \"BASIC\", \"ENTERPRISE\"]"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `vulnerability_mode`.\nSets which mode to use for vulnerability scanning. Possible values: [\"VULNERABILITY_DISABLED\", \"VULNERABILITY_BASIC\", \"VULNERABILITY_ENTERPRISE\"]"]
    pub fn set_vulnerability_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.vulnerability_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeHubFleet_DefaultClusterConfig_SecurityPostureConfig {
    type O = terrars::BlockAssignable<GkeHubFleet_DefaultClusterConfig_SecurityPostureConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFleet_DefaultClusterConfig_SecurityPostureConfig {}

impl BuildGkeHubFleet_DefaultClusterConfig_SecurityPostureConfig {
    pub fn build(self) -> GkeHubFleet_DefaultClusterConfig_SecurityPostureConfig {
        GkeHubFleet_DefaultClusterConfig_SecurityPostureConfig {
            mode: core::default::Default::default(),
            vulnerability_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeHubFleet_DefaultClusterConfig_SecurityPostureConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFleet_DefaultClusterConfig_SecurityPostureConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubFleet_DefaultClusterConfig_SecurityPostureConfigRef {
        GkeHubFleet_DefaultClusterConfig_SecurityPostureConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFleet_DefaultClusterConfig_SecurityPostureConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nSets which mode to use for Security Posture features. Possible values: [\"DISABLED\", \"BASIC\", \"ENTERPRISE\"]"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vulnerability_mode` after provisioning.\nSets which mode to use for vulnerability scanning. Possible values: [\"VULNERABILITY_DISABLED\", \"VULNERABILITY_BASIC\", \"VULNERABILITY_ENTERPRISE\"]"]
    pub fn vulnerability_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vulnerability_mode", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeHubFleet_DefaultClusterConfigDynamic {
    binary_authorization_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeHubFleet_DefaultClusterConfig_BinaryAuthorizationConfig>>,
    compliance_posture_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeHubFleet_DefaultClusterConfig_CompliancePostureConfig>>,
    security_posture_config: std
    ::option
    ::Option<terrars::DynamicBlock<GkeHubFleet_DefaultClusterConfig_SecurityPostureConfig>>,
}

#[derive(Serialize)]
pub struct GkeHubFleet_DefaultClusterConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    binary_authorization_config: std
    ::option
    ::Option<std::vec::Vec<GkeHubFleet_DefaultClusterConfig_BinaryAuthorizationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compliance_posture_config: std
    ::option
    ::Option<std::vec::Vec<GkeHubFleet_DefaultClusterConfig_CompliancePostureConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_posture_config: std::option::Option<std::vec::Vec<GkeHubFleet_DefaultClusterConfig_SecurityPostureConfig>>,
    dynamic: GkeHubFleet_DefaultClusterConfigDynamic,
}

impl GkeHubFleet_DefaultClusterConfig {
    #[doc = "Set the field `binary_authorization_config`.\n"]
    pub fn set_binary_authorization_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeHubFleet_DefaultClusterConfig_BinaryAuthorizationConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.binary_authorization_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.binary_authorization_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `compliance_posture_config`.\n"]
    pub fn set_compliance_posture_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeHubFleet_DefaultClusterConfig_CompliancePostureConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.compliance_posture_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.compliance_posture_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `security_posture_config`.\n"]
    pub fn set_security_posture_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeHubFleet_DefaultClusterConfig_SecurityPostureConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.security_posture_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.security_posture_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeHubFleet_DefaultClusterConfig {
    type O = terrars::BlockAssignable<GkeHubFleet_DefaultClusterConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFleet_DefaultClusterConfig {}

impl BuildGkeHubFleet_DefaultClusterConfig {
    pub fn build(self) -> GkeHubFleet_DefaultClusterConfig {
        GkeHubFleet_DefaultClusterConfig {
            binary_authorization_config: core::default::Default::default(),
            compliance_posture_config: core::default::Default::default(),
            security_posture_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeHubFleet_DefaultClusterConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFleet_DefaultClusterConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeHubFleet_DefaultClusterConfigRef {
        GkeHubFleet_DefaultClusterConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFleet_DefaultClusterConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `binary_authorization_config` after provisioning.\n"]
    pub fn binary_authorization_config(
        &self,
    ) -> terrars::ListRef<GkeHubFleet_DefaultClusterConfig_BinaryAuthorizationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.binary_authorization_config", self.base))
    }

    #[doc = "Get a reference to the value of field `compliance_posture_config` after provisioning.\n"]
    pub fn compliance_posture_config(
        &self,
    ) -> terrars::ListRef<GkeHubFleet_DefaultClusterConfig_CompliancePostureConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.compliance_posture_config", self.base))
    }

    #[doc = "Get a reference to the value of field `security_posture_config` after provisioning.\n"]
    pub fn security_posture_config(&self) -> terrars::ListRef<GkeHubFleet_DefaultClusterConfig_SecurityPostureConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_posture_config", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubFleet_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeHubFleet_Timeouts {
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

impl terrars::ToListMappable for GkeHubFleet_Timeouts {
    type O = terrars::BlockAssignable<GkeHubFleet_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubFleet_Timeouts {}

impl BuildGkeHubFleet_Timeouts {
    pub fn build(self) -> GkeHubFleet_Timeouts {
        GkeHubFleet_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeHubFleet_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubFleet_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeHubFleet_TimeoutsRef {
        GkeHubFleet_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubFleet_TimeoutsRef {
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
