#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeRegionInstanceGroupManagerData_ {
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
    base_instance_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    distribution_policy_target_shape: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    distribution_policy_zones: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    list_managed_instances_results: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_pools: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_stopped_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_suspended_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wait_for_instances: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wait_for_instances_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all_instances_config: std::option::Option<std::vec::Vec<ComputeRegionInstanceGroupManager_AllInstancesConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_healing_policies: std::option::Option<std::vec::Vec<ComputeRegionInstanceGroupManager_AutoHealingPolicies>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_flexibility_policy: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionInstanceGroupManager_InstanceFlexibilityPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_lifecycle_policy: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionInstanceGroupManager_InstanceLifecyclePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    named_port: std::option::Option<std::vec::Vec<ComputeRegionInstanceGroupManager_NamedPort>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_policies: std::option::Option<std::vec::Vec<ComputeRegionInstanceGroupManager_ResourcePolicies>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    standby_policy: std::option::Option<std::vec::Vec<ComputeRegionInstanceGroupManager_StandbyPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateful_disk: std::option::Option<std::vec::Vec<ComputeRegionInstanceGroupManager_StatefulDisk>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateful_external_ip: std::option::Option<std::vec::Vec<ComputeRegionInstanceGroupManager_StatefulExternalIp>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateful_internal_ip: std::option::Option<std::vec::Vec<ComputeRegionInstanceGroupManager_StatefulInternalIp>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_size_policy: std::option::Option<std::vec::Vec<ComputeRegionInstanceGroupManager_TargetSizePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeRegionInstanceGroupManager_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_policy: std::option::Option<std::vec::Vec<ComputeRegionInstanceGroupManager_UpdatePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<std::vec::Vec<ComputeRegionInstanceGroupManager_Version>>,
    dynamic: ComputeRegionInstanceGroupManagerDynamic,
}

struct ComputeRegionInstanceGroupManager_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeRegionInstanceGroupManagerData_>,
}

#[derive(Clone)]
pub struct ComputeRegionInstanceGroupManager(std::rc::Rc<ComputeRegionInstanceGroupManager_>);

impl ComputeRegionInstanceGroupManager {
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

