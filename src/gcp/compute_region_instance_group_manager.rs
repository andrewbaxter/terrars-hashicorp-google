use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeRegionInstanceGroupManagerData {
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
    wait_for_instances: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wait_for_instances_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_healing_policies: std::option::Option<std::vec::Vec<ComputeRegionInstanceGroupManagerAutoHealingPoliciesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_lifecycle_policy: std
    ::option
    ::Option<std::vec::Vec<ComputeRegionInstanceGroupManagerInstanceLifecyclePolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    named_port: std::option::Option<std::vec::Vec<ComputeRegionInstanceGroupManagerNamedPortEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateful_disk: std::option::Option<std::vec::Vec<ComputeRegionInstanceGroupManagerStatefulDiskEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateful_external_ip: std::option::Option<std::vec::Vec<ComputeRegionInstanceGroupManagerStatefulExternalIpEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateful_internal_ip: std::option::Option<std::vec::Vec<ComputeRegionInstanceGroupManagerStatefulInternalIpEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeRegionInstanceGroupManagerTimeoutsEl>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_policy: std::option::Option<std::vec::Vec<ComputeRegionInstanceGroupManagerUpdatePolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<std::vec::Vec<ComputeRegionInstanceGroupManagerVersionEl>>,
    dynamic: ComputeRegionInstanceGroupManagerDynamic,
}

