use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataComputeInstanceGroupManagerData {
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
    data: std::cell::RefCell<DataComputeInstanceGroupManagerData>,
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
        "Get a reference to the value of field `auto_healing_policies` after provisioning.\nThe autohealing policies for this managed instance group. You can specify only one value."]
    pub fn auto_healing_policies(&self) -> terrars::ListRef<DataComputeInstanceGroupManagerAutoHealingPoliciesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_healing_policies", self.extract_ref()))
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
        "Get a reference to the value of field `instance_lifecycle_policy` after provisioning.\nThe instance lifecycle policy for this managed instance group."]
    pub fn instance_lifecycle_policy(
        &self,
    ) -> terrars::ListRef<DataComputeInstanceGroupManagerInstanceLifecyclePolicyElRef> {
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
    pub fn named_port(&self) -> terrars::SetRef<DataComputeInstanceGroupManagerNamedPortElRef> {
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

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URL of the created resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `stateful_disk` after provisioning.\nDisks created on the instances that will be preserved on instance delete, update, etc."]
    pub fn stateful_disk(&self) -> terrars::SetRef<DataComputeInstanceGroupManagerStatefulDiskElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.stateful_disk", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `stateful_external_ip` after provisioning.\nExternal IPs considered stateful by the instance group. "]
    pub fn stateful_external_ip(&self) -> terrars::ListRef<DataComputeInstanceGroupManagerStatefulExternalIpElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateful_external_ip", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `stateful_internal_ip` after provisioning.\nExternal IPs considered stateful by the instance group. "]
    pub fn stateful_internal_ip(&self) -> terrars::ListRef<DataComputeInstanceGroupManagerStatefulInternalIpElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateful_internal_ip", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe status of this managed instance group."]
    pub fn status(&self) -> terrars::ListRef<DataComputeInstanceGroupManagerStatusElRef> {
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
        "Get a reference to the value of field `update_policy` after provisioning.\nThe update policy for this managed instance group."]
    pub fn update_policy(&self) -> terrars::ListRef<DataComputeInstanceGroupManagerUpdatePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.update_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nApplication versions managed by this instance group. Each version deals with a specific instance template, allowing canary release scenarios."]
    pub fn version(&self) -> terrars::ListRef<DataComputeInstanceGroupManagerVersionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
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
            data: std::cell::RefCell::new(DataComputeInstanceGroupManagerData {
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
        "Get a reference to the value of field `auto_healing_policies` after provisioning.\nThe autohealing policies for this managed instance group. You can specify only one value."]
    pub fn auto_healing_policies(&self) -> terrars::ListRef<DataComputeInstanceGroupManagerAutoHealingPoliciesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_healing_policies", self.extract_ref()))
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
        "Get a reference to the value of field `instance_lifecycle_policy` after provisioning.\nThe instance lifecycle policy for this managed instance group."]
    pub fn instance_lifecycle_policy(
        &self,
    ) -> terrars::ListRef<DataComputeInstanceGroupManagerInstanceLifecyclePolicyElRef> {
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
    pub fn named_port(&self) -> terrars::SetRef<DataComputeInstanceGroupManagerNamedPortElRef> {
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

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URL of the created resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `stateful_disk` after provisioning.\nDisks created on the instances that will be preserved on instance delete, update, etc."]
    pub fn stateful_disk(&self) -> terrars::SetRef<DataComputeInstanceGroupManagerStatefulDiskElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.stateful_disk", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `stateful_external_ip` after provisioning.\nExternal IPs considered stateful by the instance group. "]
    pub fn stateful_external_ip(&self) -> terrars::ListRef<DataComputeInstanceGroupManagerStatefulExternalIpElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateful_external_ip", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `stateful_internal_ip` after provisioning.\nExternal IPs considered stateful by the instance group. "]
    pub fn stateful_internal_ip(&self) -> terrars::ListRef<DataComputeInstanceGroupManagerStatefulInternalIpElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateful_internal_ip", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nThe status of this managed instance group."]
    pub fn status(&self) -> terrars::ListRef<DataComputeInstanceGroupManagerStatusElRef> {
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
        "Get a reference to the value of field `update_policy` after provisioning.\nThe update policy for this managed instance group."]
    pub fn update_policy(&self) -> terrars::ListRef<DataComputeInstanceGroupManagerUpdatePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.update_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nApplication versions managed by this instance group. Each version deals with a specific instance template, allowing canary release scenarios."]
    pub fn version(&self) -> terrars::ListRef<DataComputeInstanceGroupManagerVersionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
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
}

#[derive(Serialize)]
pub struct DataComputeInstanceGroupManagerAutoHealingPoliciesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_delay_sec: std::option::Option<terrars::PrimField<f64>>,
}

