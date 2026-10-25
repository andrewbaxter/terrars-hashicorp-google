#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataComputeInstanceGroupManagerData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    self_link: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataComputeInstanceGroupManager_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataComputeInstanceGroupManagerData_>,
}

#[derive(Clone)]
pub struct DataComputeInstanceGroupManager(std::rc::Rc<DataComputeInstanceGroupManager_>);

impl DataComputeInstanceGroupManager {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `name`.\nThe name of the instance group manager. Must be 1-63 characters long and comply with RFC1035. Supported characters include lowercase letters, numbers, and hyphens."]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project`.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `self_link`.\nThe URL of the created resource."]
    pub fn set_self_link(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().self_link = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone`.\nThe zone that instances in this group should be created in."]
    pub fn set_zone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `all_instances_config` after provisioning.\nSpecifies configuration that overrides the instance template configuration for the group."]
    pub fn all_instances_config(&self) -> terrars::ListRef<DataComputeInstanceGroupManager_AllInstancesConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.all_instances_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auto_healing_policies` after provisioning.\nThe autohealing policies for this managed instance group. You can specify only one value."]
    pub fn auto_healing_policies(&self) -> terrars::ListRef<DataComputeInstanceGroupManager_AutoHealingPoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_healing_policies", self.extract_ref()))
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
        "Get a reference to the value of field `instance_lifecycle_policy` after provisioning.\nThe instance lifecycle policy for this managed instance group."]
    pub fn instance_lifecycle_policy(
        &self,
    ) -> terrars::ListRef<DataComputeInstanceGroupManager_InstanceLifecyclePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_lifecycle_policy", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `named_port` after provisioning.\nThe named port configuration."]
    pub fn named_port(&self) -> terrars::SetRef<DataComputeInstanceGroupManager_NamedPortRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.named_port", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `resource_policies` after provisioning.\nResource policies for this managed instance group."]
    pub fn resource_policies(&self) -> terrars::ListRef<DataComputeInstanceGroupManager_ResourcePoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_policies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URL of the created resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `standby_policy` after provisioning.\nStandby policy for stopped and suspended instances."]
    pub fn standby_policy(&self) -> terrars::ListRef<DataComputeInstanceGroupManager_StandbyPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.standby_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `stateful_disk` after provisioning.\nDisks created on the instances that will be preserved on instance delete, update, etc."]
    pub fn stateful_disk(&self) -> terrars::SetRef<DataComputeInstanceGroupManager_StatefulDiskRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.stateful_disk", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `stateful_external_ip` after provisioning.\nExternal IPs considered stateful by the instance group. "]
    pub fn stateful_external_ip(&self) -> terrars::ListRef<DataComputeInstanceGroupManager_StatefulExternalIpRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateful_external_ip", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `stateful_internal_ip` after provisioning.\nExternal IPs considered stateful by the instance group. "]
    pub fn stateful_internal_ip(&self) -> terrars::ListRef<DataComputeInstanceGroupManager_StatefulInternalIpRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateful_internal_ip", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe status of this managed instance group."]
    pub fn status(&self) -> terrars::ListRef<DataComputeInstanceGroupManager_StatusRef> {
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
        "Get a reference to the value of field `target_size_policy` after provisioning.\nThe policy that specifies how the MIG creates its VMs to achieve the target size."]
    pub fn target_size_policy(&self) -> terrars::ListRef<DataComputeInstanceGroupManager_TargetSizePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_size_policy", self.extract_ref()))
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
        "Get a reference to the value of field `update_policy` after provisioning.\nThe update policy for this managed instance group."]
    pub fn update_policy(&self) -> terrars::ListRef<DataComputeInstanceGroupManager_UpdatePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.update_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nApplication versions managed by this instance group. Each version deals with a specific instance template, allowing canary release scenarios."]
    pub fn version(&self) -> terrars::ListRef<DataComputeInstanceGroupManager_VersionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
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
}

