#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct LustreInstanceData_ {
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
    capacity_gib: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    filesystem: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gke_support_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    instance_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    network: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_unit_storage_throughput: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_rules_options: std::option::Option<std::vec::Vec<LustreInstance_AccessRulesOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dynamic_tier_options: std::option::Option<std::vec::Vec<LustreInstance_DynamicTierOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_policy: std::option::Option<std::vec::Vec<LustreInstance_MaintenancePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<LustreInstance_Timeouts>,
    dynamic: LustreInstanceDynamic,
}

struct LustreInstance_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LustreInstanceData_>,
}

#[derive(Clone)]
pub struct LustreInstance(std::rc::Rc<LustreInstance_>);

impl LustreInstance {
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

    #[doc = "Set the field `description`.\nA user-readable description of the instance."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `gke_support_enabled`.\nIndicates whether you want to enable support for GKE clients. By default,\nGKE clients are not supported."]
    pub fn set_gke_support_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().gke_support_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kms_key`.\nThe Cloud KMS key name to use for data encryption.\nIf not set, the instance will use Google-managed encryption keys.\nIf set, the instance will use customer-managed encryption keys.\nThe key must be in the same region as the instance.\nThe key format is:\nprojects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{key}"]
    pub fn set_kms_key(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key = std::option::Option::Some(v.into());
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

    #[doc =
        "Set the field `per_unit_storage_throughput`.\nThe throughput of the instance in MBps per TiB. Valid values are 125, 250,\n500, 1000.\nSee [Performance tiers and maximum storage\ncapacities](https://cloud.google.com/managed-lustre/docs/create-instance#performance-tiers)\nfor more information.\n\nIf the instance is using the Dynamic tier, this field must not be set or\nmust be set to zero."]
    pub fn set_per_unit_storage_throughput(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().per_unit_storage_throughput = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `placement_policy`.\nThe placement policy name for the instance in the format of\nprojects/{project}/locations/{location}/resourcePolicies/{resource_policy}"]
    pub fn set_placement_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().placement_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `target_version`.\nThe version to upgrade this instance to. Set this to the value reported in\n'availableVersion', or to 'latest' to move to the newest version available\nat the time of the upgrade.\nThis field cannot be set when the instance is created; new instances are\nalways provisioned from the current release. It also cannot be changed in\nthe same operation as 'capacityGib' or 'maintenancePolicy', and the\ninstance must be ACTIVE and outside of the hour preceding a scheduled\nmaintenance window.\nThe API clears this field once the upgrade finishes, so it always reads\nback as empty on an idle instance."]
    pub fn set_target_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().target_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `access_rules_options`.\n"]
    pub fn set_access_rules_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LustreInstance_AccessRulesOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().access_rules_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.access_rules_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dynamic_tier_options`.\n"]
    pub fn set_dynamic_tier_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LustreInstance_DynamicTierOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dynamic_tier_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dynamic_tier_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `maintenance_policy`.\n"]
    pub fn set_maintenance_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LustreInstance_MaintenancePolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().maintenance_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.maintenance_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<LustreInstance_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `available_version` after provisioning.\nThe version this instance can be upgraded to, if one is available. Empty\nwhen the instance is already running the newest release."]
    pub fn available_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `capacity_gib` after provisioning.\nThe storage capacity of the instance in gibibytes (GiB). Allowed values\nare from '9000' to '7632000', depending on the 'perUnitStorageThroughput'.\nSee [Performance tiers and maximum storage\ncapacities](https://cloud.google.com/managed-lustre/docs/create-instance#performance-tiers)\nfor specific minimums, maximums, and step sizes for each performance tier."]
    pub fn capacity_gib(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_gib", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the instance was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA user-readable description of the instance."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_version` after provisioning.\nThe version of Managed Lustre software that this instance is currently\nrunning."]
    pub fn effective_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effective_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `filesystem` after provisioning.\nThe filesystem name for this instance. This name is used by client-side\ntools, including when mounting the instance. Must be eight characters or\nless and can only contain letters and numbers."]
    pub fn filesystem(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filesystem", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gke_support_enabled` after provisioning.\nIndicates whether you want to enable support for GKE clients. By default,\nGKE clients are not supported."]
    pub fn gke_support_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gke_support_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_id` after provisioning.\nThe name of the Managed Lustre instance.\n\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter."]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key` after provisioning.\nThe Cloud KMS key name to use for data encryption.\nIf not set, the instance will use Google-managed encryption keys.\nIf set, the instance will use customer-managed encryption keys.\nThe key must be in the same region as the instance.\nThe key format is:\nprojects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{key}"]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `mount_point` after provisioning.\nMount point of the instance in the format 'IP_ADDRESS@tcp:/FILESYSTEM'."]
    pub fn mount_point(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mount_point", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the instance."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe full name of the VPC network to which the instance is connected.\nMust be in the format\n'projects/{project_id}/global/networks/{network_name}'."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `per_unit_storage_throughput` after provisioning.\nThe throughput of the instance in MBps per TiB. Valid values are 125, 250,\n500, 1000.\nSee [Performance tiers and maximum storage\ncapacities](https://cloud.google.com/managed-lustre/docs/create-instance#performance-tiers)\nfor more information.\n\nIf the instance is using the Dynamic tier, this field must not be set or\nmust be set to zero."]
    pub fn per_unit_storage_throughput(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.per_unit_storage_throughput", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `placement_policy` after provisioning.\nThe placement policy name for the instance in the format of\nprojects/{project}/locations/{location}/resourcePolicies/{resource_policy}"]
    pub fn placement_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.placement_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe state of the instance.\nPossible values:\nACTIVE\nCREATING\nDELETING\nUPGRADING\nREPAIRING\nSTOPPED\nUPDATING\nSUSPENDED"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state_reason` after provisioning.\nThe reason why the instance is in a certain state (e.g. SUSPENDED)."]
    pub fn state_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_reason", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_version` after provisioning.\nThe version to upgrade this instance to. Set this to the value reported in\n'availableVersion', or to 'latest' to move to the newest version available\nat the time of the upgrade.\nThis field cannot be set when the instance is created; new instances are\nalways provisioned from the current release. It also cannot be changed in\nthe same operation as 'capacityGib' or 'maintenancePolicy', and the\ninstance must be ACTIVE and outside of the hour preceding a scheduled\nmaintenance window.\nThe API clears this field once the upgrade finishes, so it always reads\nback as empty on an idle instance."]
    pub fn target_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nUnique ID of the resource.\nThis is unrelated to the access rules which allow specifying the root\nsquash uid."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `upcoming_maintenance_schedule` after provisioning.\nRepresents a scheduled maintenance event."]
    pub fn upcoming_maintenance_schedule(&self) -> terrars::ListRef<LustreInstance_UpcomingMaintenanceScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upcoming_maintenance_schedule", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when the instance was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `access_rules_options` after provisioning.\n"]
    pub fn access_rules_options(&self) -> terrars::ListRef<LustreInstance_AccessRulesOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_rules_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dynamic_tier_options` after provisioning.\n"]
    pub fn dynamic_tier_options(&self) -> terrars::ListRef<LustreInstance_DynamicTierOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dynamic_tier_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> terrars::ListRef<LustreInstance_MaintenancePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LustreInstance_TimeoutsRef {
        LustreInstance_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for LustreInstance {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for LustreInstance { }

impl terrars::ToListMappable for LustreInstance {
    type O = terrars::ListRef<LustreInstanceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for LustreInstance_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_lustre_instance".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLustreInstance {
    pub tf_id: std::string::String,
    #[doc =
        "The storage capacity of the instance in gibibytes (GiB). Allowed values\nare from '9000' to '7632000', depending on the 'perUnitStorageThroughput'.\nSee [Performance tiers and maximum storage\ncapacities](https://cloud.google.com/managed-lustre/docs/create-instance#performance-tiers)\nfor specific minimums, maximums, and step sizes for each performance tier."]
    pub capacity_gib: terrars::PrimField<std::string::String>,
    #[doc =
        "The filesystem name for this instance. This name is used by client-side\ntools, including when mounting the instance. Must be eight characters or\nless and can only contain letters and numbers."]
    pub filesystem: terrars::PrimField<std::string::String>,
    #[doc =
        "The name of the Managed Lustre instance.\n\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter."]
    pub instance_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "The full name of the VPC network to which the instance is connected.\nMust be in the format\n'projects/{project_id}/global/networks/{network_name}'."]
    pub network: terrars::PrimField<std::string::String>,
}

impl BuildLustreInstance {
    pub fn build(self, stack: &mut terrars::Stack) -> LustreInstance {
        let out = LustreInstance(std::rc::Rc::new(LustreInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(LustreInstanceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                capacity_gib: self.capacity_gib,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                filesystem: self.filesystem,
                gke_support_enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                kms_key: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                network: self.network,
                per_unit_storage_throughput: core::default::Default::default(),
                placement_policy: core::default::Default::default(),
                project: core::default::Default::default(),
                target_version: core::default::Default::default(),
                access_rules_options: core::default::Default::default(),
                dynamic_tier_options: core::default::Default::default(),
                maintenance_policy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LustreInstanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LustreInstanceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LustreInstanceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `available_version` after provisioning.\nThe version this instance can be upgraded to, if one is available. Empty\nwhen the instance is already running the newest release."]
    pub fn available_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `capacity_gib` after provisioning.\nThe storage capacity of the instance in gibibytes (GiB). Allowed values\nare from '9000' to '7632000', depending on the 'perUnitStorageThroughput'.\nSee [Performance tiers and maximum storage\ncapacities](https://cloud.google.com/managed-lustre/docs/create-instance#performance-tiers)\nfor specific minimums, maximums, and step sizes for each performance tier."]
    pub fn capacity_gib(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_gib", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the instance was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA user-readable description of the instance."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_version` after provisioning.\nThe version of Managed Lustre software that this instance is currently\nrunning."]
    pub fn effective_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effective_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `filesystem` after provisioning.\nThe filesystem name for this instance. This name is used by client-side\ntools, including when mounting the instance. Must be eight characters or\nless and can only contain letters and numbers."]
    pub fn filesystem(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filesystem", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gke_support_enabled` after provisioning.\nIndicates whether you want to enable support for GKE clients. By default,\nGKE clients are not supported."]
    pub fn gke_support_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gke_support_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_id` after provisioning.\nThe name of the Managed Lustre instance.\n\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter."]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key` after provisioning.\nThe Cloud KMS key name to use for data encryption.\nIf not set, the instance will use Google-managed encryption keys.\nIf set, the instance will use customer-managed encryption keys.\nThe key must be in the same region as the instance.\nThe key format is:\nprojects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{key}"]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `mount_point` after provisioning.\nMount point of the instance in the format 'IP_ADDRESS@tcp:/FILESYSTEM'."]
    pub fn mount_point(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mount_point", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the instance."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe full name of the VPC network to which the instance is connected.\nMust be in the format\n'projects/{project_id}/global/networks/{network_name}'."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `per_unit_storage_throughput` after provisioning.\nThe throughput of the instance in MBps per TiB. Valid values are 125, 250,\n500, 1000.\nSee [Performance tiers and maximum storage\ncapacities](https://cloud.google.com/managed-lustre/docs/create-instance#performance-tiers)\nfor more information.\n\nIf the instance is using the Dynamic tier, this field must not be set or\nmust be set to zero."]
    pub fn per_unit_storage_throughput(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.per_unit_storage_throughput", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `placement_policy` after provisioning.\nThe placement policy name for the instance in the format of\nprojects/{project}/locations/{location}/resourcePolicies/{resource_policy}"]
    pub fn placement_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.placement_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe state of the instance.\nPossible values:\nACTIVE\nCREATING\nDELETING\nUPGRADING\nREPAIRING\nSTOPPED\nUPDATING\nSUSPENDED"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state_reason` after provisioning.\nThe reason why the instance is in a certain state (e.g. SUSPENDED)."]
    pub fn state_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_reason", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_version` after provisioning.\nThe version to upgrade this instance to. Set this to the value reported in\n'availableVersion', or to 'latest' to move to the newest version available\nat the time of the upgrade.\nThis field cannot be set when the instance is created; new instances are\nalways provisioned from the current release. It also cannot be changed in\nthe same operation as 'capacityGib' or 'maintenancePolicy', and the\ninstance must be ACTIVE and outside of the hour preceding a scheduled\nmaintenance window.\nThe API clears this field once the upgrade finishes, so it always reads\nback as empty on an idle instance."]
    pub fn target_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nUnique ID of the resource.\nThis is unrelated to the access rules which allow specifying the root\nsquash uid."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `upcoming_maintenance_schedule` after provisioning.\nRepresents a scheduled maintenance event."]
    pub fn upcoming_maintenance_schedule(&self) -> terrars::ListRef<LustreInstance_UpcomingMaintenanceScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upcoming_maintenance_schedule", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when the instance was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `access_rules_options` after provisioning.\n"]
    pub fn access_rules_options(&self) -> terrars::ListRef<LustreInstance_AccessRulesOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_rules_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dynamic_tier_options` after provisioning.\n"]
    pub fn dynamic_tier_options(&self) -> terrars::ListRef<LustreInstance_DynamicTierOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dynamic_tier_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> terrars::ListRef<LustreInstance_MaintenancePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LustreInstance_TimeoutsRef {
        LustreInstance_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct LustreInstanceDynamic {
    access_rules_options: std::option::Option<terrars::DynamicBlock<LustreInstance_AccessRulesOptions>>,
    dynamic_tier_options: std::option::Option<terrars::DynamicBlock<LustreInstance_DynamicTierOptions>>,
    maintenance_policy: std::option::Option<terrars::DynamicBlock<LustreInstance_MaintenancePolicy>>,
}

#[derive(Serialize)]
pub struct LustreInstance_UpcomingMaintenanceSchedule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LustreInstance_UpcomingMaintenanceSchedule {
    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LustreInstance_UpcomingMaintenanceSchedule {
    type O = terrars::BlockAssignable<LustreInstance_UpcomingMaintenanceSchedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLustreInstance_UpcomingMaintenanceSchedule {}

impl BuildLustreInstance_UpcomingMaintenanceSchedule {
    pub fn build(self) -> LustreInstance_UpcomingMaintenanceSchedule {
        LustreInstance_UpcomingMaintenanceSchedule {
            end_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LustreInstance_UpcomingMaintenanceScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LustreInstance_UpcomingMaintenanceScheduleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LustreInstance_UpcomingMaintenanceScheduleRef {
        LustreInstance_UpcomingMaintenanceScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LustreInstance_UpcomingMaintenanceScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct LustreInstance_AccessRulesOptions_AccessRules {
    ip_address_ranges: terrars::ListField<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    squash_mode: terrars::PrimField<std::string::String>,
}

impl LustreInstance_AccessRulesOptions_AccessRules { }

impl terrars::ToListMappable for LustreInstance_AccessRulesOptions_AccessRules {
    type O = terrars::BlockAssignable<LustreInstance_AccessRulesOptions_AccessRules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLustreInstance_AccessRulesOptions_AccessRules {
    #[doc =
        "The IP address ranges to which to apply this access rule. Accepts\nnon-overlapping CIDR ranges (e.g., '192.168.1.0/24') and IP addresses\n(e.g., '192.168.1.0')."]
    pub ip_address_ranges: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc =
        "The name of the access rule policy group.\nMust be 16 characters or less and include only alphanumeric characters\nor '_'."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "Squash mode for the access rule.\nPossible values:\nNO_SQUASH\nROOT_SQUASH"]
    pub squash_mode: terrars::PrimField<std::string::String>,
}

impl BuildLustreInstance_AccessRulesOptions_AccessRules {
    pub fn build(self) -> LustreInstance_AccessRulesOptions_AccessRules {
        LustreInstance_AccessRulesOptions_AccessRules {
            ip_address_ranges: self.ip_address_ranges,
            name: self.name,
            squash_mode: self.squash_mode,
        }
    }
}

#[allow(dead_code)]
pub struct LustreInstance_AccessRulesOptions_AccessRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LustreInstance_AccessRulesOptions_AccessRulesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LustreInstance_AccessRulesOptions_AccessRulesRef {
        LustreInstance_AccessRulesOptions_AccessRulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LustreInstance_AccessRulesOptions_AccessRulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ip_address_ranges` after provisioning.\nThe IP address ranges to which to apply this access rule. Accepts\nnon-overlapping CIDR ranges (e.g., '192.168.1.0/24') and IP addresses\n(e.g., '192.168.1.0')."]
    pub fn ip_address_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_address_ranges", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the access rule policy group.\nMust be 16 characters or less and include only alphanumeric characters\nor '_'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `squash_mode` after provisioning.\nSquash mode for the access rule.\nPossible values:\nNO_SQUASH\nROOT_SQUASH"]
    pub fn squash_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.squash_mode", self.base))
    }
}