impl DataComputeInstanceGroupManagerAutoHealingPoliciesEl {
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

impl terrars::ToListMappable for DataComputeInstanceGroupManagerAutoHealingPoliciesEl {
    type O = terrars::BlockAssignable<DataComputeInstanceGroupManagerAutoHealingPoliciesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceGroupManagerAutoHealingPoliciesEl {}

impl BuildDataComputeInstanceGroupManagerAutoHealingPoliciesEl {
    pub fn build(self) -> DataComputeInstanceGroupManagerAutoHealingPoliciesEl {
        DataComputeInstanceGroupManagerAutoHealingPoliciesEl {
            health_check: core::default::Default::default(),
            initial_delay_sec: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceGroupManagerAutoHealingPoliciesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceGroupManagerAutoHealingPoliciesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceGroupManagerAutoHealingPoliciesElRef {
        DataComputeInstanceGroupManagerAutoHealingPoliciesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceGroupManagerAutoHealingPoliciesElRef {
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
pub struct DataComputeInstanceGroupManagerInstanceLifecyclePolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_update_on_repair: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeInstanceGroupManagerInstanceLifecyclePolicyEl {
    #[doc = "Set the field `force_update_on_repair`.\n"]
    pub fn set_force_update_on_repair(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.force_update_on_repair = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeInstanceGroupManagerInstanceLifecyclePolicyEl {
    type O = terrars::BlockAssignable<DataComputeInstanceGroupManagerInstanceLifecyclePolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceGroupManagerInstanceLifecyclePolicyEl {}

impl BuildDataComputeInstanceGroupManagerInstanceLifecyclePolicyEl {
    pub fn build(self) -> DataComputeInstanceGroupManagerInstanceLifecyclePolicyEl {
        DataComputeInstanceGroupManagerInstanceLifecyclePolicyEl {
            force_update_on_repair: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceGroupManagerInstanceLifecyclePolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceGroupManagerInstanceLifecyclePolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceGroupManagerInstanceLifecyclePolicyElRef {
        DataComputeInstanceGroupManagerInstanceLifecyclePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceGroupManagerInstanceLifecyclePolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `force_update_on_repair` after provisioning.\n"]
    pub fn force_update_on_repair(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_update_on_repair", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceGroupManagerNamedPortEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
}

impl DataComputeInstanceGroupManagerNamedPortEl {
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

impl terrars::ToListMappable for DataComputeInstanceGroupManagerNamedPortEl {
    type O = terrars::BlockAssignable<DataComputeInstanceGroupManagerNamedPortEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceGroupManagerNamedPortEl {}

impl BuildDataComputeInstanceGroupManagerNamedPortEl {
    pub fn build(self) -> DataComputeInstanceGroupManagerNamedPortEl {
        DataComputeInstanceGroupManagerNamedPortEl {
            name: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceGroupManagerNamedPortElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceGroupManagerNamedPortElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeInstanceGroupManagerNamedPortElRef {
        DataComputeInstanceGroupManagerNamedPortElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceGroupManagerNamedPortElRef {
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
pub struct DataComputeInstanceGroupManagerStatefulDiskEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_rule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeInstanceGroupManagerStatefulDiskEl {
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

impl terrars::ToListMappable for DataComputeInstanceGroupManagerStatefulDiskEl {
    type O = terrars::BlockAssignable<DataComputeInstanceGroupManagerStatefulDiskEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceGroupManagerStatefulDiskEl {}

impl BuildDataComputeInstanceGroupManagerStatefulDiskEl {
    pub fn build(self) -> DataComputeInstanceGroupManagerStatefulDiskEl {
        DataComputeInstanceGroupManagerStatefulDiskEl {
            delete_rule: core::default::Default::default(),
            device_name: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceGroupManagerStatefulDiskElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceGroupManagerStatefulDiskElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceGroupManagerStatefulDiskElRef {
        DataComputeInstanceGroupManagerStatefulDiskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceGroupManagerStatefulDiskElRef {
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
pub struct DataComputeInstanceGroupManagerStatefulExternalIpEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_rule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interface_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeInstanceGroupManagerStatefulExternalIpEl {
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

impl terrars::ToListMappable for DataComputeInstanceGroupManagerStatefulExternalIpEl {
    type O = terrars::BlockAssignable<DataComputeInstanceGroupManagerStatefulExternalIpEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceGroupManagerStatefulExternalIpEl {}

impl BuildDataComputeInstanceGroupManagerStatefulExternalIpEl {
    pub fn build(self) -> DataComputeInstanceGroupManagerStatefulExternalIpEl {
        DataComputeInstanceGroupManagerStatefulExternalIpEl {
            delete_rule: core::default::Default::default(),
            interface_name: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceGroupManagerStatefulExternalIpElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceGroupManagerStatefulExternalIpElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceGroupManagerStatefulExternalIpElRef {
        DataComputeInstanceGroupManagerStatefulExternalIpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceGroupManagerStatefulExternalIpElRef {
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
pub struct DataComputeInstanceGroupManagerStatefulInternalIpEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_rule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interface_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeInstanceGroupManagerStatefulInternalIpEl {
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

impl terrars::ToListMappable for DataComputeInstanceGroupManagerStatefulInternalIpEl {
    type O = terrars::BlockAssignable<DataComputeInstanceGroupManagerStatefulInternalIpEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceGroupManagerStatefulInternalIpEl {}

impl BuildDataComputeInstanceGroupManagerStatefulInternalIpEl {
    pub fn build(self) -> DataComputeInstanceGroupManagerStatefulInternalIpEl {
        DataComputeInstanceGroupManagerStatefulInternalIpEl {
            delete_rule: core::default::Default::default(),
            interface_name: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceGroupManagerStatefulInternalIpElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceGroupManagerStatefulInternalIpElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceGroupManagerStatefulInternalIpElRef {
        DataComputeInstanceGroupManagerStatefulInternalIpElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceGroupManagerStatefulInternalIpElRef {
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
pub struct DataComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all_effective: std::option::Option<terrars::PrimField<bool>>,
}

impl DataComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl {
    #[doc = "Set the field `all_effective`.\n"]
    pub fn set_all_effective(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.all_effective = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl {
    type O = terrars::BlockAssignable<DataComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl {}

impl BuildDataComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl {
    pub fn build(self) -> DataComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl {
        DataComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl {
            all_effective: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsElRef {
        DataComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `all_effective` after provisioning.\n"]
    pub fn all_effective(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.all_effective", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceGroupManagerStatusElStatefulEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    has_stateful_config: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_instance_configs: std
    ::option
    ::Option<terrars::ListField<DataComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl>>,
}

impl DataComputeInstanceGroupManagerStatusElStatefulEl {
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
                    ::Into<terrars::ListField<DataComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsEl>>,
    ) -> Self {
        self.per_instance_configs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeInstanceGroupManagerStatusElStatefulEl {
    type O = terrars::BlockAssignable<DataComputeInstanceGroupManagerStatusElStatefulEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceGroupManagerStatusElStatefulEl {}

impl BuildDataComputeInstanceGroupManagerStatusElStatefulEl {
    pub fn build(self) -> DataComputeInstanceGroupManagerStatusElStatefulEl {
        DataComputeInstanceGroupManagerStatusElStatefulEl {
            has_stateful_config: core::default::Default::default(),
            per_instance_configs: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceGroupManagerStatusElStatefulElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceGroupManagerStatusElStatefulElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceGroupManagerStatusElStatefulElRef {
        DataComputeInstanceGroupManagerStatusElStatefulElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceGroupManagerStatusElStatefulElRef {
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
    ) -> terrars::ListRef<DataComputeInstanceGroupManagerStatusElStatefulElPerInstanceConfigsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.per_instance_configs", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceGroupManagerStatusElVersionTargetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_reached: std::option::Option<terrars::PrimField<bool>>,
}

impl DataComputeInstanceGroupManagerStatusElVersionTargetEl {
    #[doc = "Set the field `is_reached`.\n"]
    pub fn set_is_reached(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_reached = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeInstanceGroupManagerStatusElVersionTargetEl {
    type O = terrars::BlockAssignable<DataComputeInstanceGroupManagerStatusElVersionTargetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceGroupManagerStatusElVersionTargetEl {}

impl BuildDataComputeInstanceGroupManagerStatusElVersionTargetEl {
    pub fn build(self) -> DataComputeInstanceGroupManagerStatusElVersionTargetEl {
        DataComputeInstanceGroupManagerStatusElVersionTargetEl { is_reached: core::default::Default::default() }
    }
}

pub struct DataComputeInstanceGroupManagerStatusElVersionTargetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceGroupManagerStatusElVersionTargetElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceGroupManagerStatusElVersionTargetElRef {
        DataComputeInstanceGroupManagerStatusElVersionTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceGroupManagerStatusElVersionTargetElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `is_reached` after provisioning.\n"]
    pub fn is_reached(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_reached", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceGroupManagerStatusEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_stable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateful: std::option::Option<terrars::ListField<DataComputeInstanceGroupManagerStatusElStatefulEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version_target: std::option::Option<terrars::ListField<DataComputeInstanceGroupManagerStatusElVersionTargetEl>>,
}

impl DataComputeInstanceGroupManagerStatusEl {
    #[doc = "Set the field `is_stable`.\n"]
    pub fn set_is_stable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_stable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stateful`.\n"]
    pub fn set_stateful(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeInstanceGroupManagerStatusElStatefulEl>>,
    ) -> Self {
        self.stateful = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version_target`.\n"]
    pub fn set_version_target(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeInstanceGroupManagerStatusElVersionTargetEl>>,
    ) -> Self {
        self.version_target = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeInstanceGroupManagerStatusEl {
    type O = terrars::BlockAssignable<DataComputeInstanceGroupManagerStatusEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceGroupManagerStatusEl {}

impl BuildDataComputeInstanceGroupManagerStatusEl {
    pub fn build(self) -> DataComputeInstanceGroupManagerStatusEl {
        DataComputeInstanceGroupManagerStatusEl {
            is_stable: core::default::Default::default(),
            stateful: core::default::Default::default(),
            version_target: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceGroupManagerStatusElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceGroupManagerStatusElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeInstanceGroupManagerStatusElRef {
        DataComputeInstanceGroupManagerStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceGroupManagerStatusElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `is_stable` after provisioning.\n"]
    pub fn is_stable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_stable", self.base))
    }

    #[doc = "Get a reference to the value of field `stateful` after provisioning.\n"]
    pub fn stateful(&self) -> terrars::ListRef<DataComputeInstanceGroupManagerStatusElStatefulElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateful", self.base))
    }

    #[doc = "Get a reference to the value of field `version_target` after provisioning.\n"]
    pub fn version_target(&self) -> terrars::ListRef<DataComputeInstanceGroupManagerStatusElVersionTargetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.version_target", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceGroupManagerUpdatePolicyEl {
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

impl DataComputeInstanceGroupManagerUpdatePolicyEl {
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

impl terrars::ToListMappable for DataComputeInstanceGroupManagerUpdatePolicyEl {
    type O = terrars::BlockAssignable<DataComputeInstanceGroupManagerUpdatePolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceGroupManagerUpdatePolicyEl {}

impl BuildDataComputeInstanceGroupManagerUpdatePolicyEl {
    pub fn build(self) -> DataComputeInstanceGroupManagerUpdatePolicyEl {
        DataComputeInstanceGroupManagerUpdatePolicyEl {
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

pub struct DataComputeInstanceGroupManagerUpdatePolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceGroupManagerUpdatePolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceGroupManagerUpdatePolicyElRef {
        DataComputeInstanceGroupManagerUpdatePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceGroupManagerUpdatePolicyElRef {
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
pub struct DataComputeInstanceGroupManagerVersionElTargetSizeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percent: std::option::Option<terrars::PrimField<f64>>,
}

impl DataComputeInstanceGroupManagerVersionElTargetSizeEl {
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

impl terrars::ToListMappable for DataComputeInstanceGroupManagerVersionElTargetSizeEl {
    type O = terrars::BlockAssignable<DataComputeInstanceGroupManagerVersionElTargetSizeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceGroupManagerVersionElTargetSizeEl {}

impl BuildDataComputeInstanceGroupManagerVersionElTargetSizeEl {
    pub fn build(self) -> DataComputeInstanceGroupManagerVersionElTargetSizeEl {
        DataComputeInstanceGroupManagerVersionElTargetSizeEl {
            fixed: core::default::Default::default(),
            percent: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceGroupManagerVersionElTargetSizeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceGroupManagerVersionElTargetSizeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceGroupManagerVersionElTargetSizeElRef {
        DataComputeInstanceGroupManagerVersionElTargetSizeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceGroupManagerVersionElTargetSizeElRef {
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
pub struct DataComputeInstanceGroupManagerVersionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_template: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_size: std::option::Option<terrars::ListField<DataComputeInstanceGroupManagerVersionElTargetSizeEl>>,
}

impl DataComputeInstanceGroupManagerVersionEl {
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
        v: impl std::convert::Into<terrars::ListField<DataComputeInstanceGroupManagerVersionElTargetSizeEl>>,
    ) -> Self {
        self.target_size = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeInstanceGroupManagerVersionEl {
    type O = terrars::BlockAssignable<DataComputeInstanceGroupManagerVersionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceGroupManagerVersionEl {}

impl BuildDataComputeInstanceGroupManagerVersionEl {
    pub fn build(self) -> DataComputeInstanceGroupManagerVersionEl {
        DataComputeInstanceGroupManagerVersionEl {
            instance_template: core::default::Default::default(),
            name: core::default::Default::default(),
            target_size: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceGroupManagerVersionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceGroupManagerVersionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeInstanceGroupManagerVersionElRef {
        DataComputeInstanceGroupManagerVersionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceGroupManagerVersionElRef {
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
    pub fn target_size(&self) -> terrars::ListRef<DataComputeInstanceGroupManagerVersionElTargetSizeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_size", self.base))
    }
}
