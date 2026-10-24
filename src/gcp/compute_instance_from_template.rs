use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeInstanceFromTemplateData {
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
    allow_stopping_for_update: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attached_disk: std::option::Option<terrars::ListField<ComputeInstanceFromTemplateAttachedDiskEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    can_ip_forward: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    desired_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_display: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guest_accelerator: std::option::Option<terrars::ListField<ComputeInstanceFromTemplateGuestAcceleratorEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata_startup_script: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_cpu_platform: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_policies: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scratch_disk: std::option::Option<terrars::ListField<ComputeInstanceFromTemplateScratchDiskEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::ListField<ComputeInstanceFromTemplateServiceAccountEl>>,
    source_instance_template: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_machine_features: std::option::Option<std::vec::Vec<ComputeInstanceFromTemplateAdvancedMachineFeaturesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk: std::option::Option<std::vec::Vec<ComputeInstanceFromTemplateBootDiskEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confidential_instance_config: std
    ::option
    ::Option<std::vec::Vec<ComputeInstanceFromTemplateConfidentialInstanceConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_interface: std::option::Option<std::vec::Vec<ComputeInstanceFromTemplateNetworkInterfaceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_performance_config: std::option::Option<std::vec::Vec<ComputeInstanceFromTemplateNetworkPerformanceConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    params: std::option::Option<std::vec::Vec<ComputeInstanceFromTemplateParamsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reservation_affinity: std::option::Option<std::vec::Vec<ComputeInstanceFromTemplateReservationAffinityEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheduling: std::option::Option<std::vec::Vec<ComputeInstanceFromTemplateSchedulingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shielded_instance_config: std::option::Option<std::vec::Vec<ComputeInstanceFromTemplateShieldedInstanceConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeInstanceFromTemplateTimeoutsEl>,
    dynamic: ComputeInstanceFromTemplateDynamic,
}

struct ComputeInstanceFromTemplate_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeInstanceFromTemplateData>,
}

#[derive(Clone)]
pub struct ComputeInstanceFromTemplate(std::rc::Rc<ComputeInstanceFromTemplate_>);