#[derive(Serialize, Default)]
struct LustreInstance_AccessRulesOptionsDynamic {
    access_rules: std::option::Option<terrars::DynamicBlock<LustreInstance_AccessRulesOptions_AccessRules>>,
}

#[derive(Serialize)]
pub struct LustreInstance_AccessRulesOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_squash_gid: std::option::Option<terrars::PrimField<f64>>,
    default_squash_mode: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_squash_uid: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_rules: std::option::Option<std::vec::Vec<LustreInstance_AccessRulesOptions_AccessRules>>,
    dynamic: LustreInstance_AccessRulesOptionsDynamic,
}

impl LustreInstance_AccessRulesOptions {
    #[doc =
        "Set the field `default_squash_gid`.\nThe user squash GID for the default access rule.\nThis user squash GID applies to all root users connecting from clients\nthat are not matched by any of the access rules. If not set, the default\nis 0 (no GID squash)."]
    pub fn set_default_squash_gid(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.default_squash_gid = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `default_squash_uid`.\nThe user squash UID for the default access rule.\nThis user squash UID applies to all root users connecting from clients\nthat are not matched by any of the access rules. If not set, the default\nis 0 (no UID squash)."]
    pub fn set_default_squash_uid(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.default_squash_uid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `access_rules`.\n"]
    pub fn set_access_rules(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LustreInstance_AccessRulesOptions_AccessRules>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.access_rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.access_rules = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LustreInstance_AccessRulesOptions {
    type O = terrars::BlockAssignable<LustreInstance_AccessRulesOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLustreInstance_AccessRulesOptions {
    #[doc = "The squash mode for the default access rule.\nPossible values:\nNO_SQUASH\nROOT_SQUASH"]
    pub default_squash_mode: terrars::PrimField<std::string::String>,
}

impl BuildLustreInstance_AccessRulesOptions {
    pub fn build(self) -> LustreInstance_AccessRulesOptions {
        LustreInstance_AccessRulesOptions {
            default_squash_gid: core::default::Default::default(),
            default_squash_mode: self.default_squash_mode,
            default_squash_uid: core::default::Default::default(),
            access_rules: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LustreInstance_AccessRulesOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LustreInstance_AccessRulesOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LustreInstance_AccessRulesOptionsRef {
        LustreInstance_AccessRulesOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LustreInstance_AccessRulesOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `default_squash_gid` after provisioning.\nThe user squash GID for the default access rule.\nThis user squash GID applies to all root users connecting from clients\nthat are not matched by any of the access rules. If not set, the default\nis 0 (no GID squash)."]
    pub fn default_squash_gid(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_squash_gid", self.base))
    }

    #[doc =
        "Get a reference to the value of field `default_squash_mode` after provisioning.\nThe squash mode for the default access rule.\nPossible values:\nNO_SQUASH\nROOT_SQUASH"]
    pub fn default_squash_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_squash_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `default_squash_uid` after provisioning.\nThe user squash UID for the default access rule.\nThis user squash UID applies to all root users connecting from clients\nthat are not matched by any of the access rules. If not set, the default\nis 0 (no UID squash)."]
    pub fn default_squash_uid(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_squash_uid", self.base))
    }

    #[doc = "Get a reference to the value of field `access_rules` after provisioning.\n"]
    pub fn access_rules(&self) -> terrars::ListRef<LustreInstance_AccessRulesOptions_AccessRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_rules", self.base))
    }
}

#[derive(Serialize)]
pub struct LustreInstance_DynamicTierOptions {
    mode: terrars::PrimField<std::string::String>,
}

impl LustreInstance_DynamicTierOptions { }

impl terrars::ToListMappable for LustreInstance_DynamicTierOptions {
    type O = terrars::BlockAssignable<LustreInstance_DynamicTierOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLustreInstance_DynamicTierOptions {
    #[doc = "The dynamic tier mode of the instance.\nPossible values:\nDISABLED\nDEFAULT_CACHE"]
    pub mode: terrars::PrimField<std::string::String>,
}

impl BuildLustreInstance_DynamicTierOptions {
    pub fn build(self) -> LustreInstance_DynamicTierOptions {
        LustreInstance_DynamicTierOptions { mode: self.mode }
    }
}

#[allow(dead_code)]
pub struct LustreInstance_DynamicTierOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LustreInstance_DynamicTierOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LustreInstance_DynamicTierOptionsRef {
        LustreInstance_DynamicTierOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LustreInstance_DynamicTierOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nThe dynamic tier mode of the instance.\nPossible values:\nDISABLED\nDEFAULT_CACHE"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_EndDate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    month: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    year: std::option::Option<terrars::PrimField<f64>>,
}

impl LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_EndDate {
    #[doc =
        "Set the field `day`.\nDay of a month. Must be from 1 to 31 and valid for the year and month, or 0\nto specify a year by itself or a year and month where the day isn't\nsignificant."]
    pub fn set_day(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.day = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `month`.\nMonth of a year. Must be from 1 to 12, or 0 to specify a year without a\nmonth and day."]
    pub fn set_month(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.month = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `year`.\nYear of the date. Must be from 1 to 9999, or 0 to specify a date without\na year."]
    pub fn set_year(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.year = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_EndDate {
    type O = terrars::BlockAssignable<LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_EndDate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_EndDate {}

impl BuildLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_EndDate {
    pub fn build(self) -> LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_EndDate {
        LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_EndDate {
            day: core::default::Default::default(),
            month: core::default::Default::default(),
            year: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_EndDateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_EndDateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_EndDateRef {
        LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_EndDateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_EndDateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `day` after provisioning.\nDay of a month. Must be from 1 to 31 and valid for the year and month, or 0\nto specify a year by itself or a year and month where the day isn't\nsignificant."]
    pub fn day(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc =
        "Get a reference to the value of field `month` after provisioning.\nMonth of a year. Must be from 1 to 12, or 0 to specify a year without a\nmonth and day."]
    pub fn month(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.month", self.base))
    }

    #[doc =
        "Get a reference to the value of field `year` after provisioning.\nYear of the date. Must be from 1 to 9999, or 0 to specify a date without\na year."]
    pub fn year(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.year", self.base))
    }
}

#[derive(Serialize)]
pub struct LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_StartDate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    month: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    year: std::option::Option<terrars::PrimField<f64>>,
}

impl LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_StartDate {
    #[doc =
        "Set the field `day`.\nDay of a month. Must be from 1 to 31 and valid for the year and month, or 0\nto specify a year by itself or a year and month where the day isn't\nsignificant."]
    pub fn set_day(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.day = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `month`.\nMonth of a year. Must be from 1 to 12, or 0 to specify a year without a\nmonth and day."]
    pub fn set_month(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.month = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `year`.\nYear of the date. Must be from 1 to 9999, or 0 to specify a date without\na year."]
    pub fn set_year(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.year = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_StartDate {
    type O = terrars::BlockAssignable<LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_StartDate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_StartDate {}

impl BuildLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_StartDate {
    pub fn build(self) -> LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_StartDate {
        LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_StartDate {
            day: core::default::Default::default(),
            month: core::default::Default::default(),
            year: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_StartDateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_StartDateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_StartDateRef {
        LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_StartDateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_StartDateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `day` after provisioning.\nDay of a month. Must be from 1 to 31 and valid for the year and month, or 0\nto specify a year by itself or a year and month where the day isn't\nsignificant."]
    pub fn day(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc =
        "Get a reference to the value of field `month` after provisioning.\nMonth of a year. Must be from 1 to 12, or 0 to specify a year without a\nmonth and day."]
    pub fn month(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.month", self.base))
    }

    #[doc =
        "Get a reference to the value of field `year` after provisioning.\nYear of the date. Must be from 1 to 9999, or 0 to specify a date without\na year."]
    pub fn year(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.year", self.base))
    }
}

#[derive(Serialize)]
pub struct LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_Time {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_Time {
    #[doc =
        "Set the field `hours`.\nHours of a day in 24 hour format. Must be greater than or equal to 0 and\ntypically must be less than or equal to 23. An API may choose to allow the\nvalue \"24:00:00\" for scenarios like business closing time."]
    pub fn set_hours(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hours = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `minutes`.\nMinutes of an hour. Must be greater than or equal to 0 and less than or\nequal to 59."]
    pub fn set_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `nanos`.\nFractions of seconds, in nanoseconds. Must be greater than or equal to 0\nand less than or equal to 999,999,999."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `seconds`.\nSeconds of a minute. Must be greater than or equal to 0 and typically must\nbe less than or equal to 59. An API may allow the value 60 if it allows\nleap-seconds."]
    pub fn set_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_Time {
    type O = terrars::BlockAssignable<LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_Time>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_Time {}

impl BuildLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_Time {
    pub fn build(self) -> LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_Time {
        LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_Time {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_TimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_TimeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_TimeRef {
        LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_TimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_TimeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `hours` after provisioning.\nHours of a day in 24 hour format. Must be greater than or equal to 0 and\ntypically must be less than or equal to 23. An API may choose to allow the\nvalue \"24:00:00\" for scenarios like business closing time."]
    pub fn hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }

    #[doc =
        "Get a reference to the value of field `minutes` after provisioning.\nMinutes of an hour. Must be greater than or equal to 0 and less than or\nequal to 59."]
    pub fn minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nanos` after provisioning.\nFractions of seconds, in nanoseconds. Must be greater than or equal to 0\nand less than or equal to 999,999,999."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `seconds` after provisioning.\nSeconds of a minute. Must be greater than or equal to 0 and typically must\nbe less than or equal to 59. An API may allow the value 60 if it allows\nleap-seconds."]
    pub fn seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct LustreInstance_MaintenancePolicy_MaintenanceExclusionWindowDynamic {
    end_date: std
    ::option
    ::Option<terrars::DynamicBlock<LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_EndDate>>,
    start_date: std
    ::option
    ::Option<terrars::DynamicBlock<LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_StartDate>>,
    time: std
    ::option
    ::Option<terrars::DynamicBlock<LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_Time>>,
}

#[derive(Serialize)]
pub struct LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_date: std::option::Option<std::vec::Vec<LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_EndDate>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_date: std::option::Option<std::vec::Vec<LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_StartDate>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time: std::option::Option<std::vec::Vec<LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_Time>>,
    dynamic: LustreInstance_MaintenancePolicy_MaintenanceExclusionWindowDynamic,
}

impl LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow {
    #[doc = "Set the field `end_date`.\n"]
    pub fn set_end_date(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_EndDate>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.end_date = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.end_date = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `start_date`.\n"]
    pub fn set_start_date(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_StartDate>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.start_date = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.start_date = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `time`.\n"]
    pub fn set_time(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_Time>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.time = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.time = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow {
    type O = terrars::BlockAssignable<LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow {}

impl BuildLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow {
    pub fn build(self) -> LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow {
        LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow {
            end_date: core::default::Default::default(),
            start_date: core::default::Default::default(),
            time: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LustreInstance_MaintenancePolicy_MaintenanceExclusionWindowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LustreInstance_MaintenancePolicy_MaintenanceExclusionWindowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LustreInstance_MaintenancePolicy_MaintenanceExclusionWindowRef {
        LustreInstance_MaintenancePolicy_MaintenanceExclusionWindowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LustreInstance_MaintenancePolicy_MaintenanceExclusionWindowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_date` after provisioning.\n"]
    pub fn end_date(&self) -> terrars::ListRef<LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_EndDateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.end_date", self.base))
    }

    #[doc = "Get a reference to the value of field `start_date` after provisioning.\n"]
    pub fn start_date(
        &self,
    ) -> terrars::ListRef<LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_StartDateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start_date", self.base))
    }

    #[doc = "Get a reference to the value of field `time` after provisioning.\n"]
    pub fn time(&self) -> terrars::ListRef<LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_TimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time", self.base))
    }
}

#[derive(Serialize)]
pub struct LustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows_StartTime {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl LustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows_StartTime {
    #[doc =
        "Set the field `hours`.\nHours of a day in 24 hour format. Must be greater than or equal to 0 and\ntypically must be less than or equal to 23. An API may choose to allow the\nvalue \"24:00:00\" for scenarios like business closing time."]
    pub fn set_hours(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hours = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `minutes`.\nMinutes of an hour. Must be greater than or equal to 0 and less than or\nequal to 59."]
    pub fn set_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `nanos`.\nFractions of seconds, in nanoseconds. Must be greater than or equal to 0\nand less than or equal to 999,999,999."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `seconds`.\nSeconds of a minute. Must be greater than or equal to 0 and typically must\nbe less than or equal to 59. An API may allow the value 60 if it allows\nleap-seconds."]
    pub fn set_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows_StartTime {
    type O = terrars::BlockAssignable<LustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows_StartTime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows_StartTime {}

impl BuildLustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows_StartTime {
    pub fn build(self) -> LustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows_StartTime {
        LustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows_StartTime {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows_StartTimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows_StartTimeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows_StartTimeRef {
        LustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows_StartTimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows_StartTimeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `hours` after provisioning.\nHours of a day in 24 hour format. Must be greater than or equal to 0 and\ntypically must be less than or equal to 23. An API may choose to allow the\nvalue \"24:00:00\" for scenarios like business closing time."]
    pub fn hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }

    #[doc =
        "Get a reference to the value of field `minutes` after provisioning.\nMinutes of an hour. Must be greater than or equal to 0 and less than or\nequal to 59."]
    pub fn minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nanos` after provisioning.\nFractions of seconds, in nanoseconds. Must be greater than or equal to 0\nand less than or equal to 999,999,999."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `seconds` after provisioning.\nSeconds of a minute. Must be greater than or equal to 0 and typically must\nbe less than or equal to 59. An API may allow the value 60 if it allows\nleap-seconds."]
    pub fn seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct LustreInstance_MaintenancePolicy_WeeklyMaintenanceWindowsDynamic {
    start_time: std
    ::option
    ::Option<terrars::DynamicBlock<LustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows_StartTime>>,
}

#[derive(Serialize)]
pub struct LustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows {
    day_of_week: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<std::vec::Vec<LustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows_StartTime>>,
    dynamic: LustreInstance_MaintenancePolicy_WeeklyMaintenanceWindowsDynamic,
}

impl LustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows {
    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<LustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows_StartTime>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.start_time = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.start_time = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows {
    type O = terrars::BlockAssignable<LustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows {
    #[doc = "Possible values:\nMONDAY\nTUESDAY\nWEDNESDAY\nTHURSDAY\nFRIDAY\nSATURDAY\nSUNDAY"]
    pub day_of_week: terrars::PrimField<std::string::String>,
}

impl BuildLustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows {
    pub fn build(self) -> LustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows {
        LustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows {
            day_of_week: self.day_of_week,
            start_time: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LustreInstance_MaintenancePolicy_WeeklyMaintenanceWindowsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LustreInstance_MaintenancePolicy_WeeklyMaintenanceWindowsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LustreInstance_MaintenancePolicy_WeeklyMaintenanceWindowsRef {
        LustreInstance_MaintenancePolicy_WeeklyMaintenanceWindowsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LustreInstance_MaintenancePolicy_WeeklyMaintenanceWindowsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `day_of_week` after provisioning.\nPossible values:\nMONDAY\nTUESDAY\nWEDNESDAY\nTHURSDAY\nFRIDAY\nSATURDAY\nSUNDAY"]
    pub fn day_of_week(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day_of_week", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::ListRef<LustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows_StartTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize, Default)]
struct LustreInstance_MaintenancePolicyDynamic {
    maintenance_exclusion_window: std
    ::option
    ::Option<terrars::DynamicBlock<LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow>>,
    weekly_maintenance_windows: std
    ::option
    ::Option<terrars::DynamicBlock<LustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows>>,
}

#[derive(Serialize)]
pub struct LustreInstance_MaintenancePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_exclusion_window: std
    ::option
    ::Option<std::vec::Vec<LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weekly_maintenance_windows: std
    ::option
    ::Option<std::vec::Vec<LustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows>>,
    dynamic: LustreInstance_MaintenancePolicyDynamic,
}

impl LustreInstance_MaintenancePolicy {
    #[doc = "Set the field `maintenance_exclusion_window`.\n"]
    pub fn set_maintenance_exclusion_window(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<LustreInstance_MaintenancePolicy_MaintenanceExclusionWindow>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.maintenance_exclusion_window = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.maintenance_exclusion_window = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `weekly_maintenance_windows`.\n"]
    pub fn set_weekly_maintenance_windows(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<LustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.weekly_maintenance_windows = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.weekly_maintenance_windows = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LustreInstance_MaintenancePolicy {
    type O = terrars::BlockAssignable<LustreInstance_MaintenancePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLustreInstance_MaintenancePolicy {}

impl BuildLustreInstance_MaintenancePolicy {
    pub fn build(self) -> LustreInstance_MaintenancePolicy {
        LustreInstance_MaintenancePolicy {
            maintenance_exclusion_window: core::default::Default::default(),
            weekly_maintenance_windows: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LustreInstance_MaintenancePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LustreInstance_MaintenancePolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LustreInstance_MaintenancePolicyRef {
        LustreInstance_MaintenancePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LustreInstance_MaintenancePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `maintenance_exclusion_window` after provisioning.\n"]
    pub fn maintenance_exclusion_window(
        &self,
    ) -> terrars::ListRef<LustreInstance_MaintenancePolicy_MaintenanceExclusionWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_exclusion_window", self.base))
    }

    #[doc = "Get a reference to the value of field `weekly_maintenance_windows` after provisioning.\n"]
    pub fn weekly_maintenance_windows(
        &self,
    ) -> terrars::ListRef<LustreInstance_MaintenancePolicy_WeeklyMaintenanceWindowsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weekly_maintenance_windows", self.base))
    }
}

#[derive(Serialize)]
pub struct LustreInstance_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LustreInstance_Timeouts {
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

impl terrars::ToListMappable for LustreInstance_Timeouts {
    type O = terrars::BlockAssignable<LustreInstance_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLustreInstance_Timeouts {}

impl BuildLustreInstance_Timeouts {
    pub fn build(self) -> LustreInstance_Timeouts {
        LustreInstance_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LustreInstance_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LustreInstance_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LustreInstance_TimeoutsRef {
        LustreInstance_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LustreInstance_TimeoutsRef {
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