    #[doc = "Set the field `description`.\nAn optional textual description of the instance group manager."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `distribution_policy_target_shape`.\nThe shape to which the group converges either proactively or on resize events (depending on the value set in updatePolicy.instanceRedistributionType)."]
    pub fn set_distribution_policy_target_shape(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().distribution_policy_target_shape = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `distribution_policy_zones`.\nThe distribution policy for this managed instance group. You can specify one or more values."]
    pub fn set_distribution_policy_zones(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().distribution_policy_zones = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `list_managed_instances_results`.\nPagination behavior of the listManagedInstances API method for this managed instance group. Valid values are: \"PAGELESS\", \"PAGINATED\". If PAGELESS (default), Pagination is disabled for the group's listManagedInstances API method. maxResults and pageToken query parameters are ignored and all instances are returned in a single response. If PAGINATED, pagination is enabled, maxResults and pageToken query parameters are respected."]
    pub fn set_list_managed_instances_results(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().list_managed_instances_results = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project`.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nThe region where the managed instance group resides."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `target_pools`.\nThe full URL of all target pools to which new instances in the group are added. Updating the target pools attribute does not affect existing instances."]
    pub fn set_target_pools(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().target_pools = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `target_size`.\nThe target number of running instances for this managed instance group. This value should always be explicitly set unless this resource is attached to an autoscaler, in which case it should never be set. Defaults to 0."]
    pub fn set_target_size(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().target_size = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `target_stopped_size`.\nThe target number of stopped instances for this managed instance group."]
    pub fn set_target_stopped_size(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().target_stopped_size = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `target_suspended_size`.\nThe target number of suspended instances for this managed instance group."]
    pub fn set_target_suspended_size(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().target_suspended_size = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `wait_for_instances`.\nWhether to wait for all instances to be created/updated before returning. Note that if this is set to true and the operation does not succeed, Terraform will continue trying until it times out."]
    pub fn set_wait_for_instances(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().wait_for_instances = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `wait_for_instances_status`.\nWhen used with wait_for_instances specifies the status to wait for. When STABLE is specified this resource will wait until the instances are stable before returning. When UPDATED is set, it will wait for the version target to be reached and any per instance configs to be effective and all instances configs to be effective as well as all instances to be stable before returning."]
    pub fn set_wait_for_instances_status(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().wait_for_instances_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `all_instances_config`.\n"]
    pub fn set_all_instances_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionInstanceGroupManager_AllInstancesConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().all_instances_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.all_instances_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `auto_healing_policies`.\n"]
    pub fn set_auto_healing_policies(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionInstanceGroupManager_AutoHealingPolicies>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auto_healing_policies = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auto_healing_policies = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `instance_flexibility_policy`.\n"]
    pub fn set_instance_flexibility_policy(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeRegionInstanceGroupManager_InstanceFlexibilityPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().instance_flexibility_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.instance_flexibility_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `instance_lifecycle_policy`.\n"]
    pub fn set_instance_lifecycle_policy(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeRegionInstanceGroupManager_InstanceLifecyclePolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().instance_lifecycle_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.instance_lifecycle_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `named_port`.\n"]
    pub fn set_named_port(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionInstanceGroupManager_NamedPort>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().named_port = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.named_port = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_policies`.\n"]
    pub fn set_resource_policies(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionInstanceGroupManager_ResourcePolicies>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().resource_policies = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.resource_policies = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `standby_policy`.\n"]
    pub fn set_standby_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionInstanceGroupManager_StandbyPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().standby_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.standby_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `stateful_disk`.\n"]
    pub fn set_stateful_disk(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionInstanceGroupManager_StatefulDisk>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().stateful_disk = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.stateful_disk = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `stateful_external_ip`.\n"]
    pub fn set_stateful_external_ip(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionInstanceGroupManager_StatefulExternalIp>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().stateful_external_ip = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.stateful_external_ip = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `stateful_internal_ip`.\n"]
    pub fn set_stateful_internal_ip(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionInstanceGroupManager_StatefulInternalIp>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().stateful_internal_ip = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.stateful_internal_ip = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target_size_policy`.\n"]
    pub fn set_target_size_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionInstanceGroupManager_TargetSizePolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target_size_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target_size_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeRegionInstanceGroupManager_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_policy`.\n"]
    pub fn set_update_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionInstanceGroupManager_UpdatePolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().update_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.update_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionInstanceGroupManager_Version>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().version = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.version = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `base_instance_name` after provisioning.\nThe base instance name to use for instances in this group. The value must be a valid RFC1035 name. Supported characters are lowercase letters, numbers, and hyphens (-). Instances are named by appending a hyphen and a random four-character string to the base instance name."]
    pub fn base_instance_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base_instance_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional textual description of the instance group manager."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `distribution_policy_target_shape` after provisioning.\nThe shape to which the group converges either proactively or on resize events (depending on the value set in updatePolicy.instanceRedistributionType)."]
    pub fn distribution_policy_target_shape(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.distribution_policy_target_shape", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `distribution_policy_zones` after provisioning.\nThe distribution policy for this managed instance group. You can specify one or more values."]
    pub fn distribution_policy_zones(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.distribution_policy_zones", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fingerprint` after provisioning.\nThe fingerprint of the instance group manager."]
    pub fn fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fingerprint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_group` after provisioning.\nThe full URL of the instance group created by the manager."]
    pub fn instance_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_group", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_group_manager_id` after provisioning.\nThe unique identifier number for the resource. This identifier is defined by the server."]
    pub fn instance_group_manager_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_group_manager_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `list_managed_instances_results` after provisioning.\nPagination behavior of the listManagedInstances API method for this managed instance group. Valid values are: \"PAGELESS\", \"PAGINATED\". If PAGELESS (default), Pagination is disabled for the group's listManagedInstances API method. maxResults and pageToken query parameters are ignored and all instances are returned in a single response. If PAGINATED, pagination is enabled, maxResults and pageToken query parameters are respected."]
    pub fn list_managed_instances_results(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.list_managed_instances_results", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the instance group manager. Must be 1-63 characters long and comply with RFC1035. Supported characters include lowercase letters, numbers, and hyphens."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region where the managed instance group resides."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URL of the created resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe status of this managed instance group."]
    pub fn status(&self) -> terrars::ListRef<ComputeRegionInstanceGroupManager_StatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_pools` after provisioning.\nThe full URL of all target pools to which new instances in the group are added. Updating the target pools attribute does not affect existing instances."]
    pub fn target_pools(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.target_pools", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_size` after provisioning.\nThe target number of running instances for this managed instance group. This value should always be explicitly set unless this resource is attached to an autoscaler, in which case it should never be set. Defaults to 0."]
    pub fn target_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_size", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_stopped_size` after provisioning.\nThe target number of stopped instances for this managed instance group."]
    pub fn target_stopped_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_stopped_size", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_suspended_size` after provisioning.\nThe target number of suspended instances for this managed instance group."]
    pub fn target_suspended_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_suspended_size", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `wait_for_instances` after provisioning.\nWhether to wait for all instances to be created/updated before returning. Note that if this is set to true and the operation does not succeed, Terraform will continue trying until it times out."]
    pub fn wait_for_instances(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_for_instances", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `wait_for_instances_status` after provisioning.\nWhen used with wait_for_instances specifies the status to wait for. When STABLE is specified this resource will wait until the instances are stable before returning. When UPDATED is set, it will wait for the version target to be reached and any per instance configs to be effective and all instances configs to be effective as well as all instances to be stable before returning."]
    pub fn wait_for_instances_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_for_instances_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `all_instances_config` after provisioning.\n"]
    pub fn all_instances_config(&self) -> terrars::ListRef<ComputeRegionInstanceGroupManager_AllInstancesConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.all_instances_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_healing_policies` after provisioning.\n"]
    pub fn auto_healing_policies(&self) -> terrars::ListRef<ComputeRegionInstanceGroupManager_AutoHealingPoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_healing_policies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_flexibility_policy` after provisioning.\n"]
    pub fn instance_flexibility_policy(
        &self,
    ) -> terrars::ListRef<ComputeRegionInstanceGroupManager_InstanceFlexibilityPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_flexibility_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_lifecycle_policy` after provisioning.\n"]
    pub fn instance_lifecycle_policy(
        &self,
    ) -> terrars::ListRef<ComputeRegionInstanceGroupManager_InstanceLifecyclePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_lifecycle_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_policies` after provisioning.\n"]
    pub fn resource_policies(&self) -> terrars::ListRef<ComputeRegionInstanceGroupManager_ResourcePoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_policies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `standby_policy` after provisioning.\n"]
    pub fn standby_policy(&self) -> terrars::ListRef<ComputeRegionInstanceGroupManager_StandbyPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.standby_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stateful_external_ip` after provisioning.\n"]
    pub fn stateful_external_ip(&self) -> terrars::ListRef<ComputeRegionInstanceGroupManager_StatefulExternalIpRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateful_external_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stateful_internal_ip` after provisioning.\n"]
    pub fn stateful_internal_ip(&self) -> terrars::ListRef<ComputeRegionInstanceGroupManager_StatefulInternalIpRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateful_internal_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_size_policy` after provisioning.\n"]
    pub fn target_size_policy(&self) -> terrars::ListRef<ComputeRegionInstanceGroupManager_TargetSizePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_size_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRegionInstanceGroupManager_TimeoutsRef {
        ComputeRegionInstanceGroupManager_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `update_policy` after provisioning.\n"]
    pub fn update_policy(&self) -> terrars::ListRef<ComputeRegionInstanceGroupManager_UpdatePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.update_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::ListRef<ComputeRegionInstanceGroupManager_VersionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

impl terrars::Referable for ComputeRegionInstanceGroupManager {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputeRegionInstanceGroupManager { }

impl terrars::ToListMappable for ComputeRegionInstanceGroupManager {
    type O = terrars::ListRef<ComputeRegionInstanceGroupManagerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputeRegionInstanceGroupManager_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_compute_region_instance_group_manager".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeRegionInstanceGroupManager {
    pub tf_id: std::string::String,
    #[doc =
        "The base instance name to use for instances in this group. The value must be a valid RFC1035 name. Supported characters are lowercase letters, numbers, and hyphens (-). Instances are named by appending a hyphen and a random four-character string to the base instance name."]
    pub base_instance_name: terrars::PrimField<std::string::String>,
    #[doc =
        "The name of the instance group manager. Must be 1-63 characters long and comply with RFC1035. Supported characters include lowercase letters, numbers, and hyphens."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionInstanceGroupManager {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputeRegionInstanceGroupManager {
        let out = ComputeRegionInstanceGroupManager(std::rc::Rc::new(ComputeRegionInstanceGroupManager_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputeRegionInstanceGroupManagerData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                base_instance_name: self.base_instance_name,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                distribution_policy_target_shape: core::default::Default::default(),
                distribution_policy_zones: core::default::Default::default(),
                id: core::default::Default::default(),
                list_managed_instances_results: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                target_pools: core::default::Default::default(),
                target_size: core::default::Default::default(),
                target_stopped_size: core::default::Default::default(),
                target_suspended_size: core::default::Default::default(),
                wait_for_instances: core::default::Default::default(),
                wait_for_instances_status: core::default::Default::default(),
                all_instances_config: core::default::Default::default(),
                auto_healing_policies: core::default::Default::default(),
                instance_flexibility_policy: core::default::Default::default(),
                instance_lifecycle_policy: core::default::Default::default(),
                named_port: core::default::Default::default(),
                resource_policies: core::default::Default::default(),
                standby_policy: core::default::Default::default(),
                stateful_disk: core::default::Default::default(),
                stateful_external_ip: core::default::Default::default(),
                stateful_internal_ip: core::default::Default::default(),
                target_size_policy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                update_policy: core::default::Default::default(),
                version: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeRegionInstanceGroupManagerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionInstanceGroupManagerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeRegionInstanceGroupManagerRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `base_instance_name` after provisioning.\nThe base instance name to use for instances in this group. The value must be a valid RFC1035 name. Supported characters are lowercase letters, numbers, and hyphens (-). Instances are named by appending a hyphen and a random four-character string to the base instance name."]
    pub fn base_instance_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base_instance_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional textual description of the instance group manager."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `distribution_policy_target_shape` after provisioning.\nThe shape to which the group converges either proactively or on resize events (depending on the value set in updatePolicy.instanceRedistributionType)."]
    pub fn distribution_policy_target_shape(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.distribution_policy_target_shape", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `distribution_policy_zones` after provisioning.\nThe distribution policy for this managed instance group. You can specify one or more values."]
    pub fn distribution_policy_zones(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.distribution_policy_zones", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fingerprint` after provisioning.\nThe fingerprint of the instance group manager."]
    pub fn fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fingerprint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_group` after provisioning.\nThe full URL of the instance group created by the manager."]
    pub fn instance_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_group", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_group_manager_id` after provisioning.\nThe unique identifier number for the resource. This identifier is defined by the server."]
    pub fn instance_group_manager_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_group_manager_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `list_managed_instances_results` after provisioning.\nPagination behavior of the listManagedInstances API method for this managed instance group. Valid values are: \"PAGELESS\", \"PAGINATED\". If PAGELESS (default), Pagination is disabled for the group's listManagedInstances API method. maxResults and pageToken query parameters are ignored and all instances are returned in a single response. If PAGINATED, pagination is enabled, maxResults and pageToken query parameters are respected."]
    pub fn list_managed_instances_results(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.list_managed_instances_results", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the instance group manager. Must be 1-63 characters long and comply with RFC1035. Supported characters include lowercase letters, numbers, and hyphens."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region where the managed instance group resides."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URL of the created resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe status of this managed instance group."]
    pub fn status(&self) -> terrars::ListRef<ComputeRegionInstanceGroupManager_StatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_pools` after provisioning.\nThe full URL of all target pools to which new instances in the group are added. Updating the target pools attribute does not affect existing instances."]
    pub fn target_pools(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.target_pools", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_size` after provisioning.\nThe target number of running instances for this managed instance group. This value should always be explicitly set unless this resource is attached to an autoscaler, in which case it should never be set. Defaults to 0."]
    pub fn target_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_size", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_stopped_size` after provisioning.\nThe target number of stopped instances for this managed instance group."]
    pub fn target_stopped_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_stopped_size", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_suspended_size` after provisioning.\nThe target number of suspended instances for this managed instance group."]
    pub fn target_suspended_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_suspended_size", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `wait_for_instances` after provisioning.\nWhether to wait for all instances to be created/updated before returning. Note that if this is set to true and the operation does not succeed, Terraform will continue trying until it times out."]
    pub fn wait_for_instances(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_for_instances", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `wait_for_instances_status` after provisioning.\nWhen used with wait_for_instances specifies the status to wait for. When STABLE is specified this resource will wait until the instances are stable before returning. When UPDATED is set, it will wait for the version target to be reached and any per instance configs to be effective and all instances configs to be effective as well as all instances to be stable before returning."]
    pub fn wait_for_instances_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_for_instances_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `all_instances_config` after provisioning.\n"]
    pub fn all_instances_config(&self) -> terrars::ListRef<ComputeRegionInstanceGroupManager_AllInstancesConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.all_instances_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_healing_policies` after provisioning.\n"]
    pub fn auto_healing_policies(&self) -> terrars::ListRef<ComputeRegionInstanceGroupManager_AutoHealingPoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_healing_policies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_flexibility_policy` after provisioning.\n"]
    pub fn instance_flexibility_policy(
        &self,
    ) -> terrars::ListRef<ComputeRegionInstanceGroupManager_InstanceFlexibilityPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_flexibility_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_lifecycle_policy` after provisioning.\n"]
    pub fn instance_lifecycle_policy(
        &self,
    ) -> terrars::ListRef<ComputeRegionInstanceGroupManager_InstanceLifecyclePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_lifecycle_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_policies` after provisioning.\n"]
    pub fn resource_policies(&self) -> terrars::ListRef<ComputeRegionInstanceGroupManager_ResourcePoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_policies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `standby_policy` after provisioning.\n"]
    pub fn standby_policy(&self) -> terrars::ListRef<ComputeRegionInstanceGroupManager_StandbyPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.standby_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stateful_external_ip` after provisioning.\n"]
    pub fn stateful_external_ip(&self) -> terrars::ListRef<ComputeRegionInstanceGroupManager_StatefulExternalIpRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateful_external_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stateful_internal_ip` after provisioning.\n"]
    pub fn stateful_internal_ip(&self) -> terrars::ListRef<ComputeRegionInstanceGroupManager_StatefulInternalIpRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateful_internal_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_size_policy` after provisioning.\n"]
    pub fn target_size_policy(&self) -> terrars::ListRef<ComputeRegionInstanceGroupManager_TargetSizePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_size_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRegionInstanceGroupManager_TimeoutsRef {
        ComputeRegionInstanceGroupManager_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `update_policy` after provisioning.\n"]
    pub fn update_policy(&self) -> terrars::ListRef<ComputeRegionInstanceGroupManager_UpdatePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.update_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::ListRef<ComputeRegionInstanceGroupManager_VersionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionInstanceGroupManagerDynamic {
    all_instances_config: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionInstanceGroupManager_AllInstancesConfig>>,
    auto_healing_policies: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionInstanceGroupManager_AutoHealingPolicies>>,
    instance_flexibility_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionInstanceGroupManager_InstanceFlexibilityPolicy>>,
    instance_lifecycle_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionInstanceGroupManager_InstanceLifecyclePolicy>>,
    named_port: std::option::Option<terrars::DynamicBlock<ComputeRegionInstanceGroupManager_NamedPort>>,
    resource_policies: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionInstanceGroupManager_ResourcePolicies>>,
    standby_policy: std::option::Option<terrars::DynamicBlock<ComputeRegionInstanceGroupManager_StandbyPolicy>>,
    stateful_disk: std::option::Option<terrars::DynamicBlock<ComputeRegionInstanceGroupManager_StatefulDisk>>,
    stateful_external_ip: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionInstanceGroupManager_StatefulExternalIp>>,
    stateful_internal_ip: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionInstanceGroupManager_StatefulInternalIp>>,
    target_size_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionInstanceGroupManager_TargetSizePolicy>>,
    update_policy: std::option::Option<terrars::DynamicBlock<ComputeRegionInstanceGroupManager_UpdatePolicy>>,
    version: std::option::Option<terrars::DynamicBlock<ComputeRegionInstanceGroupManager_Version>>,
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceGroupManager_Status_AllInstancesConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    current_revision: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effective: std::option::Option<terrars::PrimField<bool>>,
}

impl ComputeRegionInstanceGroupManager_Status_AllInstancesConfig {
    #[doc = "Set the field `current_revision`.\n"]
    pub fn set_current_revision(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.current_revision = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `effective`.\n"]
    pub fn set_effective(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.effective = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionInstanceGroupManager_Status_AllInstancesConfig {
    type O = terrars::BlockAssignable<ComputeRegionInstanceGroupManager_Status_AllInstancesConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceGroupManager_Status_AllInstancesConfig {}

impl BuildComputeRegionInstanceGroupManager_Status_AllInstancesConfig {
    pub fn build(self) -> ComputeRegionInstanceGroupManager_Status_AllInstancesConfig {
        ComputeRegionInstanceGroupManager_Status_AllInstancesConfig {
            current_revision: core::default::Default::default(),
            effective: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionInstanceGroupManager_Status_AllInstancesConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionInstanceGroupManager_Status_AllInstancesConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionInstanceGroupManager_Status_AllInstancesConfigRef {
        ComputeRegionInstanceGroupManager_Status_AllInstancesConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceGroupManager_Status_AllInstancesConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `current_revision` after provisioning.\n"]
    pub fn current_revision(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.current_revision", self.base))
    }

    #[doc = "Get a reference to the value of field `effective` after provisioning.\n"]
    pub fn effective(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effective", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceGroupManager_Status_Stateful_PerInstanceConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all_effective: std::option::Option<terrars::PrimField<bool>>,
}

impl ComputeRegionInstanceGroupManager_Status_Stateful_PerInstanceConfigs {
    #[doc = "Set the field `all_effective`.\n"]
    pub fn set_all_effective(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.all_effective = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionInstanceGroupManager_Status_Stateful_PerInstanceConfigs {
    type O = terrars::BlockAssignable<ComputeRegionInstanceGroupManager_Status_Stateful_PerInstanceConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceGroupManager_Status_Stateful_PerInstanceConfigs {}

impl BuildComputeRegionInstanceGroupManager_Status_Stateful_PerInstanceConfigs {
    pub fn build(self) -> ComputeRegionInstanceGroupManager_Status_Stateful_PerInstanceConfigs {
        ComputeRegionInstanceGroupManager_Status_Stateful_PerInstanceConfigs {
            all_effective: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionInstanceGroupManager_Status_Stateful_PerInstanceConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionInstanceGroupManager_Status_Stateful_PerInstanceConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionInstanceGroupManager_Status_Stateful_PerInstanceConfigsRef {
        ComputeRegionInstanceGroupManager_Status_Stateful_PerInstanceConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceGroupManager_Status_Stateful_PerInstanceConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `all_effective` after provisioning.\n"]
    pub fn all_effective(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.all_effective", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceGroupManager_Status_Stateful {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    has_stateful_config: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_instance_configs: std
    ::option
    ::Option<terrars::ListField<ComputeRegionInstanceGroupManager_Status_Stateful_PerInstanceConfigs>>,
}

impl ComputeRegionInstanceGroupManager_Status_Stateful {
    #[doc = "Set the field `has_stateful_config`.\n"]
    pub fn set_has_stateful_config(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.has_stateful_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `per_instance_configs`.\n"]
    pub fn set_per_instance_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<ComputeRegionInstanceGroupManager_Status_Stateful_PerInstanceConfigs>>,
    ) -> Self {
        self.per_instance_configs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionInstanceGroupManager_Status_Stateful {
    type O = terrars::BlockAssignable<ComputeRegionInstanceGroupManager_Status_Stateful>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceGroupManager_Status_Stateful {}

impl BuildComputeRegionInstanceGroupManager_Status_Stateful {
    pub fn build(self) -> ComputeRegionInstanceGroupManager_Status_Stateful {
        ComputeRegionInstanceGroupManager_Status_Stateful {
            has_stateful_config: core::default::Default::default(),
            per_instance_configs: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionInstanceGroupManager_Status_StatefulRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionInstanceGroupManager_Status_StatefulRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionInstanceGroupManager_Status_StatefulRef {
        ComputeRegionInstanceGroupManager_Status_StatefulRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceGroupManager_Status_StatefulRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `has_stateful_config` after provisioning.\n"]
    pub fn has_stateful_config(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.has_stateful_config", self.base))
    }

    #[doc = "Get a reference to the value of field `per_instance_configs` after provisioning.\n"]
    pub fn per_instance_configs(
        &self,
    ) -> terrars::ListRef<ComputeRegionInstanceGroupManager_Status_Stateful_PerInstanceConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.per_instance_configs", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceGroupManager_Status_VersionTarget {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_reached: std::option::Option<terrars::PrimField<bool>>,
}

impl ComputeRegionInstanceGroupManager_Status_VersionTarget {
    #[doc = "Set the field `is_reached`.\n"]
    pub fn set_is_reached(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_reached = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionInstanceGroupManager_Status_VersionTarget {
    type O = terrars::BlockAssignable<ComputeRegionInstanceGroupManager_Status_VersionTarget>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceGroupManager_Status_VersionTarget {}

impl BuildComputeRegionInstanceGroupManager_Status_VersionTarget {
    pub fn build(self) -> ComputeRegionInstanceGroupManager_Status_VersionTarget {
        ComputeRegionInstanceGroupManager_Status_VersionTarget { is_reached: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionInstanceGroupManager_Status_VersionTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionInstanceGroupManager_Status_VersionTargetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionInstanceGroupManager_Status_VersionTargetRef {
        ComputeRegionInstanceGroupManager_Status_VersionTargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceGroupManager_Status_VersionTargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `is_reached` after provisioning.\n"]
    pub fn is_reached(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_reached", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceGroupManager_Status {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all_instances_config: std
    ::option
    ::Option<terrars::ListField<ComputeRegionInstanceGroupManager_Status_AllInstancesConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_stable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateful: std::option::Option<terrars::ListField<ComputeRegionInstanceGroupManager_Status_Stateful>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version_target: std::option::Option<terrars::ListField<ComputeRegionInstanceGroupManager_Status_VersionTarget>>,
}

impl ComputeRegionInstanceGroupManager_Status {
    #[doc = "Set the field `all_instances_config`.\n"]
    pub fn set_all_instances_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ComputeRegionInstanceGroupManager_Status_AllInstancesConfig>>,
    ) -> Self {
        self.all_instances_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_stable`.\n"]
    pub fn set_is_stable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_stable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stateful`.\n"]
    pub fn set_stateful(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ComputeRegionInstanceGroupManager_Status_Stateful>>,
    ) -> Self {
        self.stateful = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version_target`.\n"]
    pub fn set_version_target(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ComputeRegionInstanceGroupManager_Status_VersionTarget>>,
    ) -> Self {
        self.version_target = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionInstanceGroupManager_Status {
    type O = terrars::BlockAssignable<ComputeRegionInstanceGroupManager_Status>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceGroupManager_Status {}

impl BuildComputeRegionInstanceGroupManager_Status {
    pub fn build(self) -> ComputeRegionInstanceGroupManager_Status {
        ComputeRegionInstanceGroupManager_Status {
            all_instances_config: core::default::Default::default(),
            is_stable: core::default::Default::default(),
            stateful: core::default::Default::default(),
            version_target: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionInstanceGroupManager_StatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionInstanceGroupManager_StatusRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRegionInstanceGroupManager_StatusRef {
        ComputeRegionInstanceGroupManager_StatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceGroupManager_StatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `all_instances_config` after provisioning.\n"]
    pub fn all_instances_config(
        &self,
    ) -> terrars::ListRef<ComputeRegionInstanceGroupManager_Status_AllInstancesConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.all_instances_config", self.base))
    }

    #[doc = "Get a reference to the value of field `is_stable` after provisioning.\n"]
    pub fn is_stable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_stable", self.base))
    }

    #[doc = "Get a reference to the value of field `stateful` after provisioning.\n"]
    pub fn stateful(&self) -> terrars::ListRef<ComputeRegionInstanceGroupManager_Status_StatefulRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateful", self.base))
    }

    #[doc = "Get a reference to the value of field `version_target` after provisioning.\n"]
    pub fn version_target(&self) -> terrars::ListRef<ComputeRegionInstanceGroupManager_Status_VersionTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.version_target", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceGroupManager_AllInstancesConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl ComputeRegionInstanceGroupManager_AllInstancesConfig {
    #[doc = "Set the field `labels`.\nThe label key-value pairs that you want to patch onto the instance,"]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `metadata`.\nThe metadata key-value pairs that you want to patch onto the instance. For more information, see Project and instance metadata,"]
    pub fn set_metadata(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.metadata = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionInstanceGroupManager_AllInstancesConfig {
    type O = terrars::BlockAssignable<ComputeRegionInstanceGroupManager_AllInstancesConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceGroupManager_AllInstancesConfig {}

impl BuildComputeRegionInstanceGroupManager_AllInstancesConfig {
    pub fn build(self) -> ComputeRegionInstanceGroupManager_AllInstancesConfig {
        ComputeRegionInstanceGroupManager_AllInstancesConfig {
            labels: core::default::Default::default(),
            metadata: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionInstanceGroupManager_AllInstancesConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionInstanceGroupManager_AllInstancesConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionInstanceGroupManager_AllInstancesConfigRef {
        ComputeRegionInstanceGroupManager_AllInstancesConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceGroupManager_AllInstancesConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe label key-value pairs that you want to patch onto the instance,"]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `metadata` after provisioning.\nThe metadata key-value pairs that you want to patch onto the instance. For more information, see Project and instance metadata,"]
    pub fn metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceGroupManager_AutoHealingPolicies {
    health_check: terrars::PrimField<std::string::String>,
    initial_delay_sec: terrars::PrimField<f64>,
}

impl ComputeRegionInstanceGroupManager_AutoHealingPolicies { }

impl terrars::ToListMappable for ComputeRegionInstanceGroupManager_AutoHealingPolicies {
    type O = terrars::BlockAssignable<ComputeRegionInstanceGroupManager_AutoHealingPolicies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceGroupManager_AutoHealingPolicies {
    #[doc = "The health check resource that signals autohealing."]
    pub health_check: terrars::PrimField<std::string::String>,
    #[doc =
        "The number of seconds that the managed instance group waits before it applies autohealing policies to new instances or recently recreated instances. Between 0 and 3600."]
    pub initial_delay_sec: terrars::PrimField<f64>,
}

impl BuildComputeRegionInstanceGroupManager_AutoHealingPolicies {
    pub fn build(self) -> ComputeRegionInstanceGroupManager_AutoHealingPolicies {
        ComputeRegionInstanceGroupManager_AutoHealingPolicies {
            health_check: self.health_check,
            initial_delay_sec: self.initial_delay_sec,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionInstanceGroupManager_AutoHealingPoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionInstanceGroupManager_AutoHealingPoliciesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionInstanceGroupManager_AutoHealingPoliciesRef {
        ComputeRegionInstanceGroupManager_AutoHealingPoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceGroupManager_AutoHealingPoliciesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `health_check` after provisioning.\nThe health check resource that signals autohealing."]
    pub fn health_check(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_check", self.base))
    }

    #[doc =
        "Get a reference to the value of field `initial_delay_sec` after provisioning.\nThe number of seconds that the managed instance group waits before it applies autohealing policies to new instances or recently recreated instances. Between 0 and 3600."]
    pub fn initial_delay_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_delay_sec", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceGroupManager_InstanceFlexibilityPolicy_InstanceSelections {
    machine_types: terrars::SetField<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rank: std::option::Option<terrars::PrimField<f64>>,
}

impl ComputeRegionInstanceGroupManager_InstanceFlexibilityPolicy_InstanceSelections {
    #[doc =
        "Set the field `rank`.\nPreference of this instance selection. Lower number means higher preference. MIG will first try to create a VM based on the machine-type with lowest rank and fallback to next rank based on availability. Machine types and instance selections with the same rank have the same preference."]
    pub fn set_rank(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.rank = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionInstanceGroupManager_InstanceFlexibilityPolicy_InstanceSelections {
    type O =
        terrars::BlockAssignable<ComputeRegionInstanceGroupManager_InstanceFlexibilityPolicy_InstanceSelections>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceGroupManager_InstanceFlexibilityPolicy_InstanceSelections {
    #[doc = "Full machine-type names, e.g. \"n1-standard-16\""]
    pub machine_types: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = "Instance selection name."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionInstanceGroupManager_InstanceFlexibilityPolicy_InstanceSelections {
    pub fn build(self) -> ComputeRegionInstanceGroupManager_InstanceFlexibilityPolicy_InstanceSelections {
        ComputeRegionInstanceGroupManager_InstanceFlexibilityPolicy_InstanceSelections {
            machine_types: self.machine_types,
            name: self.name,
            rank: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionInstanceGroupManager_InstanceFlexibilityPolicy_InstanceSelectionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionInstanceGroupManager_InstanceFlexibilityPolicy_InstanceSelectionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionInstanceGroupManager_InstanceFlexibilityPolicy_InstanceSelectionsRef {
        ComputeRegionInstanceGroupManager_InstanceFlexibilityPolicy_InstanceSelectionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceGroupManager_InstanceFlexibilityPolicy_InstanceSelectionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `machine_types` after provisioning.\nFull machine-type names, e.g. \"n1-standard-16\""]
    pub fn machine_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.machine_types", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nInstance selection name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rank` after provisioning.\nPreference of this instance selection. Lower number means higher preference. MIG will first try to create a VM based on the machine-type with lowest rank and fallback to next rank based on availability. Machine types and instance selections with the same rank have the same preference."]
    pub fn rank(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rank", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionInstanceGroupManager_InstanceFlexibilityPolicyDynamic {
    instance_selections: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionInstanceGroupManager_InstanceFlexibilityPolicy_InstanceSelections>>,
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceGroupManager_InstanceFlexibilityPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_selections: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionInstanceGroupManager_InstanceFlexibilityPolicy_InstanceSelections>>,
    dynamic: ComputeRegionInstanceGroupManager_InstanceFlexibilityPolicyDynamic,
}

impl ComputeRegionInstanceGroupManager_InstanceFlexibilityPolicy {
    #[doc = "Set the field `instance_selections`.\n"]
    pub fn set_instance_selections(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeRegionInstanceGroupManager_InstanceFlexibilityPolicy_InstanceSelections,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.instance_selections = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_selections = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeRegionInstanceGroupManager_InstanceFlexibilityPolicy {
    type O = terrars::BlockAssignable<ComputeRegionInstanceGroupManager_InstanceFlexibilityPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceGroupManager_InstanceFlexibilityPolicy {}

impl BuildComputeRegionInstanceGroupManager_InstanceFlexibilityPolicy {
    pub fn build(self) -> ComputeRegionInstanceGroupManager_InstanceFlexibilityPolicy {
        ComputeRegionInstanceGroupManager_InstanceFlexibilityPolicy {
            instance_selections: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionInstanceGroupManager_InstanceFlexibilityPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionInstanceGroupManager_InstanceFlexibilityPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionInstanceGroupManager_InstanceFlexibilityPolicyRef {
        ComputeRegionInstanceGroupManager_InstanceFlexibilityPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceGroupManager_InstanceFlexibilityPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceGroupManager_InstanceLifecyclePolicy_OnRepair {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_changing_zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionInstanceGroupManager_InstanceLifecyclePolicy_OnRepair {
    #[doc =
        "Set the field `allow_changing_zone`.\nSpecifies whether the MIG can change a VM's zone during a repair. If \"YES\", MIG can select a different zone for the VM during a repair. Else if \"NO\", MIG cannot change a VM's zone during a repair. The default value of allow_changing_zone is \"NO\"."]
    pub fn set_allow_changing_zone(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.allow_changing_zone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionInstanceGroupManager_InstanceLifecyclePolicy_OnRepair {
    type O = terrars::BlockAssignable<ComputeRegionInstanceGroupManager_InstanceLifecyclePolicy_OnRepair>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceGroupManager_InstanceLifecyclePolicy_OnRepair {}

impl BuildComputeRegionInstanceGroupManager_InstanceLifecyclePolicy_OnRepair {
    pub fn build(self) -> ComputeRegionInstanceGroupManager_InstanceLifecyclePolicy_OnRepair {
        ComputeRegionInstanceGroupManager_InstanceLifecyclePolicy_OnRepair {
            allow_changing_zone: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionInstanceGroupManager_InstanceLifecyclePolicy_OnRepairRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionInstanceGroupManager_InstanceLifecyclePolicy_OnRepairRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionInstanceGroupManager_InstanceLifecyclePolicy_OnRepairRef {
        ComputeRegionInstanceGroupManager_InstanceLifecyclePolicy_OnRepairRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceGroupManager_InstanceLifecyclePolicy_OnRepairRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_changing_zone` after provisioning.\nSpecifies whether the MIG can change a VM's zone during a repair. If \"YES\", MIG can select a different zone for the VM during a repair. Else if \"NO\", MIG cannot change a VM's zone during a repair. The default value of allow_changing_zone is \"NO\"."]
    pub fn allow_changing_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_changing_zone", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionInstanceGroupManager_InstanceLifecyclePolicyDynamic {
    on_repair: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionInstanceGroupManager_InstanceLifecyclePolicy_OnRepair>>,
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceGroupManager_InstanceLifecyclePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_action_on_failure: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_update_on_repair: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_failed_health_check: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_repair: std::option::Option<std::vec::Vec<ComputeRegionInstanceGroupManager_InstanceLifecyclePolicy_OnRepair>>,
    dynamic: ComputeRegionInstanceGroupManager_InstanceLifecyclePolicyDynamic,
}

impl ComputeRegionInstanceGroupManager_InstanceLifecyclePolicy {
    #[doc =
        "Set the field `default_action_on_failure`.\nSpecifies the action that a MIG performs on a failed VM. If the value of the \"on_failed_health_check\" field is DEFAULT_ACTION, then the same action also applies to the VMs on which your application fails a health check. Valid values are: REPAIR, DO_NOTHING. If REPAIR (default), then MIG automatically repairs a failed VM by recreating it. For more information, see about repairing VMs in a MIG. If DO_NOTHING, then MIG does not repair a failed VM."]
    pub fn set_default_action_on_failure(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.default_action_on_failure = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `force_update_on_repair`.\nSpecifies whether to apply the group's latest configuration when repairing a VM. Valid options are: YES, NO. If YES and you updated the group's instance template or per-instance configurations after the VM was created, then these changes are applied when VM is repaired. If NO (default), then updates are applied in accordance with the group's update policy type."]
    pub fn set_force_update_on_repair(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.force_update_on_repair = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `on_failed_health_check`.\nSpecifies the action that a MIG performs on an unhealthy VM. A VM is marked as unhealthy when the application running on that VM fails a health check. Valid values are: DEFAULT_ACTION, DO_NOTHING, REPAIR. If DEFAULT_ACTION (default), then MIG uses the same action configured for the  \"default_action_on_failure\" field. If DO_NOTHING, then MIG does not repair unhealthy VM. If REPAIR, then MIG automatically repairs an unhealthy VM by recreating it."]
    pub fn set_on_failed_health_check(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.on_failed_health_check = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `on_repair`.\n"]
    pub fn set_on_repair(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ComputeRegionInstanceGroupManager_InstanceLifecyclePolicy_OnRepair>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.on_repair = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.on_repair = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeRegionInstanceGroupManager_InstanceLifecyclePolicy {
    type O = terrars::BlockAssignable<ComputeRegionInstanceGroupManager_InstanceLifecyclePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceGroupManager_InstanceLifecyclePolicy {}

impl BuildComputeRegionInstanceGroupManager_InstanceLifecyclePolicy {
    pub fn build(self) -> ComputeRegionInstanceGroupManager_InstanceLifecyclePolicy {
        ComputeRegionInstanceGroupManager_InstanceLifecyclePolicy {
            default_action_on_failure: core::default::Default::default(),
            force_update_on_repair: core::default::Default::default(),
            on_failed_health_check: core::default::Default::default(),
            on_repair: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionInstanceGroupManager_InstanceLifecyclePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionInstanceGroupManager_InstanceLifecyclePolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionInstanceGroupManager_InstanceLifecyclePolicyRef {
        ComputeRegionInstanceGroupManager_InstanceLifecyclePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceGroupManager_InstanceLifecyclePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `default_action_on_failure` after provisioning.\nSpecifies the action that a MIG performs on a failed VM. If the value of the \"on_failed_health_check\" field is DEFAULT_ACTION, then the same action also applies to the VMs on which your application fails a health check. Valid values are: REPAIR, DO_NOTHING. If REPAIR (default), then MIG automatically repairs a failed VM by recreating it. For more information, see about repairing VMs in a MIG. If DO_NOTHING, then MIG does not repair a failed VM."]
    pub fn default_action_on_failure(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_action_on_failure", self.base))
    }

    #[doc =
        "Get a reference to the value of field `force_update_on_repair` after provisioning.\nSpecifies whether to apply the group's latest configuration when repairing a VM. Valid options are: YES, NO. If YES and you updated the group's instance template or per-instance configurations after the VM was created, then these changes are applied when VM is repaired. If NO (default), then updates are applied in accordance with the group's update policy type."]
    pub fn force_update_on_repair(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_update_on_repair", self.base))
    }

    #[doc =
        "Get a reference to the value of field `on_failed_health_check` after provisioning.\nSpecifies the action that a MIG performs on an unhealthy VM. A VM is marked as unhealthy when the application running on that VM fails a health check. Valid values are: DEFAULT_ACTION, DO_NOTHING, REPAIR. If DEFAULT_ACTION (default), then MIG uses the same action configured for the  \"default_action_on_failure\" field. If DO_NOTHING, then MIG does not repair unhealthy VM. If REPAIR, then MIG automatically repairs an unhealthy VM by recreating it."]
    pub fn on_failed_health_check(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_failed_health_check", self.base))
    }

    #[doc = "Get a reference to the value of field `on_repair` after provisioning.\n"]
    pub fn on_repair(&self) -> terrars::ListRef<ComputeRegionInstanceGroupManager_InstanceLifecyclePolicy_OnRepairRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.on_repair", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceGroupManager_NamedPort {
    name: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<f64>,
}

impl ComputeRegionInstanceGroupManager_NamedPort { }

impl terrars::ToListMappable for ComputeRegionInstanceGroupManager_NamedPort {
    type O = terrars::BlockAssignable<ComputeRegionInstanceGroupManager_NamedPort>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceGroupManager_NamedPort {
    #[doc = "The name of the port."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The port number."]
    pub port: terrars::PrimField<f64>,
}

impl BuildComputeRegionInstanceGroupManager_NamedPort {
    pub fn build(self) -> ComputeRegionInstanceGroupManager_NamedPort {
        ComputeRegionInstanceGroupManager_NamedPort {
            name: self.name,
            port: self.port,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionInstanceGroupManager_NamedPortRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionInstanceGroupManager_NamedPortRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionInstanceGroupManager_NamedPortRef {
        ComputeRegionInstanceGroupManager_NamedPortRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceGroupManager_NamedPortRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the port."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\nThe port number."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceGroupManager_ResourcePolicies {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workload_policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionInstanceGroupManager_ResourcePolicies {
    #[doc =
        "Set the field `workload_policy`.\nThe URL of the workload policy that is specified for this managed instance group. It can be a full or partial URL."]
    pub fn set_workload_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.workload_policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionInstanceGroupManager_ResourcePolicies {
    type O = terrars::BlockAssignable<ComputeRegionInstanceGroupManager_ResourcePolicies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceGroupManager_ResourcePolicies {}

impl BuildComputeRegionInstanceGroupManager_ResourcePolicies {
    pub fn build(self) -> ComputeRegionInstanceGroupManager_ResourcePolicies {
        ComputeRegionInstanceGroupManager_ResourcePolicies { workload_policy: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionInstanceGroupManager_ResourcePoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionInstanceGroupManager_ResourcePoliciesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionInstanceGroupManager_ResourcePoliciesRef {
        ComputeRegionInstanceGroupManager_ResourcePoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceGroupManager_ResourcePoliciesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `workload_policy` after provisioning.\nThe URL of the workload policy that is specified for this managed instance group. It can be a full or partial URL."]
    pub fn workload_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workload_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceGroupManager_StandbyPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_delay_sec: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionInstanceGroupManager_StandbyPolicy {
    #[doc =
        "Set the field `initial_delay_sec`.\nSpecifies the number of seconds that the MIG should wait to suspend or stop a VM after that VM was created. The initial delay gives the initialization script the time to prepare your VM for a quick scale out. The value of initial delay must be between 0 and 3600 seconds. The default value is 0."]
    pub fn set_initial_delay_sec(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.initial_delay_sec = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `mode`.\nDefines how a MIG resumes or starts VMs from a standby pool when the group scales out. The default mode is \"MANUAL\"."]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionInstanceGroupManager_StandbyPolicy {
    type O = terrars::BlockAssignable<ComputeRegionInstanceGroupManager_StandbyPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceGroupManager_StandbyPolicy {}

impl BuildComputeRegionInstanceGroupManager_StandbyPolicy {
    pub fn build(self) -> ComputeRegionInstanceGroupManager_StandbyPolicy {
        ComputeRegionInstanceGroupManager_StandbyPolicy {
            initial_delay_sec: core::default::Default::default(),
            mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionInstanceGroupManager_StandbyPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionInstanceGroupManager_StandbyPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionInstanceGroupManager_StandbyPolicyRef {
        ComputeRegionInstanceGroupManager_StandbyPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceGroupManager_StandbyPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `initial_delay_sec` after provisioning.\nSpecifies the number of seconds that the MIG should wait to suspend or stop a VM after that VM was created. The initial delay gives the initialization script the time to prepare your VM for a quick scale out. The value of initial delay must be between 0 and 3600 seconds. The default value is 0."]
    pub fn initial_delay_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_delay_sec", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nDefines how a MIG resumes or starts VMs from a standby pool when the group scales out. The default mode is \"MANUAL\"."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceGroupManager_StatefulDisk {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_rule: std::option::Option<terrars::PrimField<std::string::String>>,
    device_name: terrars::PrimField<std::string::String>,
}

impl ComputeRegionInstanceGroupManager_StatefulDisk {
    #[doc =
        "Set the field `delete_rule`.\nA value that prescribes what should happen to the stateful disk when the VM instance is deleted. The available options are NEVER and ON_PERMANENT_INSTANCE_DELETION. NEVER - detach the disk when the VM is deleted, but do not delete the disk. ON_PERMANENT_INSTANCE_DELETION will delete the stateful disk when the VM is permanently deleted from the instance group. The default is NEVER."]
    pub fn set_delete_rule(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete_rule = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionInstanceGroupManager_StatefulDisk {
    type O = terrars::BlockAssignable<ComputeRegionInstanceGroupManager_StatefulDisk>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceGroupManager_StatefulDisk {
    #[doc = "The device name of the disk to be attached."]
    pub device_name: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionInstanceGroupManager_StatefulDisk {
    pub fn build(self) -> ComputeRegionInstanceGroupManager_StatefulDisk {
        ComputeRegionInstanceGroupManager_StatefulDisk {
            delete_rule: core::default::Default::default(),
            device_name: self.device_name,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionInstanceGroupManager_StatefulDiskRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionInstanceGroupManager_StatefulDiskRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionInstanceGroupManager_StatefulDiskRef {
        ComputeRegionInstanceGroupManager_StatefulDiskRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceGroupManager_StatefulDiskRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `delete_rule` after provisioning.\nA value that prescribes what should happen to the stateful disk when the VM instance is deleted. The available options are NEVER and ON_PERMANENT_INSTANCE_DELETION. NEVER - detach the disk when the VM is deleted, but do not delete the disk. ON_PERMANENT_INSTANCE_DELETION will delete the stateful disk when the VM is permanently deleted from the instance group. The default is NEVER."]
    pub fn delete_rule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_rule", self.base))
    }

    #[doc =
        "Get a reference to the value of field `device_name` after provisioning.\nThe device name of the disk to be attached."]
    pub fn device_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceGroupManager_StatefulExternalIp {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_rule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interface_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionInstanceGroupManager_StatefulExternalIp {
    #[doc =
        "Set the field `delete_rule`.\nA value that prescribes what should happen to an associated static Address resource when a VM instance is permanently deleted. The available options are NEVER and ON_PERMANENT_INSTANCE_DELETION. NEVER - detach the IP when the VM is deleted, but do not delete the address resource. ON_PERMANENT_INSTANCE_DELETION will delete the stateful address when the VM is permanently deleted from the instance group. The default is NEVER."]
    pub fn set_delete_rule(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete_rule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interface_name`.\nThe network interface name"]
    pub fn set_interface_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.interface_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionInstanceGroupManager_StatefulExternalIp {
    type O = terrars::BlockAssignable<ComputeRegionInstanceGroupManager_StatefulExternalIp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceGroupManager_StatefulExternalIp {}

impl BuildComputeRegionInstanceGroupManager_StatefulExternalIp {
    pub fn build(self) -> ComputeRegionInstanceGroupManager_StatefulExternalIp {
        ComputeRegionInstanceGroupManager_StatefulExternalIp {
            delete_rule: core::default::Default::default(),
            interface_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionInstanceGroupManager_StatefulExternalIpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionInstanceGroupManager_StatefulExternalIpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionInstanceGroupManager_StatefulExternalIpRef {
        ComputeRegionInstanceGroupManager_StatefulExternalIpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceGroupManager_StatefulExternalIpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `delete_rule` after provisioning.\nA value that prescribes what should happen to an associated static Address resource when a VM instance is permanently deleted. The available options are NEVER and ON_PERMANENT_INSTANCE_DELETION. NEVER - detach the IP when the VM is deleted, but do not delete the address resource. ON_PERMANENT_INSTANCE_DELETION will delete the stateful address when the VM is permanently deleted from the instance group. The default is NEVER."]
    pub fn delete_rule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `interface_name` after provisioning.\nThe network interface name"]
    pub fn interface_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interface_name", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceGroupManager_StatefulInternalIp {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_rule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interface_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionInstanceGroupManager_StatefulInternalIp {
    #[doc =
        "Set the field `delete_rule`.\nA value that prescribes what should happen to an associated static Address resource when a VM instance is permanently deleted. The available options are NEVER and ON_PERMANENT_INSTANCE_DELETION. NEVER - detach the IP when the VM is deleted, but do not delete the address resource. ON_PERMANENT_INSTANCE_DELETION will delete the stateful address when the VM is permanently deleted from the instance group. The default is NEVER."]
    pub fn set_delete_rule(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete_rule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interface_name`.\nThe network interface name"]
    pub fn set_interface_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.interface_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionInstanceGroupManager_StatefulInternalIp {
    type O = terrars::BlockAssignable<ComputeRegionInstanceGroupManager_StatefulInternalIp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceGroupManager_StatefulInternalIp {}

impl BuildComputeRegionInstanceGroupManager_StatefulInternalIp {
    pub fn build(self) -> ComputeRegionInstanceGroupManager_StatefulInternalIp {
        ComputeRegionInstanceGroupManager_StatefulInternalIp {
            delete_rule: core::default::Default::default(),
            interface_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionInstanceGroupManager_StatefulInternalIpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionInstanceGroupManager_StatefulInternalIpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionInstanceGroupManager_StatefulInternalIpRef {
        ComputeRegionInstanceGroupManager_StatefulInternalIpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceGroupManager_StatefulInternalIpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `delete_rule` after provisioning.\nA value that prescribes what should happen to an associated static Address resource when a VM instance is permanently deleted. The available options are NEVER and ON_PERMANENT_INSTANCE_DELETION. NEVER - detach the IP when the VM is deleted, but do not delete the address resource. ON_PERMANENT_INSTANCE_DELETION will delete the stateful address when the VM is permanently deleted from the instance group. The default is NEVER."]
    pub fn delete_rule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `interface_name` after provisioning.\nThe network interface name"]
    pub fn interface_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interface_name", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceGroupManager_TargetSizePolicy {
    mode: terrars::PrimField<std::string::String>,
}

impl ComputeRegionInstanceGroupManager_TargetSizePolicy { }

impl terrars::ToListMappable for ComputeRegionInstanceGroupManager_TargetSizePolicy {
    type O = terrars::BlockAssignable<ComputeRegionInstanceGroupManager_TargetSizePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceGroupManager_TargetSizePolicy {
    #[doc = "The mode of target size policy based on which the MIG creates its VMs individually or all at once."]
    pub mode: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionInstanceGroupManager_TargetSizePolicy {
    pub fn build(self) -> ComputeRegionInstanceGroupManager_TargetSizePolicy {
        ComputeRegionInstanceGroupManager_TargetSizePolicy { mode: self.mode }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionInstanceGroupManager_TargetSizePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionInstanceGroupManager_TargetSizePolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionInstanceGroupManager_TargetSizePolicyRef {
        ComputeRegionInstanceGroupManager_TargetSizePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceGroupManager_TargetSizePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nThe mode of target size policy based on which the MIG creates its VMs individually or all at once."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceGroupManager_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionInstanceGroupManager_Timeouts {
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

impl terrars::ToListMappable for ComputeRegionInstanceGroupManager_Timeouts {
    type O = terrars::BlockAssignable<ComputeRegionInstanceGroupManager_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceGroupManager_Timeouts {}

impl BuildComputeRegionInstanceGroupManager_Timeouts {
    pub fn build(self) -> ComputeRegionInstanceGroupManager_Timeouts {
        ComputeRegionInstanceGroupManager_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionInstanceGroupManager_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionInstanceGroupManager_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRegionInstanceGroupManager_TimeoutsRef {
        ComputeRegionInstanceGroupManager_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceGroupManager_TimeoutsRef {
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
pub struct ComputeRegionInstanceGroupManager_UpdatePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_redistribution_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_surge_fixed: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_surge_percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_unavailable_fixed: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_unavailable_percent: std::option::Option<terrars::PrimField<f64>>,
    minimal_action: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    most_disruptive_allowed_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replacement_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl ComputeRegionInstanceGroupManager_UpdatePolicy {
    #[doc =
        "Set the field `instance_redistribution_type`.\nThe instance redistribution policy for regional managed instance groups. Valid values are: \"PROACTIVE\", \"NONE\". If PROACTIVE (default), the group attempts to maintain an even distribution of VM instances across zones in the region. If NONE, proactive redistribution is disabled."]
    pub fn set_instance_redistribution_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.instance_redistribution_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_surge_fixed`.\nSpecifies a fixed number of VM instances. This must be a positive integer. Conflicts with max_surge_percent. Both cannot be 0"]
    pub fn set_max_surge_fixed(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_surge_fixed = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_surge_percent`.\nSpecifies a percentage of instances between 0 to 100%, inclusive. For example, specify 80 for 80%. Conflicts with max_surge_fixed."]
    pub fn set_max_surge_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_surge_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_unavailable_fixed`.\nSpecifies a fixed number of VM instances. This must be a positive integer."]
    pub fn set_max_unavailable_fixed(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_unavailable_fixed = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_unavailable_percent`.\nSpecifies a percentage of instances between 0 to 100%, inclusive. For example, specify 80 for 80%."]
    pub fn set_max_unavailable_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_unavailable_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `most_disruptive_allowed_action`.\nMost disruptive action that is allowed to be taken on an instance. You can specify either NONE to forbid any actions, REFRESH to allow actions that do not need instance restart, RESTART to allow actions that can be applied without instance replacing or REPLACE to allow all possible actions. If the Updater determines that the minimal update action needed is more disruptive than most disruptive allowed action you specify it will not perform the update at all."]
    pub fn set_most_disruptive_allowed_action(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.most_disruptive_allowed_action = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `replacement_method`.\nThe instance replacement method for regional managed instance groups. Valid values are: \"RECREATE\", \"SUBSTITUTE\". If SUBSTITUTE (default), the group replaces VM instances with new instances that have randomly generated names. If RECREATE, instance names are preserved.  You must also set max_unavailable_fixed or max_unavailable_percent to be greater than 0."]
    pub fn set_replacement_method(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.replacement_method = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionInstanceGroupManager_UpdatePolicy {
    type O = terrars::BlockAssignable<ComputeRegionInstanceGroupManager_UpdatePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceGroupManager_UpdatePolicy {
    #[doc =
        "Minimal action to be taken on an instance. You can specify either NONE to forbid any actions, REFRESH to update without stopping instances, RESTART to restart existing instances or REPLACE to delete and create new instances from the target template. If you specify a REFRESH, the Updater will attempt to perform that action only. However, if the Updater determines that the minimal action you specify is not enough to perform the update, it might perform a more disruptive action."]
    pub minimal_action: terrars::PrimField<std::string::String>,
    #[doc =
        "The type of update process. You can specify either PROACTIVE so that the instance group manager proactively executes actions in order to bring instances to their target versions or OPPORTUNISTIC so that no action is proactively executed but the update will be performed as part of other actions (for example, resizes or recreateInstances calls)."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionInstanceGroupManager_UpdatePolicy {
    pub fn build(self) -> ComputeRegionInstanceGroupManager_UpdatePolicy {
        ComputeRegionInstanceGroupManager_UpdatePolicy {
            instance_redistribution_type: core::default::Default::default(),
            max_surge_fixed: core::default::Default::default(),
            max_surge_percent: core::default::Default::default(),
            max_unavailable_fixed: core::default::Default::default(),
            max_unavailable_percent: core::default::Default::default(),
            minimal_action: self.minimal_action,
            most_disruptive_allowed_action: core::default::Default::default(),
            replacement_method: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionInstanceGroupManager_UpdatePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionInstanceGroupManager_UpdatePolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionInstanceGroupManager_UpdatePolicyRef {
        ComputeRegionInstanceGroupManager_UpdatePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceGroupManager_UpdatePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `instance_redistribution_type` after provisioning.\nThe instance redistribution policy for regional managed instance groups. Valid values are: \"PROACTIVE\", \"NONE\". If PROACTIVE (default), the group attempts to maintain an even distribution of VM instances across zones in the region. If NONE, proactive redistribution is disabled."]
    pub fn instance_redistribution_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_redistribution_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_surge_fixed` after provisioning.\nSpecifies a fixed number of VM instances. This must be a positive integer. Conflicts with max_surge_percent. Both cannot be 0"]
    pub fn max_surge_fixed(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_surge_fixed", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_surge_percent` after provisioning.\nSpecifies a percentage of instances between 0 to 100%, inclusive. For example, specify 80 for 80%. Conflicts with max_surge_fixed."]
    pub fn max_surge_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_surge_percent", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_unavailable_fixed` after provisioning.\nSpecifies a fixed number of VM instances. This must be a positive integer."]
    pub fn max_unavailable_fixed(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_unavailable_fixed", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_unavailable_percent` after provisioning.\nSpecifies a percentage of instances between 0 to 100%, inclusive. For example, specify 80 for 80%."]
    pub fn max_unavailable_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_unavailable_percent", self.base))
    }

    #[doc =
        "Get a reference to the value of field `minimal_action` after provisioning.\nMinimal action to be taken on an instance. You can specify either NONE to forbid any actions, REFRESH to update without stopping instances, RESTART to restart existing instances or REPLACE to delete and create new instances from the target template. If you specify a REFRESH, the Updater will attempt to perform that action only. However, if the Updater determines that the minimal action you specify is not enough to perform the update, it might perform a more disruptive action."]
    pub fn minimal_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimal_action", self.base))
    }

    #[doc =
        "Get a reference to the value of field `most_disruptive_allowed_action` after provisioning.\nMost disruptive action that is allowed to be taken on an instance. You can specify either NONE to forbid any actions, REFRESH to allow actions that do not need instance restart, RESTART to allow actions that can be applied without instance replacing or REPLACE to allow all possible actions. If the Updater determines that the minimal update action needed is more disruptive than most disruptive allowed action you specify it will not perform the update at all."]
    pub fn most_disruptive_allowed_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.most_disruptive_allowed_action", self.base))
    }

    #[doc =
        "Get a reference to the value of field `replacement_method` after provisioning.\nThe instance replacement method for regional managed instance groups. Valid values are: \"RECREATE\", \"SUBSTITUTE\". If SUBSTITUTE (default), the group replaces VM instances with new instances that have randomly generated names. If RECREATE, instance names are preserved.  You must also set max_unavailable_fixed or max_unavailable_percent to be greater than 0."]
    pub fn replacement_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replacement_method", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of update process. You can specify either PROACTIVE so that the instance group manager proactively executes actions in order to bring instances to their target versions or OPPORTUNISTIC so that no action is proactively executed but the update will be performed as part of other actions (for example, resizes or recreateInstances calls)."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceGroupManager_Version_TargetSize {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percent: std::option::Option<terrars::PrimField<f64>>,
}

impl ComputeRegionInstanceGroupManager_Version_TargetSize {
    #[doc =
        "Set the field `fixed`.\nThe number of instances which are managed for this version. Conflicts with percent."]
    pub fn set_fixed(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.fixed = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `percent`.\nThe number of instances (calculated as percentage) which are managed for this version. Conflicts with fixed. Note that when using percent, rounding will be in favor of explicitly set target_size values; a managed instance group with 2 instances and 2 versions, one of which has a target_size.percent of 60 will create 2 instances of that version."]
    pub fn set_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.percent = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionInstanceGroupManager_Version_TargetSize {
    type O = terrars::BlockAssignable<ComputeRegionInstanceGroupManager_Version_TargetSize>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceGroupManager_Version_TargetSize {}

impl BuildComputeRegionInstanceGroupManager_Version_TargetSize {
    pub fn build(self) -> ComputeRegionInstanceGroupManager_Version_TargetSize {
        ComputeRegionInstanceGroupManager_Version_TargetSize {
            fixed: core::default::Default::default(),
            percent: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionInstanceGroupManager_Version_TargetSizeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionInstanceGroupManager_Version_TargetSizeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionInstanceGroupManager_Version_TargetSizeRef {
        ComputeRegionInstanceGroupManager_Version_TargetSizeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceGroupManager_Version_TargetSizeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `fixed` after provisioning.\nThe number of instances which are managed for this version. Conflicts with percent."]
    pub fn fixed(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fixed", self.base))
    }

    #[doc =
        "Get a reference to the value of field `percent` after provisioning.\nThe number of instances (calculated as percentage) which are managed for this version. Conflicts with fixed. Note that when using percent, rounding will be in favor of explicitly set target_size values; a managed instance group with 2 instances and 2 versions, one of which has a target_size.percent of 60 will create 2 instances of that version."]
    pub fn percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percent", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionInstanceGroupManager_VersionDynamic {
    target_size: std::option::Option<terrars::DynamicBlock<ComputeRegionInstanceGroupManager_Version_TargetSize>>,
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceGroupManager_Version {
    instance_template: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_size: std::option::Option<std::vec::Vec<ComputeRegionInstanceGroupManager_Version_TargetSize>>,
    dynamic: ComputeRegionInstanceGroupManager_VersionDynamic,
}

impl ComputeRegionInstanceGroupManager_Version {
    #[doc = "Set the field `name`.\nVersion name."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_size`.\n"]
    pub fn set_target_size(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionInstanceGroupManager_Version_TargetSize>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.target_size = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.target_size = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeRegionInstanceGroupManager_Version {
    type O = terrars::BlockAssignable<ComputeRegionInstanceGroupManager_Version>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceGroupManager_Version {
    #[doc = "The full URL to an instance template from which all new instances of this version will be created."]
    pub instance_template: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionInstanceGroupManager_Version {
    pub fn build(self) -> ComputeRegionInstanceGroupManager_Version {
        ComputeRegionInstanceGroupManager_Version {
            instance_template: self.instance_template,
            name: core::default::Default::default(),
            target_size: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeRegionInstanceGroupManager_VersionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionInstanceGroupManager_VersionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRegionInstanceGroupManager_VersionRef {
        ComputeRegionInstanceGroupManager_VersionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceGroupManager_VersionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `instance_template` after provisioning.\nThe full URL to an instance template from which all new instances of this version will be created."]
    pub fn instance_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_template", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nVersion name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `target_size` after provisioning.\n"]
    pub fn target_size(&self) -> terrars::ListRef<ComputeRegionInstanceGroupManager_Version_TargetSizeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_size", self.base))
    }
}