impl ComputeInstanceFromTemplate {
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
        "Set the field `allow_stopping_for_update`.\nIf true, allows Terraform to stop the instance to update its properties. If you try to update a property that requires stopping the instance without setting this field, the update will fail."]
    pub fn set_allow_stopping_for_update(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_stopping_for_update = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `attached_disk`.\nList of disks attached to the instance"]
    pub fn set_attached_disk(
        self,
        v: impl std::convert::Into<terrars::ListField<ComputeInstanceFromTemplateAttachedDiskEl>>,
    ) -> Self {
        self.0.data.borrow_mut().attached_disk = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `can_ip_forward`.\nWhether sending and receiving of packets with non-matching source or destination IPs is allowed."]
    pub fn set_can_ip_forward(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().can_ip_forward = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deletion_protection`.\nWhether deletion protection is enabled on this instance."]
    pub fn set_deletion_protection(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nA brief description of the resource."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `desired_status`.\nDesired status of the instance. Either \"RUNNING\" or \"TERMINATED\"."]
    pub fn set_desired_status(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().desired_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_display`.\nWhether the instance has virtual displays enabled."]
    pub fn set_enable_display(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_display = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `guest_accelerator`.\nList of the type and count of accelerator cards attached to the instance."]
    pub fn set_guest_accelerator(
        self,
        v: impl std::convert::Into<terrars::ListField<ComputeInstanceFromTemplateGuestAcceleratorEl>>,
    ) -> Self {
        self.0.data.borrow_mut().guest_accelerator = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `hostname`.\nA custom hostname for the instance. Must be a fully qualified DNS name and RFC-1035-valid. Valid format is a series of labels 1-63 characters long matching the regular expression [a-z]([-a-z0-9]*[a-z0-9]), concatenated with periods. The entire hostname must not exceed 253 characters. Changing this forces a new resource to be created."]
    pub fn set_hostname(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nA set of key/value label pairs assigned to the instance.\n\t\t\t\t\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `machine_type`.\nThe machine type to create."]
    pub fn set_machine_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().machine_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metadata`.\nMetadata key/value pairs made available within the instance."]
    pub fn set_metadata(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metadata_startup_script`.\nMetadata startup scripts made available within the instance."]
    pub fn set_metadata_startup_script(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().metadata_startup_script = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_cpu_platform`.\nThe minimum CPU platform specified for the VM instance."]
    pub fn set_min_cpu_platform(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().min_cpu_platform = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project`.\nThe ID of the project in which the resource belongs. If self_link is provided, this value is ignored. If neither self_link nor project are provided, the provider project is used."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resource_policies`.\nA list of self_links of resource policies to attach to the instance. Currently a max of 1 resource policy is supported."]
    pub fn set_resource_policies(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().resource_policies = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scratch_disk`.\nThe scratch disks attached to the instance."]
    pub fn set_scratch_disk(
        self,
        v: impl std::convert::Into<terrars::ListField<ComputeInstanceFromTemplateScratchDiskEl>>,
    ) -> Self {
        self.0.data.borrow_mut().scratch_disk = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_account`.\nThe service account to attach to the instance."]
    pub fn set_service_account(
        self,
        v: impl std::convert::Into<terrars::ListField<ComputeInstanceFromTemplateServiceAccountEl>>,
    ) -> Self {
        self.0.data.borrow_mut().service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\nThe list of tags attached to the instance."]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `zone`.\nThe zone of the instance. If self_link is provided, this value is ignored. If neither self_link nor zone are provided, the provider zone is used."]
    pub fn set_zone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `advanced_machine_features`.\n"]
    pub fn set_advanced_machine_features(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstanceFromTemplateAdvancedMachineFeaturesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().advanced_machine_features = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.advanced_machine_features = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `boot_disk`.\n"]
    pub fn set_boot_disk(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstanceFromTemplateBootDiskEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().boot_disk = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.boot_disk = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `confidential_instance_config`.\n"]
    pub fn set_confidential_instance_config(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeInstanceFromTemplateConfidentialInstanceConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().confidential_instance_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.confidential_instance_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_interface`.\n"]
    pub fn set_network_interface(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstanceFromTemplateNetworkInterfaceEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_interface = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_interface = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_performance_config`.\n"]
    pub fn set_network_performance_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstanceFromTemplateNetworkPerformanceConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_performance_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_performance_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `params`.\n"]
    pub fn set_params(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstanceFromTemplateParamsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().params = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.params = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `reservation_affinity`.\n"]
    pub fn set_reservation_affinity(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstanceFromTemplateReservationAffinityEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().reservation_affinity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.reservation_affinity = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scheduling`.\n"]
    pub fn set_scheduling(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstanceFromTemplateSchedulingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scheduling = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scheduling = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `shielded_instance_config`.\n"]
    pub fn set_shielded_instance_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstanceFromTemplateShieldedInstanceConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().shielded_instance_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.shielded_instance_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeInstanceFromTemplateTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `allow_stopping_for_update` after provisioning.\nIf true, allows Terraform to stop the instance to update its properties. If you try to update a property that requires stopping the instance without setting this field, the update will fail."]
    pub fn allow_stopping_for_update(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_stopping_for_update", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `attached_disk` after provisioning.\nList of disks attached to the instance"]
    pub fn attached_disk(&self) -> terrars::ListRef<ComputeInstanceFromTemplateAttachedDiskElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attached_disk", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `can_ip_forward` after provisioning.\nWhether sending and receiving of packets with non-matching source or destination IPs is allowed."]
    pub fn can_ip_forward(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.can_ip_forward", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cpu_platform` after provisioning.\nThe CPU platform used by this instance."]
    pub fn cpu_platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_platform", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `current_status` after provisioning.\n\n\t\t\t\t\tCurrent status of the instance.\n\t\t\t\t\tThis could be one of the following values: PROVISIONING, STAGING, RUNNING, STOPPING, SUSPENDING, SUSPENDED, REPAIRING, and TERMINATED.\n\t\t\t\t\tFor more information about the status of the instance, see [Instance life cycle](https://cloud.google.com/compute/docs/instances/instance-life-cycle)."]
    pub fn current_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.current_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether deletion protection is enabled on this instance."]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA brief description of the resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `desired_status` after provisioning.\nDesired status of the instance. Either \"RUNNING\" or \"TERMINATED\"."]
    pub fn desired_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_display` after provisioning.\nWhether the instance has virtual displays enabled."]
    pub fn enable_display(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_display", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `guest_accelerator` after provisioning.\nList of the type and count of accelerator cards attached to the instance."]
    pub fn guest_accelerator(&self) -> terrars::ListRef<ComputeInstanceFromTemplateGuestAcceleratorElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.guest_accelerator", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hostname` after provisioning.\nA custom hostname for the instance. Must be a fully qualified DNS name and RFC-1035-valid. Valid format is a series of labels 1-63 characters long matching the regular expression [a-z]([-a-z0-9]*[a-z0-9]), concatenated with periods. The entire hostname must not exceed 253 characters. Changing this forces a new resource to be created."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_id` after provisioning.\nThe server-assigned unique identifier of this instance."]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `label_fingerprint` after provisioning.\nThe unique fingerprint of the labels."]
    pub fn label_fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label_fingerprint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs assigned to the instance.\n\t\t\t\t\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\nThe machine type to create."]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `metadata` after provisioning.\nMetadata key/value pairs made available within the instance."]
    pub fn metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.metadata", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `metadata_fingerprint` after provisioning.\nThe unique fingerprint of the metadata."]
    pub fn metadata_fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata_fingerprint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `metadata_startup_script` after provisioning.\nMetadata startup scripts made available within the instance."]
    pub fn metadata_startup_script(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata_startup_script", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `min_cpu_platform` after provisioning.\nThe minimum CPU platform specified for the VM instance."]
    pub fn min_cpu_platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_cpu_platform", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the instance. One of name or self_link must be provided."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If self_link is provided, this value is ignored. If neither self_link nor project are provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_policies` after provisioning.\nA list of self_links of resource policies to attach to the instance. Currently a max of 1 resource policy is supported."]
    pub fn resource_policies(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_policies", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scratch_disk` after provisioning.\nThe scratch disks attached to the instance."]
    pub fn scratch_disk(&self) -> terrars::ListRef<ComputeInstanceFromTemplateScratchDiskElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scratch_disk", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nThe service account to attach to the instance."]
    pub fn service_account(&self) -> terrars::ListRef<ComputeInstanceFromTemplateServiceAccountElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source_instance_template` after provisioning.\nName or self link of an instance template to create the instance based on."]
    pub fn source_instance_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_instance_template", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nThe list of tags attached to the instance."]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tags_fingerprint` after provisioning.\nThe unique fingerprint of the tags."]
    pub fn tags_fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tags_fingerprint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nThe zone of the instance. If self_link is provided, this value is ignored. If neither self_link nor zone are provided, the provider zone is used."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `advanced_machine_features` after provisioning.\n"]
    pub fn advanced_machine_features(&self) -> terrars::ListRef<ComputeInstanceFromTemplateAdvancedMachineFeaturesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_machine_features", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `boot_disk` after provisioning.\n"]
    pub fn boot_disk(&self) -> terrars::ListRef<ComputeInstanceFromTemplateBootDiskElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.boot_disk", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `confidential_instance_config` after provisioning.\n"]
    pub fn confidential_instance_config(
        &self,
    ) -> terrars::ListRef<ComputeInstanceFromTemplateConfidentialInstanceConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.confidential_instance_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_interface` after provisioning.\n"]
    pub fn network_interface(&self) -> terrars::ListRef<ComputeInstanceFromTemplateNetworkInterfaceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_interface", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_performance_config` after provisioning.\n"]
    pub fn network_performance_config(
        &self,
    ) -> terrars::ListRef<ComputeInstanceFromTemplateNetworkPerformanceConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_performance_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> terrars::ListRef<ComputeInstanceFromTemplateParamsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.params", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reservation_affinity` after provisioning.\n"]
    pub fn reservation_affinity(&self) -> terrars::ListRef<ComputeInstanceFromTemplateReservationAffinityElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reservation_affinity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scheduling` after provisioning.\n"]
    pub fn scheduling(&self) -> terrars::ListRef<ComputeInstanceFromTemplateSchedulingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scheduling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `shielded_instance_config` after provisioning.\n"]
    pub fn shielded_instance_config(&self) -> terrars::ListRef<ComputeInstanceFromTemplateShieldedInstanceConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shielded_instance_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeInstanceFromTemplateTimeoutsElRef {
        ComputeInstanceFromTemplateTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for ComputeInstanceFromTemplate {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputeInstanceFromTemplate { }

impl terrars::ToListMappable for ComputeInstanceFromTemplate {
    type O = terrars::ListRef<ComputeInstanceFromTemplateRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputeInstanceFromTemplate_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_compute_instance_from_template".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeInstanceFromTemplate {
    pub tf_id: std::string::String,
    #[doc = "The name of the instance. One of name or self_link must be provided."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "Name or self link of an instance template to create the instance based on."]
    pub source_instance_template: terrars::PrimField<std::string::String>,
}

impl BuildComputeInstanceFromTemplate {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputeInstanceFromTemplate {
        let out = ComputeInstanceFromTemplate(std::rc::Rc::new(ComputeInstanceFromTemplate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputeInstanceFromTemplateData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                allow_stopping_for_update: core::default::Default::default(),
                attached_disk: core::default::Default::default(),
                can_ip_forward: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                description: core::default::Default::default(),
                desired_status: core::default::Default::default(),
                enable_display: core::default::Default::default(),
                guest_accelerator: core::default::Default::default(),
                hostname: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                machine_type: core::default::Default::default(),
                metadata: core::default::Default::default(),
                metadata_startup_script: core::default::Default::default(),
                min_cpu_platform: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                resource_policies: core::default::Default::default(),
                scratch_disk: core::default::Default::default(),
                service_account: core::default::Default::default(),
                source_instance_template: self.source_instance_template,
                tags: core::default::Default::default(),
                zone: core::default::Default::default(),
                advanced_machine_features: core::default::Default::default(),
                boot_disk: core::default::Default::default(),
                confidential_instance_config: core::default::Default::default(),
                network_interface: core::default::Default::default(),
                network_performance_config: core::default::Default::default(),
                params: core::default::Default::default(),
                reservation_affinity: core::default::Default::default(),
                scheduling: core::default::Default::default(),
                shielded_instance_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeInstanceFromTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceFromTemplateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeInstanceFromTemplateRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_stopping_for_update` after provisioning.\nIf true, allows Terraform to stop the instance to update its properties. If you try to update a property that requires stopping the instance without setting this field, the update will fail."]
    pub fn allow_stopping_for_update(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_stopping_for_update", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `attached_disk` after provisioning.\nList of disks attached to the instance"]
    pub fn attached_disk(&self) -> terrars::ListRef<ComputeInstanceFromTemplateAttachedDiskElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attached_disk", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `can_ip_forward` after provisioning.\nWhether sending and receiving of packets with non-matching source or destination IPs is allowed."]
    pub fn can_ip_forward(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.can_ip_forward", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cpu_platform` after provisioning.\nThe CPU platform used by this instance."]
    pub fn cpu_platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_platform", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `current_status` after provisioning.\n\n\t\t\t\t\tCurrent status of the instance.\n\t\t\t\t\tThis could be one of the following values: PROVISIONING, STAGING, RUNNING, STOPPING, SUSPENDING, SUSPENDED, REPAIRING, and TERMINATED.\n\t\t\t\t\tFor more information about the status of the instance, see [Instance life cycle](https://cloud.google.com/compute/docs/instances/instance-life-cycle)."]
    pub fn current_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.current_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether deletion protection is enabled on this instance."]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA brief description of the resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `desired_status` after provisioning.\nDesired status of the instance. Either \"RUNNING\" or \"TERMINATED\"."]
    pub fn desired_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_display` after provisioning.\nWhether the instance has virtual displays enabled."]
    pub fn enable_display(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_display", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `guest_accelerator` after provisioning.\nList of the type and count of accelerator cards attached to the instance."]
    pub fn guest_accelerator(&self) -> terrars::ListRef<ComputeInstanceFromTemplateGuestAcceleratorElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.guest_accelerator", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hostname` after provisioning.\nA custom hostname for the instance. Must be a fully qualified DNS name and RFC-1035-valid. Valid format is a series of labels 1-63 characters long matching the regular expression [a-z]([-a-z0-9]*[a-z0-9]), concatenated with periods. The entire hostname must not exceed 253 characters. Changing this forces a new resource to be created."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_id` after provisioning.\nThe server-assigned unique identifier of this instance."]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `label_fingerprint` after provisioning.\nThe unique fingerprint of the labels."]
    pub fn label_fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label_fingerprint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs assigned to the instance.\n\t\t\t\t\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\nThe machine type to create."]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `metadata` after provisioning.\nMetadata key/value pairs made available within the instance."]
    pub fn metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.metadata", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `metadata_fingerprint` after provisioning.\nThe unique fingerprint of the metadata."]
    pub fn metadata_fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata_fingerprint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `metadata_startup_script` after provisioning.\nMetadata startup scripts made available within the instance."]
    pub fn metadata_startup_script(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata_startup_script", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `min_cpu_platform` after provisioning.\nThe minimum CPU platform specified for the VM instance."]
    pub fn min_cpu_platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_cpu_platform", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the instance. One of name or self_link must be provided."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If self_link is provided, this value is ignored. If neither self_link nor project are provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_policies` after provisioning.\nA list of self_links of resource policies to attach to the instance. Currently a max of 1 resource policy is supported."]
    pub fn resource_policies(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_policies", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scratch_disk` after provisioning.\nThe scratch disks attached to the instance."]
    pub fn scratch_disk(&self) -> terrars::ListRef<ComputeInstanceFromTemplateScratchDiskElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scratch_disk", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nThe service account to attach to the instance."]
    pub fn service_account(&self) -> terrars::ListRef<ComputeInstanceFromTemplateServiceAccountElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source_instance_template` after provisioning.\nName or self link of an instance template to create the instance based on."]
    pub fn source_instance_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_instance_template", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nThe list of tags attached to the instance."]
    pub fn tags(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tags_fingerprint` after provisioning.\nThe unique fingerprint of the tags."]
    pub fn tags_fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tags_fingerprint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nThe zone of the instance. If self_link is provided, this value is ignored. If neither self_link nor zone are provided, the provider zone is used."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `advanced_machine_features` after provisioning.\n"]
    pub fn advanced_machine_features(&self) -> terrars::ListRef<ComputeInstanceFromTemplateAdvancedMachineFeaturesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_machine_features", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `boot_disk` after provisioning.\n"]
    pub fn boot_disk(&self) -> terrars::ListRef<ComputeInstanceFromTemplateBootDiskElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.boot_disk", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `confidential_instance_config` after provisioning.\n"]
    pub fn confidential_instance_config(
        &self,
    ) -> terrars::ListRef<ComputeInstanceFromTemplateConfidentialInstanceConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.confidential_instance_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_interface` after provisioning.\n"]
    pub fn network_interface(&self) -> terrars::ListRef<ComputeInstanceFromTemplateNetworkInterfaceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_interface", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_performance_config` after provisioning.\n"]
    pub fn network_performance_config(
        &self,
    ) -> terrars::ListRef<ComputeInstanceFromTemplateNetworkPerformanceConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_performance_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> terrars::ListRef<ComputeInstanceFromTemplateParamsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.params", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reservation_affinity` after provisioning.\n"]
    pub fn reservation_affinity(&self) -> terrars::ListRef<ComputeInstanceFromTemplateReservationAffinityElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reservation_affinity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scheduling` after provisioning.\n"]
    pub fn scheduling(&self) -> terrars::ListRef<ComputeInstanceFromTemplateSchedulingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scheduling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `shielded_instance_config` after provisioning.\n"]
    pub fn shielded_instance_config(&self) -> terrars::ListRef<ComputeInstanceFromTemplateShieldedInstanceConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shielded_instance_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeInstanceFromTemplateTimeoutsElRef {
        ComputeInstanceFromTemplateTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct ComputeInstanceFromTemplateAttachedDiskEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_encryption_key_raw: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_encryption_key_sha256: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_self_link: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeInstanceFromTemplateAttachedDiskEl {
    #[doc = "Set the field `device_name`.\n"]
    pub fn set_device_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.device_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk_encryption_key_raw`.\n"]
    pub fn set_disk_encryption_key_raw(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.disk_encryption_key_raw = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk_encryption_key_sha256`.\n"]
    pub fn set_disk_encryption_key_sha256(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.disk_encryption_key_sha256 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_self_link`.\n"]
    pub fn set_kms_key_self_link(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_self_link = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInstanceFromTemplateAttachedDiskEl {
    type O = terrars::BlockAssignable<ComputeInstanceFromTemplateAttachedDiskEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceFromTemplateAttachedDiskEl {}

impl BuildComputeInstanceFromTemplateAttachedDiskEl {
    pub fn build(self) -> ComputeInstanceFromTemplateAttachedDiskEl {
        ComputeInstanceFromTemplateAttachedDiskEl {
            device_name: core::default::Default::default(),
            disk_encryption_key_raw: core::default::Default::default(),
            disk_encryption_key_sha256: core::default::Default::default(),
            kms_key_self_link: core::default::Default::default(),
            mode: core::default::Default::default(),
            source: core::default::Default::default(),
        }
    }
}

pub struct ComputeInstanceFromTemplateAttachedDiskElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceFromTemplateAttachedDiskElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInstanceFromTemplateAttachedDiskElRef {
        ComputeInstanceFromTemplateAttachedDiskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceFromTemplateAttachedDiskElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `device_name` after provisioning.\n"]
    pub fn device_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }

    #[doc = "Get a reference to the value of field `disk_encryption_key_raw` after provisioning.\n"]
    pub fn disk_encryption_key_raw(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_encryption_key_raw", self.base))
    }

    #[doc = "Get a reference to the value of field `disk_encryption_key_sha256` after provisioning.\n"]
    pub fn disk_encryption_key_sha256(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_encryption_key_sha256", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_self_link` after provisioning.\n"]
    pub fn kms_key_self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_self_link", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInstanceFromTemplateGuestAcceleratorEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeInstanceFromTemplateGuestAcceleratorEl {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInstanceFromTemplateGuestAcceleratorEl {
    type O = terrars::BlockAssignable<ComputeInstanceFromTemplateGuestAcceleratorEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceFromTemplateGuestAcceleratorEl {}

impl BuildComputeInstanceFromTemplateGuestAcceleratorEl {
    pub fn build(self) -> ComputeInstanceFromTemplateGuestAcceleratorEl {
        ComputeInstanceFromTemplateGuestAcceleratorEl {
            count: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct ComputeInstanceFromTemplateGuestAcceleratorElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceFromTemplateGuestAcceleratorElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstanceFromTemplateGuestAcceleratorElRef {
        ComputeInstanceFromTemplateGuestAcceleratorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceFromTemplateGuestAcceleratorElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInstanceFromTemplateScratchDiskEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interface: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size: std::option::Option<terrars::PrimField<f64>>,
}

impl ComputeInstanceFromTemplateScratchDiskEl {
    #[doc = "Set the field `device_name`.\n"]
    pub fn set_device_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.device_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interface`.\n"]
    pub fn set_interface(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.interface = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `size`.\n"]
    pub fn set_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInstanceFromTemplateScratchDiskEl {
    type O = terrars::BlockAssignable<ComputeInstanceFromTemplateScratchDiskEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceFromTemplateScratchDiskEl {}

impl BuildComputeInstanceFromTemplateScratchDiskEl {
    pub fn build(self) -> ComputeInstanceFromTemplateScratchDiskEl {
        ComputeInstanceFromTemplateScratchDiskEl {
            device_name: core::default::Default::default(),
            interface: core::default::Default::default(),
            size: core::default::Default::default(),
        }
    }
}

pub struct ComputeInstanceFromTemplateScratchDiskElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceFromTemplateScratchDiskElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInstanceFromTemplateScratchDiskElRef {
        ComputeInstanceFromTemplateScratchDiskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceFromTemplateScratchDiskElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `device_name` after provisioning.\n"]
    pub fn device_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }

    #[doc = "Get a reference to the value of field `interface` after provisioning.\n"]
    pub fn interface(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interface", self.base))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInstanceFromTemplateServiceAccountEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scopes: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl ComputeInstanceFromTemplateServiceAccountEl {
    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scopes`.\n"]
    pub fn set_scopes(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.scopes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInstanceFromTemplateServiceAccountEl {
    type O = terrars::BlockAssignable<ComputeInstanceFromTemplateServiceAccountEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceFromTemplateServiceAccountEl {}

impl BuildComputeInstanceFromTemplateServiceAccountEl {
    pub fn build(self) -> ComputeInstanceFromTemplateServiceAccountEl {
        ComputeInstanceFromTemplateServiceAccountEl {
            email: core::default::Default::default(),
            scopes: core::default::Default::default(),
        }
    }
}

pub struct ComputeInstanceFromTemplateServiceAccountElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceFromTemplateServiceAccountElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstanceFromTemplateServiceAccountElRef {
        ComputeInstanceFromTemplateServiceAccountElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceFromTemplateServiceAccountElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `scopes` after provisioning.\n"]
    pub fn scopes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInstanceFromTemplateAdvancedMachineFeaturesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_nested_virtualization: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threads_per_core: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    visible_core_count: std::option::Option<terrars::PrimField<f64>>,
}

impl ComputeInstanceFromTemplateAdvancedMachineFeaturesEl {
    #[doc = "Set the field `enable_nested_virtualization`.\nWhether to enable nested virtualization or not."]
    pub fn set_enable_nested_virtualization(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_nested_virtualization = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `threads_per_core`.\nThe number of threads per physical core. To disable simultaneous multithreading (SMT) set this to 1. If unset, the maximum number of threads supported per core by the underlying processor is assumed."]
    pub fn set_threads_per_core(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.threads_per_core = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `visible_core_count`.\nThe number of physical cores to expose to an instance. Multiply by the number of threads per core to compute the total number of virtual CPUs to expose to the instance. If unset, the number of cores is inferred from the instance\\'s nominal CPU count and the underlying platform\\'s SMT width."]
    pub fn set_visible_core_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.visible_core_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInstanceFromTemplateAdvancedMachineFeaturesEl {
    type O = terrars::BlockAssignable<ComputeInstanceFromTemplateAdvancedMachineFeaturesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceFromTemplateAdvancedMachineFeaturesEl {}

impl BuildComputeInstanceFromTemplateAdvancedMachineFeaturesEl {
    pub fn build(self) -> ComputeInstanceFromTemplateAdvancedMachineFeaturesEl {
        ComputeInstanceFromTemplateAdvancedMachineFeaturesEl {
            enable_nested_virtualization: core::default::Default::default(),
            threads_per_core: core::default::Default::default(),
            visible_core_count: core::default::Default::default(),
        }
    }
}

pub struct ComputeInstanceFromTemplateAdvancedMachineFeaturesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceFromTemplateAdvancedMachineFeaturesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstanceFromTemplateAdvancedMachineFeaturesElRef {
        ComputeInstanceFromTemplateAdvancedMachineFeaturesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceFromTemplateAdvancedMachineFeaturesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_nested_virtualization` after provisioning.\nWhether to enable nested virtualization or not."]
    pub fn enable_nested_virtualization(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_nested_virtualization", self.base))
    }

    #[doc =
        "Get a reference to the value of field `threads_per_core` after provisioning.\nThe number of threads per physical core. To disable simultaneous multithreading (SMT) set this to 1. If unset, the maximum number of threads supported per core by the underlying processor is assumed."]
    pub fn threads_per_core(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threads_per_core", self.base))
    }

    #[doc =
        "Get a reference to the value of field `visible_core_count` after provisioning.\nThe number of physical cores to expose to an instance. Multiply by the number of threads per core to compute the total number of virtual CPUs to expose to the instance. If unset, the number of cores is inferred from the instance\\'s nominal CPU count and the underlying platform\\'s SMT width."]
    pub fn visible_core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.visible_core_count", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInstanceFromTemplateBootDiskElInitializeParamsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_manager_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeInstanceFromTemplateBootDiskElInitializeParamsEl {
    #[doc = "Set the field `image`.\nThe image from which this disk was initialised."]
    pub fn set_image(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `labels`.\nA set of key/value label pairs assigned to the disk."]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `resource_manager_tags`.\nA map of resource manager tags. Resource manager tag keys and values have the same definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/456. The field is ignored (both PUT & PATCH) when empty."]
    pub fn set_resource_manager_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resource_manager_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `size`.\nThe size of the image in gigabytes."]
    pub fn set_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\nThe Google Compute Engine disk type. Such as pd-standard, pd-ssd or pd-balanced."]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInstanceFromTemplateBootDiskElInitializeParamsEl {
    type O = terrars::BlockAssignable<ComputeInstanceFromTemplateBootDiskElInitializeParamsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceFromTemplateBootDiskElInitializeParamsEl {}

impl BuildComputeInstanceFromTemplateBootDiskElInitializeParamsEl {
    pub fn build(self) -> ComputeInstanceFromTemplateBootDiskElInitializeParamsEl {
        ComputeInstanceFromTemplateBootDiskElInitializeParamsEl {
            image: core::default::Default::default(),
            labels: core::default::Default::default(),
            resource_manager_tags: core::default::Default::default(),
            size: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct ComputeInstanceFromTemplateBootDiskElInitializeParamsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceFromTemplateBootDiskElInitializeParamsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstanceFromTemplateBootDiskElInitializeParamsElRef {
        ComputeInstanceFromTemplateBootDiskElInitializeParamsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceFromTemplateBootDiskElInitializeParamsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `image` after provisioning.\nThe image from which this disk was initialised."]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs assigned to the disk."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource_manager_tags` after provisioning.\nA map of resource manager tags. Resource manager tag keys and values have the same definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/456. The field is ignored (both PUT & PATCH) when empty."]
    pub fn resource_manager_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.resource_manager_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\nThe size of the image in gigabytes."]
    pub fn size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe Google Compute Engine disk type. Such as pd-standard, pd-ssd or pd-balanced."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeInstanceFromTemplateBootDiskElDynamic {
    initialize_params: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeInstanceFromTemplateBootDiskElInitializeParamsEl>>,
}

#[derive(Serialize)]
pub struct ComputeInstanceFromTemplateBootDiskEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_delete: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_encryption_key_raw: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_self_link: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initialize_params: std::option::Option<std::vec::Vec<ComputeInstanceFromTemplateBootDiskElInitializeParamsEl>>,
    dynamic: ComputeInstanceFromTemplateBootDiskElDynamic,
}

impl ComputeInstanceFromTemplateBootDiskEl {
    #[doc = "Set the field `auto_delete`.\nWhether the disk will be auto-deleted when the instance is deleted."]
    pub fn set_auto_delete(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.auto_delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_name`.\nName with which attached disk will be accessible under /dev/disk/by-id/"]
    pub fn set_device_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.device_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disk_encryption_key_raw`.\nA 256-bit customer-supplied encryption key, encoded in RFC 4648 base64 to encrypt this disk. Only one of kms_key_self_link and disk_encryption_key_raw may be set."]
    pub fn set_disk_encryption_key_raw(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.disk_encryption_key_raw = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kms_key_self_link`.\nThe self_link of the encryption key that is stored in Google Cloud KMS to encrypt this disk. Only one of kms_key_self_link and disk_encryption_key_raw may be set."]
    pub fn set_kms_key_self_link(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_self_link = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mode`.\nRead/write mode for the disk. One of \"READ_ONLY\" or \"READ_WRITE\"."]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source`.\nThe name or self_link of the disk attached to this instance."]
    pub fn set_source(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `initialize_params`.\n"]
    pub fn set_initialize_params(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeInstanceFromTemplateBootDiskElInitializeParamsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.initialize_params = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.initialize_params = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeInstanceFromTemplateBootDiskEl {
    type O = terrars::BlockAssignable<ComputeInstanceFromTemplateBootDiskEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceFromTemplateBootDiskEl {}

impl BuildComputeInstanceFromTemplateBootDiskEl {
    pub fn build(self) -> ComputeInstanceFromTemplateBootDiskEl {
        ComputeInstanceFromTemplateBootDiskEl {
            auto_delete: core::default::Default::default(),
            device_name: core::default::Default::default(),
            disk_encryption_key_raw: core::default::Default::default(),
            kms_key_self_link: core::default::Default::default(),
            mode: core::default::Default::default(),
            source: core::default::Default::default(),
            initialize_params: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeInstanceFromTemplateBootDiskElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceFromTemplateBootDiskElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInstanceFromTemplateBootDiskElRef {
        ComputeInstanceFromTemplateBootDiskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceFromTemplateBootDiskElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `auto_delete` after provisioning.\nWhether the disk will be auto-deleted when the instance is deleted."]
    pub fn auto_delete(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `device_name` after provisioning.\nName with which attached disk will be accessible under /dev/disk/by-id/"]
    pub fn device_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disk_encryption_key_raw` after provisioning.\nA 256-bit customer-supplied encryption key, encoded in RFC 4648 base64 to encrypt this disk. Only one of kms_key_self_link and disk_encryption_key_raw may be set."]
    pub fn disk_encryption_key_raw(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_encryption_key_raw", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disk_encryption_key_sha256` after provisioning.\nThe RFC 4648 base64 encoded SHA-256 hash of the customer-supplied encryption key that protects this resource."]
    pub fn disk_encryption_key_sha256(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_encryption_key_sha256", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_self_link` after provisioning.\nThe self_link of the encryption key that is stored in Google Cloud KMS to encrypt this disk. Only one of kms_key_self_link and disk_encryption_key_raw may be set."]
    pub fn kms_key_self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_self_link", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nRead/write mode for the disk. One of \"READ_ONLY\" or \"READ_WRITE\"."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source` after provisioning.\nThe name or self_link of the disk attached to this instance."]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc = "Get a reference to the value of field `initialize_params` after provisioning.\n"]
    pub fn initialize_params(&self) -> terrars::ListRef<ComputeInstanceFromTemplateBootDiskElInitializeParamsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.initialize_params", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInstanceFromTemplateConfidentialInstanceConfigEl {
    enable_confidential_compute: terrars::PrimField<bool>,
}

impl ComputeInstanceFromTemplateConfidentialInstanceConfigEl { }

impl terrars::ToListMappable for ComputeInstanceFromTemplateConfidentialInstanceConfigEl {
    type O = terrars::BlockAssignable<ComputeInstanceFromTemplateConfidentialInstanceConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceFromTemplateConfidentialInstanceConfigEl {
    #[doc = "Defines whether the instance should have confidential compute enabled."]
    pub enable_confidential_compute: terrars::PrimField<bool>,
}

impl BuildComputeInstanceFromTemplateConfidentialInstanceConfigEl {
    pub fn build(self) -> ComputeInstanceFromTemplateConfidentialInstanceConfigEl {
        ComputeInstanceFromTemplateConfidentialInstanceConfigEl {
            enable_confidential_compute: self.enable_confidential_compute,
        }
    }
}

pub struct ComputeInstanceFromTemplateConfidentialInstanceConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceFromTemplateConfidentialInstanceConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstanceFromTemplateConfidentialInstanceConfigElRef {
        ComputeInstanceFromTemplateConfidentialInstanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceFromTemplateConfidentialInstanceConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_confidential_compute` after provisioning.\nDefines whether the instance should have confidential compute enabled."]
    pub fn enable_confidential_compute(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_confidential_compute", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInstanceFromTemplateNetworkInterfaceElAccessConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nat_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_tier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_ptr_domain_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeInstanceFromTemplateNetworkInterfaceElAccessConfigEl {
    #[doc = "Set the field `nat_ip`.\n"]
    pub fn set_nat_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.nat_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_tier`.\n"]
    pub fn set_network_tier(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network_tier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `public_ptr_domain_name`.\n"]
    pub fn set_public_ptr_domain_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.public_ptr_domain_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInstanceFromTemplateNetworkInterfaceElAccessConfigEl {
    type O = terrars::BlockAssignable<ComputeInstanceFromTemplateNetworkInterfaceElAccessConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceFromTemplateNetworkInterfaceElAccessConfigEl {}

impl BuildComputeInstanceFromTemplateNetworkInterfaceElAccessConfigEl {
    pub fn build(self) -> ComputeInstanceFromTemplateNetworkInterfaceElAccessConfigEl {
        ComputeInstanceFromTemplateNetworkInterfaceElAccessConfigEl {
            nat_ip: core::default::Default::default(),
            network_tier: core::default::Default::default(),
            public_ptr_domain_name: core::default::Default::default(),
        }
    }
}

pub struct ComputeInstanceFromTemplateNetworkInterfaceElAccessConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceFromTemplateNetworkInterfaceElAccessConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstanceFromTemplateNetworkInterfaceElAccessConfigElRef {
        ComputeInstanceFromTemplateNetworkInterfaceElAccessConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceFromTemplateNetworkInterfaceElAccessConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `nat_ip` after provisioning.\n"]
    pub fn nat_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nat_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `network_tier` after provisioning.\n"]
    pub fn network_tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_tier", self.base))
    }

    #[doc = "Get a reference to the value of field `public_ptr_domain_name` after provisioning.\n"]
    pub fn public_ptr_domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_ptr_domain_name", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInstanceFromTemplateNetworkInterfaceElAliasIpRangeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_cidr_range: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork_range_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeInstanceFromTemplateNetworkInterfaceElAliasIpRangeEl {
    #[doc = "Set the field `ip_cidr_range`.\n"]
    pub fn set_ip_cidr_range(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_cidr_range = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnetwork_range_name`.\n"]
    pub fn set_subnetwork_range_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.subnetwork_range_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInstanceFromTemplateNetworkInterfaceElAliasIpRangeEl {
    type O = terrars::BlockAssignable<ComputeInstanceFromTemplateNetworkInterfaceElAliasIpRangeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceFromTemplateNetworkInterfaceElAliasIpRangeEl {}

impl BuildComputeInstanceFromTemplateNetworkInterfaceElAliasIpRangeEl {
    pub fn build(self) -> ComputeInstanceFromTemplateNetworkInterfaceElAliasIpRangeEl {
        ComputeInstanceFromTemplateNetworkInterfaceElAliasIpRangeEl {
            ip_cidr_range: core::default::Default::default(),
            subnetwork_range_name: core::default::Default::default(),
        }
    }
}

pub struct ComputeInstanceFromTemplateNetworkInterfaceElAliasIpRangeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceFromTemplateNetworkInterfaceElAliasIpRangeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstanceFromTemplateNetworkInterfaceElAliasIpRangeElRef {
        ComputeInstanceFromTemplateNetworkInterfaceElAliasIpRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceFromTemplateNetworkInterfaceElAliasIpRangeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip_cidr_range` after provisioning.\n"]
    pub fn ip_cidr_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_cidr_range", self.base))
    }

    #[doc = "Get a reference to the value of field `subnetwork_range_name` after provisioning.\n"]
    pub fn subnetwork_range_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork_range_name", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInstanceFromTemplateNetworkInterfaceElIpv6AccessConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_ipv6: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_ipv6_prefix_length: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    network_tier: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_ptr_domain_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeInstanceFromTemplateNetworkInterfaceElIpv6AccessConfigEl {
    #[doc =
        "Set the field `external_ipv6`.\nThe first IPv6 address of the external IPv6 range associated with this instance, prefix length is stored in externalIpv6PrefixLength in ipv6AccessConfig. To use a static external IP address, it must be unused and in the same region as the instance's zone. If not specified, Google Cloud will automatically assign an external IPv6 address from the instance's subnetwork."]
    pub fn set_external_ipv6(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.external_ipv6 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_ipv6_prefix_length`.\nThe prefix length of the external IPv6 range."]
    pub fn set_external_ipv6_prefix_length(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.external_ipv6_prefix_length = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `name`.\nThe name of this access configuration. In ipv6AccessConfigs, the recommended name is External IPv6."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `public_ptr_domain_name`.\nThe domain name to be used when creating DNSv6 records for the external IPv6 ranges."]
    pub fn set_public_ptr_domain_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.public_ptr_domain_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInstanceFromTemplateNetworkInterfaceElIpv6AccessConfigEl {
    type O = terrars::BlockAssignable<ComputeInstanceFromTemplateNetworkInterfaceElIpv6AccessConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceFromTemplateNetworkInterfaceElIpv6AccessConfigEl {
    #[doc =
        "The service-level to be provided for IPv6 traffic when the subnet has an external subnet. Only PREMIUM tier is valid for IPv6"]
    pub network_tier: terrars::PrimField<std::string::String>,
}

impl BuildComputeInstanceFromTemplateNetworkInterfaceElIpv6AccessConfigEl {
    pub fn build(self) -> ComputeInstanceFromTemplateNetworkInterfaceElIpv6AccessConfigEl {
        ComputeInstanceFromTemplateNetworkInterfaceElIpv6AccessConfigEl {
            external_ipv6: core::default::Default::default(),
            external_ipv6_prefix_length: core::default::Default::default(),
            name: core::default::Default::default(),
            network_tier: self.network_tier,
            public_ptr_domain_name: core::default::Default::default(),
        }
    }
}

pub struct ComputeInstanceFromTemplateNetworkInterfaceElIpv6AccessConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceFromTemplateNetworkInterfaceElIpv6AccessConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstanceFromTemplateNetworkInterfaceElIpv6AccessConfigElRef {
        ComputeInstanceFromTemplateNetworkInterfaceElIpv6AccessConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceFromTemplateNetworkInterfaceElIpv6AccessConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `external_ipv6` after provisioning.\nThe first IPv6 address of the external IPv6 range associated with this instance, prefix length is stored in externalIpv6PrefixLength in ipv6AccessConfig. To use a static external IP address, it must be unused and in the same region as the instance's zone. If not specified, Google Cloud will automatically assign an external IPv6 address from the instance's subnetwork."]
    pub fn external_ipv6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_ipv6", self.base))
    }

    #[doc =
        "Get a reference to the value of field `external_ipv6_prefix_length` after provisioning.\nThe prefix length of the external IPv6 range."]
    pub fn external_ipv6_prefix_length(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_ipv6_prefix_length", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of this access configuration. In ipv6AccessConfigs, the recommended name is External IPv6."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network_tier` after provisioning.\nThe service-level to be provided for IPv6 traffic when the subnet has an external subnet. Only PREMIUM tier is valid for IPv6"]
    pub fn network_tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_tier", self.base))
    }

    #[doc =
        "Get a reference to the value of field `public_ptr_domain_name` after provisioning.\nThe domain name to be used when creating DNSv6 records for the external IPv6 ranges."]
    pub fn public_ptr_domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_ptr_domain_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeInstanceFromTemplateNetworkInterfaceElDynamic {
    ipv6_access_config: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeInstanceFromTemplateNetworkInterfaceElIpv6AccessConfigEl>>,
}

#[derive(Serialize)]
pub struct ComputeInstanceFromTemplateNetworkInterfaceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_config: std::option::Option<terrars::ListField<ComputeInstanceFromTemplateNetworkInterfaceElAccessConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alias_ip_range: std::option::Option<terrars::ListField<ComputeInstanceFromTemplateNetworkInterfaceElAliasIpRangeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internal_ipv6_prefix_length: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nic_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    queue_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stack_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork_project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_access_config: std
    ::option
    ::Option<std::vec::Vec<ComputeInstanceFromTemplateNetworkInterfaceElIpv6AccessConfigEl>>,
    dynamic: ComputeInstanceFromTemplateNetworkInterfaceElDynamic,
}

impl ComputeInstanceFromTemplateNetworkInterfaceEl {
    #[doc =
        "Set the field `access_config`.\nAccess configurations, i.e. IPs via which this instance can be accessed via the Internet."]
    pub fn set_access_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ComputeInstanceFromTemplateNetworkInterfaceElAccessConfigEl>>,
    ) -> Self {
        self.access_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `alias_ip_range`.\nAn array of alias IP ranges for this network interface."]
    pub fn set_alias_ip_range(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ComputeInstanceFromTemplateNetworkInterfaceElAliasIpRangeEl>>,
    ) -> Self {
        self.alias_ip_range = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `internal_ipv6_prefix_length`.\nThe prefix length of the primary internal IPv6 range."]
    pub fn set_internal_ipv6_prefix_length(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.internal_ipv6_prefix_length = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ipv6_address`.\nAn IPv6 internal network address for this network interface. If not specified, Google Cloud will automatically assign an internal IPv6 address from the instance's subnetwork."]
    pub fn set_ipv6_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ipv6_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network`.\nThe name or self_link of the network attached to this interface."]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_ip`.\nThe private IP address assigned to the instance."]
    pub fn set_network_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `nic_type`.\nThe type of vNIC to be used on this interface. Possible values:GVNIC, VIRTIO_NET"]
    pub fn set_nic_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.nic_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `queue_count`.\nThe networking queue count that's specified by users for the network interface. Both Rx and Tx queues will be set to this number. It will be empty if not specified."]
    pub fn set_queue_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.queue_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `stack_type`.\nThe stack type for this network interface to identify whether the IPv6 feature is enabled or not. If not specified, IPV4_ONLY will be used."]
    pub fn set_stack_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.stack_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnetwork`.\nThe name or self_link of the subnetwork attached to this interface."]
    pub fn set_subnetwork(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnetwork = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnetwork_project`.\nThe project in which the subnetwork belongs."]
    pub fn set_subnetwork_project(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.subnetwork_project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_access_config`.\n"]
    pub fn set_ipv6_access_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeInstanceFromTemplateNetworkInterfaceElIpv6AccessConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ipv6_access_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ipv6_access_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeInstanceFromTemplateNetworkInterfaceEl {
    type O = terrars::BlockAssignable<ComputeInstanceFromTemplateNetworkInterfaceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceFromTemplateNetworkInterfaceEl {}

impl BuildComputeInstanceFromTemplateNetworkInterfaceEl {
    pub fn build(self) -> ComputeInstanceFromTemplateNetworkInterfaceEl {
        ComputeInstanceFromTemplateNetworkInterfaceEl {
            access_config: core::default::Default::default(),
            alias_ip_range: core::default::Default::default(),
            internal_ipv6_prefix_length: core::default::Default::default(),
            ipv6_address: core::default::Default::default(),
            network: core::default::Default::default(),
            network_ip: core::default::Default::default(),
            nic_type: core::default::Default::default(),
            queue_count: core::default::Default::default(),
            stack_type: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
            subnetwork_project: core::default::Default::default(),
            ipv6_access_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeInstanceFromTemplateNetworkInterfaceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceFromTemplateNetworkInterfaceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstanceFromTemplateNetworkInterfaceElRef {
        ComputeInstanceFromTemplateNetworkInterfaceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceFromTemplateNetworkInterfaceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_config` after provisioning.\nAccess configurations, i.e. IPs via which this instance can be accessed via the Internet."]
    pub fn access_config(&self) -> terrars::ListRef<ComputeInstanceFromTemplateNetworkInterfaceElAccessConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_config", self.base))
    }

    #[doc =
        "Get a reference to the value of field `alias_ip_range` after provisioning.\nAn array of alias IP ranges for this network interface."]
    pub fn alias_ip_range(&self) -> terrars::ListRef<ComputeInstanceFromTemplateNetworkInterfaceElAliasIpRangeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alias_ip_range", self.base))
    }

    #[doc =
        "Get a reference to the value of field `internal_ipv6_prefix_length` after provisioning.\nThe prefix length of the primary internal IPv6 range."]
    pub fn internal_ipv6_prefix_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.internal_ipv6_prefix_length", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ipv6_access_type` after provisioning.\nOne of EXTERNAL, INTERNAL to indicate whether the IP can be accessed from the Internet. This field is always inherited from its subnetwork."]
    pub fn ipv6_access_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_access_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ipv6_address` after provisioning.\nAn IPv6 internal network address for this network interface. If not specified, Google Cloud will automatically assign an internal IPv6 address from the instance's subnetwork."]
    pub fn ipv6_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_address", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the interface"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe name or self_link of the network attached to this interface."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network_ip` after provisioning.\nThe private IP address assigned to the instance."]
    pub fn network_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_ip", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nic_type` after provisioning.\nThe type of vNIC to be used on this interface. Possible values:GVNIC, VIRTIO_NET"]
    pub fn nic_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nic_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `queue_count` after provisioning.\nThe networking queue count that's specified by users for the network interface. Both Rx and Tx queues will be set to this number. It will be empty if not specified."]
    pub fn queue_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queue_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `stack_type` after provisioning.\nThe stack type for this network interface to identify whether the IPv6 feature is enabled or not. If not specified, IPV4_ONLY will be used."]
    pub fn stack_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stack_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subnetwork` after provisioning.\nThe name or self_link of the subnetwork attached to this interface."]
    pub fn subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subnetwork_project` after provisioning.\nThe project in which the subnetwork belongs."]
    pub fn subnetwork_project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork_project", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv6_access_config` after provisioning.\n"]
    pub fn ipv6_access_config(
        &self,
    ) -> terrars::ListRef<ComputeInstanceFromTemplateNetworkInterfaceElIpv6AccessConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ipv6_access_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInstanceFromTemplateNetworkPerformanceConfigEl {
    total_egress_bandwidth_tier: terrars::PrimField<std::string::String>,
}

impl ComputeInstanceFromTemplateNetworkPerformanceConfigEl { }

impl terrars::ToListMappable for ComputeInstanceFromTemplateNetworkPerformanceConfigEl {
    type O = terrars::BlockAssignable<ComputeInstanceFromTemplateNetworkPerformanceConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceFromTemplateNetworkPerformanceConfigEl {
    #[doc = "The egress bandwidth tier to enable. Possible values:TIER_1, DEFAULT"]
    pub total_egress_bandwidth_tier: terrars::PrimField<std::string::String>,
}

impl BuildComputeInstanceFromTemplateNetworkPerformanceConfigEl {
    pub fn build(self) -> ComputeInstanceFromTemplateNetworkPerformanceConfigEl {
        ComputeInstanceFromTemplateNetworkPerformanceConfigEl {
            total_egress_bandwidth_tier: self.total_egress_bandwidth_tier,
        }
    }
}

pub struct ComputeInstanceFromTemplateNetworkPerformanceConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceFromTemplateNetworkPerformanceConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstanceFromTemplateNetworkPerformanceConfigElRef {
        ComputeInstanceFromTemplateNetworkPerformanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceFromTemplateNetworkPerformanceConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `total_egress_bandwidth_tier` after provisioning.\nThe egress bandwidth tier to enable. Possible values:TIER_1, DEFAULT"]
    pub fn total_egress_bandwidth_tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_egress_bandwidth_tier", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInstanceFromTemplateParamsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_manager_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl ComputeInstanceFromTemplateParamsEl {
    #[doc =
        "Set the field `resource_manager_tags`.\nA map of resource manager tags. Resource manager tag keys and values have the same definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/456. The field is ignored (both PUT & PATCH) when empty."]
    pub fn set_resource_manager_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resource_manager_tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInstanceFromTemplateParamsEl {
    type O = terrars::BlockAssignable<ComputeInstanceFromTemplateParamsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceFromTemplateParamsEl {}

impl BuildComputeInstanceFromTemplateParamsEl {
    pub fn build(self) -> ComputeInstanceFromTemplateParamsEl {
        ComputeInstanceFromTemplateParamsEl { resource_manager_tags: core::default::Default::default() }
    }
}

pub struct ComputeInstanceFromTemplateParamsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceFromTemplateParamsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInstanceFromTemplateParamsElRef {
        ComputeInstanceFromTemplateParamsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceFromTemplateParamsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `resource_manager_tags` after provisioning.\nA map of resource manager tags. Resource manager tag keys and values have the same definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/456. The field is ignored (both PUT & PATCH) when empty."]
    pub fn resource_manager_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.resource_manager_tags", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInstanceFromTemplateReservationAffinityElSpecificReservationEl {
    key: terrars::PrimField<std::string::String>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl ComputeInstanceFromTemplateReservationAffinityElSpecificReservationEl { }

impl terrars::ToListMappable for ComputeInstanceFromTemplateReservationAffinityElSpecificReservationEl {
    type O = terrars::BlockAssignable<ComputeInstanceFromTemplateReservationAffinityElSpecificReservationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceFromTemplateReservationAffinityElSpecificReservationEl {
    #[doc =
        "Corresponds to the label key of a reservation resource. To target a SPECIFIC_RESERVATION by name, specify compute.googleapis.com/reservation-name as the key and specify the name of your reservation as the only value."]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = "Corresponds to the label values of a reservation resource."]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildComputeInstanceFromTemplateReservationAffinityElSpecificReservationEl {
    pub fn build(self) -> ComputeInstanceFromTemplateReservationAffinityElSpecificReservationEl {
        ComputeInstanceFromTemplateReservationAffinityElSpecificReservationEl {
            key: self.key,
            values: self.values,
        }
    }
}

pub struct ComputeInstanceFromTemplateReservationAffinityElSpecificReservationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceFromTemplateReservationAffinityElSpecificReservationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstanceFromTemplateReservationAffinityElSpecificReservationElRef {
        ComputeInstanceFromTemplateReservationAffinityElSpecificReservationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceFromTemplateReservationAffinityElSpecificReservationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `key` after provisioning.\nCorresponds to the label key of a reservation resource. To target a SPECIFIC_RESERVATION by name, specify compute.googleapis.com/reservation-name as the key and specify the name of your reservation as the only value."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `values` after provisioning.\nCorresponds to the label values of a reservation resource."]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeInstanceFromTemplateReservationAffinityElDynamic {
    specific_reservation: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeInstanceFromTemplateReservationAffinityElSpecificReservationEl>>,
}

#[derive(Serialize)]
pub struct ComputeInstanceFromTemplateReservationAffinityEl {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    specific_reservation: std
    ::option
    ::Option<std::vec::Vec<ComputeInstanceFromTemplateReservationAffinityElSpecificReservationEl>>,
    dynamic: ComputeInstanceFromTemplateReservationAffinityElDynamic,
}

impl ComputeInstanceFromTemplateReservationAffinityEl {
    #[doc = "Set the field `specific_reservation`.\n"]
    pub fn set_specific_reservation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ComputeInstanceFromTemplateReservationAffinityElSpecificReservationEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.specific_reservation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.specific_reservation = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeInstanceFromTemplateReservationAffinityEl {
    type O = terrars::BlockAssignable<ComputeInstanceFromTemplateReservationAffinityEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceFromTemplateReservationAffinityEl {
    #[doc = "The type of reservation from which this instance can consume resources."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildComputeInstanceFromTemplateReservationAffinityEl {
    pub fn build(self) -> ComputeInstanceFromTemplateReservationAffinityEl {
        ComputeInstanceFromTemplateReservationAffinityEl {
            type_: self.type_,
            specific_reservation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeInstanceFromTemplateReservationAffinityElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceFromTemplateReservationAffinityElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstanceFromTemplateReservationAffinityElRef {
        ComputeInstanceFromTemplateReservationAffinityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceFromTemplateReservationAffinityElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of reservation from which this instance can consume resources."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `specific_reservation` after provisioning.\n"]
    pub fn specific_reservation(
        &self,
    ) -> terrars::ListRef<ComputeInstanceFromTemplateReservationAffinityElSpecificReservationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.specific_reservation", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInstanceFromTemplateSchedulingElLocalSsdRecoveryTimeoutEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    seconds: terrars::PrimField<f64>,
}

impl ComputeInstanceFromTemplateSchedulingElLocalSsdRecoveryTimeoutEl {
    #[doc =
        "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond\nresolution. Durations less than one second are represented\nwith a 0 seconds field and a positive nanos field. Must\nbe from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInstanceFromTemplateSchedulingElLocalSsdRecoveryTimeoutEl {
    type O = terrars::BlockAssignable<ComputeInstanceFromTemplateSchedulingElLocalSsdRecoveryTimeoutEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceFromTemplateSchedulingElLocalSsdRecoveryTimeoutEl {
    #[doc = "Span of time at a resolution of a second.\nMust be from 0 to 315,576,000,000 inclusive."]
    pub seconds: terrars::PrimField<f64>,
}

impl BuildComputeInstanceFromTemplateSchedulingElLocalSsdRecoveryTimeoutEl {
    pub fn build(self) -> ComputeInstanceFromTemplateSchedulingElLocalSsdRecoveryTimeoutEl {
        ComputeInstanceFromTemplateSchedulingElLocalSsdRecoveryTimeoutEl {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}

pub struct ComputeInstanceFromTemplateSchedulingElLocalSsdRecoveryTimeoutElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceFromTemplateSchedulingElLocalSsdRecoveryTimeoutElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstanceFromTemplateSchedulingElLocalSsdRecoveryTimeoutElRef {
        ComputeInstanceFromTemplateSchedulingElLocalSsdRecoveryTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceFromTemplateSchedulingElLocalSsdRecoveryTimeoutElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond\nresolution. Durations less than one second are represented\nwith a 0 seconds field and a positive nanos field. Must\nbe from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second.\nMust be from 0 to 315,576,000,000 inclusive."]
    pub fn seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInstanceFromTemplateSchedulingElNodeAffinitiesEl {
    key: terrars::PrimField<std::string::String>,
    operator: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl ComputeInstanceFromTemplateSchedulingElNodeAffinitiesEl { }

impl terrars::ToListMappable for ComputeInstanceFromTemplateSchedulingElNodeAffinitiesEl {
    type O = terrars::BlockAssignable<ComputeInstanceFromTemplateSchedulingElNodeAffinitiesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceFromTemplateSchedulingElNodeAffinitiesEl {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub operator: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildComputeInstanceFromTemplateSchedulingElNodeAffinitiesEl {
    pub fn build(self) -> ComputeInstanceFromTemplateSchedulingElNodeAffinitiesEl {
        ComputeInstanceFromTemplateSchedulingElNodeAffinitiesEl {
            key: self.key,
            operator: self.operator,
            values: self.values,
        }
    }
}

pub struct ComputeInstanceFromTemplateSchedulingElNodeAffinitiesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceFromTemplateSchedulingElNodeAffinitiesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstanceFromTemplateSchedulingElNodeAffinitiesElRef {
        ComputeInstanceFromTemplateSchedulingElNodeAffinitiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceFromTemplateSchedulingElNodeAffinitiesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeInstanceFromTemplateSchedulingElDynamic {
    local_ssd_recovery_timeout: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeInstanceFromTemplateSchedulingElLocalSsdRecoveryTimeoutEl>>,
    node_affinities: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeInstanceFromTemplateSchedulingElNodeAffinitiesEl>>,
}

#[derive(Serialize)]
pub struct ComputeInstanceFromTemplateSchedulingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automatic_restart: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_termination_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_node_cpus: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_host_maintenance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preemptible: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisioning_model: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_ssd_recovery_timeout: std
    ::option
    ::Option<std::vec::Vec<ComputeInstanceFromTemplateSchedulingElLocalSsdRecoveryTimeoutEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_affinities: std::option::Option<std::vec::Vec<ComputeInstanceFromTemplateSchedulingElNodeAffinitiesEl>>,
    dynamic: ComputeInstanceFromTemplateSchedulingElDynamic,
}

impl ComputeInstanceFromTemplateSchedulingEl {
    #[doc =
        "Set the field `automatic_restart`.\nSpecifies if the instance should be restarted if it was terminated by Compute Engine (not a user)."]
    pub fn set_automatic_restart(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.automatic_restart = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `instance_termination_action`.\nSpecifies the action GCE should take when SPOT VM is preempted."]
    pub fn set_instance_termination_action(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.instance_termination_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_node_cpus`.\n"]
    pub fn set_min_node_cpus(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_node_cpus = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `on_host_maintenance`.\nDescribes maintenance behavior for the instance. One of MIGRATE or TERMINATE,"]
    pub fn set_on_host_maintenance(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.on_host_maintenance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preemptible`.\nWhether the instance is preemptible."]
    pub fn set_preemptible(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.preemptible = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `provisioning_model`.\nWhether the instance is spot. If this is set as SPOT."]
    pub fn set_provisioning_model(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.provisioning_model = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_ssd_recovery_timeout`.\n"]
    pub fn set_local_ssd_recovery_timeout(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ComputeInstanceFromTemplateSchedulingElLocalSsdRecoveryTimeoutEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.local_ssd_recovery_timeout = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.local_ssd_recovery_timeout = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `node_affinities`.\n"]
    pub fn set_node_affinities(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeInstanceFromTemplateSchedulingElNodeAffinitiesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.node_affinities = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.node_affinities = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeInstanceFromTemplateSchedulingEl {
    type O = terrars::BlockAssignable<ComputeInstanceFromTemplateSchedulingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceFromTemplateSchedulingEl {}

impl BuildComputeInstanceFromTemplateSchedulingEl {
    pub fn build(self) -> ComputeInstanceFromTemplateSchedulingEl {
        ComputeInstanceFromTemplateSchedulingEl {
            automatic_restart: core::default::Default::default(),
            instance_termination_action: core::default::Default::default(),
            min_node_cpus: core::default::Default::default(),
            on_host_maintenance: core::default::Default::default(),
            preemptible: core::default::Default::default(),
            provisioning_model: core::default::Default::default(),
            local_ssd_recovery_timeout: core::default::Default::default(),
            node_affinities: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeInstanceFromTemplateSchedulingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceFromTemplateSchedulingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInstanceFromTemplateSchedulingElRef {
        ComputeInstanceFromTemplateSchedulingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceFromTemplateSchedulingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `automatic_restart` after provisioning.\nSpecifies if the instance should be restarted if it was terminated by Compute Engine (not a user)."]
    pub fn automatic_restart(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automatic_restart", self.base))
    }

    #[doc =
        "Get a reference to the value of field `instance_termination_action` after provisioning.\nSpecifies the action GCE should take when SPOT VM is preempted."]
    pub fn instance_termination_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_termination_action", self.base))
    }

    #[doc = "Get a reference to the value of field `min_node_cpus` after provisioning.\n"]
    pub fn min_node_cpus(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_node_cpus", self.base))
    }

    #[doc =
        "Get a reference to the value of field `on_host_maintenance` after provisioning.\nDescribes maintenance behavior for the instance. One of MIGRATE or TERMINATE,"]
    pub fn on_host_maintenance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_host_maintenance", self.base))
    }

    #[doc =
        "Get a reference to the value of field `preemptible` after provisioning.\nWhether the instance is preemptible."]
    pub fn preemptible(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preemptible", self.base))
    }

    #[doc =
        "Get a reference to the value of field `provisioning_model` after provisioning.\nWhether the instance is spot. If this is set as SPOT."]
    pub fn provisioning_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioning_model", self.base))
    }

    #[doc = "Get a reference to the value of field `local_ssd_recovery_timeout` after provisioning.\n"]
    pub fn local_ssd_recovery_timeout(
        &self,
    ) -> terrars::ListRef<ComputeInstanceFromTemplateSchedulingElLocalSsdRecoveryTimeoutElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.local_ssd_recovery_timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInstanceFromTemplateShieldedInstanceConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_integrity_monitoring: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_secure_boot: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_vtpm: std::option::Option<terrars::PrimField<bool>>,
}

impl ComputeInstanceFromTemplateShieldedInstanceConfigEl {
    #[doc = "Set the field `enable_integrity_monitoring`.\nWhether integrity monitoring is enabled for the instance."]
    pub fn set_enable_integrity_monitoring(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_integrity_monitoring = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_secure_boot`.\nWhether secure boot is enabled for the instance."]
    pub fn set_enable_secure_boot(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_secure_boot = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_vtpm`.\nWhether the instance uses vTPM."]
    pub fn set_enable_vtpm(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_vtpm = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInstanceFromTemplateShieldedInstanceConfigEl {
    type O = terrars::BlockAssignable<ComputeInstanceFromTemplateShieldedInstanceConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceFromTemplateShieldedInstanceConfigEl {}

impl BuildComputeInstanceFromTemplateShieldedInstanceConfigEl {
    pub fn build(self) -> ComputeInstanceFromTemplateShieldedInstanceConfigEl {
        ComputeInstanceFromTemplateShieldedInstanceConfigEl {
            enable_integrity_monitoring: core::default::Default::default(),
            enable_secure_boot: core::default::Default::default(),
            enable_vtpm: core::default::Default::default(),
        }
    }
}

pub struct ComputeInstanceFromTemplateShieldedInstanceConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceFromTemplateShieldedInstanceConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstanceFromTemplateShieldedInstanceConfigElRef {
        ComputeInstanceFromTemplateShieldedInstanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceFromTemplateShieldedInstanceConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_integrity_monitoring` after provisioning.\nWhether integrity monitoring is enabled for the instance."]
    pub fn enable_integrity_monitoring(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_integrity_monitoring", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_secure_boot` after provisioning.\nWhether secure boot is enabled for the instance."]
    pub fn enable_secure_boot(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_secure_boot", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_vtpm` after provisioning.\nWhether the instance uses vTPM."]
    pub fn enable_vtpm(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_vtpm", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInstanceFromTemplateTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeInstanceFromTemplateTimeoutsEl {
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

impl terrars::ToListMappable for ComputeInstanceFromTemplateTimeoutsEl {
    type O = terrars::BlockAssignable<ComputeInstanceFromTemplateTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstanceFromTemplateTimeoutsEl {}

impl BuildComputeInstanceFromTemplateTimeoutsEl {
    pub fn build(self) -> ComputeInstanceFromTemplateTimeoutsEl {
        ComputeInstanceFromTemplateTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ComputeInstanceFromTemplateTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceFromTemplateTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInstanceFromTemplateTimeoutsElRef {
        ComputeInstanceFromTemplateTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstanceFromTemplateTimeoutsElRef {
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

#[derive(Serialize, Default)]
struct ComputeInstanceFromTemplateDynamic {
    advanced_machine_features: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeInstanceFromTemplateAdvancedMachineFeaturesEl>>,
    boot_disk: std::option::Option<terrars::DynamicBlock<ComputeInstanceFromTemplateBootDiskEl>>,
    confidential_instance_config: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeInstanceFromTemplateConfidentialInstanceConfigEl>>,
    network_interface: std::option::Option<terrars::DynamicBlock<ComputeInstanceFromTemplateNetworkInterfaceEl>>,
    network_performance_config: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeInstanceFromTemplateNetworkPerformanceConfigEl>>,
    params: std::option::Option<terrars::DynamicBlock<ComputeInstanceFromTemplateParamsEl>>,
    reservation_affinity: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeInstanceFromTemplateReservationAffinityEl>>,
    scheduling: std::option::Option<terrars::DynamicBlock<ComputeInstanceFromTemplateSchedulingEl>>,
    shielded_instance_config: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeInstanceFromTemplateShieldedInstanceConfigEl>>,
}
