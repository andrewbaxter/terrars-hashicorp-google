#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeInstanceGroupManagerData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    list_managed_instances_results: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
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
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all_instances_config: std::option::Option<std::vec::Vec<ComputeInstanceGroupManager_AllInstancesConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_healing_policies: std::option::Option<std::vec::Vec<ComputeInstanceGroupManager_AutoHealingPolicies>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_lifecycle_policy: std::option::Option<std::vec::Vec<ComputeInstanceGroupManager_InstanceLifecyclePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    named_port: std::option::Option<std::vec::Vec<ComputeInstanceGroupManager_NamedPort>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_policies: std::option::Option<std::vec::Vec<ComputeInstanceGroupManager_ResourcePolicies>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    standby_policy: std::option::Option<std::vec::Vec<ComputeInstanceGroupManager_StandbyPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateful_disk: std::option::Option<std::vec::Vec<ComputeInstanceGroupManager_StatefulDisk>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateful_external_ip: std::option::Option<std::vec::Vec<ComputeInstanceGroupManager_StatefulExternalIp>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateful_internal_ip: std::option::Option<std::vec::Vec<ComputeInstanceGroupManager_StatefulInternalIp>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_size_policy: std::option::Option<std::vec::Vec<ComputeInstanceGroupManager_TargetSizePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeInstanceGroupManager_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_policy: std::option::Option<std::vec::Vec<ComputeInstanceGroupManager_UpdatePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<std::vec::Vec<ComputeInstanceGroupManager_Version>>,
    dynamic: ComputeInstanceGroupManagerDynamic,
}

struct ComputeInstanceGroupManager_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeInstanceGroupManagerData_>,
}

#[derive(Clone)]
pub struct ComputeInstanceGroupManager(std::rc::Rc<ComputeInstanceGroupManager_>);

impl ComputeInstanceGroupManager {
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

    #[doc = "Set the field `zone`.\nThe zone that instances in this group should be created in."]
    pub fn set_zone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `all_instances_config`.\n"]
    pub fn set_all_instances_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstanceGroupManager_AllInstancesConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstanceGroupManager_AutoHealingPolicies>>,
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