impl terrars::Referable for DataComputeInstanceGroupManager {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataComputeInstanceGroupManager { }

impl terrars::ToListMappable for DataComputeInstanceGroupManager {
    type O = terrars::ListRef<DataComputeInstanceGroupManagerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataComputeInstanceGroupManager_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_compute_instance_group_manager".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataComputeInstanceGroupManager {
    pub tf_id: std::string::String,
}

impl BuildDataComputeInstanceGroupManager {
    pub fn build(self, stack: &mut terrars::Stack) -> DataComputeInstanceGroupManager {
        let out = DataComputeInstanceGroupManager(std::rc::Rc::new(DataComputeInstanceGroupManager_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataComputeInstanceGroupManagerData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                project: core::default::Default::default(),
                self_link: core::default::Default::default(),
                zone: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataComputeInstanceGroupManagerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceGroupManagerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataComputeInstanceGroupManagerRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `all_instances_config` after provisioning.\nSpecifies configuration that overrides the instance template configuration for the group."]
    pub fn all_instances_config(&self) -> terrars::ListRef<DataComputeInstanceGroupManager_AllInstancesConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.all_instances_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auto_healing_policies` after provisioning.\nThe autohealing policies for this managed instance group. You can specify only one value."]
    pub fn auto_healing_policies(&self) -> terrars::ListRef<DataComputeInstanceGroupManager_AutoHealingPoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_healing_policies", self.extract_ref()))
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
        "Get a reference to the value of field `instance_lifecycle_policy` after provisioning.\nThe instance lifecycle policy for this managed instance group."]
    pub fn instance_lifecycle_policy(
        &self,
    ) -> terrars::ListRef<DataComputeInstanceGroupManager_InstanceLifecyclePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_lifecycle_policy", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `named_port` after provisioning.\nThe named port configuration."]
    pub fn named_port(&self) -> terrars::SetRef<DataComputeInstanceGroupManager_NamedPortRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.named_port", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `resource_policies` after provisioning.\nResource policies for this managed instance group."]
    pub fn resource_policies(&self) -> terrars::ListRef<DataComputeInstanceGroupManager_ResourcePoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_policies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URL of the created resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `standby_policy` after provisioning.\nStandby policy for stopped and suspended instances."]
    pub fn standby_policy(&self) -> terrars::ListRef<DataComputeInstanceGroupManager_StandbyPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.standby_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `stateful_disk` after provisioning.\nDisks created on the instances that will be preserved on instance delete, update, etc."]
    pub fn stateful_disk(&self) -> terrars::SetRef<DataComputeInstanceGroupManager_StatefulDiskRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.stateful_disk", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `stateful_external_ip` after provisioning.\nExternal IPs considered stateful by the instance group. "]
    pub fn stateful_external_ip(&self) -> terrars::ListRef<DataComputeInstanceGroupManager_StatefulExternalIpRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateful_external_ip", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `stateful_internal_ip` after provisioning.\nExternal IPs considered stateful by the instance group. "]
    pub fn stateful_internal_ip(&self) -> terrars::ListRef<DataComputeInstanceGroupManager_StatefulInternalIpRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateful_internal_ip", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe status of this managed instance group."]
    pub fn status(&self) -> terrars::ListRef<DataComputeInstanceGroupManager_StatusRef> {
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
        "Get a reference to the value of field `target_size_policy` after provisioning.\nThe policy that specifies how the MIG creates its VMs to achieve the target size."]
    pub fn target_size_policy(&self) -> terrars::ListRef<DataComputeInstanceGroupManager_TargetSizePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_size_policy", self.extract_ref()))
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
        "Get a reference to the value of field `update_policy` after provisioning.\nThe update policy for this managed instance group."]
    pub fn update_policy(&self) -> terrars::ListRef<DataComputeInstanceGroupManager_UpdatePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.update_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nApplication versions managed by this instance group. Each version deals with a specific instance template, allowing canary release scenarios."]
    pub fn version(&self) -> terrars::ListRef<DataComputeInstanceGroupManager_VersionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
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
}

#[derive(Serialize)]
pub struct DataComputeInstanceGroupManager_AllInstancesConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataComputeInstanceGroupManager_AllInstancesConfig {
    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.metadata = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeInstanceGroupManager_AllInstancesConfig {
    type O = terrars::BlockAssignable<DataComputeInstanceGroupManager_AllInstancesConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceGroupManager_AllInstancesConfig {}

impl BuildDataComputeInstanceGroupManager_AllInstancesConfig {
    pub fn build(self) -> DataComputeInstanceGroupManager_AllInstancesConfig {
        DataComputeInstanceGroupManager_AllInstancesConfig {
            labels: core::default::Default::default(),
            metadata: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeInstanceGroupManager_AllInstancesConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceGroupManager_AllInstancesConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceGroupManager_AllInstancesConfigRef {
        DataComputeInstanceGroupManager_AllInstancesConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceGroupManager_AllInstancesConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceGroupManager_AutoHealingPolicies {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_delay_sec: std::option::Option<terrars::PrimField<f64>>,
}

impl DataComputeInstanceGroupManager_AutoHealingPolicies {
    #[doc = "Set the field `health_check`.\n"]
    pub fn set_health_check(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.health_check = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `initial_delay_sec`.\n"]
    pub fn set_initial_delay_sec(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.initial_delay_sec = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeInstanceGroupManager_AutoHealingPolicies {
    type O = terrars::BlockAssignable<DataComputeInstanceGroupManager_AutoHealingPolicies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceGroupManager_AutoHealingPolicies {}

impl BuildDataComputeInstanceGroupManager_AutoHealingPolicies {
    pub fn build(self) -> DataComputeInstanceGroupManager_AutoHealingPolicies {
        DataComputeInstanceGroupManager_AutoHealingPolicies {
            health_check: core::default::Default::default(),
            initial_delay_sec: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeInstanceGroupManager_AutoHealingPoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceGroupManager_AutoHealingPoliciesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceGroupManager_AutoHealingPoliciesRef {
        DataComputeInstanceGroupManager_AutoHealingPoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceGroupManager_AutoHealingPoliciesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_check", self.base))
    }

    #[doc = "Get a reference to the value of field `initial_delay_sec` after provisioning.\n"]
    pub fn initial_delay_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_delay_sec", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceGroupManager_InstanceLifecyclePolicy_OnRepair {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_changing_zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeInstanceGroupManager_InstanceLifecyclePolicy_OnRepair {
    #[doc = "Set the field `allow_changing_zone`.\n"]
    pub fn set_allow_changing_zone(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.allow_changing_zone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeInstanceGroupManager_InstanceLifecyclePolicy_OnRepair {
    type O = terrars::BlockAssignable<DataComputeInstanceGroupManager_InstanceLifecyclePolicy_OnRepair>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceGroupManager_InstanceLifecyclePolicy_OnRepair {}

impl BuildDataComputeInstanceGroupManager_InstanceLifecyclePolicy_OnRepair {
    pub fn build(self) -> DataComputeInstanceGroupManager_InstanceLifecyclePolicy_OnRepair {
        DataComputeInstanceGroupManager_InstanceLifecyclePolicy_OnRepair {
            allow_changing_zone: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeInstanceGroupManager_InstanceLifecyclePolicy_OnRepairRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceGroupManager_InstanceLifecyclePolicy_OnRepairRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceGroupManager_InstanceLifecyclePolicy_OnRepairRef {
        DataComputeInstanceGroupManager_InstanceLifecyclePolicy_OnRepairRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceGroupManager_InstanceLifecyclePolicy_OnRepairRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_changing_zone` after provisioning.\n"]
    pub fn allow_changing_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_changing_zone", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceGroupManager_InstanceLifecyclePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_action_on_failure: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_update_on_repair: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_failed_health_check: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_repair: std::option::Option<terrars::ListField<DataComputeInstanceGroupManager_InstanceLifecyclePolicy_OnRepair>>,
}

impl DataComputeInstanceGroupManager_InstanceLifecyclePolicy {
    #[doc = "Set the field `default_action_on_failure`.\n"]
    pub fn set_default_action_on_failure(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.default_action_on_failure = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `force_update_on_repair`.\n"]
    pub fn set_force_update_on_repair(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.force_update_on_repair = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `on_failed_health_check`.\n"]
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
                    ::Into<terrars::ListField<DataComputeInstanceGroupManager_InstanceLifecyclePolicy_OnRepair>>,
    ) -> Self {
        self.on_repair = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeInstanceGroupManager_InstanceLifecyclePolicy {
    type O = terrars::BlockAssignable<DataComputeInstanceGroupManager_InstanceLifecyclePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceGroupManager_InstanceLifecyclePolicy {}

impl BuildDataComputeInstanceGroupManager_InstanceLifecyclePolicy {
    pub fn build(self) -> DataComputeInstanceGroupManager_InstanceLifecyclePolicy {
        DataComputeInstanceGroupManager_InstanceLifecyclePolicy {
            default_action_on_failure: core::default::Default::default(),
            force_update_on_repair: core::default::Default::default(),
            on_failed_health_check: core::default::Default::default(),
            on_repair: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeInstanceGroupManager_InstanceLifecyclePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceGroupManager_InstanceLifecyclePolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceGroupManager_InstanceLifecyclePolicyRef {
        DataComputeInstanceGroupManager_InstanceLifecyclePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceGroupManager_InstanceLifecyclePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_action_on_failure` after provisioning.\n"]
    pub fn default_action_on_failure(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_action_on_failure", self.base))
    }

    #[doc = "Get a reference to the value of field `force_update_on_repair` after provisioning.\n"]
    pub fn force_update_on_repair(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_update_on_repair", self.base))
    }

    #[doc = "Get a reference to the value of field `on_failed_health_check` after provisioning.\n"]
    pub fn on_failed_health_check(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_failed_health_check", self.base))
    }

    #[doc = "Get a reference to the value of field `on_repair` after provisioning.\n"]
    pub fn on_repair(&self) -> terrars::ListRef<DataComputeInstanceGroupManager_InstanceLifecyclePolicy_OnRepairRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.on_repair", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceGroupManager_NamedPort {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
}

impl DataComputeInstanceGroupManager_NamedPort {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeInstanceGroupManager_NamedPort {
    type O = terrars::BlockAssignable<DataComputeInstanceGroupManager_NamedPort>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceGroupManager_NamedPort {}

impl BuildDataComputeInstanceGroupManager_NamedPort {
    pub fn build(self) -> DataComputeInstanceGroupManager_NamedPort {
        DataComputeInstanceGroupManager_NamedPort {
            name: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeInstanceGroupManager_NamedPortRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceGroupManager_NamedPortRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeInstanceGroupManager_NamedPortRef {
        DataComputeInstanceGroupManager_NamedPortRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceGroupManager_NamedPortRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceGroupManager_ResourcePolicies {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workload_policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeInstanceGroupManager_ResourcePolicies {
    #[doc = "Set the field `workload_policy`.\n"]
    pub fn set_workload_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.workload_policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeInstanceGroupManager_ResourcePolicies {
    type O = terrars::BlockAssignable<DataComputeInstanceGroupManager_ResourcePolicies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceGroupManager_ResourcePolicies {}

impl BuildDataComputeInstanceGroupManager_ResourcePolicies {
    pub fn build(self) -> DataComputeInstanceGroupManager_ResourcePolicies {
        DataComputeInstanceGroupManager_ResourcePolicies { workload_policy: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataComputeInstanceGroupManager_ResourcePoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceGroupManager_ResourcePoliciesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceGroupManager_ResourcePoliciesRef {
        DataComputeInstanceGroupManager_ResourcePoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceGroupManager_ResourcePoliciesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `workload_policy` after provisioning.\n"]
    pub fn workload_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workload_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceGroupManager_StandbyPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_delay_sec: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeInstanceGroupManager_StandbyPolicy {
    #[doc = "Set the field `initial_delay_sec`.\n"]
    pub fn set_initial_delay_sec(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.initial_delay_sec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeInstanceGroupManager_StandbyPolicy {
    type O = terrars::BlockAssignable<DataComputeInstanceGroupManager_StandbyPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceGroupManager_StandbyPolicy {}

impl BuildDataComputeInstanceGroupManager_StandbyPolicy {
    pub fn build(self) -> DataComputeInstanceGroupManager_StandbyPolicy {
        DataComputeInstanceGroupManager_StandbyPolicy {
            initial_delay_sec: core::default::Default::default(),
            mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeInstanceGroupManager_StandbyPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceGroupManager_StandbyPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceGroupManager_StandbyPolicyRef {
        DataComputeInstanceGroupManager_StandbyPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceGroupManager_StandbyPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `initial_delay_sec` after provisioning.\n"]
    pub fn initial_delay_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_delay_sec", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceGroupManager_StatefulDisk {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_rule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeInstanceGroupManager_StatefulDisk {
    #[doc = "Set the field `delete_rule`.\n"]
    pub fn set_delete_rule(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete_rule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_name`.\n"]
    pub fn set_device_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.device_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeInstanceGroupManager_StatefulDisk {
    type O = terrars::BlockAssignable<DataComputeInstanceGroupManager_StatefulDisk>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceGroupManager_StatefulDisk {}

impl BuildDataComputeInstanceGroupManager_StatefulDisk {
    pub fn build(self) -> DataComputeInstanceGroupManager_StatefulDisk {
        DataComputeInstanceGroupManager_StatefulDisk {
            delete_rule: core::default::Default::default(),
            device_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeInstanceGroupManager_StatefulDiskRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceGroupManager_StatefulDiskRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceGroupManager_StatefulDiskRef {
        DataComputeInstanceGroupManager_StatefulDiskRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceGroupManager_StatefulDiskRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete_rule` after provisioning.\n"]
    pub fn delete_rule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `device_name` after provisioning.\n"]
    pub fn device_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceGroupManager_StatefulExternalIp {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_rule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interface_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeInstanceGroupManager_StatefulExternalIp {
    #[doc = "Set the field `delete_rule`.\n"]
    pub fn set_delete_rule(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete_rule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interface_name`.\n"]
    pub fn set_interface_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.interface_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeInstanceGroupManager_StatefulExternalIp {
    type O = terrars::BlockAssignable<DataComputeInstanceGroupManager_StatefulExternalIp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceGroupManager_StatefulExternalIp {}

impl BuildDataComputeInstanceGroupManager_StatefulExternalIp {
    pub fn build(self) -> DataComputeInstanceGroupManager_StatefulExternalIp {
        DataComputeInstanceGroupManager_StatefulExternalIp {
            delete_rule: core::default::Default::default(),
            interface_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeInstanceGroupManager_StatefulExternalIpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceGroupManager_StatefulExternalIpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceGroupManager_StatefulExternalIpRef {
        DataComputeInstanceGroupManager_StatefulExternalIpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceGroupManager_StatefulExternalIpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete_rule` after provisioning.\n"]
    pub fn delete_rule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `interface_name` after provisioning.\n"]
    pub fn interface_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interface_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceGroupManager_StatefulInternalIp {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_rule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interface_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeInstanceGroupManager_StatefulInternalIp {
    #[doc = "Set the field `delete_rule`.\n"]
    pub fn set_delete_rule(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete_rule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interface_name`.\n"]
    pub fn set_interface_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.interface_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeInstanceGroupManager_StatefulInternalIp {
    type O = terrars::BlockAssignable<DataComputeInstanceGroupManager_StatefulInternalIp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceGroupManager_StatefulInternalIp {}

impl BuildDataComputeInstanceGroupManager_StatefulInternalIp {
    pub fn build(self) -> DataComputeInstanceGroupManager_StatefulInternalIp {
        DataComputeInstanceGroupManager_StatefulInternalIp {
            delete_rule: core::default::Default::default(),
            interface_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeInstanceGroupManager_StatefulInternalIpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceGroupManager_StatefulInternalIpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceGroupManager_StatefulInternalIpRef {
        DataComputeInstanceGroupManager_StatefulInternalIpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceGroupManager_StatefulInternalIpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete_rule` after provisioning.\n"]
    pub fn delete_rule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `interface_name` after provisioning.\n"]
    pub fn interface_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interface_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceGroupManager_Status_AllInstancesConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    current_revision: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effective: std::option::Option<terrars::PrimField<bool>>,
}

impl DataComputeInstanceGroupManager_Status_AllInstancesConfig {
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

impl terrars::ToListMappable for DataComputeInstanceGroupManager_Status_AllInstancesConfig {
    type O = terrars::BlockAssignable<DataComputeInstanceGroupManager_Status_AllInstancesConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceGroupManager_Status_AllInstancesConfig {}

impl BuildDataComputeInstanceGroupManager_Status_AllInstancesConfig {
    pub fn build(self) -> DataComputeInstanceGroupManager_Status_AllInstancesConfig {
        DataComputeInstanceGroupManager_Status_AllInstancesConfig {
            current_revision: core::default::Default::default(),
            effective: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeInstanceGroupManager_Status_AllInstancesConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceGroupManager_Status_AllInstancesConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceGroupManager_Status_AllInstancesConfigRef {
        DataComputeInstanceGroupManager_Status_AllInstancesConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceGroupManager_Status_AllInstancesConfigRef {
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
pub struct DataComputeInstanceGroupManager_Status_Stateful_PerInstanceConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all_effective: std::option::Option<terrars::PrimField<bool>>,
}

impl DataComputeInstanceGroupManager_Status_Stateful_PerInstanceConfigs {
    #[doc = "Set the field `all_effective`.\n"]
    pub fn set_all_effective(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.all_effective = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeInstanceGroupManager_Status_Stateful_PerInstanceConfigs {
    type O = terrars::BlockAssignable<DataComputeInstanceGroupManager_Status_Stateful_PerInstanceConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceGroupManager_Status_Stateful_PerInstanceConfigs {}

impl BuildDataComputeInstanceGroupManager_Status_Stateful_PerInstanceConfigs {
    pub fn build(self) -> DataComputeInstanceGroupManager_Status_Stateful_PerInstanceConfigs {
        DataComputeInstanceGroupManager_Status_Stateful_PerInstanceConfigs {
            all_effective: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeInstanceGroupManager_Status_Stateful_PerInstanceConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceGroupManager_Status_Stateful_PerInstanceConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceGroupManager_Status_Stateful_PerInstanceConfigsRef {
        DataComputeInstanceGroupManager_Status_Stateful_PerInstanceConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceGroupManager_Status_Stateful_PerInstanceConfigsRef {
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
pub struct DataComputeInstanceGroupManager_Status_Stateful {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    has_stateful_config: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_instance_configs: std
    ::option
    ::Option<terrars::ListField<DataComputeInstanceGroupManager_Status_Stateful_PerInstanceConfigs>>,
}

impl DataComputeInstanceGroupManager_Status_Stateful {
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
                    ::Into<terrars::ListField<DataComputeInstanceGroupManager_Status_Stateful_PerInstanceConfigs>>,
    ) -> Self {
        self.per_instance_configs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeInstanceGroupManager_Status_Stateful {
    type O = terrars::BlockAssignable<DataComputeInstanceGroupManager_Status_Stateful>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceGroupManager_Status_Stateful {}

impl BuildDataComputeInstanceGroupManager_Status_Stateful {
    pub fn build(self) -> DataComputeInstanceGroupManager_Status_Stateful {
        DataComputeInstanceGroupManager_Status_Stateful {
            has_stateful_config: core::default::Default::default(),
            per_instance_configs: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeInstanceGroupManager_Status_StatefulRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceGroupManager_Status_StatefulRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceGroupManager_Status_StatefulRef {
        DataComputeInstanceGroupManager_Status_StatefulRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceGroupManager_Status_StatefulRef {
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
    ) -> terrars::ListRef<DataComputeInstanceGroupManager_Status_Stateful_PerInstanceConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.per_instance_configs", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceGroupManager_Status_VersionTarget {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_reached: std::option::Option<terrars::PrimField<bool>>,
}

impl DataComputeInstanceGroupManager_Status_VersionTarget {
    #[doc = "Set the field `is_reached`.\n"]
    pub fn set_is_reached(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_reached = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeInstanceGroupManager_Status_VersionTarget {
    type O = terrars::BlockAssignable<DataComputeInstanceGroupManager_Status_VersionTarget>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceGroupManager_Status_VersionTarget {}

impl BuildDataComputeInstanceGroupManager_Status_VersionTarget {
    pub fn build(self) -> DataComputeInstanceGroupManager_Status_VersionTarget {
        DataComputeInstanceGroupManager_Status_VersionTarget { is_reached: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataComputeInstanceGroupManager_Status_VersionTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceGroupManager_Status_VersionTargetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceGroupManager_Status_VersionTargetRef {
        DataComputeInstanceGroupManager_Status_VersionTargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceGroupManager_Status_VersionTargetRef {
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
pub struct DataComputeInstanceGroupManager_Status {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all_instances_config: std
    ::option
    ::Option<terrars::ListField<DataComputeInstanceGroupManager_Status_AllInstancesConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_stable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateful: std::option::Option<terrars::ListField<DataComputeInstanceGroupManager_Status_Stateful>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version_target: std::option::Option<terrars::ListField<DataComputeInstanceGroupManager_Status_VersionTarget>>,
}

impl DataComputeInstanceGroupManager_Status {
    #[doc = "Set the field `all_instances_config`.\n"]
    pub fn set_all_instances_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeInstanceGroupManager_Status_AllInstancesConfig>>,
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
        v: impl std::convert::Into<terrars::ListField<DataComputeInstanceGroupManager_Status_Stateful>>,
    ) -> Self {
        self.stateful = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version_target`.\n"]
    pub fn set_version_target(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeInstanceGroupManager_Status_VersionTarget>>,
    ) -> Self {
        self.version_target = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeInstanceGroupManager_Status {
    type O = terrars::BlockAssignable<DataComputeInstanceGroupManager_Status>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceGroupManager_Status {}

impl BuildDataComputeInstanceGroupManager_Status {
    pub fn build(self) -> DataComputeInstanceGroupManager_Status {
        DataComputeInstanceGroupManager_Status {
            all_instances_config: core::default::Default::default(),
            is_stable: core::default::Default::default(),
            stateful: core::default::Default::default(),
            version_target: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeInstanceGroupManager_StatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceGroupManager_StatusRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeInstanceGroupManager_StatusRef {
        DataComputeInstanceGroupManager_StatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceGroupManager_StatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `all_instances_config` after provisioning.\n"]
    pub fn all_instances_config(&self) -> terrars::ListRef<DataComputeInstanceGroupManager_Status_AllInstancesConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.all_instances_config", self.base))
    }

    #[doc = "Get a reference to the value of field `is_stable` after provisioning.\n"]
    pub fn is_stable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_stable", self.base))
    }

    #[doc = "Get a reference to the value of field `stateful` after provisioning.\n"]
    pub fn stateful(&self) -> terrars::ListRef<DataComputeInstanceGroupManager_Status_StatefulRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateful", self.base))
    }

    #[doc = "Get a reference to the value of field `version_target` after provisioning.\n"]
    pub fn version_target(&self) -> terrars::ListRef<DataComputeInstanceGroupManager_Status_VersionTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.version_target", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceGroupManager_TargetSizePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeInstanceGroupManager_TargetSizePolicy {
    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeInstanceGroupManager_TargetSizePolicy {
    type O = terrars::BlockAssignable<DataComputeInstanceGroupManager_TargetSizePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceGroupManager_TargetSizePolicy {}

impl BuildDataComputeInstanceGroupManager_TargetSizePolicy {
    pub fn build(self) -> DataComputeInstanceGroupManager_TargetSizePolicy {
        DataComputeInstanceGroupManager_TargetSizePolicy { mode: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataComputeInstanceGroupManager_TargetSizePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceGroupManager_TargetSizePolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceGroupManager_TargetSizePolicyRef {
        DataComputeInstanceGroupManager_TargetSizePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceGroupManager_TargetSizePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceGroupManager_UpdatePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_surge_fixed: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_surge_percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_unavailable_fixed: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_unavailable_percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimal_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    most_disruptive_allowed_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replacement_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeInstanceGroupManager_UpdatePolicy {
    #[doc = "Set the field `max_surge_fixed`.\n"]
    pub fn set_max_surge_fixed(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_surge_fixed = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_surge_percent`.\n"]
    pub fn set_max_surge_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_surge_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_unavailable_fixed`.\n"]
    pub fn set_max_unavailable_fixed(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_unavailable_fixed = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_unavailable_percent`.\n"]
    pub fn set_max_unavailable_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_unavailable_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minimal_action`.\n"]
    pub fn set_minimal_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.minimal_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `most_disruptive_allowed_action`.\n"]
    pub fn set_most_disruptive_allowed_action(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.most_disruptive_allowed_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replacement_method`.\n"]
    pub fn set_replacement_method(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.replacement_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeInstanceGroupManager_UpdatePolicy {
    type O = terrars::BlockAssignable<DataComputeInstanceGroupManager_UpdatePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceGroupManager_UpdatePolicy {}

impl BuildDataComputeInstanceGroupManager_UpdatePolicy {
    pub fn build(self) -> DataComputeInstanceGroupManager_UpdatePolicy {
        DataComputeInstanceGroupManager_UpdatePolicy {
            max_surge_fixed: core::default::Default::default(),
            max_surge_percent: core::default::Default::default(),
            max_unavailable_fixed: core::default::Default::default(),
            max_unavailable_percent: core::default::Default::default(),
            minimal_action: core::default::Default::default(),
            most_disruptive_allowed_action: core::default::Default::default(),
            replacement_method: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeInstanceGroupManager_UpdatePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceGroupManager_UpdatePolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceGroupManager_UpdatePolicyRef {
        DataComputeInstanceGroupManager_UpdatePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceGroupManager_UpdatePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_surge_fixed` after provisioning.\n"]
    pub fn max_surge_fixed(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_surge_fixed", self.base))
    }

    #[doc = "Get a reference to the value of field `max_surge_percent` after provisioning.\n"]
    pub fn max_surge_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_surge_percent", self.base))
    }

    #[doc = "Get a reference to the value of field `max_unavailable_fixed` after provisioning.\n"]
    pub fn max_unavailable_fixed(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_unavailable_fixed", self.base))
    }

    #[doc = "Get a reference to the value of field `max_unavailable_percent` after provisioning.\n"]
    pub fn max_unavailable_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_unavailable_percent", self.base))
    }

    #[doc = "Get a reference to the value of field `minimal_action` after provisioning.\n"]
    pub fn minimal_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimal_action", self.base))
    }

    #[doc = "Get a reference to the value of field `most_disruptive_allowed_action` after provisioning.\n"]
    pub fn most_disruptive_allowed_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.most_disruptive_allowed_action", self.base))
    }

    #[doc = "Get a reference to the value of field `replacement_method` after provisioning.\n"]
    pub fn replacement_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replacement_method", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceGroupManager_Version_TargetSize {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percent: std::option::Option<terrars::PrimField<f64>>,
}

impl DataComputeInstanceGroupManager_Version_TargetSize {
    #[doc = "Set the field `fixed`.\n"]
    pub fn set_fixed(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.fixed = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `percent`.\n"]
    pub fn set_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.percent = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeInstanceGroupManager_Version_TargetSize {
    type O = terrars::BlockAssignable<DataComputeInstanceGroupManager_Version_TargetSize>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceGroupManager_Version_TargetSize {}

impl BuildDataComputeInstanceGroupManager_Version_TargetSize {
    pub fn build(self) -> DataComputeInstanceGroupManager_Version_TargetSize {
        DataComputeInstanceGroupManager_Version_TargetSize {
            fixed: core::default::Default::default(),
            percent: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeInstanceGroupManager_Version_TargetSizeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceGroupManager_Version_TargetSizeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceGroupManager_Version_TargetSizeRef {
        DataComputeInstanceGroupManager_Version_TargetSizeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceGroupManager_Version_TargetSizeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `fixed` after provisioning.\n"]
    pub fn fixed(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fixed", self.base))
    }

    #[doc = "Get a reference to the value of field `percent` after provisioning.\n"]
    pub fn percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percent", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceGroupManager_Version {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_template: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_size: std::option::Option<terrars::ListField<DataComputeInstanceGroupManager_Version_TargetSize>>,
}

impl DataComputeInstanceGroupManager_Version {
    #[doc = "Set the field `instance_template`.\n"]
    pub fn set_instance_template(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_template = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_size`.\n"]
    pub fn set_target_size(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeInstanceGroupManager_Version_TargetSize>>,
    ) -> Self {
        self.target_size = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeInstanceGroupManager_Version {
    type O = terrars::BlockAssignable<DataComputeInstanceGroupManager_Version>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceGroupManager_Version {}

impl BuildDataComputeInstanceGroupManager_Version {
    pub fn build(self) -> DataComputeInstanceGroupManager_Version {
        DataComputeInstanceGroupManager_Version {
            instance_template: core::default::Default::default(),
            name: core::default::Default::default(),
            target_size: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataComputeInstanceGroupManager_VersionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceGroupManager_VersionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeInstanceGroupManager_VersionRef {
        DataComputeInstanceGroupManager_VersionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceGroupManager_VersionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_template` after provisioning.\n"]
    pub fn instance_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_template", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `target_size` after provisioning.\n"]
    pub fn target_size(&self) -> terrars::ListRef<DataComputeInstanceGroupManager_Version_TargetSizeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_size", self.base))
    }
}
