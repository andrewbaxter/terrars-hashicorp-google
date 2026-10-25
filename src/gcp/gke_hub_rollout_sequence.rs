#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct GkeHubRolloutSequenceData_ {
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
    min_control_plane_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_node_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    rollout_sequence_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_upgrade_config: std::option::Option<std::vec::Vec<GkeHubRolloutSequence_AutoUpgradeConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignored_clusters_selector: std::option::Option<std::vec::Vec<GkeHubRolloutSequence_IgnoredClustersSelector>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stages: std::option::Option<std::vec::Vec<GkeHubRolloutSequence_Stages>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<GkeHubRolloutSequence_Timeouts>,
    dynamic: GkeHubRolloutSequenceDynamic,
}

struct GkeHubRolloutSequence_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<GkeHubRolloutSequenceData_>,
}

#[derive(Clone)]
pub struct GkeHubRolloutSequence(std::rc::Rc<GkeHubRolloutSequence_>);

impl GkeHubRolloutSequence {
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

    #[doc = "Set the field `display_name`.\nHuman readable display name of the Rollout Sequence."]
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
        "Set the field `labels`.\nLabels for this Rollout Sequence.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_control_plane_version`.\nMinimum control plane version that the clusters in the sequence should be upgraded to.\nSetting this field will cause the creation of a rollout to the specified version.\nAny rollout of the same type already running on the first stage of the sequence will be cancelled to allow for the creation of the new rollout.\nShould be a valid [semantic version](https://semver.org/).\nVersion aliases are supported, as described in the [cluster version docs](https://docs.cloud.google.com/kubernetes-engine/versioning#specifying_cluster_version).\nNote that the 'latest' and '-' aliases are not supported for this field.\nSupported formats: '1.X', '1.X.Y', '1.X.Y-gke.N'."]
    pub fn set_min_control_plane_version(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().min_control_plane_version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_node_version`.\nMinimum node version that the clusters in the sequence should be upgraded to.\nSetting this field will cause the creation of a rollout to the specified version.\nAny rollout of the same type already running on the first stage of the sequence will be cancelled to allow for the creation of the new rollout.\nShould be a valid [semantic version](https://semver.org/).\nVersion aliases are supported, as described in the [cluster version docs](https://docs.cloud.google.com/kubernetes-engine/versioning#specifying_cluster_version).\nNote that the 'latest' and '-' aliases are not supported for this field.\nSupported formats: '1.X', '1.X.Y', '1.X.Y-gke.N'."]
    pub fn set_min_node_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().min_node_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auto_upgrade_config`.\n"]
    pub fn set_auto_upgrade_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeHubRolloutSequence_AutoUpgradeConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auto_upgrade_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auto_upgrade_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ignored_clusters_selector`.\n"]
    pub fn set_ignored_clusters_selector(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeHubRolloutSequence_IgnoredClustersSelector>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ignored_clusters_selector = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ignored_clusters_selector = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `stages`.\n"]
    pub fn set_stages(self, v: impl std::convert::Into<terrars::BlockAssignable<GkeHubRolloutSequence_Stages>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().stages = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.stages = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<GkeHubRolloutSequence_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp at which the Rollout Sequence was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_time` after provisioning.\nThe timestamp at the Rollout Sequence was deleted."]
    pub fn delete_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nHuman readable display name of the Rollout Sequence."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\netag of the Rollout Sequence."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels for this Rollout Sequence.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `min_control_plane_version` after provisioning.\nMinimum control plane version that the clusters in the sequence should be upgraded to.\nSetting this field will cause the creation of a rollout to the specified version.\nAny rollout of the same type already running on the first stage of the sequence will be cancelled to allow for the creation of the new rollout.\nShould be a valid [semantic version](https://semver.org/).\nVersion aliases are supported, as described in the [cluster version docs](https://docs.cloud.google.com/kubernetes-engine/versioning#specifying_cluster_version).\nNote that the 'latest' and '-' aliases are not supported for this field.\nSupported formats: '1.X', '1.X.Y', '1.X.Y-gke.N'."]
    pub fn min_control_plane_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_control_plane_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `min_node_version` after provisioning.\nMinimum node version that the clusters in the sequence should be upgraded to.\nSetting this field will cause the creation of a rollout to the specified version.\nAny rollout of the same type already running on the first stage of the sequence will be cancelled to allow for the creation of the new rollout.\nShould be a valid [semantic version](https://semver.org/).\nVersion aliases are supported, as described in the [cluster version docs](https://docs.cloud.google.com/kubernetes-engine/versioning#specifying_cluster_version).\nNote that the 'latest' and '-' aliases are not supported for this field.\nSupported formats: '1.X', '1.X.Y', '1.X.Y-gke.N'."]
    pub fn min_node_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_node_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe full resource name of the RolloutSequence."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `operational_state` after provisioning.\nThe operational state of the rollout sequence."]
    pub fn operational_state(&self) -> terrars::ListRef<GkeHubRolloutSequence_OperationalStateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.operational_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rollout_sequence_id` after provisioning.\nThe user-provided identifier of the RolloutSequence."]
    pub fn rollout_sequence_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rollout_sequence_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_control_plane_version` after provisioning.\nThe current target control plane version."]
    pub fn target_control_plane_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_control_plane_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_node_version` after provisioning.\nThe current target node version."]
    pub fn target_node_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_node_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uid` after provisioning.\nGoogle-generated UUID for this resource."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp at which the Rollout Sequence was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_upgrade_config` after provisioning.\n"]
    pub fn auto_upgrade_config(&self) -> terrars::ListRef<GkeHubRolloutSequence_AutoUpgradeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_upgrade_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ignored_clusters_selector` after provisioning.\n"]
    pub fn ignored_clusters_selector(&self) -> terrars::ListRef<GkeHubRolloutSequence_IgnoredClustersSelectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ignored_clusters_selector", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stages` after provisioning.\n"]
    pub fn stages(&self) -> terrars::ListRef<GkeHubRolloutSequence_StagesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stages", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeHubRolloutSequence_TimeoutsRef {
        GkeHubRolloutSequence_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for GkeHubRolloutSequence {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for GkeHubRolloutSequence { }

impl terrars::ToListMappable for GkeHubRolloutSequence {
    type O = terrars::ListRef<GkeHubRolloutSequenceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for GkeHubRolloutSequence_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_gke_hub_rollout_sequence".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGkeHubRolloutSequence {
    pub tf_id: std::string::String,
    #[doc = "The user-provided identifier of the RolloutSequence."]
    pub rollout_sequence_id: terrars::PrimField<std::string::String>,
}

impl BuildGkeHubRolloutSequence {
    pub fn build(self, stack: &mut terrars::Stack) -> GkeHubRolloutSequence {
        let out = GkeHubRolloutSequence(std::rc::Rc::new(GkeHubRolloutSequence_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(GkeHubRolloutSequenceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                min_control_plane_version: core::default::Default::default(),
                min_node_version: core::default::Default::default(),
                project: core::default::Default::default(),
                rollout_sequence_id: self.rollout_sequence_id,
                auto_upgrade_config: core::default::Default::default(),
                ignored_clusters_selector: core::default::Default::default(),
                stages: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GkeHubRolloutSequenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubRolloutSequenceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GkeHubRolloutSequenceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp at which the Rollout Sequence was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_time` after provisioning.\nThe timestamp at the Rollout Sequence was deleted."]
    pub fn delete_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nHuman readable display name of the Rollout Sequence."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\netag of the Rollout Sequence."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels for this Rollout Sequence.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `min_control_plane_version` after provisioning.\nMinimum control plane version that the clusters in the sequence should be upgraded to.\nSetting this field will cause the creation of a rollout to the specified version.\nAny rollout of the same type already running on the first stage of the sequence will be cancelled to allow for the creation of the new rollout.\nShould be a valid [semantic version](https://semver.org/).\nVersion aliases are supported, as described in the [cluster version docs](https://docs.cloud.google.com/kubernetes-engine/versioning#specifying_cluster_version).\nNote that the 'latest' and '-' aliases are not supported for this field.\nSupported formats: '1.X', '1.X.Y', '1.X.Y-gke.N'."]
    pub fn min_control_plane_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_control_plane_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `min_node_version` after provisioning.\nMinimum node version that the clusters in the sequence should be upgraded to.\nSetting this field will cause the creation of a rollout to the specified version.\nAny rollout of the same type already running on the first stage of the sequence will be cancelled to allow for the creation of the new rollout.\nShould be a valid [semantic version](https://semver.org/).\nVersion aliases are supported, as described in the [cluster version docs](https://docs.cloud.google.com/kubernetes-engine/versioning#specifying_cluster_version).\nNote that the 'latest' and '-' aliases are not supported for this field.\nSupported formats: '1.X', '1.X.Y', '1.X.Y-gke.N'."]
    pub fn min_node_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_node_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe full resource name of the RolloutSequence."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `operational_state` after provisioning.\nThe operational state of the rollout sequence."]
    pub fn operational_state(&self) -> terrars::ListRef<GkeHubRolloutSequence_OperationalStateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.operational_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rollout_sequence_id` after provisioning.\nThe user-provided identifier of the RolloutSequence."]
    pub fn rollout_sequence_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rollout_sequence_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_control_plane_version` after provisioning.\nThe current target control plane version."]
    pub fn target_control_plane_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_control_plane_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_node_version` after provisioning.\nThe current target node version."]
    pub fn target_node_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_node_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uid` after provisioning.\nGoogle-generated UUID for this resource."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp at which the Rollout Sequence was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_upgrade_config` after provisioning.\n"]
    pub fn auto_upgrade_config(&self) -> terrars::ListRef<GkeHubRolloutSequence_AutoUpgradeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_upgrade_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ignored_clusters_selector` after provisioning.\n"]
    pub fn ignored_clusters_selector(&self) -> terrars::ListRef<GkeHubRolloutSequence_IgnoredClustersSelectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ignored_clusters_selector", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stages` after provisioning.\n"]
    pub fn stages(&self) -> terrars::ListRef<GkeHubRolloutSequence_StagesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stages", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GkeHubRolloutSequence_TimeoutsRef {
        GkeHubRolloutSequence_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct GkeHubRolloutSequenceDynamic {
    auto_upgrade_config: std::option::Option<terrars::DynamicBlock<GkeHubRolloutSequence_AutoUpgradeConfig>>,
    ignored_clusters_selector: std
    ::option
    ::Option<terrars::DynamicBlock<GkeHubRolloutSequence_IgnoredClustersSelector>>,
    stages: std::option::Option<terrars::DynamicBlock<GkeHubRolloutSequence_Stages>>,
}

#[derive(Serialize)]
pub struct GkeHubRolloutSequence_OperationalState {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeHubRolloutSequence_OperationalState {
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeHubRolloutSequence_OperationalState {
    type O = terrars::BlockAssignable<GkeHubRolloutSequence_OperationalState>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubRolloutSequence_OperationalState {}

impl BuildGkeHubRolloutSequence_OperationalState {
    pub fn build(self) -> GkeHubRolloutSequence_OperationalState {
        GkeHubRolloutSequence_OperationalState { state: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct GkeHubRolloutSequence_OperationalStateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubRolloutSequence_OperationalStateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeHubRolloutSequence_OperationalStateRef {
        GkeHubRolloutSequence_OperationalStateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubRolloutSequence_OperationalStateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubRolloutSequence_AutoUpgradeConfig_RolloutCreationScope {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upgrade_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl GkeHubRolloutSequence_AutoUpgradeConfig_RolloutCreationScope {
    #[doc =
        "Set the field `upgrade_types`.\nThe list of enabled upgrade types.\nCurrent valid values are 'CONTROL_PLANE_MINOR', 'CONTROL_PLANE_PATCH', 'NODE_MINOR', and 'NODE_PATCH'."]
    pub fn set_upgrade_types(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.upgrade_types = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GkeHubRolloutSequence_AutoUpgradeConfig_RolloutCreationScope {
    type O = terrars::BlockAssignable<GkeHubRolloutSequence_AutoUpgradeConfig_RolloutCreationScope>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubRolloutSequence_AutoUpgradeConfig_RolloutCreationScope {}

impl BuildGkeHubRolloutSequence_AutoUpgradeConfig_RolloutCreationScope {
    pub fn build(self) -> GkeHubRolloutSequence_AutoUpgradeConfig_RolloutCreationScope {
        GkeHubRolloutSequence_AutoUpgradeConfig_RolloutCreationScope {
            upgrade_types: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeHubRolloutSequence_AutoUpgradeConfig_RolloutCreationScopeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubRolloutSequence_AutoUpgradeConfig_RolloutCreationScopeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubRolloutSequence_AutoUpgradeConfig_RolloutCreationScopeRef {
        GkeHubRolloutSequence_AutoUpgradeConfig_RolloutCreationScopeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubRolloutSequence_AutoUpgradeConfig_RolloutCreationScopeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `upgrade_types` after provisioning.\nThe list of enabled upgrade types.\nCurrent valid values are 'CONTROL_PLANE_MINOR', 'CONTROL_PLANE_PATCH', 'NODE_MINOR', and 'NODE_PATCH'."]
    pub fn upgrade_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.upgrade_types", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeHubRolloutSequence_AutoUpgradeConfigDynamic {
    rollout_creation_scope: std
    ::option
    ::Option<terrars::DynamicBlock<GkeHubRolloutSequence_AutoUpgradeConfig_RolloutCreationScope>>,
}

#[derive(Serialize)]
pub struct GkeHubRolloutSequence_AutoUpgradeConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rollout_creation_scope: std
    ::option
    ::Option<std::vec::Vec<GkeHubRolloutSequence_AutoUpgradeConfig_RolloutCreationScope>>,
    dynamic: GkeHubRolloutSequence_AutoUpgradeConfigDynamic,
}

impl GkeHubRolloutSequence_AutoUpgradeConfig {
    #[doc = "Set the field `rollout_creation_scope`.\n"]
    pub fn set_rollout_creation_scope(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GkeHubRolloutSequence_AutoUpgradeConfig_RolloutCreationScope>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rollout_creation_scope = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rollout_creation_scope = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeHubRolloutSequence_AutoUpgradeConfig {
    type O = terrars::BlockAssignable<GkeHubRolloutSequence_AutoUpgradeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubRolloutSequence_AutoUpgradeConfig {}

impl BuildGkeHubRolloutSequence_AutoUpgradeConfig {
    pub fn build(self) -> GkeHubRolloutSequence_AutoUpgradeConfig {
        GkeHubRolloutSequence_AutoUpgradeConfig {
            rollout_creation_scope: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeHubRolloutSequence_AutoUpgradeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubRolloutSequence_AutoUpgradeConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeHubRolloutSequence_AutoUpgradeConfigRef {
        GkeHubRolloutSequence_AutoUpgradeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubRolloutSequence_AutoUpgradeConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rollout_creation_scope` after provisioning.\n"]
    pub fn rollout_creation_scope(
        &self,
    ) -> terrars::ListRef<GkeHubRolloutSequence_AutoUpgradeConfig_RolloutCreationScopeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rollout_creation_scope", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubRolloutSequence_IgnoredClustersSelector {
    label_selector: terrars::PrimField<std::string::String>,
}

impl GkeHubRolloutSequence_IgnoredClustersSelector { }

impl terrars::ToListMappable for GkeHubRolloutSequence_IgnoredClustersSelector {
    type O = terrars::BlockAssignable<GkeHubRolloutSequence_IgnoredClustersSelector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubRolloutSequence_IgnoredClustersSelector {
    #[doc =
        "The label selector must be a valid CEL (Common Expression Language) expression which\nevaluates resource.labels."]
    pub label_selector: terrars::PrimField<std::string::String>,
}

impl BuildGkeHubRolloutSequence_IgnoredClustersSelector {
    pub fn build(self) -> GkeHubRolloutSequence_IgnoredClustersSelector {
        GkeHubRolloutSequence_IgnoredClustersSelector { label_selector: self.label_selector }
    }
}

#[allow(dead_code)]
pub struct GkeHubRolloutSequence_IgnoredClustersSelectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubRolloutSequence_IgnoredClustersSelectorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubRolloutSequence_IgnoredClustersSelectorRef {
        GkeHubRolloutSequence_IgnoredClustersSelectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubRolloutSequence_IgnoredClustersSelectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `label_selector` after provisioning.\nThe label selector must be a valid CEL (Common Expression Language) expression which\nevaluates resource.labels."]
    pub fn label_selector(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label_selector", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubRolloutSequence_Stages_ClusterSelector {
    label_selector: terrars::PrimField<std::string::String>,
}

impl GkeHubRolloutSequence_Stages_ClusterSelector { }

impl terrars::ToListMappable for GkeHubRolloutSequence_Stages_ClusterSelector {
    type O = terrars::BlockAssignable<GkeHubRolloutSequence_Stages_ClusterSelector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubRolloutSequence_Stages_ClusterSelector {
    #[doc =
        "The label selector must be a valid CEL (Common Expression Language) expression which\nevaluates resource.labels."]
    pub label_selector: terrars::PrimField<std::string::String>,
}

impl BuildGkeHubRolloutSequence_Stages_ClusterSelector {
    pub fn build(self) -> GkeHubRolloutSequence_Stages_ClusterSelector {
        GkeHubRolloutSequence_Stages_ClusterSelector { label_selector: self.label_selector }
    }
}

#[allow(dead_code)]
pub struct GkeHubRolloutSequence_Stages_ClusterSelectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubRolloutSequence_Stages_ClusterSelectorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GkeHubRolloutSequence_Stages_ClusterSelectorRef {
        GkeHubRolloutSequence_Stages_ClusterSelectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubRolloutSequence_Stages_ClusterSelectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `label_selector` after provisioning.\nThe label selector must be a valid CEL (Common Expression Language) expression which\nevaluates resource.labels."]
    pub fn label_selector(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label_selector", self.base))
    }
}

#[derive(Serialize, Default)]
struct GkeHubRolloutSequence_StagesDynamic {
    cluster_selector: std::option::Option<terrars::DynamicBlock<GkeHubRolloutSequence_Stages_ClusterSelector>>,
}

#[derive(Serialize)]
pub struct GkeHubRolloutSequence_Stages {
    fleet_projects: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    soak_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_selector: std::option::Option<std::vec::Vec<GkeHubRolloutSequence_Stages_ClusterSelector>>,
    dynamic: GkeHubRolloutSequence_StagesDynamic,
}

impl GkeHubRolloutSequence_Stages {
    #[doc =
        "Set the field `soak_duration`.\nSoak time after upgrading all the clusters in the stage.\nHas to be specified in seconds, minutes, hours or days."]
    pub fn set_soak_duration(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.soak_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_selector`.\n"]
    pub fn set_cluster_selector(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GkeHubRolloutSequence_Stages_ClusterSelector>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cluster_selector = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cluster_selector = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GkeHubRolloutSequence_Stages {
    type O = terrars::BlockAssignable<GkeHubRolloutSequence_Stages>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubRolloutSequence_Stages {
    #[doc = "List of Fleet projects to select the clusters from.\nExpected format: projects/{project}"]
    pub fleet_projects: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildGkeHubRolloutSequence_Stages {
    pub fn build(self) -> GkeHubRolloutSequence_Stages {
        GkeHubRolloutSequence_Stages {
            fleet_projects: self.fleet_projects,
            soak_duration: core::default::Default::default(),
            cluster_selector: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeHubRolloutSequence_StagesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubRolloutSequence_StagesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeHubRolloutSequence_StagesRef {
        GkeHubRolloutSequence_StagesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubRolloutSequence_StagesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `fleet_projects` after provisioning.\nList of Fleet projects to select the clusters from.\nExpected format: projects/{project}"]
    pub fn fleet_projects(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fleet_projects", self.base))
    }

    #[doc =
        "Get a reference to the value of field `soak_duration` after provisioning.\nSoak time after upgrading all the clusters in the stage.\nHas to be specified in seconds, minutes, hours or days."]
    pub fn soak_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.soak_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `cluster_selector` after provisioning.\n"]
    pub fn cluster_selector(&self) -> terrars::ListRef<GkeHubRolloutSequence_Stages_ClusterSelectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cluster_selector", self.base))
    }
}

#[derive(Serialize)]
pub struct GkeHubRolloutSequence_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GkeHubRolloutSequence_Timeouts {
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

impl terrars::ToListMappable for GkeHubRolloutSequence_Timeouts {
    type O = terrars::BlockAssignable<GkeHubRolloutSequence_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGkeHubRolloutSequence_Timeouts {}

impl BuildGkeHubRolloutSequence_Timeouts {
    pub fn build(self) -> GkeHubRolloutSequence_Timeouts {
        GkeHubRolloutSequence_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GkeHubRolloutSequence_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GkeHubRolloutSequence_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GkeHubRolloutSequence_TimeoutsRef {
        GkeHubRolloutSequence_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GkeHubRolloutSequence_TimeoutsRef {
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