    #[doc = "Set the field `instance_lifecycle_policy`.\n"]
    pub fn set_instance_lifecycle_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstanceGroupManager_InstanceLifecyclePolicy>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstanceGroupManager_NamedPort>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstanceGroupManager_ResourcePolicies>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstanceGroupManager_StandbyPolicy>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstanceGroupManager_StatefulDisk>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstanceGroupManager_StatefulExternalIp>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstanceGroupManager_StatefulInternalIp>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstanceGroupManager_TargetSizePolicy>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeInstanceGroupManager_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_policy`.\n"]
    pub fn set_update_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstanceGroupManager_UpdatePolicy>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstanceGroupManager_Version>>,
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

    #[doc = "Get a reference to the value of field `operation` after provisioning.\n"]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URL of the created resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe status of this managed instance group."]
    pub fn status(&self) -> terrars::ListRef<ComputeInstanceGroupManager_StatusRef> {
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

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nThe zone that instances in this group should be created in."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `all_instances_config` after provisioning.\n"]
    pub fn all_instances_config(&self) -> terrars::ListRef<ComputeInstanceGroupManager_AllInstancesConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.all_instances_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_healing_policies` after provisioning.\n"]
    pub fn auto_healing_policies(&self) -> terrars::ListRef<ComputeInstanceGroupManager_AutoHealingPoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_healing_policies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_lifecycle_policy` after provisioning.\n"]
    pub fn instance_lifecycle_policy(&self) -> terrars::ListRef<ComputeInstanceGroupManager_InstanceLifecyclePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_lifecycle_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_policies` after provisioning.\n"]
    pub fn resource_policies(&self) -> terrars::ListRef<ComputeInstanceGroupManager_ResourcePoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_policies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `standby_policy` after provisioning.\n"]
    pub fn standby_policy(&self) -> terrars::ListRef<ComputeInstanceGroupManager_StandbyPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.standby_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stateful_external_ip` after provisioning.\n"]
    pub fn stateful_external_ip(&self) -> terrars::ListRef<ComputeInstanceGroupManager_StatefulExternalIpRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateful_external_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stateful_internal_ip` after provisioning.\n"]
    pub fn stateful_internal_ip(&self) -> terrars::ListRef<ComputeInstanceGroupManager_StatefulInternalIpRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateful_internal_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_size_policy` after provisioning.\n"]
    pub fn target_size_policy(&self) -> terrars::ListRef<ComputeInstanceGroupManager_TargetSizePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_size_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeInstanceGroupManager_TimeoutsRef {
        ComputeInstanceGroupManager_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `update_policy` after provisioning.\n"]
    pub fn update_policy(&self) -> terrars::ListRef<ComputeInstanceGroupManager_UpdatePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.update_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::ListRef<ComputeInstanceGroupManager_VersionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

impl terrars::Referable for ComputeInstanceGroupManager {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputeInstanceGroupManager { }

impl terrars::ToListMappable for ComputeInstanceGroupManager {
    type O = terrars::ListRef<ComputeInstanceGroupManagerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputeInstanceGroupManager_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_compute_instance_group_manager".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeInstanceGroupManager {
    pub tf_id: std::string::String,
    #[doc =
        "The base instance name to use for instances in this group. The value must be a valid RFC1035 name. Supported characters are lowercase letters, numbers, and hyphens (-). Instances are named by appending a hyphen and a random four-character string to the base instance name."]
    pub base_instance_name: terrars::PrimField<std::string::String>,
    #[doc =
        "The name of the instance group manager. Must be 1-63 characters long and comply with RFC1035. Supported characters include lowercase letters, numbers, and hyphens."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeInstanceGroupManager {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputeInstanceGroupManager {
        let out = ComputeInstanceGroupManager(std::rc::Rc::new(ComputeInstanceGroupManager_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputeInstanceGroupManagerData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                base_instance_name: self.base_instance_name,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                list_managed_instances_results: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                target_pools: core::default::Default::default(),
                target_size: core::default::Default::default(),
                target_stopped_size: core::default::Default::default(),
                target_suspended_size: core::default::Default::default(),
                wait_for_instances: core::default::Default::default(),
                wait_for_instances_status: core::default::Default::default(),
                zone: core::default::Default::default(),
                all_instances_config: core::default::Default::default(),
                auto_healing_policies: core::default::Default::default(),
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

pub struct ComputeInstanceGroupManagerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceGroupManagerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeInstanceGroupManagerRef {
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

    #[doc = "Get a reference to the value of field `operation` after provisioning.\n"]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URL of the created resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe status of this managed instance group."]
    pub fn status(&self) -> terrars::ListRef<ComputeInstanceGroupManager_StatusRef> {
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

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nThe zone that instances in this group should be created in."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `all_instances_config` after provisioning.\n"]
    pub fn all_instances_config(&self) -> terrars::ListRef<ComputeInstanceGroupManager_AllInstancesConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.all_instances_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_healing_policies` after provisioning.\n"]
    pub fn auto_healing_policies(&self) -> terrars::ListRef<ComputeInstanceGroupManager_AutoHealingPoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_healing_policies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_lifecycle_policy` after provisioning.\n"]
    pub fn instance_lifecycle_policy(&self) -> terrars::ListRef<ComputeInstanceGroupManager_InstanceLifecyclePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_lifecycle_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_policies` after provisioning.\n"]
    pub fn resource_policies(&self) -> terrars::ListRef<ComputeInstanceGroupManager_ResourcePoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_policies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `standby_policy` after provisioning.\n"]
    pub fn standby_policy(&self) -> terrars::ListRef<ComputeInstanceGroupManager_StandbyPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.standby_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stateful_external_ip` after provisioning.\n"]
    pub fn stateful_external_ip(&self) -> terrars::ListRef<ComputeInstanceGroupManager_StatefulExternalIpRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateful_external_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stateful_internal_ip` after provisioning.\n"]
    pub fn stateful_internal_ip(&self) -> terrars::ListRef<ComputeInstanceGroupManager_StatefulInternalIpRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateful_internal_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_size_policy` after provisioning.\n"]
    pub fn target_size_policy(&self) -> terrars::ListRef<ComputeInstanceGroupManager_TargetSizePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_size_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeInstanceGroupManager_TimeoutsRef {
        ComputeInstanceGroupManager_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `update_policy` after provisioning.\n"]
    pub fn update_policy(&self) -> terrars::ListRef<ComputeInstanceGroupManager_UpdatePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.update_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::ListRef<ComputeInstanceGroupManager_VersionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ComputeInstanceGroupManagerDynamic {
    all_instances_config: std::option::Option<terrars::DynamicBlock<ComputeInstanceGroupManager_AllInstancesConfig>>,
    auto_healing_policies: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeInstanceGroupManager_AutoHealingPolicies>>,
    instance_lifecycle_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeInstanceGroupManager_InstanceLifecyclePolicy>>,
    named_port: std::option::Option<terrars::DynamicBlock<ComputeInstanceGroupManager_NamedPort>>,
    resource_policies: std::option::Option<terrars::DynamicBlock<ComputeInstanceGroupManager_ResourcePolicies>>,
    standby_policy: std::option::Option<terrars::DynamicBlock<ComputeInstanceGroupManager_StandbyPolicy>>,
    stateful_disk: std::option::Option<terrars::DynamicBlock<ComputeInstanceGroupManager_StatefulDisk>>,
    stateful_external_ip: std::option::Option<terrars::DynamicBlock<ComputeInstanceGroupManager_StatefulExternalIp>>,
    stateful_internal_ip: std::option::Option<terrars::DynamicBlock<ComputeInstanceGroupManager_StatefulInternalIp>>,
    target_size_policy: std::option::Option<terrars::DynamicBlock<ComputeInstanceGroupManager_TargetSizePolicy>>,
    update_policy: std::option::Option<terrars::DynamicBlock<ComputeInstanceGroupManager_UpdatePolicy>>,
    version: std::option::Option<terrars::DynamicBlock<ComputeInstanceGroupManager_Version>>,
}

#[derive(Serialize)]
pub struct ComputeInstanceGroupManager_Status_AllInstancesConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    current_revision: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effective: std::option::Option<terrars::PrimField<bool>>,
}

impl ComputeInstanceGroupManager_Status_AllInstancesConfig {
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

impl terrars::ToListMappable for ComputeInstanceGroupManager_Status_AllInstancesConfig {
    type O = terrars::BlockAssignable<ComputeInstanceGroupManager_Status_AllInstancesConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceGroupManager_Status_AllInstancesConfig {}

impl BuildComputeInstanceGroupManager_Status_AllInstancesConfig {
    pub fn build(self) -> ComputeInstanceGroupManager_Status_AllInstancesConfig {
        ComputeInstanceGroupManager_Status_AllInstancesConfig {
            current_revision: core::default::Default::default(),
            effective: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstanceGroupManager_Status_AllInstancesConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceGroupManager_Status_AllInstancesConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstanceGroupManager_Status_AllInstancesConfigRef {
        ComputeInstanceGroupManager_Status_AllInstancesConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceGroupManager_Status_AllInstancesConfigRef {
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
pub struct ComputeInstanceGroupManager_Status_Stateful_PerInstanceConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all_effective: std::option::Option<terrars::PrimField<bool>>,
}

impl ComputeInstanceGroupManager_Status_Stateful_PerInstanceConfigs {
    #[doc = "Set the field `all_effective`.\n"]
    pub fn set_all_effective(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.all_effective = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInstanceGroupManager_Status_Stateful_PerInstanceConfigs {
    type O = terrars::BlockAssignable<ComputeInstanceGroupManager_Status_Stateful_PerInstanceConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceGroupManager_Status_Stateful_PerInstanceConfigs {}

impl BuildComputeInstanceGroupManager_Status_Stateful_PerInstanceConfigs {
    pub fn build(self) -> ComputeInstanceGroupManager_Status_Stateful_PerInstanceConfigs {
        ComputeInstanceGroupManager_Status_Stateful_PerInstanceConfigs {
            all_effective: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstanceGroupManager_Status_Stateful_PerInstanceConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceGroupManager_Status_Stateful_PerInstanceConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstanceGroupManager_Status_Stateful_PerInstanceConfigsRef {
        ComputeInstanceGroupManager_Status_Stateful_PerInstanceConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceGroupManager_Status_Stateful_PerInstanceConfigsRef {
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
pub struct ComputeInstanceGroupManager_Status_Stateful {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    has_stateful_config: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_instance_configs: std
    ::option
    ::Option<terrars::ListField<ComputeInstanceGroupManager_Status_Stateful_PerInstanceConfigs>>,
}

impl ComputeInstanceGroupManager_Status_Stateful {
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
                    ::Into<terrars::ListField<ComputeInstanceGroupManager_Status_Stateful_PerInstanceConfigs>>,
    ) -> Self {
        self.per_instance_configs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInstanceGroupManager_Status_Stateful {
    type O = terrars::BlockAssignable<ComputeInstanceGroupManager_Status_Stateful>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceGroupManager_Status_Stateful {}

impl BuildComputeInstanceGroupManager_Status_Stateful {
    pub fn build(self) -> ComputeInstanceGroupManager_Status_Stateful {
        ComputeInstanceGroupManager_Status_Stateful {
            has_stateful_config: core::default::Default::default(),
            per_instance_configs: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstanceGroupManager_Status_StatefulRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceGroupManager_Status_StatefulRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstanceGroupManager_Status_StatefulRef {
        ComputeInstanceGroupManager_Status_StatefulRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceGroupManager_Status_StatefulRef {
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
    ) -> terrars::ListRef<ComputeInstanceGroupManager_Status_Stateful_PerInstanceConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.per_instance_configs", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInstanceGroupManager_Status_VersionTarget {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_reached: std::option::Option<terrars::PrimField<bool>>,
}

impl ComputeInstanceGroupManager_Status_VersionTarget {
    #[doc = "Set the field `is_reached`.\n"]
    pub fn set_is_reached(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_reached = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInstanceGroupManager_Status_VersionTarget {
    type O = terrars::BlockAssignable<ComputeInstanceGroupManager_Status_VersionTarget>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceGroupManager_Status_VersionTarget {}

impl BuildComputeInstanceGroupManager_Status_VersionTarget {
    pub fn build(self) -> ComputeInstanceGroupManager_Status_VersionTarget {
        ComputeInstanceGroupManager_Status_VersionTarget { is_reached: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ComputeInstanceGroupManager_Status_VersionTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceGroupManager_Status_VersionTargetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstanceGroupManager_Status_VersionTargetRef {
        ComputeInstanceGroupManager_Status_VersionTargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceGroupManager_Status_VersionTargetRef {
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
pub struct ComputeInstanceGroupManager_Status {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all_instances_config: std::option::Option<terrars::ListField<ComputeInstanceGroupManager_Status_AllInstancesConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_stable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateful: std::option::Option<terrars::ListField<ComputeInstanceGroupManager_Status_Stateful>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version_target: std::option::Option<terrars::ListField<ComputeInstanceGroupManager_Status_VersionTarget>>,
}

impl ComputeInstanceGroupManager_Status {
    #[doc = "Set the field `all_instances_config`.\n"]
    pub fn set_all_instances_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ComputeInstanceGroupManager_Status_AllInstancesConfig>>,
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
        v: impl std::convert::Into<terrars::ListField<ComputeInstanceGroupManager_Status_Stateful>>,
    ) -> Self {
        self.stateful = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version_target`.\n"]
    pub fn set_version_target(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ComputeInstanceGroupManager_Status_VersionTarget>>,
    ) -> Self {
        self.version_target = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInstanceGroupManager_Status {
    type O = terrars::BlockAssignable<ComputeInstanceGroupManager_Status>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceGroupManager_Status {}

impl BuildComputeInstanceGroupManager_Status {
    pub fn build(self) -> ComputeInstanceGroupManager_Status {
        ComputeInstanceGroupManager_Status {
            all_instances_config: core::default::Default::default(),
            is_stable: core::default::Default::default(),
            stateful: core::default::Default::default(),
            version_target: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstanceGroupManager_StatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceGroupManager_StatusRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInstanceGroupManager_StatusRef {
        ComputeInstanceGroupManager_StatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceGroupManager_StatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `all_instances_config` after provisioning.\n"]
    pub fn all_instances_config(&self) -> terrars::ListRef<ComputeInstanceGroupManager_Status_AllInstancesConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.all_instances_config", self.base))
    }

    #[doc = "Get a reference to the value of field `is_stable` after provisioning.\n"]
    pub fn is_stable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_stable", self.base))
    }

    #[doc = "Get a reference to the value of field `stateful` after provisioning.\n"]
    pub fn stateful(&self) -> terrars::ListRef<ComputeInstanceGroupManager_Status_StatefulRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateful", self.base))
    }

    #[doc = "Get a reference to the value of field `version_target` after provisioning.\n"]
    pub fn version_target(&self) -> terrars::ListRef<ComputeInstanceGroupManager_Status_VersionTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.version_target", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInstanceGroupManager_AllInstancesConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl ComputeInstanceGroupManager_AllInstancesConfig {
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

impl terrars::ToListMappable for ComputeInstanceGroupManager_AllInstancesConfig {
    type O = terrars::BlockAssignable<ComputeInstanceGroupManager_AllInstancesConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceGroupManager_AllInstancesConfig {}

impl BuildComputeInstanceGroupManager_AllInstancesConfig {
    pub fn build(self) -> ComputeInstanceGroupManager_AllInstancesConfig {
        ComputeInstanceGroupManager_AllInstancesConfig {
            labels: core::default::Default::default(),
            metadata: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstanceGroupManager_AllInstancesConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceGroupManager_AllInstancesConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstanceGroupManager_AllInstancesConfigRef {
        ComputeInstanceGroupManager_AllInstancesConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceGroupManager_AllInstancesConfigRef {
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
pub struct ComputeInstanceGroupManager_AutoHealingPolicies {
    health_check: terrars::PrimField<std::string::String>,
    initial_delay_sec: terrars::PrimField<f64>,
}

impl ComputeInstanceGroupManager_AutoHealingPolicies { }

impl terrars::ToListMappable for ComputeInstanceGroupManager_AutoHealingPolicies {
    type O = terrars::BlockAssignable<ComputeInstanceGroupManager_AutoHealingPolicies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceGroupManager_AutoHealingPolicies {
    #[doc = "The health check resource that signals autohealing."]
    pub health_check: terrars::PrimField<std::string::String>,
    #[doc =
        "The number of seconds that the managed instance group waits before it applies autohealing policies to new instances or recently recreated instances. Between 0 and 3600."]
    pub initial_delay_sec: terrars::PrimField<f64>,
}

impl BuildComputeInstanceGroupManager_AutoHealingPolicies {
    pub fn build(self) -> ComputeInstanceGroupManager_AutoHealingPolicies {
        ComputeInstanceGroupManager_AutoHealingPolicies {
            health_check: self.health_check,
            initial_delay_sec: self.initial_delay_sec,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstanceGroupManager_AutoHealingPoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceGroupManager_AutoHealingPoliciesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstanceGroupManager_AutoHealingPoliciesRef {
        ComputeInstanceGroupManager_AutoHealingPoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceGroupManager_AutoHealingPoliciesRef {
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
pub struct ComputeInstanceGroupManager_InstanceLifecyclePolicy_OnRepair {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_changing_zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeInstanceGroupManager_InstanceLifecyclePolicy_OnRepair {
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

impl terrars::ToListMappable for ComputeInstanceGroupManager_InstanceLifecyclePolicy_OnRepair {
    type O = terrars::BlockAssignable<ComputeInstanceGroupManager_InstanceLifecyclePolicy_OnRepair>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceGroupManager_InstanceLifecyclePolicy_OnRepair {}

impl BuildComputeInstanceGroupManager_InstanceLifecyclePolicy_OnRepair {
    pub fn build(self) -> ComputeInstanceGroupManager_InstanceLifecyclePolicy_OnRepair {
        ComputeInstanceGroupManager_InstanceLifecyclePolicy_OnRepair {
            allow_changing_zone: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstanceGroupManager_InstanceLifecyclePolicy_OnRepairRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceGroupManager_InstanceLifecyclePolicy_OnRepairRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstanceGroupManager_InstanceLifecyclePolicy_OnRepairRef {
        ComputeInstanceGroupManager_InstanceLifecyclePolicy_OnRepairRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceGroupManager_InstanceLifecyclePolicy_OnRepairRef {
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
struct ComputeInstanceGroupManager_InstanceLifecyclePolicyDynamic {
    on_repair: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeInstanceGroupManager_InstanceLifecyclePolicy_OnRepair>>,
}

#[derive(Serialize)]
pub struct ComputeInstanceGroupManager_InstanceLifecyclePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_action_on_failure: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_update_on_repair: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_failed_health_check: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_repair: std::option::Option<std::vec::Vec<ComputeInstanceGroupManager_InstanceLifecyclePolicy_OnRepair>>,
    dynamic: ComputeInstanceGroupManager_InstanceLifecyclePolicyDynamic,
}

impl ComputeInstanceGroupManager_InstanceLifecyclePolicy {
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
                    ::Into<terrars::BlockAssignable<ComputeInstanceGroupManager_InstanceLifecyclePolicy_OnRepair>>,
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

impl terrars::ToListMappable for ComputeInstanceGroupManager_InstanceLifecyclePolicy {
    type O = terrars::BlockAssignable<ComputeInstanceGroupManager_InstanceLifecyclePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceGroupManager_InstanceLifecyclePolicy {}

impl BuildComputeInstanceGroupManager_InstanceLifecyclePolicy {
    pub fn build(self) -> ComputeInstanceGroupManager_InstanceLifecyclePolicy {
        ComputeInstanceGroupManager_InstanceLifecyclePolicy {
            default_action_on_failure: core::default::Default::default(),
            force_update_on_repair: core::default::Default::default(),
            on_failed_health_check: core::default::Default::default(),
            on_repair: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstanceGroupManager_InstanceLifecyclePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceGroupManager_InstanceLifecyclePolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstanceGroupManager_InstanceLifecyclePolicyRef {
        ComputeInstanceGroupManager_InstanceLifecyclePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceGroupManager_InstanceLifecyclePolicyRef {
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
    pub fn on_repair(&self) -> terrars::ListRef<ComputeInstanceGroupManager_InstanceLifecyclePolicy_OnRepairRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.on_repair", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInstanceGroupManager_NamedPort {
    name: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<f64>,
}

impl ComputeInstanceGroupManager_NamedPort { }

impl terrars::ToListMappable for ComputeInstanceGroupManager_NamedPort {
    type O = terrars::BlockAssignable<ComputeInstanceGroupManager_NamedPort>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceGroupManager_NamedPort {
    #[doc = "The name of the port."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The port number."]
    pub port: terrars::PrimField<f64>,
}

impl BuildComputeInstanceGroupManager_NamedPort {
    pub fn build(self) -> ComputeInstanceGroupManager_NamedPort {
        ComputeInstanceGroupManager_NamedPort {
            name: self.name,
            port: self.port,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstanceGroupManager_NamedPortRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceGroupManager_NamedPortRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInstanceGroupManager_NamedPortRef {
        ComputeInstanceGroupManager_NamedPortRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceGroupManager_NamedPortRef {
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
pub struct ComputeInstanceGroupManager_ResourcePolicies {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workload_policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeInstanceGroupManager_ResourcePolicies {
    #[doc =
        "Set the field `workload_policy`.\nThe URL of the workload policy that is specified for this managed instance group. It can be a full or partial URL."]
    pub fn set_workload_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.workload_policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInstanceGroupManager_ResourcePolicies {
    type O = terrars::BlockAssignable<ComputeInstanceGroupManager_ResourcePolicies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceGroupManager_ResourcePolicies {}

impl BuildComputeInstanceGroupManager_ResourcePolicies {
    pub fn build(self) -> ComputeInstanceGroupManager_ResourcePolicies {
        ComputeInstanceGroupManager_ResourcePolicies { workload_policy: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ComputeInstanceGroupManager_ResourcePoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceGroupManager_ResourcePoliciesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstanceGroupManager_ResourcePoliciesRef {
        ComputeInstanceGroupManager_ResourcePoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceGroupManager_ResourcePoliciesRef {
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
pub struct ComputeInstanceGroupManager_StandbyPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_delay_sec: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeInstanceGroupManager_StandbyPolicy {
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

impl terrars::ToListMappable for ComputeInstanceGroupManager_StandbyPolicy {
    type O = terrars::BlockAssignable<ComputeInstanceGroupManager_StandbyPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceGroupManager_StandbyPolicy {}

impl BuildComputeInstanceGroupManager_StandbyPolicy {
    pub fn build(self) -> ComputeInstanceGroupManager_StandbyPolicy {
        ComputeInstanceGroupManager_StandbyPolicy {
            initial_delay_sec: core::default::Default::default(),
            mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstanceGroupManager_StandbyPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceGroupManager_StandbyPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInstanceGroupManager_StandbyPolicyRef {
        ComputeInstanceGroupManager_StandbyPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceGroupManager_StandbyPolicyRef {
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
pub struct ComputeInstanceGroupManager_StatefulDisk {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_rule: std::option::Option<terrars::PrimField<std::string::String>>,
    device_name: terrars::PrimField<std::string::String>,
}

impl ComputeInstanceGroupManager_StatefulDisk {
    #[doc =
        "Set the field `delete_rule`.\nA value that prescribes what should happen to the stateful disk when the VM instance is deleted. The available options are NEVER and ON_PERMANENT_INSTANCE_DELETION. NEVER - detach the disk when the VM is deleted, but do not delete the disk. ON_PERMANENT_INSTANCE_DELETION will delete the stateful disk when the VM is permanently deleted from the instance group. The default is NEVER."]
    pub fn set_delete_rule(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete_rule = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInstanceGroupManager_StatefulDisk {
    type O = terrars::BlockAssignable<ComputeInstanceGroupManager_StatefulDisk>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceGroupManager_StatefulDisk {
    #[doc = "The device name of the disk to be attached."]
    pub device_name: terrars::PrimField<std::string::String>,
}

impl BuildComputeInstanceGroupManager_StatefulDisk {
    pub fn build(self) -> ComputeInstanceGroupManager_StatefulDisk {
        ComputeInstanceGroupManager_StatefulDisk {
            delete_rule: core::default::Default::default(),
            device_name: self.device_name,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstanceGroupManager_StatefulDiskRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceGroupManager_StatefulDiskRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInstanceGroupManager_StatefulDiskRef {
        ComputeInstanceGroupManager_StatefulDiskRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceGroupManager_StatefulDiskRef {
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
pub struct ComputeInstanceGroupManager_StatefulExternalIp {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_rule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interface_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeInstanceGroupManager_StatefulExternalIp {
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

impl terrars::ToListMappable for ComputeInstanceGroupManager_StatefulExternalIp {
    type O = terrars::BlockAssignable<ComputeInstanceGroupManager_StatefulExternalIp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceGroupManager_StatefulExternalIp {}

impl BuildComputeInstanceGroupManager_StatefulExternalIp {
    pub fn build(self) -> ComputeInstanceGroupManager_StatefulExternalIp {
        ComputeInstanceGroupManager_StatefulExternalIp {
            delete_rule: core::default::Default::default(),
            interface_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstanceGroupManager_StatefulExternalIpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceGroupManager_StatefulExternalIpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstanceGroupManager_StatefulExternalIpRef {
        ComputeInstanceGroupManager_StatefulExternalIpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceGroupManager_StatefulExternalIpRef {
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
pub struct ComputeInstanceGroupManager_StatefulInternalIp {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_rule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interface_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeInstanceGroupManager_StatefulInternalIp {
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

impl terrars::ToListMappable for ComputeInstanceGroupManager_StatefulInternalIp {
    type O = terrars::BlockAssignable<ComputeInstanceGroupManager_StatefulInternalIp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceGroupManager_StatefulInternalIp {}

impl BuildComputeInstanceGroupManager_StatefulInternalIp {
    pub fn build(self) -> ComputeInstanceGroupManager_StatefulInternalIp {
        ComputeInstanceGroupManager_StatefulInternalIp {
            delete_rule: core::default::Default::default(),
            interface_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstanceGroupManager_StatefulInternalIpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceGroupManager_StatefulInternalIpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstanceGroupManager_StatefulInternalIpRef {
        ComputeInstanceGroupManager_StatefulInternalIpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceGroupManager_StatefulInternalIpRef {
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
pub struct ComputeInstanceGroupManager_TargetSizePolicy {
    mode: terrars::PrimField<std::string::String>,
}

impl ComputeInstanceGroupManager_TargetSizePolicy { }

impl terrars::ToListMappable for ComputeInstanceGroupManager_TargetSizePolicy {
    type O = terrars::BlockAssignable<ComputeInstanceGroupManager_TargetSizePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceGroupManager_TargetSizePolicy {
    #[doc = "The mode of target size policy based on which the MIG creates its VMs individually or all at once."]
    pub mode: terrars::PrimField<std::string::String>,
}

impl BuildComputeInstanceGroupManager_TargetSizePolicy {
    pub fn build(self) -> ComputeInstanceGroupManager_TargetSizePolicy {
        ComputeInstanceGroupManager_TargetSizePolicy { mode: self.mode }
    }
}

#[allow(dead_code)]
pub struct ComputeInstanceGroupManager_TargetSizePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceGroupManager_TargetSizePolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstanceGroupManager_TargetSizePolicyRef {
        ComputeInstanceGroupManager_TargetSizePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceGroupManager_TargetSizePolicyRef {
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
pub struct ComputeInstanceGroupManager_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeInstanceGroupManager_Timeouts {
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

impl terrars::ToListMappable for ComputeInstanceGroupManager_Timeouts {
    type O = terrars::BlockAssignable<ComputeInstanceGroupManager_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceGroupManager_Timeouts {}

impl BuildComputeInstanceGroupManager_Timeouts {
    pub fn build(self) -> ComputeInstanceGroupManager_Timeouts {
        ComputeInstanceGroupManager_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstanceGroupManager_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceGroupManager_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInstanceGroupManager_TimeoutsRef {
        ComputeInstanceGroupManager_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceGroupManager_TimeoutsRef {
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
pub struct ComputeInstanceGroupManager_UpdatePolicy {
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

impl ComputeInstanceGroupManager_UpdatePolicy {
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
        "Set the field `replacement_method`.\nThe instance replacement method for managed instance groups. Valid values are: \"RECREATE\", \"SUBSTITUTE\". If SUBSTITUTE (default), the group replaces VM instances with new instances that have randomly generated names. If RECREATE, instance names are preserved.  You must also set max_unavailable_fixed or max_unavailable_percent to be greater than 0."]
    pub fn set_replacement_method(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.replacement_method = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInstanceGroupManager_UpdatePolicy {
    type O = terrars::BlockAssignable<ComputeInstanceGroupManager_UpdatePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceGroupManager_UpdatePolicy {
    #[doc =
        "Minimal action to be taken on an instance. You can specify either NONE to forbid any actions, REFRESH to update without stopping instances, RESTART to restart existing instances or REPLACE to delete and create new instances from the target template. If you specify a REFRESH, the Updater will attempt to perform that action only. However, if the Updater determines that the minimal action you specify is not enough to perform the update, it might perform a more disruptive action."]
    pub minimal_action: terrars::PrimField<std::string::String>,
    #[doc =
        "The type of update process. You can specify either PROACTIVE so that the instance group manager proactively executes actions in order to bring instances to their target versions or OPPORTUNISTIC so that no action is proactively executed but the update will be performed as part of other actions (for example, resizes or recreateInstances calls)."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildComputeInstanceGroupManager_UpdatePolicy {
    pub fn build(self) -> ComputeInstanceGroupManager_UpdatePolicy {
        ComputeInstanceGroupManager_UpdatePolicy {
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
pub struct ComputeInstanceGroupManager_UpdatePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceGroupManager_UpdatePolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInstanceGroupManager_UpdatePolicyRef {
        ComputeInstanceGroupManager_UpdatePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceGroupManager_UpdatePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
        "Get a reference to the value of field `replacement_method` after provisioning.\nThe instance replacement method for managed instance groups. Valid values are: \"RECREATE\", \"SUBSTITUTE\". If SUBSTITUTE (default), the group replaces VM instances with new instances that have randomly generated names. If RECREATE, instance names are preserved.  You must also set max_unavailable_fixed or max_unavailable_percent to be greater than 0."]
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
pub struct ComputeInstanceGroupManager_Version_TargetSize {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percent: std::option::Option<terrars::PrimField<f64>>,
}

impl ComputeInstanceGroupManager_Version_TargetSize {
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

impl terrars::ToListMappable for ComputeInstanceGroupManager_Version_TargetSize {
    type O = terrars::BlockAssignable<ComputeInstanceGroupManager_Version_TargetSize>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceGroupManager_Version_TargetSize {}

impl BuildComputeInstanceGroupManager_Version_TargetSize {
    pub fn build(self) -> ComputeInstanceGroupManager_Version_TargetSize {
        ComputeInstanceGroupManager_Version_TargetSize {
            fixed: core::default::Default::default(),
            percent: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstanceGroupManager_Version_TargetSizeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceGroupManager_Version_TargetSizeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstanceGroupManager_Version_TargetSizeRef {
        ComputeInstanceGroupManager_Version_TargetSizeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceGroupManager_Version_TargetSizeRef {
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
struct ComputeInstanceGroupManager_VersionDynamic {
    target_size: std::option::Option<terrars::DynamicBlock<ComputeInstanceGroupManager_Version_TargetSize>>,
}

#[derive(Serialize)]
pub struct ComputeInstanceGroupManager_Version {
    instance_template: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_size: std::option::Option<std::vec::Vec<ComputeInstanceGroupManager_Version_TargetSize>>,
    dynamic: ComputeInstanceGroupManager_VersionDynamic,
}

impl ComputeInstanceGroupManager_Version {
    #[doc = "Set the field `name`.\nVersion name."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_size`.\n"]
    pub fn set_target_size(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstanceGroupManager_Version_TargetSize>>,
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

impl terrars::ToListMappable for ComputeInstanceGroupManager_Version {
    type O = terrars::BlockAssignable<ComputeInstanceGroupManager_Version>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceGroupManager_Version {
    #[doc = "The full URL to an instance template from which all new instances of this version will be created."]
    pub instance_template: terrars::PrimField<std::string::String>,
}

impl BuildComputeInstanceGroupManager_Version {
    pub fn build(self) -> ComputeInstanceGroupManager_Version {
        ComputeInstanceGroupManager_Version {
            instance_template: self.instance_template,
            name: core::default::Default::default(),
            target_size: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstanceGroupManager_VersionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceGroupManager_VersionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInstanceGroupManager_VersionRef {
        ComputeInstanceGroupManager_VersionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceGroupManager_VersionRef {
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
    pub fn target_size(&self) -> terrars::ListRef<ComputeInstanceGroupManager_Version_TargetSizeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_size", self.base))
    }
}
