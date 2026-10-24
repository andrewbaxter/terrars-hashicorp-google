use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeInstanceGroupManagerData {
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
    wait_for_instances: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wait_for_instances_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_healing_policies: std::option::Option<std::vec::Vec<ComputeInstanceGroupManagerAutoHealingPoliciesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_lifecycle_policy: std::option::Option<std::vec::Vec<ComputeInstanceGroupManagerInstanceLifecyclePolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    named_port: std::option::Option<std::vec::Vec<ComputeInstanceGroupManagerNamedPortEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateful_disk: std::option::Option<std::vec::Vec<ComputeInstanceGroupManagerStatefulDiskEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateful_external_ip: std::option::Option<std::vec::Vec<ComputeInstanceGroupManagerStatefulExternalIpEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateful_internal_ip: std::option::Option<std::vec::Vec<ComputeInstanceGroupManagerStatefulInternalIpEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeInstanceGroupManagerTimeoutsEl>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_policy: std::option::Option<std::vec::Vec<ComputeInstanceGroupManagerUpdatePolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<std::vec::Vec<ComputeInstanceGroupManagerVersionEl>>,
    dynamic: ComputeInstanceGroupManagerDynamic,
}

struct ComputeInstanceGroupManager_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeInstanceGroupManagerData>,
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

    #[doc = "Set the field `zone`.\nThe zone that instances in this group should be created in."]
    pub fn set_zone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auto_healing_policies`.\n"]
    pub fn set_auto_healing_policies(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstanceGroupManagerAutoHealingPoliciesEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstanceGroupManagerInstanceLifecyclePolicyEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstanceGroupManagerNamedPortEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstanceGroupManagerStatefulDiskEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstanceGroupManagerStatefulExternalIpEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstanceGroupManagerStatefulInternalIpEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeInstanceGroupManagerTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_policy`.\n"]
    pub fn set_update_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstanceGroupManagerUpdatePolicyEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstanceGroupManagerVersionEl>>,
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
    pub fn status(&self) -> terrars::ListRef<ComputeInstanceGroupManagerStatusElRef> {
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

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nThe zone that instances in this group should be created in."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_healing_policies` after provisioning.\n"]
    pub fn auto_healing_policies(&self) -> terrars::ListRef<ComputeInstanceGroupManagerAutoHealingPoliciesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_healing_policies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_lifecycle_policy` after provisioning.\n"]
    pub fn instance_lifecycle_policy(&self) -> terrars::ListRef<ComputeInstanceGroupManagerInstanceLifecyclePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_lifecycle_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stateful_external_ip` after provisioning.\n"]
    pub fn stateful_external_ip(&self) -> terrars::ListRef<ComputeInstanceGroupManagerStatefulExternalIpElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateful_external_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stateful_internal_ip` after provisioning.\n"]
    pub fn stateful_internal_ip(&self) -> terrars::ListRef<ComputeInstanceGroupManagerStatefulInternalIpElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateful_internal_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeInstanceGroupManagerTimeoutsElRef {
        ComputeInstanceGroupManagerTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `update_policy` after provisioning.\n"]
    pub fn update_policy(&self) -> terrars::ListRef<ComputeInstanceGroupManagerUpdatePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.update_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::ListRef<ComputeInstanceGroupManagerVersionElRef> {
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
            data: std::cell::RefCell::new(ComputeInstanceGroupManagerData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                base_instance_name: self.base_instance_name,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                list_managed_instances_results: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                target_pools: core::default::Default::default(),
                target_size: core::default::Default::default(),
                wait_for_instances: core::default::Default::default(),
                wait_for_instances_status: core::default::Default::default(),
                zone: core::default::Default::default(),
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
    pub fn status(&self) -> terrars::ListRef<ComputeInstanceGroupManagerStatusElRef> {
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

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nThe zone that instances in this group should be created in."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_healing_policies` after provisioning.\n"]
    pub fn auto_healing_policies(&self) -> terrars::ListRef<ComputeInstanceGroupManagerAutoHealingPoliciesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_healing_policies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_lifecycle_policy` after provisioning.\n"]
    pub fn instance_lifecycle_policy(&self) -> terrars::ListRef<ComputeInstanceGroupManagerInstanceLifecyclePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_lifecycle_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stateful_external_ip` after provisioning.\n"]
    pub fn stateful_external_ip(&self) -> terrars::ListRef<ComputeInstanceGroupManagerStatefulExternalIpElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateful_external_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stateful_internal_ip` after provisioning.\n"]
    pub fn stateful_internal_ip(&self) -> terrars::ListRef<ComputeInstanceGroupManagerStatefulInternalIpElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateful_internal_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeInstanceGroupManagerTimeoutsElRef {
        ComputeInstanceGroupManagerTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `update_policy` after provisioning.\n"]
    pub fn update_policy(&self) -> terrars::ListRef<ComputeInstanceGroupManagerUpdatePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.update_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::ListRef<ComputeInstanceGroupManagerVersionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all_effective: std::option::Option<terrars::PrimField<bool>>,
}

impl ComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl {
    #[doc = "Set the field `all_effective`.\n"]
    pub fn set_all_effective(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.all_effective = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl {
    type O = terrars::BlockAssignable<ComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl {}

impl BuildComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl {
    pub fn build(self) -> ComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl {
        ComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl {
            all_effective: core::default::Default::default(),
        }
    }
}

pub struct ComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsElRef {
        ComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `all_effective` after provisioning.\n"]
    pub fn all_effective(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.all_effective", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInstanceGroupManagerStatusElStatefulEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    has_stateful_config: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_instance_configs: std
    ::option
    ::Option<terrars::ListField<ComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl>>,
}

impl ComputeInstanceGroupManagerStatusElStatefulEl {
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
                    ::Into<terrars::ListField<ComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl>>,
    ) -> Self {
        self.per_instance_configs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInstanceGroupManagerStatusElStatefulEl {
    type O = terrars::BlockAssignable<ComputeInstanceGroupManagerStatusElStatefulEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceGroupManagerStatusElStatefulEl {}

impl BuildComputeInstanceGroupManagerStatusElStatefulEl {
    pub fn build(self) -> ComputeInstanceGroupManagerStatusElStatefulEl {
        ComputeInstanceGroupManagerStatusElStatefulEl {
            has_stateful_config: core::default::Default::default(),
            per_instance_configs: core::default::Default::default(),
        }
    }
}

pub struct ComputeInstanceGroupManagerStatusElStatefulElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceGroupManagerStatusElStatefulElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstanceGroupManagerStatusElStatefulElRef {
        ComputeInstanceGroupManagerStatusElStatefulElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceGroupManagerStatusElStatefulElRef {
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
    ) -> terrars::ListRef<ComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.per_instance_configs", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInstanceGroupManagerStatusElVersionTargetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_reached: std::option::Option<terrars::PrimField<bool>>,
}

impl ComputeInstanceGroupManagerStatusElVersionTargetEl {
    #[doc = "Set the field `is_reached`.\n"]
    pub fn set_is_reached(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_reached = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInstanceGroupManagerStatusElVersionTargetEl {
    type O = terrars::BlockAssignable<ComputeInstanceGroupManagerStatusElVersionTargetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceGroupManagerStatusElVersionTargetEl {}

impl BuildComputeInstanceGroupManagerStatusElVersionTargetEl {
    pub fn build(self) -> ComputeInstanceGroupManagerStatusElVersionTargetEl {
        ComputeInstanceGroupManagerStatusElVersionTargetEl { is_reached: core::default::Default::default() }
    }
}

pub struct ComputeInstanceGroupManagerStatusElVersionTargetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceGroupManagerStatusElVersionTargetElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstanceGroupManagerStatusElVersionTargetElRef {
        ComputeInstanceGroupManagerStatusElVersionTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceGroupManagerStatusElVersionTargetElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `is_reached` after provisioning.\n"]
    pub fn is_reached(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_reached", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInstanceGroupManagerStatusEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_stable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateful: std::option::Option<terrars::ListField<ComputeInstanceGroupManagerStatusElStatefulEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version_target: std::option::Option<terrars::ListField<ComputeInstanceGroupManagerStatusElVersionTargetEl>>,
}

impl ComputeInstanceGroupManagerStatusEl {
    #[doc = "Set the field `is_stable`.\n"]
    pub fn set_is_stable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_stable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stateful`.\n"]
    pub fn set_stateful(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ComputeInstanceGroupManagerStatusElStatefulEl>>,
    ) -> Self {
        self.stateful = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version_target`.\n"]
    pub fn set_version_target(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ComputeInstanceGroupManagerStatusElVersionTargetEl>>,
    ) -> Self {
        self.version_target = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInstanceGroupManagerStatusEl {
    type O = terrars::BlockAssignable<ComputeInstanceGroupManagerStatusEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceGroupManagerStatusEl {}

impl BuildComputeInstanceGroupManagerStatusEl {
    pub fn build(self) -> ComputeInstanceGroupManagerStatusEl {
        ComputeInstanceGroupManagerStatusEl {
            is_stable: core::default::Default::default(),
            stateful: core::default::Default::default(),
            version_target: core::default::Default::default(),
        }
    }
}

pub struct ComputeInstanceGroupManagerStatusElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceGroupManagerStatusElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInstanceGroupManagerStatusElRef {
        ComputeInstanceGroupManagerStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceGroupManagerStatusElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `is_stable` after provisioning.\n"]
    pub fn is_stable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_stable", self.base))
    }

    #[doc = "Get a reference to the value of field `stateful` after provisioning.\n"]
    pub fn stateful(&self) -> terrars::ListRef<ComputeInstanceGroupManagerStatusElStatefulElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateful", self.base))
    }

    #[doc = "Get a reference to the value of field `version_target` after provisioning.\n"]
    pub fn version_target(&self) -> terrars::ListRef<ComputeInstanceGroupManagerStatusElVersionTargetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.version_target", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInstanceGroupManagerAutoHealingPoliciesEl {
    health_check: terrars::PrimField<std::string::String>,
    initial_delay_sec: terrars::PrimField<f64>,
}

impl ComputeInstanceGroupManagerAutoHealingPoliciesEl { }

impl terrars::ToListMappable for ComputeInstanceGroupManagerAutoHealingPoliciesEl {
    type O = terrars::BlockAssignable<ComputeInstanceGroupManagerAutoHealingPoliciesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceGroupManagerAutoHealingPoliciesEl {
    #[doc = "The health check resource that signals autohealing."]
    pub health_check: terrars::PrimField<std::string::String>,
    #[doc =
        "The number of seconds that the managed instance group waits before it applies autohealing policies to new instances or recently recreated instances. Between 0 and 3600."]
    pub initial_delay_sec: terrars::PrimField<f64>,
}

impl BuildComputeInstanceGroupManagerAutoHealingPoliciesEl {
    pub fn build(self) -> ComputeInstanceGroupManagerAutoHealingPoliciesEl {
        ComputeInstanceGroupManagerAutoHealingPoliciesEl {
            health_check: self.health_check,
            initial_delay_sec: self.initial_delay_sec,
        }
    }
}

pub struct ComputeInstanceGroupManagerAutoHealingPoliciesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceGroupManagerAutoHealingPoliciesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstanceGroupManagerAutoHealingPoliciesElRef {
        ComputeInstanceGroupManagerAutoHealingPoliciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceGroupManagerAutoHealingPoliciesElRef {
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
pub struct ComputeInstanceGroupManagerInstanceLifecyclePolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_update_on_repair: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeInstanceGroupManagerInstanceLifecyclePolicyEl {
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

impl terrars::ToListMappable for ComputeInstanceGroupManagerInstanceLifecyclePolicyEl {
    type O = terrars::BlockAssignable<ComputeInstanceGroupManagerInstanceLifecyclePolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceGroupManagerInstanceLifecyclePolicyEl {}

impl BuildComputeInstanceGroupManagerInstanceLifecyclePolicyEl {
    pub fn build(self) -> ComputeInstanceGroupManagerInstanceLifecyclePolicyEl {
        ComputeInstanceGroupManagerInstanceLifecyclePolicyEl {
            force_update_on_repair: core::default::Default::default(),
        }
    }
}

pub struct ComputeInstanceGroupManagerInstanceLifecyclePolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceGroupManagerInstanceLifecyclePolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstanceGroupManagerInstanceLifecyclePolicyElRef {
        ComputeInstanceGroupManagerInstanceLifecyclePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceGroupManagerInstanceLifecyclePolicyElRef {
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
pub struct ComputeInstanceGroupManagerNamedPortEl {
    name: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<f64>,
}

impl ComputeInstanceGroupManagerNamedPortEl { }

impl terrars::ToListMappable for ComputeInstanceGroupManagerNamedPortEl {
    type O = terrars::BlockAssignable<ComputeInstanceGroupManagerNamedPortEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceGroupManagerNamedPortEl {
    #[doc = "The name of the port."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The port number."]
    pub port: terrars::PrimField<f64>,
}

impl BuildComputeInstanceGroupManagerNamedPortEl {
    pub fn build(self) -> ComputeInstanceGroupManagerNamedPortEl {
        ComputeInstanceGroupManagerNamedPortEl {
            name: self.name,
            port: self.port,
        }
    }
}

pub struct ComputeInstanceGroupManagerNamedPortElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceGroupManagerNamedPortElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInstanceGroupManagerNamedPortElRef {
        ComputeInstanceGroupManagerNamedPortElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceGroupManagerNamedPortElRef {
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
pub struct ComputeInstanceGroupManagerStatefulDiskEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_rule: std::option::Option<terrars::PrimField<std::string::String>>,
    device_name: terrars::PrimField<std::string::String>,
}

impl ComputeInstanceGroupManagerStatefulDiskEl {
    #[doc =
        "Set the field `delete_rule`.\nA value that prescribes what should happen to the stateful disk when the VM instance is deleted. The available options are NEVER and ON_PERMANENT_INSTANCE_DELETION. NEVER - detach the disk when the VM is deleted, but do not delete the disk. ON_PERMANENT_INSTANCE_DELETION will delete the stateful disk when the VM is permanently deleted from the instance group. The default is NEVER."]
    pub fn set_delete_rule(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete_rule = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInstanceGroupManagerStatefulDiskEl {
    type O = terrars::BlockAssignable<ComputeInstanceGroupManagerStatefulDiskEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceGroupManagerStatefulDiskEl {
    #[doc = "The device name of the disk to be attached."]
    pub device_name: terrars::PrimField<std::string::String>,
}

impl BuildComputeInstanceGroupManagerStatefulDiskEl {
    pub fn build(self) -> ComputeInstanceGroupManagerStatefulDiskEl {
        ComputeInstanceGroupManagerStatefulDiskEl {
            delete_rule: core::default::Default::default(),
            device_name: self.device_name,
        }
    }
}

pub struct ComputeInstanceGroupManagerStatefulDiskElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceGroupManagerStatefulDiskElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInstanceGroupManagerStatefulDiskElRef {
        ComputeInstanceGroupManagerStatefulDiskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceGroupManagerStatefulDiskElRef {
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
pub struct ComputeInstanceGroupManagerStatefulExternalIpEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_rule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interface_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeInstanceGroupManagerStatefulExternalIpEl {
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

impl terrars::ToListMappable for ComputeInstanceGroupManagerStatefulExternalIpEl {
    type O = terrars::BlockAssignable<ComputeInstanceGroupManagerStatefulExternalIpEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceGroupManagerStatefulExternalIpEl {}

impl BuildComputeInstanceGroupManagerStatefulExternalIpEl {
    pub fn build(self) -> ComputeInstanceGroupManagerStatefulExternalIpEl {
        ComputeInstanceGroupManagerStatefulExternalIpEl {
            delete_rule: core::default::Default::default(),
            interface_name: core::default::Default::default(),
        }
    }
}

pub struct ComputeInstanceGroupManagerStatefulExternalIpElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceGroupManagerStatefulExternalIpElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstanceGroupManagerStatefulExternalIpElRef {
        ComputeInstanceGroupManagerStatefulExternalIpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceGroupManagerStatefulExternalIpElRef {
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
pub struct ComputeInstanceGroupManagerStatefulInternalIpEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_rule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interface_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeInstanceGroupManagerStatefulInternalIpEl {
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

impl terrars::ToListMappable for ComputeInstanceGroupManagerStatefulInternalIpEl {
    type O = terrars::BlockAssignable<ComputeInstanceGroupManagerStatefulInternalIpEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceGroupManagerStatefulInternalIpEl {}

impl BuildComputeInstanceGroupManagerStatefulInternalIpEl {
    pub fn build(self) -> ComputeInstanceGroupManagerStatefulInternalIpEl {
        ComputeInstanceGroupManagerStatefulInternalIpEl {
            delete_rule: core::default::Default::default(),
            interface_name: core::default::Default::default(),
        }
    }
}

pub struct ComputeInstanceGroupManagerStatefulInternalIpElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceGroupManagerStatefulInternalIpElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstanceGroupManagerStatefulInternalIpElRef {
        ComputeInstanceGroupManagerStatefulInternalIpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceGroupManagerStatefulInternalIpElRef {
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
pub struct ComputeInstanceGroupManagerTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeInstanceGroupManagerTimeoutsEl {
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

impl terrars::ToListMappable for ComputeInstanceGroupManagerTimeoutsEl {
    type O = terrars::BlockAssignable<ComputeInstanceGroupManagerTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceGroupManagerTimeoutsEl {}

impl BuildComputeInstanceGroupManagerTimeoutsEl {
    pub fn build(self) -> ComputeInstanceGroupManagerTimeoutsEl {
        ComputeInstanceGroupManagerTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ComputeInstanceGroupManagerTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceGroupManagerTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInstanceGroupManagerTimeoutsElRef {
        ComputeInstanceGroupManagerTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceGroupManagerTimeoutsElRef {
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
pub struct ComputeInstanceGroupManagerUpdatePolicyEl {
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

impl ComputeInstanceGroupManagerUpdatePolicyEl {
    #[doc =
        "Set the field `max_surge_fixed`.\nThe maximum number of instances that can be created above the specified targetSize during the update process. Conflicts with max_surge_percent. If neither is set, defaults to 1"]
    pub fn set_max_surge_fixed(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_surge_fixed = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_surge_percent`.\nThe maximum number of instances(calculated as percentage) that can be created above the specified targetSize during the update process. Conflicts with max_surge_fixed."]
    pub fn set_max_surge_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_surge_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_unavailable_fixed`.\nThe maximum number of instances that can be unavailable during the update process. Conflicts with max_unavailable_percent. If neither is set, defaults to 1."]
    pub fn set_max_unavailable_fixed(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_unavailable_fixed = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_unavailable_percent`.\nThe maximum number of instances(calculated as percentage) that can be unavailable during the update process. Conflicts with max_unavailable_fixed."]
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

impl terrars::ToListMappable for ComputeInstanceGroupManagerUpdatePolicyEl {
    type O = terrars::BlockAssignable<ComputeInstanceGroupManagerUpdatePolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceGroupManagerUpdatePolicyEl {
    #[doc =
        "Minimal action to be taken on an instance. You can specify either REFRESH to update without stopping instances, RESTART to restart existing instances or REPLACE to delete and create new instances from the target template. If you specify a REFRESH, the Updater will attempt to perform that action only. However, if the Updater determines that the minimal action you specify is not enough to perform the update, it might perform a more disruptive action."]
    pub minimal_action: terrars::PrimField<std::string::String>,
    #[doc =
        "The type of update process. You can specify either PROACTIVE so that the instance group manager proactively executes actions in order to bring instances to their target versions or OPPORTUNISTIC so that no action is proactively executed but the update will be performed as part of other actions (for example, resizes or recreateInstances calls)."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildComputeInstanceGroupManagerUpdatePolicyEl {
    pub fn build(self) -> ComputeInstanceGroupManagerUpdatePolicyEl {
        ComputeInstanceGroupManagerUpdatePolicyEl {
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

pub struct ComputeInstanceGroupManagerUpdatePolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceGroupManagerUpdatePolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInstanceGroupManagerUpdatePolicyElRef {
        ComputeInstanceGroupManagerUpdatePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceGroupManagerUpdatePolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_surge_fixed` after provisioning.\nThe maximum number of instances that can be created above the specified targetSize during the update process. Conflicts with max_surge_percent. If neither is set, defaults to 1"]
    pub fn max_surge_fixed(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_surge_fixed", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_surge_percent` after provisioning.\nThe maximum number of instances(calculated as percentage) that can be created above the specified targetSize during the update process. Conflicts with max_surge_fixed."]
    pub fn max_surge_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_surge_percent", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_unavailable_fixed` after provisioning.\nThe maximum number of instances that can be unavailable during the update process. Conflicts with max_unavailable_percent. If neither is set, defaults to 1."]
    pub fn max_unavailable_fixed(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_unavailable_fixed", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_unavailable_percent` after provisioning.\nThe maximum number of instances(calculated as percentage) that can be unavailable during the update process. Conflicts with max_unavailable_fixed."]
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
pub struct ComputeInstanceGroupManagerVersionElTargetSizeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percent: std::option::Option<terrars::PrimField<f64>>,
}

impl ComputeInstanceGroupManagerVersionElTargetSizeEl {
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

impl terrars::ToListMappable for ComputeInstanceGroupManagerVersionElTargetSizeEl {
    type O = terrars::BlockAssignable<ComputeInstanceGroupManagerVersionElTargetSizeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceGroupManagerVersionElTargetSizeEl {}

impl BuildComputeInstanceGroupManagerVersionElTargetSizeEl {
    pub fn build(self) -> ComputeInstanceGroupManagerVersionElTargetSizeEl {
        ComputeInstanceGroupManagerVersionElTargetSizeEl {
            fixed: core::default::Default::default(),
            percent: core::default::Default::default(),
        }
    }
}

pub struct ComputeInstanceGroupManagerVersionElTargetSizeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceGroupManagerVersionElTargetSizeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstanceGroupManagerVersionElTargetSizeElRef {
        ComputeInstanceGroupManagerVersionElTargetSizeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceGroupManagerVersionElTargetSizeElRef {
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
struct ComputeInstanceGroupManagerVersionElDynamic {
    target_size: std::option::Option<terrars::DynamicBlock<ComputeInstanceGroupManagerVersionElTargetSizeEl>>,
}

#[derive(Serialize)]
pub struct ComputeInstanceGroupManagerVersionEl {
    instance_template: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_size: std::option::Option<std::vec::Vec<ComputeInstanceGroupManagerVersionElTargetSizeEl>>,
    dynamic: ComputeInstanceGroupManagerVersionElDynamic,
}

impl ComputeInstanceGroupManagerVersionEl {
    #[doc = "Set the field `name`.\nVersion name."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_size`.\n"]
    pub fn set_target_size(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstanceGroupManagerVersionElTargetSizeEl>>,
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

impl terrars::ToListMappable for ComputeInstanceGroupManagerVersionEl {
    type O = terrars::BlockAssignable<ComputeInstanceGroupManagerVersionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceGroupManagerVersionEl {
    #[doc = "The full URL to an instance template from which all new instances of this version will be created."]
    pub instance_template: terrars::PrimField<std::string::String>,
}

impl BuildComputeInstanceGroupManagerVersionEl {
    pub fn build(self) -> ComputeInstanceGroupManagerVersionEl {
        ComputeInstanceGroupManagerVersionEl {
            instance_template: self.instance_template,
            name: core::default::Default::default(),
            target_size: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeInstanceGroupManagerVersionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceGroupManagerVersionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInstanceGroupManagerVersionElRef {
        ComputeInstanceGroupManagerVersionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceGroupManagerVersionElRef {
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
    pub fn target_size(&self) -> terrars::ListRef<ComputeInstanceGroupManagerVersionElTargetSizeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_size", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeInstanceGroupManagerDynamic {
    auto_healing_policies: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeInstanceGroupManagerAutoHealingPoliciesEl>>,
    instance_lifecycle_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeInstanceGroupManagerInstanceLifecyclePolicyEl>>,
    named_port: std::option::Option<terrars::DynamicBlock<ComputeInstanceGroupManagerNamedPortEl>>,
    stateful_disk: std::option::Option<terrars::DynamicBlock<ComputeInstanceGroupManagerStatefulDiskEl>>,
    stateful_external_ip: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeInstanceGroupManagerStatefulExternalIpEl>>,
    stateful_internal_ip: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeInstanceGroupManagerStatefulInternalIpEl>>,
    update_policy: std::option::Option<terrars::DynamicBlock<ComputeInstanceGroupManagerUpdatePolicyEl>>,
    version: std::option::Option<terrars::DynamicBlock<ComputeInstanceGroupManagerVersionEl>>,
}