struct ComputeRegionInstanceGroupManager_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeRegionInstanceGroupManagerData>,
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
        "Set the field `wait_for_instances`.\nWhether to wait for all instances to be created/updated before returning. Note that if this is set to true and the operation does not succeed, Terraform will continue trying until it times out."]
    pub fn set_wait_for_instances(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().wait_for_instances = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `wait_for_instances_status`.\nWhen used with wait_for_instances specifies the status to wait for. When STABLE is specified this resource will wait until the instances are stable before returning. When UPDATED is set, it will wait for the version target to be reached and any per instance configs to be effective as well as all instances to be stable before returning."]
    pub fn set_wait_for_instances_status(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().wait_for_instances_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auto_healing_policies`.\n"]
    pub fn set_auto_healing_policies(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionInstanceGroupManagerAutoHealingPoliciesEl>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeRegionInstanceGroupManagerInstanceLifecyclePolicyEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionInstanceGroupManagerNamedPortEl>>,
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

    #[doc = "Set the field `stateful_disk`.\n"]
    pub fn set_stateful_disk(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionInstanceGroupManagerStatefulDiskEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionInstanceGroupManagerStatefulExternalIpEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionInstanceGroupManagerStatefulInternalIpEl>>,
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeRegionInstanceGroupManagerTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_policy`.\n"]
    pub fn set_update_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionInstanceGroupManagerUpdatePolicyEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionInstanceGroupManagerVersionEl>>,
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
    pub fn status(&self) -> terrars::ListRef<ComputeRegionInstanceGroupManagerStatusElRef> {
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
        "Get a reference to the value of field `wait_for_instances` after provisioning.\nWhether to wait for all instances to be created/updated before returning. Note that if this is set to true and the operation does not succeed, Terraform will continue trying until it times out."]
    pub fn wait_for_instances(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_for_instances", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `wait_for_instances_status` after provisioning.\nWhen used with wait_for_instances specifies the status to wait for. When STABLE is specified this resource will wait until the instances are stable before returning. When UPDATED is set, it will wait for the version target to be reached and any per instance configs to be effective as well as all instances to be stable before returning."]
    pub fn wait_for_instances_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_for_instances_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_healing_policies` after provisioning.\n"]
    pub fn auto_healing_policies(&self) -> terrars::ListRef<ComputeRegionInstanceGroupManagerAutoHealingPoliciesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_healing_policies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_lifecycle_policy` after provisioning.\n"]
    pub fn instance_lifecycle_policy(
        &self,
    ) -> terrars::ListRef<ComputeRegionInstanceGroupManagerInstanceLifecyclePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_lifecycle_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stateful_external_ip` after provisioning.\n"]
    pub fn stateful_external_ip(&self) -> terrars::ListRef<ComputeRegionInstanceGroupManagerStatefulExternalIpElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateful_external_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stateful_internal_ip` after provisioning.\n"]
    pub fn stateful_internal_ip(&self) -> terrars::ListRef<ComputeRegionInstanceGroupManagerStatefulInternalIpElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateful_internal_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRegionInstanceGroupManagerTimeoutsElRef {
        ComputeRegionInstanceGroupManagerTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `update_policy` after provisioning.\n"]
    pub fn update_policy(&self) -> terrars::ListRef<ComputeRegionInstanceGroupManagerUpdatePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.update_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::ListRef<ComputeRegionInstanceGroupManagerVersionElRef> {
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
            data: std::cell::RefCell::new(ComputeRegionInstanceGroupManagerData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                base_instance_name: self.base_instance_name,
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
                wait_for_instances: core::default::Default::default(),
                wait_for_instances_status: core::default::Default::default(),
                auto_healing_policies: core::default::Default::default(),
                instance_lifecycle_policy: core::default::Default::default(),
                named_port: core::default::Default::default(),
                stateful_disk: core::default::Default::default(),
                stateful_external_ip: core::default::Default::default(),
                stateful_internal_ip: core::default::Default::default(),
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
    pub fn status(&self) -> terrars::ListRef<ComputeRegionInstanceGroupManagerStatusElRef> {
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
        "Get a reference to the value of field `wait_for_instances` after provisioning.\nWhether to wait for all instances to be created/updated before returning. Note that if this is set to true and the operation does not succeed, Terraform will continue trying until it times out."]
    pub fn wait_for_instances(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_for_instances", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `wait_for_instances_status` after provisioning.\nWhen used with wait_for_instances specifies the status to wait for. When STABLE is specified this resource will wait until the instances are stable before returning. When UPDATED is set, it will wait for the version target to be reached and any per instance configs to be effective as well as all instances to be stable before returning."]
    pub fn wait_for_instances_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_for_instances_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_healing_policies` after provisioning.\n"]
    pub fn auto_healing_policies(&self) -> terrars::ListRef<ComputeRegionInstanceGroupManagerAutoHealingPoliciesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_healing_policies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_lifecycle_policy` after provisioning.\n"]
    pub fn instance_lifecycle_policy(
        &self,
    ) -> terrars::ListRef<ComputeRegionInstanceGroupManagerInstanceLifecyclePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_lifecycle_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stateful_external_ip` after provisioning.\n"]
    pub fn stateful_external_ip(&self) -> terrars::ListRef<ComputeRegionInstanceGroupManagerStatefulExternalIpElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateful_external_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stateful_internal_ip` after provisioning.\n"]
    pub fn stateful_internal_ip(&self) -> terrars::ListRef<ComputeRegionInstanceGroupManagerStatefulInternalIpElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateful_internal_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRegionInstanceGroupManagerTimeoutsElRef {
        ComputeRegionInstanceGroupManagerTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `update_policy` after provisioning.\n"]
    pub fn update_policy(&self) -> terrars::ListRef<ComputeRegionInstanceGroupManagerUpdatePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.update_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::ListRef<ComputeRegionInstanceGroupManagerVersionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all_effective: std::option::Option<terrars::PrimField<bool>>,
}

impl ComputeRegionInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl {
    #[doc = "Set the field `all_effective`.\n"]
    pub fn set_all_effective(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.all_effective = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl {
    type O = terrars::BlockAssignable<ComputeRegionInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl {}

impl BuildComputeRegionInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl {
    pub fn build(self) -> ComputeRegionInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl {
        ComputeRegionInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl {
            all_effective: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionInstanceGroupManagerStatusElStatefulElPerInstanceConfigsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionInstanceGroupManagerStatusElStatefulElPerInstanceConfigsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionInstanceGroupManagerStatusElStatefulElPerInstanceConfigsElRef {
        ComputeRegionInstanceGroupManagerStatusElStatefulElPerInstanceConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceGroupManagerStatusElStatefulElPerInstanceConfigsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `all_effective` after provisioning.\n"]
    pub fn all_effective(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.all_effective", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceGroupManagerStatusElStatefulEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    has_stateful_config: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_instance_configs: std
    ::option
    ::Option<terrars::ListField<ComputeRegionInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl>>,
}

impl ComputeRegionInstanceGroupManagerStatusElStatefulEl {
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
                    ::Into<
                        terrars::ListField<ComputeRegionInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl>,
                    >,
    ) -> Self {
        self.per_instance_configs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionInstanceGroupManagerStatusElStatefulEl {
    type O = terrars::BlockAssignable<ComputeRegionInstanceGroupManagerStatusElStatefulEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceGroupManagerStatusElStatefulEl {}

impl BuildComputeRegionInstanceGroupManagerStatusElStatefulEl {
    pub fn build(self) -> ComputeRegionInstanceGroupManagerStatusElStatefulEl {
        ComputeRegionInstanceGroupManagerStatusElStatefulEl {
            has_stateful_config: core::default::Default::default(),
            per_instance_configs: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionInstanceGroupManagerStatusElStatefulElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionInstanceGroupManagerStatusElStatefulElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionInstanceGroupManagerStatusElStatefulElRef {
        ComputeRegionInstanceGroupManagerStatusElStatefulElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceGroupManagerStatusElStatefulElRef {
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
    ) -> terrars::ListRef<ComputeRegionInstanceGroupManagerStatusElStatefulElPerInstanceConfigsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.per_instance_configs", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceGroupManagerStatusElVersionTargetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_reached: std::option::Option<terrars::PrimField<bool>>,
}

impl ComputeRegionInstanceGroupManagerStatusElVersionTargetEl {
    #[doc = "Set the field `is_reached`.\n"]
    pub fn set_is_reached(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_reached = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionInstanceGroupManagerStatusElVersionTargetEl {
    type O = terrars::BlockAssignable<ComputeRegionInstanceGroupManagerStatusElVersionTargetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceGroupManagerStatusElVersionTargetEl {}

impl BuildComputeRegionInstanceGroupManagerStatusElVersionTargetEl {
    pub fn build(self) -> ComputeRegionInstanceGroupManagerStatusElVersionTargetEl {
        ComputeRegionInstanceGroupManagerStatusElVersionTargetEl { is_reached: core::default::Default::default() }
    }
}

pub struct ComputeRegionInstanceGroupManagerStatusElVersionTargetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionInstanceGroupManagerStatusElVersionTargetElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionInstanceGroupManagerStatusElVersionTargetElRef {
        ComputeRegionInstanceGroupManagerStatusElVersionTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceGroupManagerStatusElVersionTargetElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `is_reached` after provisioning.\n"]
    pub fn is_reached(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_reached", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceGroupManagerStatusEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_stable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateful: std::option::Option<terrars::ListField<ComputeRegionInstanceGroupManagerStatusElStatefulEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version_target: std::option::Option<terrars::ListField<ComputeRegionInstanceGroupManagerStatusElVersionTargetEl>>,
}

impl ComputeRegionInstanceGroupManagerStatusEl {
    #[doc = "Set the field `is_stable`.\n"]
    pub fn set_is_stable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_stable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stateful`.\n"]
    pub fn set_stateful(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ComputeRegionInstanceGroupManagerStatusElStatefulEl>>,
    ) -> Self {
        self.stateful = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version_target`.\n"]
    pub fn set_version_target(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ComputeRegionInstanceGroupManagerStatusElVersionTargetEl>>,
    ) -> Self {
        self.version_target = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionInstanceGroupManagerStatusEl {
    type O = terrars::BlockAssignable<ComputeRegionInstanceGroupManagerStatusEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceGroupManagerStatusEl {}

impl BuildComputeRegionInstanceGroupManagerStatusEl {
    pub fn build(self) -> ComputeRegionInstanceGroupManagerStatusEl {
        ComputeRegionInstanceGroupManagerStatusEl {
            is_stable: core::default::Default::default(),
            stateful: core::default::Default::default(),
            version_target: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionInstanceGroupManagerStatusElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionInstanceGroupManagerStatusElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRegionInstanceGroupManagerStatusElRef {
        ComputeRegionInstanceGroupManagerStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceGroupManagerStatusElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `is_stable` after provisioning.\n"]
    pub fn is_stable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_stable", self.base))
    }

    #[doc = "Get a reference to the value of field `stateful` after provisioning.\n"]
    pub fn stateful(&self) -> terrars::ListRef<ComputeRegionInstanceGroupManagerStatusElStatefulElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateful", self.base))
    }

    #[doc = "Get a reference to the value of field `version_target` after provisioning.\n"]
    pub fn version_target(&self) -> terrars::ListRef<ComputeRegionInstanceGroupManagerStatusElVersionTargetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.version_target", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceGroupManagerAutoHealingPoliciesEl {
    health_check: terrars::PrimField<std::string::String>,
    initial_delay_sec: terrars::PrimField<f64>,
}

impl ComputeRegionInstanceGroupManagerAutoHealingPoliciesEl { }

impl terrars::ToListMappable for ComputeRegionInstanceGroupManagerAutoHealingPoliciesEl {
    type O = terrars::BlockAssignable<ComputeRegionInstanceGroupManagerAutoHealingPoliciesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceGroupManagerAutoHealingPoliciesEl {
    #[doc = "The health check resource that signals autohealing."]
    pub health_check: terrars::PrimField<std::string::String>,
    #[doc =
        "The number of seconds that the managed instance group waits before it applies autohealing policies to new instances or recently recreated instances. Between 0 and 3600."]
    pub initial_delay_sec: terrars::PrimField<f64>,
}

impl BuildComputeRegionInstanceGroupManagerAutoHealingPoliciesEl {
    pub fn build(self) -> ComputeRegionInstanceGroupManagerAutoHealingPoliciesEl {
        ComputeRegionInstanceGroupManagerAutoHealingPoliciesEl {
            health_check: self.health_check,
            initial_delay_sec: self.initial_delay_sec,
        }
    }
}

pub struct ComputeRegionInstanceGroupManagerAutoHealingPoliciesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionInstanceGroupManagerAutoHealingPoliciesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionInstanceGroupManagerAutoHealingPoliciesElRef {
        ComputeRegionInstanceGroupManagerAutoHealingPoliciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceGroupManagerAutoHealingPoliciesElRef {
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
pub struct ComputeRegionInstanceGroupManagerInstanceLifecyclePolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_update_on_repair: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionInstanceGroupManagerInstanceLifecyclePolicyEl {
    #[doc =
        "Set the field `force_update_on_repair`.\nSpecifies whether to apply the group's latest configuration when repairing a VM. Valid options are: YES, NO. If YES and you updated the group's instance template or per-instance configurations after the VM was created, then these changes are applied when VM is repaired. If NO (default), then updates are applied in accordance with the group's update policy type."]
    pub fn set_force_update_on_repair(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.force_update_on_repair = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionInstanceGroupManagerInstanceLifecyclePolicyEl {
    type O = terrars::BlockAssignable<ComputeRegionInstanceGroupManagerInstanceLifecyclePolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceGroupManagerInstanceLifecyclePolicyEl {}

impl BuildComputeRegionInstanceGroupManagerInstanceLifecyclePolicyEl {
    pub fn build(self) -> ComputeRegionInstanceGroupManagerInstanceLifecyclePolicyEl {
        ComputeRegionInstanceGroupManagerInstanceLifecyclePolicyEl {
            force_update_on_repair: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionInstanceGroupManagerInstanceLifecyclePolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionInstanceGroupManagerInstanceLifecyclePolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionInstanceGroupManagerInstanceLifecyclePolicyElRef {
        ComputeRegionInstanceGroupManagerInstanceLifecyclePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceGroupManagerInstanceLifecyclePolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `force_update_on_repair` after provisioning.\nSpecifies whether to apply the group's latest configuration when repairing a VM. Valid options are: YES, NO. If YES and you updated the group's instance template or per-instance configurations after the VM was created, then these changes are applied when VM is repaired. If NO (default), then updates are applied in accordance with the group's update policy type."]
    pub fn force_update_on_repair(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_update_on_repair", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceGroupManagerNamedPortEl {
    name: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<f64>,
}

impl ComputeRegionInstanceGroupManagerNamedPortEl { }

impl terrars::ToListMappable for ComputeRegionInstanceGroupManagerNamedPortEl {
    type O = terrars::BlockAssignable<ComputeRegionInstanceGroupManagerNamedPortEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceGroupManagerNamedPortEl {
    #[doc = "The name of the port."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The port number."]
    pub port: terrars::PrimField<f64>,
}

impl BuildComputeRegionInstanceGroupManagerNamedPortEl {
    pub fn build(self) -> ComputeRegionInstanceGroupManagerNamedPortEl {
        ComputeRegionInstanceGroupManagerNamedPortEl {
            name: self.name,
            port: self.port,
        }
    }
}

pub struct ComputeRegionInstanceGroupManagerNamedPortElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionInstanceGroupManagerNamedPortElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionInstanceGroupManagerNamedPortElRef {
        ComputeRegionInstanceGroupManagerNamedPortElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceGroupManagerNamedPortElRef {
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
pub struct ComputeRegionInstanceGroupManagerStatefulDiskEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_rule: std::option::Option<terrars::PrimField<std::string::String>>,
    device_name: terrars::PrimField<std::string::String>,
}

impl ComputeRegionInstanceGroupManagerStatefulDiskEl {
    #[doc =
        "Set the field `delete_rule`.\nA value that prescribes what should happen to the stateful disk when the VM instance is deleted. The available options are NEVER and ON_PERMANENT_INSTANCE_DELETION. NEVER - detach the disk when the VM is deleted, but do not delete the disk. ON_PERMANENT_INSTANCE_DELETION will delete the stateful disk when the VM is permanently deleted from the instance group. The default is NEVER."]
    pub fn set_delete_rule(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete_rule = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeRegionInstanceGroupManagerStatefulDiskEl {
    type O = terrars::BlockAssignable<ComputeRegionInstanceGroupManagerStatefulDiskEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceGroupManagerStatefulDiskEl {
    #[doc = "The device name of the disk to be attached."]
    pub device_name: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionInstanceGroupManagerStatefulDiskEl {
    pub fn build(self) -> ComputeRegionInstanceGroupManagerStatefulDiskEl {
        ComputeRegionInstanceGroupManagerStatefulDiskEl {
            delete_rule: core::default::Default::default(),
            device_name: self.device_name,
        }
    }
}

pub struct ComputeRegionInstanceGroupManagerStatefulDiskElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionInstanceGroupManagerStatefulDiskElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionInstanceGroupManagerStatefulDiskElRef {
        ComputeRegionInstanceGroupManagerStatefulDiskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceGroupManagerStatefulDiskElRef {
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
pub struct ComputeRegionInstanceGroupManagerStatefulExternalIpEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_rule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interface_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionInstanceGroupManagerStatefulExternalIpEl {
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

impl terrars::ToListMappable for ComputeRegionInstanceGroupManagerStatefulExternalIpEl {
    type O = terrars::BlockAssignable<ComputeRegionInstanceGroupManagerStatefulExternalIpEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceGroupManagerStatefulExternalIpEl {}

impl BuildComputeRegionInstanceGroupManagerStatefulExternalIpEl {
    pub fn build(self) -> ComputeRegionInstanceGroupManagerStatefulExternalIpEl {
        ComputeRegionInstanceGroupManagerStatefulExternalIpEl {
            delete_rule: core::default::Default::default(),
            interface_name: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionInstanceGroupManagerStatefulExternalIpElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionInstanceGroupManagerStatefulExternalIpElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionInstanceGroupManagerStatefulExternalIpElRef {
        ComputeRegionInstanceGroupManagerStatefulExternalIpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceGroupManagerStatefulExternalIpElRef {
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
pub struct ComputeRegionInstanceGroupManagerStatefulInternalIpEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_rule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interface_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionInstanceGroupManagerStatefulInternalIpEl {
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

impl terrars::ToListMappable for ComputeRegionInstanceGroupManagerStatefulInternalIpEl {
    type O = terrars::BlockAssignable<ComputeRegionInstanceGroupManagerStatefulInternalIpEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceGroupManagerStatefulInternalIpEl {}

impl BuildComputeRegionInstanceGroupManagerStatefulInternalIpEl {
    pub fn build(self) -> ComputeRegionInstanceGroupManagerStatefulInternalIpEl {
        ComputeRegionInstanceGroupManagerStatefulInternalIpEl {
            delete_rule: core::default::Default::default(),
            interface_name: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionInstanceGroupManagerStatefulInternalIpElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionInstanceGroupManagerStatefulInternalIpElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionInstanceGroupManagerStatefulInternalIpElRef {
        ComputeRegionInstanceGroupManagerStatefulInternalIpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceGroupManagerStatefulInternalIpElRef {
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
pub struct ComputeRegionInstanceGroupManagerTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeRegionInstanceGroupManagerTimeoutsEl {
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

impl terrars::ToListMappable for ComputeRegionInstanceGroupManagerTimeoutsEl {
    type O = terrars::BlockAssignable<ComputeRegionInstanceGroupManagerTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceGroupManagerTimeoutsEl {}

impl BuildComputeRegionInstanceGroupManagerTimeoutsEl {
    pub fn build(self) -> ComputeRegionInstanceGroupManagerTimeoutsEl {
        ComputeRegionInstanceGroupManagerTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionInstanceGroupManagerTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionInstanceGroupManagerTimeoutsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionInstanceGroupManagerTimeoutsElRef {
        ComputeRegionInstanceGroupManagerTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceGroupManagerTimeoutsElRef {
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
pub struct ComputeRegionInstanceGroupManagerUpdatePolicyEl {
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

impl ComputeRegionInstanceGroupManagerUpdatePolicyEl {
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
        "Set the field `max_surge_fixed`.\nThe maximum number of instances that can be created above the specified targetSize during the update process. Conflicts with max_surge_percent. It has to be either 0 or at least equal to the number of zones. If fixed values are used, at least one of max_unavailable_fixed or max_surge_fixed must be greater than 0."]
    pub fn set_max_surge_fixed(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_surge_fixed = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_surge_percent`.\nThe maximum number of instances(calculated as percentage) that can be created above the specified targetSize during the update process. Conflicts with max_surge_fixed. Percent value is only allowed for regional managed instance groups with size at least 10."]
    pub fn set_max_surge_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_surge_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_unavailable_fixed`.\nThe maximum number of instances that can be unavailable during the update process. Conflicts with max_unavailable_percent. It has to be either 0 or at least equal to the number of zones. If fixed values are used, at least one of max_unavailable_fixed or max_surge_fixed must be greater than 0."]
    pub fn set_max_unavailable_fixed(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_unavailable_fixed = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_unavailable_percent`.\nThe maximum number of instances(calculated as percentage) that can be unavailable during the update process. Conflicts with max_unavailable_fixed. Percent value is only allowed for regional managed instance groups with size at least 10."]
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

impl terrars::ToListMappable for ComputeRegionInstanceGroupManagerUpdatePolicyEl {
    type O = terrars::BlockAssignable<ComputeRegionInstanceGroupManagerUpdatePolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceGroupManagerUpdatePolicyEl {
    #[doc =
        "Minimal action to be taken on an instance. You can specify either REFRESH to update without stopping instances, RESTART to restart existing instances or REPLACE to delete and create new instances from the target template. If you specify a REFRESH, the Updater will attempt to perform that action only. However, if the Updater determines that the minimal action you specify is not enough to perform the update, it might perform a more disruptive action."]
    pub minimal_action: terrars::PrimField<std::string::String>,
    #[doc =
        "The type of update process. You can specify either PROACTIVE so that the instance group manager proactively executes actions in order to bring instances to their target versions or OPPORTUNISTIC so that no action is proactively executed but the update will be performed as part of other actions (for example, resizes or recreateInstances calls)."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionInstanceGroupManagerUpdatePolicyEl {
    pub fn build(self) -> ComputeRegionInstanceGroupManagerUpdatePolicyEl {
        ComputeRegionInstanceGroupManagerUpdatePolicyEl {
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

pub struct ComputeRegionInstanceGroupManagerUpdatePolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionInstanceGroupManagerUpdatePolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionInstanceGroupManagerUpdatePolicyElRef {
        ComputeRegionInstanceGroupManagerUpdatePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceGroupManagerUpdatePolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `instance_redistribution_type` after provisioning.\nThe instance redistribution policy for regional managed instance groups. Valid values are: \"PROACTIVE\", \"NONE\". If PROACTIVE (default), the group attempts to maintain an even distribution of VM instances across zones in the region. If NONE, proactive redistribution is disabled."]
    pub fn instance_redistribution_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_redistribution_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_surge_fixed` after provisioning.\nThe maximum number of instances that can be created above the specified targetSize during the update process. Conflicts with max_surge_percent. It has to be either 0 or at least equal to the number of zones. If fixed values are used, at least one of max_unavailable_fixed or max_surge_fixed must be greater than 0."]
    pub fn max_surge_fixed(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_surge_fixed", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_surge_percent` after provisioning.\nThe maximum number of instances(calculated as percentage) that can be created above the specified targetSize during the update process. Conflicts with max_surge_fixed. Percent value is only allowed for regional managed instance groups with size at least 10."]
    pub fn max_surge_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_surge_percent", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_unavailable_fixed` after provisioning.\nThe maximum number of instances that can be unavailable during the update process. Conflicts with max_unavailable_percent. It has to be either 0 or at least equal to the number of zones. If fixed values are used, at least one of max_unavailable_fixed or max_surge_fixed must be greater than 0."]
    pub fn max_unavailable_fixed(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_unavailable_fixed", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_unavailable_percent` after provisioning.\nThe maximum number of instances(calculated as percentage) that can be unavailable during the update process. Conflicts with max_unavailable_fixed. Percent value is only allowed for regional managed instance groups with size at least 10."]
    pub fn max_unavailable_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_unavailable_percent", self.base))
    }

    #[doc =
        "Get a reference to the value of field `minimal_action` after provisioning.\nMinimal action to be taken on an instance. You can specify either REFRESH to update without stopping instances, RESTART to restart existing instances or REPLACE to delete and create new instances from the target template. If you specify a REFRESH, the Updater will attempt to perform that action only. However, if the Updater determines that the minimal action you specify is not enough to perform the update, it might perform a more disruptive action."]
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
pub struct ComputeRegionInstanceGroupManagerVersionElTargetSizeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percent: std::option::Option<terrars::PrimField<f64>>,
}

impl ComputeRegionInstanceGroupManagerVersionElTargetSizeEl {
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

impl terrars::ToListMappable for ComputeRegionInstanceGroupManagerVersionElTargetSizeEl {
    type O = terrars::BlockAssignable<ComputeRegionInstanceGroupManagerVersionElTargetSizeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceGroupManagerVersionElTargetSizeEl {}

impl BuildComputeRegionInstanceGroupManagerVersionElTargetSizeEl {
    pub fn build(self) -> ComputeRegionInstanceGroupManagerVersionElTargetSizeEl {
        ComputeRegionInstanceGroupManagerVersionElTargetSizeEl {
            fixed: core::default::Default::default(),
            percent: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionInstanceGroupManagerVersionElTargetSizeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionInstanceGroupManagerVersionElTargetSizeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeRegionInstanceGroupManagerVersionElTargetSizeElRef {
        ComputeRegionInstanceGroupManagerVersionElTargetSizeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceGroupManagerVersionElTargetSizeElRef {
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
struct ComputeRegionInstanceGroupManagerVersionElDynamic {
    target_size: std::option::Option<terrars::DynamicBlock<ComputeRegionInstanceGroupManagerVersionElTargetSizeEl>>,
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceGroupManagerVersionEl {
    instance_template: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_size: std::option::Option<std::vec::Vec<ComputeRegionInstanceGroupManagerVersionElTargetSizeEl>>,
    dynamic: ComputeRegionInstanceGroupManagerVersionElDynamic,
}

impl ComputeRegionInstanceGroupManagerVersionEl {
    #[doc = "Set the field `name`.\nVersion name."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_size`.\n"]
    pub fn set_target_size(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeRegionInstanceGroupManagerVersionElTargetSizeEl>>,
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

impl terrars::ToListMappable for ComputeRegionInstanceGroupManagerVersionEl {
    type O = terrars::BlockAssignable<ComputeRegionInstanceGroupManagerVersionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceGroupManagerVersionEl {
    #[doc = "The full URL to an instance template from which all new instances of this version will be created."]
    pub instance_template: terrars::PrimField<std::string::String>,
}

impl BuildComputeRegionInstanceGroupManagerVersionEl {
    pub fn build(self) -> ComputeRegionInstanceGroupManagerVersionEl {
        ComputeRegionInstanceGroupManagerVersionEl {
            instance_template: self.instance_template,
            name: core::default::Default::default(),
            target_size: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionInstanceGroupManagerVersionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeRegionInstanceGroupManagerVersionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeRegionInstanceGroupManagerVersionElRef {
        ComputeRegionInstanceGroupManagerVersionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceGroupManagerVersionElRef {
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
    pub fn target_size(&self) -> terrars::ListRef<ComputeRegionInstanceGroupManagerVersionElTargetSizeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_size", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionInstanceGroupManagerDynamic {
    auto_healing_policies: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionInstanceGroupManagerAutoHealingPoliciesEl>>,
    instance_lifecycle_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionInstanceGroupManagerInstanceLifecyclePolicyEl>>,
    named_port: std::option::Option<terrars::DynamicBlock<ComputeRegionInstanceGroupManagerNamedPortEl>>,
    stateful_disk: std::option::Option<terrars::DynamicBlock<ComputeRegionInstanceGroupManagerStatefulDiskEl>>,
    stateful_external_ip: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionInstanceGroupManagerStatefulExternalIpEl>>,
    stateful_internal_ip: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeRegionInstanceGroupManagerStatefulInternalIpEl>>,
    update_policy: std::option::Option<terrars::DynamicBlock<ComputeRegionInstanceGroupManagerUpdatePolicyEl>>,
    version: std::option::Option<terrars::DynamicBlock<ComputeRegionInstanceGroupManagerVersionEl>>,
}
