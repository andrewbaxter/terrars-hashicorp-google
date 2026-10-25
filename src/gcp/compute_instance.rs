#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeInstanceData_ {
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
    can_ip_forward: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    desired_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_display: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_revocation_action_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    machine_type: terrars::PrimField<std::string::String>,
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
    tags: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_machine_features: std::option::Option<std::vec::Vec<ComputeInstance_AdvancedMachineFeatures>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attached_disk: std::option::Option<std::vec::Vec<ComputeInstance_AttachedDisk>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk: std::option::Option<std::vec::Vec<ComputeInstance_BootDisk>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confidential_instance_config: std::option::Option<std::vec::Vec<ComputeInstance_ConfidentialInstanceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guest_accelerator: std::option::Option<std::vec::Vec<ComputeInstance_GuestAccelerator>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_encryption_key: std::option::Option<std::vec::Vec<ComputeInstance_InstanceEncryptionKey>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_interface: std::option::Option<std::vec::Vec<ComputeInstance_NetworkInterface>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_performance_config: std::option::Option<std::vec::Vec<ComputeInstance_NetworkPerformanceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    params: std::option::Option<std::vec::Vec<ComputeInstance_Params>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reservation_affinity: std::option::Option<std::vec::Vec<ComputeInstance_ReservationAffinity>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheduling: std::option::Option<std::vec::Vec<ComputeInstance_Scheduling>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scratch_disk: std::option::Option<std::vec::Vec<ComputeInstance_ScratchDisk>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<std::vec::Vec<ComputeInstance_ServiceAccount>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shielded_instance_config: std::option::Option<std::vec::Vec<ComputeInstance_ShieldedInstanceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeInstance_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workload_identity_config: std::option::Option<std::vec::Vec<ComputeInstance_WorkloadIdentityConfig>>,
    dynamic: ComputeInstanceDynamic,
}

struct ComputeInstance_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeInstanceData_>,
}

#[derive(Clone)]
pub struct ComputeInstance(std::rc::Rc<ComputeInstance_>);

impl ComputeInstance {
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

    #[doc =
        "Set the field `can_ip_forward`.\nWhether sending and receiving of packets with non-matching source or destination IPs is allowed."]
    pub fn set_can_ip_forward(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().can_ip_forward = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
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

    #[doc =
        "Set the field `desired_status`.\nDesired status of the instance. Either \"RUNNING\", \"SUSPENDED\" or \"TERMINATED\"."]
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
        "Set the field `key_revocation_action_type`.\nAction to be taken when a customer's encryption key is revoked. Supports \"STOP\" and \"NONE\", with \"NONE\" being the default."]
    pub fn set_key_revocation_action_type(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().key_revocation_action_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nA set of key/value label pairs assigned to the instance.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstance_AdvancedMachineFeatures>>,
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

    #[doc = "Set the field `attached_disk`.\n"]
    pub fn set_attached_disk(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstance_AttachedDisk>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().attached_disk = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.attached_disk = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `boot_disk`.\n"]
    pub fn set_boot_disk(self, v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstance_BootDisk>>) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstance_ConfidentialInstanceConfig>>,
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

    #[doc = "Set the field `guest_accelerator`.\n"]
    pub fn set_guest_accelerator(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstance_GuestAccelerator>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().guest_accelerator = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.guest_accelerator = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `instance_encryption_key`.\n"]
    pub fn set_instance_encryption_key(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstance_InstanceEncryptionKey>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().instance_encryption_key = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.instance_encryption_key = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_interface`.\n"]
    pub fn set_network_interface(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstance_NetworkInterface>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstance_NetworkPerformanceConfig>>,
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
    pub fn set_params(self, v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstance_Params>>) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstance_ReservationAffinity>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstance_Scheduling>>,
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

    #[doc = "Set the field `scratch_disk`.\n"]
    pub fn set_scratch_disk(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstance_ScratchDisk>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scratch_disk = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scratch_disk = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_account`.\n"]
    pub fn set_service_account(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstance_ServiceAccount>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().service_account = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.service_account = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `shielded_instance_config`.\n"]
    pub fn set_shielded_instance_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstance_ShieldedInstanceConfig>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeInstance_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `workload_identity_config`.\n"]
    pub fn set_workload_identity_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstance_WorkloadIdentityConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().workload_identity_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.workload_identity_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `allow_stopping_for_update` after provisioning.\nIf true, allows Terraform to stop the instance to update its properties. If you try to update a property that requires stopping the instance without setting this field, the update will fail."]
    pub fn allow_stopping_for_update(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_stopping_for_update", self.extract_ref()))
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
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `current_status` after provisioning.\n\n\t\t\t\t\tCurrent status of the instance.\n\t\t\t\t\tThis could be one of the following values: PROVISIONING, STAGING, RUNNING, STOPPING, SUSPENDING, SUSPENDED, REPAIRING, and TERMINATED.\n\t\t\t\t\tFor more information about the status of the instance, see [Instance life cycle](https://cloud.google.com/compute/docs/instances/instance-life-cycle)."]
    pub fn current_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.current_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
        "Get a reference to the value of field `desired_status` after provisioning.\nDesired status of the instance. Either \"RUNNING\", \"SUSPENDED\" or \"TERMINATED\"."]
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
        "Get a reference to the value of field `key_revocation_action_type` after provisioning.\nAction to be taken when a customer's encryption key is revoked. Supports \"STOP\" and \"NONE\", with \"NONE\" being the default."]
    pub fn key_revocation_action_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_revocation_action_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `label_fingerprint` after provisioning.\nThe unique fingerprint of the labels."]
    pub fn label_fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label_fingerprint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs assigned to the instance.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
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
    pub fn advanced_machine_features(&self) -> terrars::ListRef<ComputeInstance_AdvancedMachineFeaturesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_machine_features", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attached_disk` after provisioning.\n"]
    pub fn attached_disk(&self) -> terrars::ListRef<ComputeInstance_AttachedDiskRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attached_disk", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `boot_disk` after provisioning.\n"]
    pub fn boot_disk(&self) -> terrars::ListRef<ComputeInstance_BootDiskRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.boot_disk", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `confidential_instance_config` after provisioning.\n"]
    pub fn confidential_instance_config(&self) -> terrars::ListRef<ComputeInstance_ConfidentialInstanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.confidential_instance_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `guest_accelerator` after provisioning.\n"]
    pub fn guest_accelerator(&self) -> terrars::ListRef<ComputeInstance_GuestAcceleratorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.guest_accelerator", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_encryption_key` after provisioning.\n"]
    pub fn instance_encryption_key(&self) -> terrars::ListRef<ComputeInstance_InstanceEncryptionKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_encryption_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_interface` after provisioning.\n"]
    pub fn network_interface(&self) -> terrars::ListRef<ComputeInstance_NetworkInterfaceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_interface", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_performance_config` after provisioning.\n"]
    pub fn network_performance_config(&self) -> terrars::ListRef<ComputeInstance_NetworkPerformanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_performance_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> terrars::ListRef<ComputeInstance_ParamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.params", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reservation_affinity` after provisioning.\n"]
    pub fn reservation_affinity(&self) -> terrars::ListRef<ComputeInstance_ReservationAffinityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reservation_affinity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scheduling` after provisioning.\n"]
    pub fn scheduling(&self) -> terrars::ListRef<ComputeInstance_SchedulingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scheduling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scratch_disk` after provisioning.\n"]
    pub fn scratch_disk(&self) -> terrars::ListRef<ComputeInstance_ScratchDiskRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scratch_disk", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_account` after provisioning.\n"]
    pub fn service_account(&self) -> terrars::ListRef<ComputeInstance_ServiceAccountRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_account", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `shielded_instance_config` after provisioning.\n"]
    pub fn shielded_instance_config(&self) -> terrars::ListRef<ComputeInstance_ShieldedInstanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shielded_instance_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeInstance_TimeoutsRef {
        ComputeInstance_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workload_identity_config` after provisioning.\n"]
    pub fn workload_identity_config(&self) -> terrars::ListRef<ComputeInstance_WorkloadIdentityConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workload_identity_config", self.extract_ref()))
    }
}

impl terrars::Referable for ComputeInstance {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputeInstance { }

impl terrars::ToListMappable for ComputeInstance {
    type O = terrars::ListRef<ComputeInstanceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputeInstance_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_compute_instance".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeInstance {
    pub tf_id: std::string::String,
    #[doc = "The machine type to create."]
    pub machine_type: terrars::PrimField<std::string::String>,
    #[doc = "The name of the instance. One of name or self_link must be provided."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeInstance {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputeInstance {
        let out = ComputeInstance(std::rc::Rc::new(ComputeInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputeInstanceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                allow_stopping_for_update: core::default::Default::default(),
                can_ip_forward: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                description: core::default::Default::default(),
                desired_status: core::default::Default::default(),
                enable_display: core::default::Default::default(),
                hostname: core::default::Default::default(),
                id: core::default::Default::default(),
                key_revocation_action_type: core::default::Default::default(),
                labels: core::default::Default::default(),
                machine_type: self.machine_type,
                metadata: core::default::Default::default(),
                metadata_startup_script: core::default::Default::default(),
                min_cpu_platform: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                resource_policies: core::default::Default::default(),
                tags: core::default::Default::default(),
                zone: core::default::Default::default(),
                advanced_machine_features: core::default::Default::default(),
                attached_disk: core::default::Default::default(),
                boot_disk: core::default::Default::default(),
                confidential_instance_config: core::default::Default::default(),
                guest_accelerator: core::default::Default::default(),
                instance_encryption_key: core::default::Default::default(),
                network_interface: core::default::Default::default(),
                network_performance_config: core::default::Default::default(),
                params: core::default::Default::default(),
                reservation_affinity: core::default::Default::default(),
                scheduling: core::default::Default::default(),
                scratch_disk: core::default::Default::default(),
                service_account: core::default::Default::default(),
                shielded_instance_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                workload_identity_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeInstanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstanceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeInstanceRef {
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
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `current_status` after provisioning.\n\n\t\t\t\t\tCurrent status of the instance.\n\t\t\t\t\tThis could be one of the following values: PROVISIONING, STAGING, RUNNING, STOPPING, SUSPENDING, SUSPENDED, REPAIRING, and TERMINATED.\n\t\t\t\t\tFor more information about the status of the instance, see [Instance life cycle](https://cloud.google.com/compute/docs/instances/instance-life-cycle)."]
    pub fn current_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.current_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
        "Get a reference to the value of field `desired_status` after provisioning.\nDesired status of the instance. Either \"RUNNING\", \"SUSPENDED\" or \"TERMINATED\"."]
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
        "Get a reference to the value of field `key_revocation_action_type` after provisioning.\nAction to be taken when a customer's encryption key is revoked. Supports \"STOP\" and \"NONE\", with \"NONE\" being the default."]
    pub fn key_revocation_action_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_revocation_action_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `label_fingerprint` after provisioning.\nThe unique fingerprint of the labels."]
    pub fn label_fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label_fingerprint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs assigned to the instance.\n\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
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
    pub fn advanced_machine_features(&self) -> terrars::ListRef<ComputeInstance_AdvancedMachineFeaturesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_machine_features", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attached_disk` after provisioning.\n"]
    pub fn attached_disk(&self) -> terrars::ListRef<ComputeInstance_AttachedDiskRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attached_disk", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `boot_disk` after provisioning.\n"]
    pub fn boot_disk(&self) -> terrars::ListRef<ComputeInstance_BootDiskRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.boot_disk", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `confidential_instance_config` after provisioning.\n"]
    pub fn confidential_instance_config(&self) -> terrars::ListRef<ComputeInstance_ConfidentialInstanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.confidential_instance_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `guest_accelerator` after provisioning.\n"]
    pub fn guest_accelerator(&self) -> terrars::ListRef<ComputeInstance_GuestAcceleratorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.guest_accelerator", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_encryption_key` after provisioning.\n"]
    pub fn instance_encryption_key(&self) -> terrars::ListRef<ComputeInstance_InstanceEncryptionKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_encryption_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_interface` after provisioning.\n"]
    pub fn network_interface(&self) -> terrars::ListRef<ComputeInstance_NetworkInterfaceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_interface", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_performance_config` after provisioning.\n"]
    pub fn network_performance_config(&self) -> terrars::ListRef<ComputeInstance_NetworkPerformanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_performance_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> terrars::ListRef<ComputeInstance_ParamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.params", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reservation_affinity` after provisioning.\n"]
    pub fn reservation_affinity(&self) -> terrars::ListRef<ComputeInstance_ReservationAffinityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reservation_affinity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scheduling` after provisioning.\n"]
    pub fn scheduling(&self) -> terrars::ListRef<ComputeInstance_SchedulingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scheduling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scratch_disk` after provisioning.\n"]
    pub fn scratch_disk(&self) -> terrars::ListRef<ComputeInstance_ScratchDiskRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scratch_disk", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_account` after provisioning.\n"]
    pub fn service_account(&self) -> terrars::ListRef<ComputeInstance_ServiceAccountRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_account", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `shielded_instance_config` after provisioning.\n"]
    pub fn shielded_instance_config(&self) -> terrars::ListRef<ComputeInstance_ShieldedInstanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shielded_instance_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeInstance_TimeoutsRef {
        ComputeInstance_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `workload_identity_config` after provisioning.\n"]
    pub fn workload_identity_config(&self) -> terrars::ListRef<ComputeInstance_WorkloadIdentityConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workload_identity_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ComputeInstanceDynamic {
    advanced_machine_features: std::option::Option<terrars::DynamicBlock<ComputeInstance_AdvancedMachineFeatures>>,
    attached_disk: std::option::Option<terrars::DynamicBlock<ComputeInstance_AttachedDisk>>,
    boot_disk: std::option::Option<terrars::DynamicBlock<ComputeInstance_BootDisk>>,
    confidential_instance_config: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeInstance_ConfidentialInstanceConfig>>,
    guest_accelerator: std::option::Option<terrars::DynamicBlock<ComputeInstance_GuestAccelerator>>,
    instance_encryption_key: std::option::Option<terrars::DynamicBlock<ComputeInstance_InstanceEncryptionKey>>,
    network_interface: std::option::Option<terrars::DynamicBlock<ComputeInstance_NetworkInterface>>,
    network_performance_config: std::option::Option<terrars::DynamicBlock<ComputeInstance_NetworkPerformanceConfig>>,
    params: std::option::Option<terrars::DynamicBlock<ComputeInstance_Params>>,
    reservation_affinity: std::option::Option<terrars::DynamicBlock<ComputeInstance_ReservationAffinity>>,
    scheduling: std::option::Option<terrars::DynamicBlock<ComputeInstance_Scheduling>>,
    scratch_disk: std::option::Option<terrars::DynamicBlock<ComputeInstance_ScratchDisk>>,
    service_account: std::option::Option<terrars::DynamicBlock<ComputeInstance_ServiceAccount>>,
    shielded_instance_config: std::option::Option<terrars::DynamicBlock<ComputeInstance_ShieldedInstanceConfig>>,
    workload_identity_config: std::option::Option<terrars::DynamicBlock<ComputeInstance_WorkloadIdentityConfig>>,
}

#[derive(Serialize)]
pub struct ComputeInstance_AdvancedMachineFeatures {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_nested_virtualization: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_uefi_networking: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    performance_monitoring_unit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threads_per_core: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    turbo_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    visible_core_count: std::option::Option<terrars::PrimField<f64>>,
}

impl ComputeInstance_AdvancedMachineFeatures {
    #[doc = "Set the field `enable_nested_virtualization`.\nWhether to enable nested virtualization or not."]
    pub fn set_enable_nested_virtualization(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_nested_virtualization = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_uefi_networking`.\nWhether to enable UEFI networking for the instance."]
    pub fn set_enable_uefi_networking(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_uefi_networking = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `performance_monitoring_unit`.\nThe PMU is a hardware component within the CPU core that monitors how the processor runs code. Valid values for the level of PMU are \"STANDARD\", \"ENHANCED\", and \"ARCHITECTURAL\"."]
    pub fn set_performance_monitoring_unit(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.performance_monitoring_unit = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `threads_per_core`.\nThe number of threads per physical core. To disable simultaneous multithreading (SMT) set this to 1. If unset, the maximum number of threads supported per core by the underlying processor is assumed."]
    pub fn set_threads_per_core(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.threads_per_core = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `turbo_mode`.\nTurbo frequency mode to use for the instance. Currently supported modes is \"ALL_CORE_MAX\"."]
    pub fn set_turbo_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.turbo_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `visible_core_count`.\nThe number of physical cores to expose to an instance. Multiply by the number of threads per core to compute the total number of virtual CPUs to expose to the instance. If unset, the number of cores is inferred from the instance\\'s nominal CPU count and the underlying platform\\'s SMT width."]
    pub fn set_visible_core_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.visible_core_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInstance_AdvancedMachineFeatures {
    type O = terrars::BlockAssignable<ComputeInstance_AdvancedMachineFeatures>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstance_AdvancedMachineFeatures {}

impl BuildComputeInstance_AdvancedMachineFeatures {
    pub fn build(self) -> ComputeInstance_AdvancedMachineFeatures {
        ComputeInstance_AdvancedMachineFeatures {
            enable_nested_virtualization: core::default::Default::default(),
            enable_uefi_networking: core::default::Default::default(),
            performance_monitoring_unit: core::default::Default::default(),
            threads_per_core: core::default::Default::default(),
            turbo_mode: core::default::Default::default(),
            visible_core_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstance_AdvancedMachineFeaturesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstance_AdvancedMachineFeaturesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInstance_AdvancedMachineFeaturesRef {
        ComputeInstance_AdvancedMachineFeaturesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstance_AdvancedMachineFeaturesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_nested_virtualization` after provisioning.\nWhether to enable nested virtualization or not."]
    pub fn enable_nested_virtualization(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_nested_virtualization", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_uefi_networking` after provisioning.\nWhether to enable UEFI networking for the instance."]
    pub fn enable_uefi_networking(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_uefi_networking", self.base))
    }

    #[doc =
        "Get a reference to the value of field `performance_monitoring_unit` after provisioning.\nThe PMU is a hardware component within the CPU core that monitors how the processor runs code. Valid values for the level of PMU are \"STANDARD\", \"ENHANCED\", and \"ARCHITECTURAL\"."]
    pub fn performance_monitoring_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.performance_monitoring_unit", self.base))
    }

    #[doc =
        "Get a reference to the value of field `threads_per_core` after provisioning.\nThe number of threads per physical core. To disable simultaneous multithreading (SMT) set this to 1. If unset, the maximum number of threads supported per core by the underlying processor is assumed."]
    pub fn threads_per_core(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threads_per_core", self.base))
    }

    #[doc =
        "Get a reference to the value of field `turbo_mode` after provisioning.\nTurbo frequency mode to use for the instance. Currently supported modes is \"ALL_CORE_MAX\"."]
    pub fn turbo_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.turbo_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `visible_core_count` after provisioning.\nThe number of physical cores to expose to an instance. Multiply by the number of threads per core to compute the total number of virtual CPUs to expose to the instance. If unset, the number of cores is inferred from the instance\\'s nominal CPU count and the underlying platform\\'s SMT width."]
    pub fn visible_core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.visible_core_count", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInstance_AttachedDisk {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_encryption_key_raw: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_encryption_key_rsa: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_encryption_service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_attach: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_self_link: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    source: terrars::PrimField<std::string::String>,
}

impl ComputeInstance_AttachedDisk {
    #[doc = "Set the field `device_name`.\nName with which the attached disk is accessible under /dev/disk/by-id/"]
    pub fn set_device_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.device_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disk_encryption_key_raw`.\nA 256-bit customer-supplied encryption key, encoded in RFC 4648 base64 to encrypt this disk. Only one of kms_key_self_link, disk_encryption_key_rsa and disk_encryption_key_raw may be set."]
    pub fn set_disk_encryption_key_raw(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.disk_encryption_key_raw = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disk_encryption_key_rsa`.\nSpecifies an RFC 4648 base64 encoded, RSA-wrapped 2048-bit customer-supplied encryption key to either encrypt or decrypt this resource. Only one of kms_key_self_link, disk_encryption_key_rsa and disk_encryption_key_raw may be set."]
    pub fn set_disk_encryption_key_rsa(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.disk_encryption_key_rsa = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disk_encryption_service_account`.\nThe service account being used for the encryption request for the given KMS key. If absent, the Compute Engine default service account is used"]
    pub fn set_disk_encryption_service_account(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.disk_encryption_service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `force_attach`.\nWhether to force attach the regional disk even if it's currently attached to another instance. If you try to force attach a zonal disk to an instance, you will receive an error. Setting this parameter cause VM recreation."]
    pub fn set_force_attach(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.force_attach = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kms_key_self_link`.\nThe self_link of the encryption key that is stored in Google Cloud KMS to encrypt this disk. Only one of kms_key_self_link, disk_encryption_key_rsa and disk_encryption_key_raw may be set."]
    pub fn set_kms_key_self_link(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_self_link = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mode`.\nRead/write mode for the disk. One of \"READ_ONLY\" or \"READ_WRITE\"."]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInstance_AttachedDisk {
    type O = terrars::BlockAssignable<ComputeInstance_AttachedDisk>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstance_AttachedDisk {
    #[doc = "The name or self_link of the disk attached to this instance."]
    pub source: terrars::PrimField<std::string::String>,
}

impl BuildComputeInstance_AttachedDisk {
    pub fn build(self) -> ComputeInstance_AttachedDisk {
        ComputeInstance_AttachedDisk {
            device_name: core::default::Default::default(),
            disk_encryption_key_raw: core::default::Default::default(),
            disk_encryption_key_rsa: core::default::Default::default(),
            disk_encryption_service_account: core::default::Default::default(),
            force_attach: core::default::Default::default(),
            kms_key_self_link: core::default::Default::default(),
            mode: core::default::Default::default(),
            source: self.source,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstance_AttachedDiskRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstance_AttachedDiskRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInstance_AttachedDiskRef {
        ComputeInstance_AttachedDiskRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstance_AttachedDiskRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `device_name` after provisioning.\nName with which the attached disk is accessible under /dev/disk/by-id/"]
    pub fn device_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disk_encryption_key_raw` after provisioning.\nA 256-bit customer-supplied encryption key, encoded in RFC 4648 base64 to encrypt this disk. Only one of kms_key_self_link, disk_encryption_key_rsa and disk_encryption_key_raw may be set."]
    pub fn disk_encryption_key_raw(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_encryption_key_raw", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disk_encryption_key_rsa` after provisioning.\nSpecifies an RFC 4648 base64 encoded, RSA-wrapped 2048-bit customer-supplied encryption key to either encrypt or decrypt this resource. Only one of kms_key_self_link, disk_encryption_key_rsa and disk_encryption_key_raw may be set."]
    pub fn disk_encryption_key_rsa(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_encryption_key_rsa", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disk_encryption_key_sha256` after provisioning.\nThe RFC 4648 base64 encoded SHA-256 hash of the customer-supplied encryption key that protects this resource."]
    pub fn disk_encryption_key_sha256(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_encryption_key_sha256", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disk_encryption_service_account` after provisioning.\nThe service account being used for the encryption request for the given KMS key. If absent, the Compute Engine default service account is used"]
    pub fn disk_encryption_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_encryption_service_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `force_attach` after provisioning.\nWhether to force attach the regional disk even if it's currently attached to another instance. If you try to force attach a zonal disk to an instance, you will receive an error. Setting this parameter cause VM recreation."]
    pub fn force_attach(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_attach", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_self_link` after provisioning.\nThe self_link of the encryption key that is stored in Google Cloud KMS to encrypt this disk. Only one of kms_key_self_link, disk_encryption_key_rsa and disk_encryption_key_raw may be set."]
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
}

#[derive(Serialize)]
pub struct ComputeInstance_BootDisk_InitializeParams_SourceImageEncryptionKey {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_self_link: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    raw_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rsa_encrypted_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeInstance_BootDisk_InitializeParams_SourceImageEncryptionKey {
    #[doc =
        "Set the field `kms_key_self_link`.\nThe self link of the encryption key that is stored in Google Cloud KMS. Only one of kms_key_self_link, rsa_encrypted_key and raw_key may be set."]
    pub fn set_kms_key_self_link(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_self_link = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kms_key_service_account`.\nThe service account being used for the encryption request for the given KMS key. If absent, the Compute Engine default service account is used."]
    pub fn set_kms_key_service_account(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.kms_key_service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `raw_key`.\nSpecifies a 256-bit customer-supplied encryption key, encoded in RFC 4648 base64 to either encrypt or decrypt this resource. Only one of kms_key_self_link, rsa_encrypted_key and raw_key may be set."]
    pub fn set_raw_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.raw_key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `rsa_encrypted_key`.\nSpecifies an RFC 4648 base64 encoded, RSA-wrapped 2048-bit customer-supplied encryption key to either encrypt or decrypt this resource. Only one of kms_key_self_link, rsa_encrypted_key and raw_key may be set."]
    pub fn set_rsa_encrypted_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rsa_encrypted_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInstance_BootDisk_InitializeParams_SourceImageEncryptionKey {
    type O = terrars::BlockAssignable<ComputeInstance_BootDisk_InitializeParams_SourceImageEncryptionKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstance_BootDisk_InitializeParams_SourceImageEncryptionKey {}

impl BuildComputeInstance_BootDisk_InitializeParams_SourceImageEncryptionKey {
    pub fn build(self) -> ComputeInstance_BootDisk_InitializeParams_SourceImageEncryptionKey {
        ComputeInstance_BootDisk_InitializeParams_SourceImageEncryptionKey {
            kms_key_self_link: core::default::Default::default(),
            kms_key_service_account: core::default::Default::default(),
            raw_key: core::default::Default::default(),
            rsa_encrypted_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstance_BootDisk_InitializeParams_SourceImageEncryptionKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstance_BootDisk_InitializeParams_SourceImageEncryptionKeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstance_BootDisk_InitializeParams_SourceImageEncryptionKeyRef {
        ComputeInstance_BootDisk_InitializeParams_SourceImageEncryptionKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstance_BootDisk_InitializeParams_SourceImageEncryptionKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_self_link` after provisioning.\nThe self link of the encryption key that is stored in Google Cloud KMS. Only one of kms_key_self_link, rsa_encrypted_key and raw_key may be set."]
    pub fn kms_key_self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_self_link", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_service_account` after provisioning.\nThe service account being used for the encryption request for the given KMS key. If absent, the Compute Engine default service account is used."]
    pub fn kms_key_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_service_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `raw_key` after provisioning.\nSpecifies a 256-bit customer-supplied encryption key, encoded in RFC 4648 base64 to either encrypt or decrypt this resource. Only one of kms_key_self_link, rsa_encrypted_key and raw_key may be set."]
    pub fn raw_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.raw_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rsa_encrypted_key` after provisioning.\nSpecifies an RFC 4648 base64 encoded, RSA-wrapped 2048-bit customer-supplied encryption key to either encrypt or decrypt this resource. Only one of kms_key_self_link, rsa_encrypted_key and raw_key may be set."]
    pub fn rsa_encrypted_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rsa_encrypted_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sha256` after provisioning.\nThe SHA256 hash of the encryption key used to encrypt this disk."]
    pub fn sha256(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sha256", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInstance_BootDisk_InitializeParams_SourceSnapshotEncryptionKey {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_self_link: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    raw_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rsa_encrypted_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeInstance_BootDisk_InitializeParams_SourceSnapshotEncryptionKey {
    #[doc =
        "Set the field `kms_key_self_link`.\nThe self link of the encryption key that is stored in Google Cloud KMS. Only one of kms_key_self_link, rsa_encrypted_key and raw_key may be set."]
    pub fn set_kms_key_self_link(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_self_link = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kms_key_service_account`.\nThe service account being used for the encryption request for the given KMS key. If absent, the Compute Engine default service account is used."]
    pub fn set_kms_key_service_account(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.kms_key_service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `raw_key`.\nSpecifies a 256-bit customer-supplied encryption key, encoded in RFC 4648 base64 to either encrypt or decrypt this resource. Only one of kms_key_self_link, rsa_encrypted_key and raw_key may be set."]
    pub fn set_raw_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.raw_key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `rsa_encrypted_key`.\nSpecifies an RFC 4648 base64 encoded, RSA-wrapped 2048-bit customer-supplied encryption key to either encrypt or decrypt this resource. Only one of kms_key_self_link, rsa_encrypted_key and raw_key may be set."]
    pub fn set_rsa_encrypted_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rsa_encrypted_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInstance_BootDisk_InitializeParams_SourceSnapshotEncryptionKey {
    type O = terrars::BlockAssignable<ComputeInstance_BootDisk_InitializeParams_SourceSnapshotEncryptionKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstance_BootDisk_InitializeParams_SourceSnapshotEncryptionKey {}

impl BuildComputeInstance_BootDisk_InitializeParams_SourceSnapshotEncryptionKey {
    pub fn build(self) -> ComputeInstance_BootDisk_InitializeParams_SourceSnapshotEncryptionKey {
        ComputeInstance_BootDisk_InitializeParams_SourceSnapshotEncryptionKey {
            kms_key_self_link: core::default::Default::default(),
            kms_key_service_account: core::default::Default::default(),
            raw_key: core::default::Default::default(),
            rsa_encrypted_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstance_BootDisk_InitializeParams_SourceSnapshotEncryptionKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstance_BootDisk_InitializeParams_SourceSnapshotEncryptionKeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstance_BootDisk_InitializeParams_SourceSnapshotEncryptionKeyRef {
        ComputeInstance_BootDisk_InitializeParams_SourceSnapshotEncryptionKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstance_BootDisk_InitializeParams_SourceSnapshotEncryptionKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_self_link` after provisioning.\nThe self link of the encryption key that is stored in Google Cloud KMS. Only one of kms_key_self_link, rsa_encrypted_key and raw_key may be set."]
    pub fn kms_key_self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_self_link", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_service_account` after provisioning.\nThe service account being used for the encryption request for the given KMS key. If absent, the Compute Engine default service account is used."]
    pub fn kms_key_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_service_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `raw_key` after provisioning.\nSpecifies a 256-bit customer-supplied encryption key, encoded in RFC 4648 base64 to either encrypt or decrypt this resource. Only one of kms_key_self_link, rsa_encrypted_key and raw_key may be set."]
    pub fn raw_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.raw_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rsa_encrypted_key` after provisioning.\nSpecifies an RFC 4648 base64 encoded, RSA-wrapped 2048-bit customer-supplied encryption key to either encrypt or decrypt this resource. Only one of kms_key_self_link, rsa_encrypted_key and raw_key may be set."]
    pub fn rsa_encrypted_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rsa_encrypted_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sha256` after provisioning.\nThe SHA256 hash of the encryption key used to encrypt this disk."]
    pub fn sha256(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sha256", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeInstance_BootDisk_InitializeParamsDynamic {
    source_image_encryption_key: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeInstance_BootDisk_InitializeParams_SourceImageEncryptionKey>>,
    source_snapshot_encryption_key: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeInstance_BootDisk_InitializeParams_SourceSnapshotEncryptionKey>>,
}

#[derive(Serialize)]
pub struct ComputeInstance_BootDisk_InitializeParams {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    architecture: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_confidential_compute: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisioned_iops: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisioned_throughput: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replica_zones: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_manager_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_policies: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_pool: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_image_encryption_key: std
    ::option
    ::Option<std::vec::Vec<ComputeInstance_BootDisk_InitializeParams_SourceImageEncryptionKey>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_snapshot_encryption_key: std
    ::option
    ::Option<std::vec::Vec<ComputeInstance_BootDisk_InitializeParams_SourceSnapshotEncryptionKey>>,
    dynamic: ComputeInstance_BootDisk_InitializeParamsDynamic,
}

impl ComputeInstance_BootDisk_InitializeParams {
    #[doc = "Set the field `architecture`.\nThe architecture of the disk. One of \"X86_64\" or \"ARM64\"."]
    pub fn set_architecture(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.architecture = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_confidential_compute`.\nA flag to enable confidential compute mode on boot disk"]
    pub fn set_enable_confidential_compute(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_confidential_compute = std::option::Option::Some(v.into());
        self
    }

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
        "Set the field `provisioned_iops`.\nIndicates how many IOPS to provision for the disk. This sets the number of I/O operations per second that the disk can handle."]
    pub fn set_provisioned_iops(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.provisioned_iops = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `provisioned_throughput`.\nIndicates how much throughput to provision for the disk. This sets the number of throughput mb per second that the disk can handle."]
    pub fn set_provisioned_throughput(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.provisioned_throughput = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `replica_zones`.\nA list of short names or self_links of zones in which to create a regional disk."]
    pub fn set_replica_zones(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.replica_zones = std::option::Option::Some(v.into());
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

    #[doc =
        "Set the field `resource_policies`.\nA list of self_links of resource policies to attach to the instance's boot disk. Modifying this list will cause the instance to recreate. Currently a max of 1 resource policy is supported."]
    pub fn set_resource_policies(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resource_policies = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `size`.\nThe size of the image in gigabytes."]
    pub fn set_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `snapshot`.\nThe snapshot from which this disk was initialised."]
    pub fn set_snapshot(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.snapshot = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_pool`.\nThe URL of the storage pool in which the new disk is created"]
    pub fn set_storage_pool(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.storage_pool = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\nThe Google Compute Engine disk type. Such as pd-standard, pd-ssd or pd-balanced."]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_image_encryption_key`.\n"]
    pub fn set_source_image_encryption_key(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ComputeInstance_BootDisk_InitializeParams_SourceImageEncryptionKey>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source_image_encryption_key = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source_image_encryption_key = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_snapshot_encryption_key`.\n"]
    pub fn set_source_snapshot_encryption_key(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ComputeInstance_BootDisk_InitializeParams_SourceSnapshotEncryptionKey>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source_snapshot_encryption_key = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source_snapshot_encryption_key = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeInstance_BootDisk_InitializeParams {
    type O = terrars::BlockAssignable<ComputeInstance_BootDisk_InitializeParams>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstance_BootDisk_InitializeParams {}

impl BuildComputeInstance_BootDisk_InitializeParams {
    pub fn build(self) -> ComputeInstance_BootDisk_InitializeParams {
        ComputeInstance_BootDisk_InitializeParams {
            architecture: core::default::Default::default(),
            enable_confidential_compute: core::default::Default::default(),
            image: core::default::Default::default(),
            labels: core::default::Default::default(),
            provisioned_iops: core::default::Default::default(),
            provisioned_throughput: core::default::Default::default(),
            replica_zones: core::default::Default::default(),
            resource_manager_tags: core::default::Default::default(),
            resource_policies: core::default::Default::default(),
            size: core::default::Default::default(),
            snapshot: core::default::Default::default(),
            storage_pool: core::default::Default::default(),
            type_: core::default::Default::default(),
            source_image_encryption_key: core::default::Default::default(),
            source_snapshot_encryption_key: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstance_BootDisk_InitializeParamsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstance_BootDisk_InitializeParamsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInstance_BootDisk_InitializeParamsRef {
        ComputeInstance_BootDisk_InitializeParamsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstance_BootDisk_InitializeParamsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `architecture` after provisioning.\nThe architecture of the disk. One of \"X86_64\" or \"ARM64\"."]
    pub fn architecture(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.architecture", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_confidential_compute` after provisioning.\nA flag to enable confidential compute mode on boot disk"]
    pub fn enable_confidential_compute(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_confidential_compute", self.base))
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
        "Get a reference to the value of field `provisioned_iops` after provisioning.\nIndicates how many IOPS to provision for the disk. This sets the number of I/O operations per second that the disk can handle."]
    pub fn provisioned_iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioned_iops", self.base))
    }

    #[doc =
        "Get a reference to the value of field `provisioned_throughput` after provisioning.\nIndicates how much throughput to provision for the disk. This sets the number of throughput mb per second that the disk can handle."]
    pub fn provisioned_throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioned_throughput", self.base))
    }

    #[doc =
        "Get a reference to the value of field `replica_zones` after provisioning.\nA list of short names or self_links of zones in which to create a regional disk."]
    pub fn replica_zones(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replica_zones", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource_manager_tags` after provisioning.\nA map of resource manager tags. Resource manager tag keys and values have the same definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/456. The field is ignored (both PUT & PATCH) when empty."]
    pub fn resource_manager_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.resource_manager_tags", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource_policies` after provisioning.\nA list of self_links of resource policies to attach to the instance's boot disk. Modifying this list will cause the instance to recreate. Currently a max of 1 resource policy is supported."]
    pub fn resource_policies(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_policies", self.base))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\nThe size of the image in gigabytes."]
    pub fn size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }

    #[doc =
        "Get a reference to the value of field `snapshot` after provisioning.\nThe snapshot from which this disk was initialised."]
    pub fn snapshot(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot", self.base))
    }

    #[doc =
        "Get a reference to the value of field `storage_pool` after provisioning.\nThe URL of the storage pool in which the new disk is created"]
    pub fn storage_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_pool", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe Google Compute Engine disk type. Such as pd-standard, pd-ssd or pd-balanced."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `source_image_encryption_key` after provisioning.\n"]
    pub fn source_image_encryption_key(
        &self,
    ) -> terrars::ListRef<ComputeInstance_BootDisk_InitializeParams_SourceImageEncryptionKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_image_encryption_key", self.base))
    }

    #[doc = "Get a reference to the value of field `source_snapshot_encryption_key` after provisioning.\n"]
    pub fn source_snapshot_encryption_key(
        &self,
    ) -> terrars::ListRef<ComputeInstance_BootDisk_InitializeParams_SourceSnapshotEncryptionKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_snapshot_encryption_key", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeInstance_BootDiskDynamic {
    initialize_params: std::option::Option<terrars::DynamicBlock<ComputeInstance_BootDisk_InitializeParams>>,
}

#[derive(Serialize)]
pub struct ComputeInstance_BootDisk {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_delete: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_encryption_key_raw: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_encryption_key_rsa: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_encryption_service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_attach: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guest_os_features: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interface: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_self_link: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initialize_params: std::option::Option<std::vec::Vec<ComputeInstance_BootDisk_InitializeParams>>,
    dynamic: ComputeInstance_BootDiskDynamic,
}

impl ComputeInstance_BootDisk {
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
        "Set the field `disk_encryption_key_raw`.\nA 256-bit customer-supplied encryption key, encoded in RFC 4648 base64 to encrypt this disk. Only one of kms_key_self_link, disk_encryption_key_raw and disk_encryption_key_rsa may be set."]
    pub fn set_disk_encryption_key_raw(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.disk_encryption_key_raw = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disk_encryption_key_rsa`.\nSpecifies an RFC 4648 base64 encoded, RSA-wrapped 2048-bit customer-supplied encryption key to either encrypt or decrypt this resource. Only one of kms_key_self_link, disk_encryption_key_raw and disk_encryption_key_rsa may be set."]
    pub fn set_disk_encryption_key_rsa(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.disk_encryption_key_rsa = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disk_encryption_service_account`.\nThe service account being used for the encryption request for the given KMS key. If absent, the Compute Engine default service account is used"]
    pub fn set_disk_encryption_service_account(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.disk_encryption_service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `force_attach`.\nWhether to force attach the regional disk even if it's currently attached to another instance. If you try to force attach a zonal disk to an instance, you will receive an error. Setting this parameter cause VM recreation."]
    pub fn set_force_attach(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.force_attach = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `guest_os_features`.\nA list of features to enable on the guest operating system. Applicable only for bootable images."]
    pub fn set_guest_os_features(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.guest_os_features = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `interface`.\nThe disk interface used for attaching this disk. One of SCSI or NVME. (This field is shared with attached_disk and only used for specific cases, please don't specify this field without advice from Google.)"]
    pub fn set_interface(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.interface = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kms_key_self_link`.\nThe self_link of the encryption key that is stored in Google Cloud KMS to encrypt this disk. Only one of kms_key_self_link, disk_encryption_key_raw and disk_encryption_key_rsa may be set."]
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstance_BootDisk_InitializeParams>>,
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

impl terrars::ToListMappable for ComputeInstance_BootDisk {
    type O = terrars::BlockAssignable<ComputeInstance_BootDisk>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstance_BootDisk {}

impl BuildComputeInstance_BootDisk {
    pub fn build(self) -> ComputeInstance_BootDisk {
        ComputeInstance_BootDisk {
            auto_delete: core::default::Default::default(),
            device_name: core::default::Default::default(),
            disk_encryption_key_raw: core::default::Default::default(),
            disk_encryption_key_rsa: core::default::Default::default(),
            disk_encryption_service_account: core::default::Default::default(),
            force_attach: core::default::Default::default(),
            guest_os_features: core::default::Default::default(),
            interface: core::default::Default::default(),
            kms_key_self_link: core::default::Default::default(),
            mode: core::default::Default::default(),
            source: core::default::Default::default(),
            initialize_params: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstance_BootDiskRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstance_BootDiskRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInstance_BootDiskRef {
        ComputeInstance_BootDiskRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstance_BootDiskRef {
    #[allow(dead_code)]
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
        "Get a reference to the value of field `disk_encryption_key_raw` after provisioning.\nA 256-bit customer-supplied encryption key, encoded in RFC 4648 base64 to encrypt this disk. Only one of kms_key_self_link, disk_encryption_key_raw and disk_encryption_key_rsa may be set."]
    pub fn disk_encryption_key_raw(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_encryption_key_raw", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disk_encryption_key_rsa` after provisioning.\nSpecifies an RFC 4648 base64 encoded, RSA-wrapped 2048-bit customer-supplied encryption key to either encrypt or decrypt this resource. Only one of kms_key_self_link, disk_encryption_key_raw and disk_encryption_key_rsa may be set."]
    pub fn disk_encryption_key_rsa(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_encryption_key_rsa", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disk_encryption_key_sha256` after provisioning.\nThe RFC 4648 base64 encoded SHA-256 hash of the customer-supplied encryption key that protects this resource."]
    pub fn disk_encryption_key_sha256(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_encryption_key_sha256", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disk_encryption_service_account` after provisioning.\nThe service account being used for the encryption request for the given KMS key. If absent, the Compute Engine default service account is used"]
    pub fn disk_encryption_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_encryption_service_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `force_attach` after provisioning.\nWhether to force attach the regional disk even if it's currently attached to another instance. If you try to force attach a zonal disk to an instance, you will receive an error. Setting this parameter cause VM recreation."]
    pub fn force_attach(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_attach", self.base))
    }

    #[doc =
        "Get a reference to the value of field `guest_os_features` after provisioning.\nA list of features to enable on the guest operating system. Applicable only for bootable images."]
    pub fn guest_os_features(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.guest_os_features", self.base))
    }

    #[doc =
        "Get a reference to the value of field `interface` after provisioning.\nThe disk interface used for attaching this disk. One of SCSI or NVME. (This field is shared with attached_disk and only used for specific cases, please don't specify this field without advice from Google.)"]
    pub fn interface(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interface", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_self_link` after provisioning.\nThe self_link of the encryption key that is stored in Google Cloud KMS to encrypt this disk. Only one of kms_key_self_link, disk_encryption_key_raw and disk_encryption_key_rsa may be set."]
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
    pub fn initialize_params(&self) -> terrars::ListRef<ComputeInstance_BootDisk_InitializeParamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.initialize_params", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInstance_ConfidentialInstanceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confidential_instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_confidential_compute: std::option::Option<terrars::PrimField<bool>>,
}

impl ComputeInstance_ConfidentialInstanceConfig {
    #[doc =
        "Set the field `confidential_instance_type`.\n\n\t\t\t\t\t\t\t\tThe confidential computing technology the instance uses.\n\t\t\t\t\t\t\t\tSEV is an AMD feature. TDX is an Intel feature. One of the following\n\t\t\t\t\t\t\t\tvalues is required: SEV, SEV_SNP, TDX. If SEV_SNP, min_cpu_platform =\n\t\t\t\t\t\t\t\t\"AMD Milan\" is currently required."]
    pub fn set_confidential_instance_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.confidential_instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_confidential_compute`.\nDefines whether the instance should have confidential compute enabled. Field will be deprecated in a future release"]
    pub fn set_enable_confidential_compute(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_confidential_compute = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInstance_ConfidentialInstanceConfig {
    type O = terrars::BlockAssignable<ComputeInstance_ConfidentialInstanceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstance_ConfidentialInstanceConfig {}

impl BuildComputeInstance_ConfidentialInstanceConfig {
    pub fn build(self) -> ComputeInstance_ConfidentialInstanceConfig {
        ComputeInstance_ConfidentialInstanceConfig {
            confidential_instance_type: core::default::Default::default(),
            enable_confidential_compute: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstance_ConfidentialInstanceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstance_ConfidentialInstanceConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInstance_ConfidentialInstanceConfigRef {
        ComputeInstance_ConfidentialInstanceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstance_ConfidentialInstanceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `confidential_instance_type` after provisioning.\n\n\t\t\t\t\t\t\t\tThe confidential computing technology the instance uses.\n\t\t\t\t\t\t\t\tSEV is an AMD feature. TDX is an Intel feature. One of the following\n\t\t\t\t\t\t\t\tvalues is required: SEV, SEV_SNP, TDX. If SEV_SNP, min_cpu_platform =\n\t\t\t\t\t\t\t\t\"AMD Milan\" is currently required."]
    pub fn confidential_instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.confidential_instance_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_confidential_compute` after provisioning.\nDefines whether the instance should have confidential compute enabled. Field will be deprecated in a future release"]
    pub fn enable_confidential_compute(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_confidential_compute", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInstance_GuestAccelerator {
    count: terrars::PrimField<f64>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl ComputeInstance_GuestAccelerator { }

impl terrars::ToListMappable for ComputeInstance_GuestAccelerator {
    type O = terrars::BlockAssignable<ComputeInstance_GuestAccelerator>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstance_GuestAccelerator {
    #[doc = "The number of the guest accelerator cards exposed to this instance."]
    pub count: terrars::PrimField<f64>,
    #[doc = "The accelerator type resource exposed to this instance. E.g. nvidia-tesla-k80."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildComputeInstance_GuestAccelerator {
    pub fn build(self) -> ComputeInstance_GuestAccelerator {
        ComputeInstance_GuestAccelerator {
            count: self.count,
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstance_GuestAcceleratorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstance_GuestAcceleratorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInstance_GuestAcceleratorRef {
        ComputeInstance_GuestAcceleratorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstance_GuestAcceleratorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `count` after provisioning.\nThe number of the guest accelerator cards exposed to this instance."]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe accelerator type resource exposed to this instance. E.g. nvidia-tesla-k80."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInstance_InstanceEncryptionKey {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_self_link: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_service_account: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeInstance_InstanceEncryptionKey {
    #[doc =
        "Set the field `kms_key_self_link`.\nThe self link of the encryption key that is stored in Google Cloud KMS."]
    pub fn set_kms_key_self_link(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_self_link = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kms_key_service_account`.\nThe service account being used for the encryption request for the given KMS key. If absent, the Compute Engine default service account is used."]
    pub fn set_kms_key_service_account(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.kms_key_service_account = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInstance_InstanceEncryptionKey {
    type O = terrars::BlockAssignable<ComputeInstance_InstanceEncryptionKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstance_InstanceEncryptionKey {}

impl BuildComputeInstance_InstanceEncryptionKey {
    pub fn build(self) -> ComputeInstance_InstanceEncryptionKey {
        ComputeInstance_InstanceEncryptionKey {
            kms_key_self_link: core::default::Default::default(),
            kms_key_service_account: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstance_InstanceEncryptionKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstance_InstanceEncryptionKeyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInstance_InstanceEncryptionKeyRef {
        ComputeInstance_InstanceEncryptionKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstance_InstanceEncryptionKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_self_link` after provisioning.\nThe self link of the encryption key that is stored in Google Cloud KMS."]
    pub fn kms_key_self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_self_link", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_service_account` after provisioning.\nThe service account being used for the encryption request for the given KMS key. If absent, the Compute Engine default service account is used."]
    pub fn kms_key_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_service_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sha256` after provisioning.\nThe SHA256 hash of the customer's encryption key."]
    pub fn sha256(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sha256", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInstance_NetworkInterface_AccessConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nat_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_tier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_ptr_domain_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeInstance_NetworkInterface_AccessConfig {
    #[doc = "Set the field `nat_ip`.\nThe IP address that is be 1:1 mapped to the instance's network ip."]
    pub fn set_nat_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.nat_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `network_tier`.\nThe networking tier used for configuring this instance. One of PREMIUM or STANDARD."]
    pub fn set_network_tier(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network_tier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `public_ptr_domain_name`.\nThe DNS domain name for the public PTR record."]
    pub fn set_public_ptr_domain_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.public_ptr_domain_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInstance_NetworkInterface_AccessConfig {
    type O = terrars::BlockAssignable<ComputeInstance_NetworkInterface_AccessConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstance_NetworkInterface_AccessConfig {}

impl BuildComputeInstance_NetworkInterface_AccessConfig {
    pub fn build(self) -> ComputeInstance_NetworkInterface_AccessConfig {
        ComputeInstance_NetworkInterface_AccessConfig {
            nat_ip: core::default::Default::default(),
            network_tier: core::default::Default::default(),
            public_ptr_domain_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstance_NetworkInterface_AccessConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstance_NetworkInterface_AccessConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstance_NetworkInterface_AccessConfigRef {
        ComputeInstance_NetworkInterface_AccessConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstance_NetworkInterface_AccessConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `nat_ip` after provisioning.\nThe IP address that is be 1:1 mapped to the instance's network ip."]
    pub fn nat_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nat_ip", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network_tier` after provisioning.\nThe networking tier used for configuring this instance. One of PREMIUM or STANDARD."]
    pub fn network_tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_tier", self.base))
    }

    #[doc =
        "Get a reference to the value of field `public_ptr_domain_name` after provisioning.\nThe DNS domain name for the public PTR record."]
    pub fn public_ptr_domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_ptr_domain_name", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInstance_NetworkInterface_AliasIpRange {
    ip_cidr_range: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork_range_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeInstance_NetworkInterface_AliasIpRange {
    #[doc =
        "Set the field `subnetwork_range_name`.\nThe subnetwork secondary range name specifying the secondary range from which to allocate the IP CIDR range for this alias IP range."]
    pub fn set_subnetwork_range_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.subnetwork_range_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInstance_NetworkInterface_AliasIpRange {
    type O = terrars::BlockAssignable<ComputeInstance_NetworkInterface_AliasIpRange>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstance_NetworkInterface_AliasIpRange {
    #[doc = "The IP CIDR range represented by this alias IP range."]
    pub ip_cidr_range: terrars::PrimField<std::string::String>,
}

impl BuildComputeInstance_NetworkInterface_AliasIpRange {
    pub fn build(self) -> ComputeInstance_NetworkInterface_AliasIpRange {
        ComputeInstance_NetworkInterface_AliasIpRange {
            ip_cidr_range: self.ip_cidr_range,
            subnetwork_range_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstance_NetworkInterface_AliasIpRangeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstance_NetworkInterface_AliasIpRangeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstance_NetworkInterface_AliasIpRangeRef {
        ComputeInstance_NetworkInterface_AliasIpRangeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstance_NetworkInterface_AliasIpRangeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ip_cidr_range` after provisioning.\nThe IP CIDR range represented by this alias IP range."]
    pub fn ip_cidr_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_cidr_range", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subnetwork_range_name` after provisioning.\nThe subnetwork secondary range name specifying the secondary range from which to allocate the IP CIDR range for this alias IP range."]
    pub fn subnetwork_range_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork_range_name", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInstance_NetworkInterface_Ipv6AccessConfig {
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

impl ComputeInstance_NetworkInterface_Ipv6AccessConfig {
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

impl terrars::ToListMappable for ComputeInstance_NetworkInterface_Ipv6AccessConfig {
    type O = terrars::BlockAssignable<ComputeInstance_NetworkInterface_Ipv6AccessConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstance_NetworkInterface_Ipv6AccessConfig {
    #[doc =
        "The service-level to be provided for IPv6 traffic when the subnet has an external subnet. Only PREMIUM tier is valid for IPv6"]
    pub network_tier: terrars::PrimField<std::string::String>,
}

impl BuildComputeInstance_NetworkInterface_Ipv6AccessConfig {
    pub fn build(self) -> ComputeInstance_NetworkInterface_Ipv6AccessConfig {
        ComputeInstance_NetworkInterface_Ipv6AccessConfig {
            external_ipv6: core::default::Default::default(),
            external_ipv6_prefix_length: core::default::Default::default(),
            name: core::default::Default::default(),
            network_tier: self.network_tier,
            public_ptr_domain_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstance_NetworkInterface_Ipv6AccessConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstance_NetworkInterface_Ipv6AccessConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstance_NetworkInterface_Ipv6AccessConfigRef {
        ComputeInstance_NetworkInterface_Ipv6AccessConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstance_NetworkInterface_Ipv6AccessConfigRef {
    #[allow(dead_code)]
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
struct ComputeInstance_NetworkInterfaceDynamic {
    access_config: std::option::Option<terrars::DynamicBlock<ComputeInstance_NetworkInterface_AccessConfig>>,
    alias_ip_range: std::option::Option<terrars::DynamicBlock<ComputeInstance_NetworkInterface_AliasIpRange>>,
    ipv6_access_config: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeInstance_NetworkInterface_Ipv6AccessConfig>>,
}

#[derive(Serialize)]
pub struct ComputeInstance_NetworkInterface {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_vpc_scoped_dns: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    igmp_query: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internal_ipv6_prefix_length: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_attachment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nic_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    queue_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_class_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stack_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork_project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vlan: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_config: std::option::Option<std::vec::Vec<ComputeInstance_NetworkInterface_AccessConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alias_ip_range: std::option::Option<std::vec::Vec<ComputeInstance_NetworkInterface_AliasIpRange>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_access_config: std::option::Option<std::vec::Vec<ComputeInstance_NetworkInterface_Ipv6AccessConfig>>,
    dynamic: ComputeInstance_NetworkInterfaceDynamic,
}

impl ComputeInstance_NetworkInterface {
    #[doc =
        "Set the field `enable_vpc_scoped_dns`.\nIf true, DNS resolution will be enabled over this interface. Only valid with network_attachment."]
    pub fn set_enable_vpc_scoped_dns(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_vpc_scoped_dns = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `igmp_query`.\nIndicates whether igmp query is enabled on the network interface or not. If enabled, also indicates the version of IGMP supported."]
    pub fn set_igmp_query(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.igmp_query = std::option::Option::Some(v.into());
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

    #[doc =
        "Set the field `network_attachment`.\nThe URL of the network attachment that this interface should connect to in the following format: projects/{projectNumber}/regions/{region_name}/networkAttachments/{network_attachment_name}."]
    pub fn set_network_attachment(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.network_attachment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_ip`.\nThe private IP address assigned to the instance."]
    pub fn set_network_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `nic_type`.\nThe type of vNIC to be used on this interface. Possible values:GVNIC, VIRTIO_NET, IDPF, MRDMA, IRDMA and IDPF"]
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
        "Set the field `service_class_id`.\nProducer Service's Service class Id for the region of this network interface. Can only be used with network_attachment. It is not possible to use on its own; however, network_attachment can be used without service_class_id."]
    pub fn set_service_class_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_class_id = std::option::Option::Some(v.into());
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

    #[doc =
        "Set the field `vlan`.\nVLAN tag of a dynamic network interface, must be an integer in the range from 2 to 255 inclusively."]
    pub fn set_vlan(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.vlan = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `access_config`.\n"]
    pub fn set_access_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstance_NetworkInterface_AccessConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.access_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.access_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `alias_ip_range`.\n"]
    pub fn set_alias_ip_range(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstance_NetworkInterface_AliasIpRange>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.alias_ip_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.alias_ip_range = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ipv6_access_config`.\n"]
    pub fn set_ipv6_access_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstance_NetworkInterface_Ipv6AccessConfig>>,
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

impl terrars::ToListMappable for ComputeInstance_NetworkInterface {
    type O = terrars::BlockAssignable<ComputeInstance_NetworkInterface>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstance_NetworkInterface {}

impl BuildComputeInstance_NetworkInterface {
    pub fn build(self) -> ComputeInstance_NetworkInterface {
        ComputeInstance_NetworkInterface {
            enable_vpc_scoped_dns: core::default::Default::default(),
            igmp_query: core::default::Default::default(),
            internal_ipv6_prefix_length: core::default::Default::default(),
            ipv6_address: core::default::Default::default(),
            network: core::default::Default::default(),
            network_attachment: core::default::Default::default(),
            network_ip: core::default::Default::default(),
            nic_type: core::default::Default::default(),
            queue_count: core::default::Default::default(),
            service_class_id: core::default::Default::default(),
            stack_type: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
            subnetwork_project: core::default::Default::default(),
            vlan: core::default::Default::default(),
            access_config: core::default::Default::default(),
            alias_ip_range: core::default::Default::default(),
            ipv6_access_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstance_NetworkInterfaceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstance_NetworkInterfaceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInstance_NetworkInterfaceRef {
        ComputeInstance_NetworkInterfaceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstance_NetworkInterfaceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_vpc_scoped_dns` after provisioning.\nIf true, DNS resolution will be enabled over this interface. Only valid with network_attachment."]
    pub fn enable_vpc_scoped_dns(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_vpc_scoped_dns", self.base))
    }

    #[doc =
        "Get a reference to the value of field `igmp_query` after provisioning.\nIndicates whether igmp query is enabled on the network interface or not. If enabled, also indicates the version of IGMP supported."]
    pub fn igmp_query(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.igmp_query", self.base))
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
        "Get a reference to the value of field `network_attachment` after provisioning.\nThe URL of the network attachment that this interface should connect to in the following format: projects/{projectNumber}/regions/{region_name}/networkAttachments/{network_attachment_name}."]
    pub fn network_attachment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_attachment", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network_ip` after provisioning.\nThe private IP address assigned to the instance."]
    pub fn network_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_ip", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nic_type` after provisioning.\nThe type of vNIC to be used on this interface. Possible values:GVNIC, VIRTIO_NET, IDPF, MRDMA, IRDMA and IDPF"]
    pub fn nic_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nic_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `parent_nic_name` after provisioning.\nName of the parent network interface of a dynamic network interface."]
    pub fn parent_nic_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent_nic_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `queue_count` after provisioning.\nThe networking queue count that's specified by users for the network interface. Both Rx and Tx queues will be set to this number. It will be empty if not specified."]
    pub fn queue_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queue_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_class_id` after provisioning.\nProducer Service's Service class Id for the region of this network interface. Can only be used with network_attachment. It is not possible to use on its own; however, network_attachment can be used without service_class_id."]
    pub fn service_class_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_class_id", self.base))
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

    #[doc =
        "Get a reference to the value of field `vlan` after provisioning.\nVLAN tag of a dynamic network interface, must be an integer in the range from 2 to 255 inclusively."]
    pub fn vlan(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vlan", self.base))
    }

    #[doc = "Get a reference to the value of field `access_config` after provisioning.\n"]
    pub fn access_config(&self) -> terrars::ListRef<ComputeInstance_NetworkInterface_AccessConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_config", self.base))
    }

    #[doc = "Get a reference to the value of field `alias_ip_range` after provisioning.\n"]
    pub fn alias_ip_range(&self) -> terrars::ListRef<ComputeInstance_NetworkInterface_AliasIpRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alias_ip_range", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv6_access_config` after provisioning.\n"]
    pub fn ipv6_access_config(&self) -> terrars::ListRef<ComputeInstance_NetworkInterface_Ipv6AccessConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ipv6_access_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInstance_NetworkPerformanceConfig {
    total_egress_bandwidth_tier: terrars::PrimField<std::string::String>,
}

impl ComputeInstance_NetworkPerformanceConfig { }

impl terrars::ToListMappable for ComputeInstance_NetworkPerformanceConfig {
    type O = terrars::BlockAssignable<ComputeInstance_NetworkPerformanceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstance_NetworkPerformanceConfig {
    #[doc = "The egress bandwidth tier to enable. Possible values:TIER_1, DEFAULT"]
    pub total_egress_bandwidth_tier: terrars::PrimField<std::string::String>,
}

impl BuildComputeInstance_NetworkPerformanceConfig {
    pub fn build(self) -> ComputeInstance_NetworkPerformanceConfig {
        ComputeInstance_NetworkPerformanceConfig { total_egress_bandwidth_tier: self.total_egress_bandwidth_tier }
    }
}

#[allow(dead_code)]
pub struct ComputeInstance_NetworkPerformanceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstance_NetworkPerformanceConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInstance_NetworkPerformanceConfigRef {
        ComputeInstance_NetworkPerformanceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstance_NetworkPerformanceConfigRef {
    #[allow(dead_code)]
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
pub struct ComputeInstance_Params {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_manager_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl ComputeInstance_Params {
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

impl terrars::ToListMappable for ComputeInstance_Params {
    type O = terrars::BlockAssignable<ComputeInstance_Params>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstance_Params {}

impl BuildComputeInstance_Params {
    pub fn build(self) -> ComputeInstance_Params {
        ComputeInstance_Params { resource_manager_tags: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ComputeInstance_ParamsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstance_ParamsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInstance_ParamsRef {
        ComputeInstance_ParamsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstance_ParamsRef {
    #[allow(dead_code)]
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
pub struct ComputeInstance_ReservationAffinity_SpecificReservation {
    key: terrars::PrimField<std::string::String>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl ComputeInstance_ReservationAffinity_SpecificReservation { }

impl terrars::ToListMappable for ComputeInstance_ReservationAffinity_SpecificReservation {
    type O = terrars::BlockAssignable<ComputeInstance_ReservationAffinity_SpecificReservation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstance_ReservationAffinity_SpecificReservation {
    #[doc =
        "Corresponds to the label key of a reservation resource. To target a SPECIFIC_RESERVATION by name, specify compute.googleapis.com/reservation-name as the key and specify the name of your reservation as the only value."]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = "Corresponds to the label values of a reservation resource."]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildComputeInstance_ReservationAffinity_SpecificReservation {
    pub fn build(self) -> ComputeInstance_ReservationAffinity_SpecificReservation {
        ComputeInstance_ReservationAffinity_SpecificReservation {
            key: self.key,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstance_ReservationAffinity_SpecificReservationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstance_ReservationAffinity_SpecificReservationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstance_ReservationAffinity_SpecificReservationRef {
        ComputeInstance_ReservationAffinity_SpecificReservationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstance_ReservationAffinity_SpecificReservationRef {
    #[allow(dead_code)]
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
struct ComputeInstance_ReservationAffinityDynamic {
    specific_reservation: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeInstance_ReservationAffinity_SpecificReservation>>,
}

#[derive(Serialize)]
pub struct ComputeInstance_ReservationAffinity {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    specific_reservation: std::option::Option<std::vec::Vec<ComputeInstance_ReservationAffinity_SpecificReservation>>,
    dynamic: ComputeInstance_ReservationAffinityDynamic,
}

impl ComputeInstance_ReservationAffinity {
    #[doc = "Set the field `specific_reservation`.\n"]
    pub fn set_specific_reservation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeInstance_ReservationAffinity_SpecificReservation>>,
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

impl terrars::ToListMappable for ComputeInstance_ReservationAffinity {
    type O = terrars::BlockAssignable<ComputeInstance_ReservationAffinity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstance_ReservationAffinity {
    #[doc = "The type of reservation from which this instance can consume resources."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildComputeInstance_ReservationAffinity {
    pub fn build(self) -> ComputeInstance_ReservationAffinity {
        ComputeInstance_ReservationAffinity {
            type_: self.type_,
            specific_reservation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstance_ReservationAffinityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstance_ReservationAffinityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInstance_ReservationAffinityRef {
        ComputeInstance_ReservationAffinityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstance_ReservationAffinityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of reservation from which this instance can consume resources."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `specific_reservation` after provisioning.\n"]
    pub fn specific_reservation(&self) -> terrars::ListRef<ComputeInstance_ReservationAffinity_SpecificReservationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.specific_reservation", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInstance_Scheduling_LocalSsdRecoveryTimeout {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    seconds: terrars::PrimField<f64>,
}

impl ComputeInstance_Scheduling_LocalSsdRecoveryTimeout {
    #[doc =
        "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond\nresolution. Durations less than one second are represented\nwith a 0 seconds field and a positive nanos field. Must\nbe from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInstance_Scheduling_LocalSsdRecoveryTimeout {
    type O = terrars::BlockAssignable<ComputeInstance_Scheduling_LocalSsdRecoveryTimeout>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstance_Scheduling_LocalSsdRecoveryTimeout {
    #[doc = "Span of time at a resolution of a second.\nMust be from 0 to 315,576,000,000 inclusive."]
    pub seconds: terrars::PrimField<f64>,
}

impl BuildComputeInstance_Scheduling_LocalSsdRecoveryTimeout {
    pub fn build(self) -> ComputeInstance_Scheduling_LocalSsdRecoveryTimeout {
        ComputeInstance_Scheduling_LocalSsdRecoveryTimeout {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstance_Scheduling_LocalSsdRecoveryTimeoutRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstance_Scheduling_LocalSsdRecoveryTimeoutRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstance_Scheduling_LocalSsdRecoveryTimeoutRef {
        ComputeInstance_Scheduling_LocalSsdRecoveryTimeoutRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstance_Scheduling_LocalSsdRecoveryTimeoutRef {
    #[allow(dead_code)]
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
pub struct ComputeInstance_Scheduling_MaxRunDuration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    seconds: terrars::PrimField<f64>,
}

impl ComputeInstance_Scheduling_MaxRunDuration {
    #[doc =
        "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond\nresolution. Durations less than one second are represented\nwith a 0 seconds field and a positive nanos field. Must\nbe from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInstance_Scheduling_MaxRunDuration {
    type O = terrars::BlockAssignable<ComputeInstance_Scheduling_MaxRunDuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstance_Scheduling_MaxRunDuration {
    #[doc = "Span of time at a resolution of a second.\nMust be from 0 to 315,576,000,000 inclusive."]
    pub seconds: terrars::PrimField<f64>,
}

impl BuildComputeInstance_Scheduling_MaxRunDuration {
    pub fn build(self) -> ComputeInstance_Scheduling_MaxRunDuration {
        ComputeInstance_Scheduling_MaxRunDuration {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstance_Scheduling_MaxRunDurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstance_Scheduling_MaxRunDurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInstance_Scheduling_MaxRunDurationRef {
        ComputeInstance_Scheduling_MaxRunDurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstance_Scheduling_MaxRunDurationRef {
    #[allow(dead_code)]
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
pub struct ComputeInstance_Scheduling_NodeAffinities {
    key: terrars::PrimField<std::string::String>,
    operator: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl ComputeInstance_Scheduling_NodeAffinities { }

impl terrars::ToListMappable for ComputeInstance_Scheduling_NodeAffinities {
    type O = terrars::BlockAssignable<ComputeInstance_Scheduling_NodeAffinities>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstance_Scheduling_NodeAffinities {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub operator: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildComputeInstance_Scheduling_NodeAffinities {
    pub fn build(self) -> ComputeInstance_Scheduling_NodeAffinities {
        ComputeInstance_Scheduling_NodeAffinities {
            key: self.key,
            operator: self.operator,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstance_Scheduling_NodeAffinitiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstance_Scheduling_NodeAffinitiesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInstance_Scheduling_NodeAffinitiesRef {
        ComputeInstance_Scheduling_NodeAffinitiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstance_Scheduling_NodeAffinitiesRef {
    #[allow(dead_code)]
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

#[derive(Serialize)]
pub struct ComputeInstance_Scheduling_OnInstanceStopAction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discard_local_ssd: std::option::Option<terrars::PrimField<bool>>,
}

impl ComputeInstance_Scheduling_OnInstanceStopAction {
    #[doc =
        "Set the field `discard_local_ssd`.\nIf true, the contents of any attached Local SSD disks will be discarded."]
    pub fn set_discard_local_ssd(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.discard_local_ssd = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInstance_Scheduling_OnInstanceStopAction {
    type O = terrars::BlockAssignable<ComputeInstance_Scheduling_OnInstanceStopAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstance_Scheduling_OnInstanceStopAction {}

impl BuildComputeInstance_Scheduling_OnInstanceStopAction {
    pub fn build(self) -> ComputeInstance_Scheduling_OnInstanceStopAction {
        ComputeInstance_Scheduling_OnInstanceStopAction { discard_local_ssd: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ComputeInstance_Scheduling_OnInstanceStopActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstance_Scheduling_OnInstanceStopActionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstance_Scheduling_OnInstanceStopActionRef {
        ComputeInstance_Scheduling_OnInstanceStopActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstance_Scheduling_OnInstanceStopActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `discard_local_ssd` after provisioning.\nIf true, the contents of any attached Local SSD disks will be discarded."]
    pub fn discard_local_ssd(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discard_local_ssd", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInstance_Scheduling_PreemptionNoticeDuration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    seconds: terrars::PrimField<f64>,
}

impl ComputeInstance_Scheduling_PreemptionNoticeDuration {
    #[doc =
        "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are represented with a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInstance_Scheduling_PreemptionNoticeDuration {
    type O = terrars::BlockAssignable<ComputeInstance_Scheduling_PreemptionNoticeDuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstance_Scheduling_PreemptionNoticeDuration {
    #[doc = "Span of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive."]
    pub seconds: terrars::PrimField<f64>,
}

impl BuildComputeInstance_Scheduling_PreemptionNoticeDuration {
    pub fn build(self) -> ComputeInstance_Scheduling_PreemptionNoticeDuration {
        ComputeInstance_Scheduling_PreemptionNoticeDuration {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstance_Scheduling_PreemptionNoticeDurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstance_Scheduling_PreemptionNoticeDurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeInstance_Scheduling_PreemptionNoticeDurationRef {
        ComputeInstance_Scheduling_PreemptionNoticeDurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstance_Scheduling_PreemptionNoticeDurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond resolution. Durations less than one second are represented with a 0 seconds field and a positive nanos field. Must be from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc =
        "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second. Must be from 0 to 315,576,000,000 inclusive."]
    pub fn seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeInstance_SchedulingDynamic {
    local_ssd_recovery_timeout: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeInstance_Scheduling_LocalSsdRecoveryTimeout>>,
    max_run_duration: std::option::Option<terrars::DynamicBlock<ComputeInstance_Scheduling_MaxRunDuration>>,
    node_affinities: std::option::Option<terrars::DynamicBlock<ComputeInstance_Scheduling_NodeAffinities>>,
    on_instance_stop_action: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeInstance_Scheduling_OnInstanceStopAction>>,
    preemption_notice_duration: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeInstance_Scheduling_PreemptionNoticeDuration>>,
}

#[derive(Serialize)]
pub struct ComputeInstance_Scheduling {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automatic_restart: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_domain: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_error_timeout_seconds: std::option::Option<terrars::PrimField<f64>>,
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
    termination_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_ssd_recovery_timeout: std::option::Option<std::vec::Vec<ComputeInstance_Scheduling_LocalSsdRecoveryTimeout>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_run_duration: std::option::Option<std::vec::Vec<ComputeInstance_Scheduling_MaxRunDuration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_affinities: std::option::Option<std::vec::Vec<ComputeInstance_Scheduling_NodeAffinities>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_instance_stop_action: std::option::Option<std::vec::Vec<ComputeInstance_Scheduling_OnInstanceStopAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preemption_notice_duration: std::option::Option<std::vec::Vec<ComputeInstance_Scheduling_PreemptionNoticeDuration>>,
    dynamic: ComputeInstance_SchedulingDynamic,
}

impl ComputeInstance_Scheduling {
    #[doc =
        "Set the field `automatic_restart`.\nSpecifies if the instance should be restarted if it was terminated by Compute Engine (not a user)."]
    pub fn set_automatic_restart(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.automatic_restart = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `availability_domain`.\nSpecifies the availability domain, which this instance should be scheduled on."]
    pub fn set_availability_domain(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.availability_domain = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `host_error_timeout_seconds`.\nSpecify the time in seconds for host error detection, the value must be within the range of [90, 330] with the increment of 30, if unset, the default behavior of host error recovery will be used."]
    pub fn set_host_error_timeout_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.host_error_timeout_seconds = std::option::Option::Some(v.into());
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

    #[doc =
        "Set the field `provisioning_model`.\nDescribes the desired provisioning model for the instance. Possible values are STANDARD, SPOT, FLEX_START, and RESERVATION_BOUND. For STANDARD, resources are provisioned immediately. For SPOT, resources are offered at a discount compared to standard pricing but may be preempted. For FLEX_START, resources are offered at a discount with flexible start times. For RESERVATION_BOUND, the instance is bound to a specific reservation and will only consume capacity from that reservation."]
    pub fn set_provisioning_model(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.provisioning_model = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `termination_time`.\nSpecifies the timestamp, when the instance will be terminated,\nin RFC3339 text format. If specified, the instance termination action\nwill be performed at the termination time."]
    pub fn set_termination_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.termination_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_ssd_recovery_timeout`.\n"]
    pub fn set_local_ssd_recovery_timeout(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstance_Scheduling_LocalSsdRecoveryTimeout>>,
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

    #[doc = "Set the field `max_run_duration`.\n"]
    pub fn set_max_run_duration(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstance_Scheduling_MaxRunDuration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.max_run_duration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.max_run_duration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `node_affinities`.\n"]
    pub fn set_node_affinities(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstance_Scheduling_NodeAffinities>>,
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

    #[doc = "Set the field `on_instance_stop_action`.\n"]
    pub fn set_on_instance_stop_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstance_Scheduling_OnInstanceStopAction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.on_instance_stop_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.on_instance_stop_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `preemption_notice_duration`.\n"]
    pub fn set_preemption_notice_duration(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeInstance_Scheduling_PreemptionNoticeDuration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.preemption_notice_duration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.preemption_notice_duration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeInstance_Scheduling {
    type O = terrars::BlockAssignable<ComputeInstance_Scheduling>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstance_Scheduling {}

impl BuildComputeInstance_Scheduling {
    pub fn build(self) -> ComputeInstance_Scheduling {
        ComputeInstance_Scheduling {
            automatic_restart: core::default::Default::default(),
            availability_domain: core::default::Default::default(),
            host_error_timeout_seconds: core::default::Default::default(),
            instance_termination_action: core::default::Default::default(),
            min_node_cpus: core::default::Default::default(),
            on_host_maintenance: core::default::Default::default(),
            preemptible: core::default::Default::default(),
            provisioning_model: core::default::Default::default(),
            termination_time: core::default::Default::default(),
            local_ssd_recovery_timeout: core::default::Default::default(),
            max_run_duration: core::default::Default::default(),
            node_affinities: core::default::Default::default(),
            on_instance_stop_action: core::default::Default::default(),
            preemption_notice_duration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstance_SchedulingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstance_SchedulingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInstance_SchedulingRef {
        ComputeInstance_SchedulingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstance_SchedulingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `automatic_restart` after provisioning.\nSpecifies if the instance should be restarted if it was terminated by Compute Engine (not a user)."]
    pub fn automatic_restart(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automatic_restart", self.base))
    }

    #[doc =
        "Get a reference to the value of field `availability_domain` after provisioning.\nSpecifies the availability domain, which this instance should be scheduled on."]
    pub fn availability_domain(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_domain", self.base))
    }

    #[doc =
        "Get a reference to the value of field `host_error_timeout_seconds` after provisioning.\nSpecify the time in seconds for host error detection, the value must be within the range of [90, 330] with the increment of 30, if unset, the default behavior of host error recovery will be used."]
    pub fn host_error_timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_error_timeout_seconds", self.base))
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
        "Get a reference to the value of field `provisioning_model` after provisioning.\nDescribes the desired provisioning model for the instance. Possible values are STANDARD, SPOT, FLEX_START, and RESERVATION_BOUND. For STANDARD, resources are provisioned immediately. For SPOT, resources are offered at a discount compared to standard pricing but may be preempted. For FLEX_START, resources are offered at a discount with flexible start times. For RESERVATION_BOUND, the instance is bound to a specific reservation and will only consume capacity from that reservation."]
    pub fn provisioning_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioning_model", self.base))
    }

    #[doc =
        "Get a reference to the value of field `termination_time` after provisioning.\nSpecifies the timestamp, when the instance will be terminated,\nin RFC3339 text format. If specified, the instance termination action\nwill be performed at the termination time."]
    pub fn termination_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.termination_time", self.base))
    }

    #[doc = "Get a reference to the value of field `local_ssd_recovery_timeout` after provisioning.\n"]
    pub fn local_ssd_recovery_timeout(&self) -> terrars::ListRef<ComputeInstance_Scheduling_LocalSsdRecoveryTimeoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.local_ssd_recovery_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `max_run_duration` after provisioning.\n"]
    pub fn max_run_duration(&self) -> terrars::ListRef<ComputeInstance_Scheduling_MaxRunDurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.max_run_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `on_instance_stop_action` after provisioning.\n"]
    pub fn on_instance_stop_action(&self) -> terrars::ListRef<ComputeInstance_Scheduling_OnInstanceStopActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.on_instance_stop_action", self.base))
    }

    #[doc = "Get a reference to the value of field `preemption_notice_duration` after provisioning.\n"]
    pub fn preemption_notice_duration(&self) -> terrars::ListRef<ComputeInstance_Scheduling_PreemptionNoticeDurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.preemption_notice_duration", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInstance_ScratchDisk {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_name: std::option::Option<terrars::PrimField<std::string::String>>,
    interface: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size: std::option::Option<terrars::PrimField<f64>>,
}

impl ComputeInstance_ScratchDisk {
    #[doc = "Set the field `device_name`.\nName with which the attached disk is accessible under /dev/disk/by-id/"]
    pub fn set_device_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.device_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `size`.\nThe size of the disk in gigabytes. One of 375 or 3000."]
    pub fn set_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInstance_ScratchDisk {
    type O = terrars::BlockAssignable<ComputeInstance_ScratchDisk>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstance_ScratchDisk {
    #[doc = "The disk interface used for attaching this disk. One of SCSI or NVME."]
    pub interface: terrars::PrimField<std::string::String>,
}

impl BuildComputeInstance_ScratchDisk {
    pub fn build(self) -> ComputeInstance_ScratchDisk {
        ComputeInstance_ScratchDisk {
            device_name: core::default::Default::default(),
            interface: self.interface,
            size: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstance_ScratchDiskRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstance_ScratchDiskRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInstance_ScratchDiskRef {
        ComputeInstance_ScratchDiskRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstance_ScratchDiskRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `device_name` after provisioning.\nName with which the attached disk is accessible under /dev/disk/by-id/"]
    pub fn device_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `interface` after provisioning.\nThe disk interface used for attaching this disk. One of SCSI or NVME."]
    pub fn interface(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interface", self.base))
    }

    #[doc =
        "Get a reference to the value of field `size` after provisioning.\nThe size of the disk in gigabytes. One of 375 or 3000."]
    pub fn size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInstance_ServiceAccount {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<terrars::PrimField<std::string::String>>,
    scopes: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl ComputeInstance_ServiceAccount {
    #[doc = "Set the field `email`.\nThe service account e-mail address."]
    pub fn set_email(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.email = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInstance_ServiceAccount {
    type O = terrars::BlockAssignable<ComputeInstance_ServiceAccount>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstance_ServiceAccount {
    #[doc = "A list of service scopes."]
    pub scopes: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildComputeInstance_ServiceAccount {
    pub fn build(self) -> ComputeInstance_ServiceAccount {
        ComputeInstance_ServiceAccount {
            email: core::default::Default::default(),
            scopes: self.scopes,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstance_ServiceAccountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstance_ServiceAccountRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInstance_ServiceAccountRef {
        ComputeInstance_ServiceAccountRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstance_ServiceAccountRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\nThe service account e-mail address."]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `scopes` after provisioning.\nA list of service scopes."]
    pub fn scopes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeInstance_ShieldedInstanceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_integrity_monitoring: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_secure_boot: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_vtpm: std::option::Option<terrars::PrimField<bool>>,
}

impl ComputeInstance_ShieldedInstanceConfig {
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

impl terrars::ToListMappable for ComputeInstance_ShieldedInstanceConfig {
    type O = terrars::BlockAssignable<ComputeInstance_ShieldedInstanceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstance_ShieldedInstanceConfig {}

impl BuildComputeInstance_ShieldedInstanceConfig {
    pub fn build(self) -> ComputeInstance_ShieldedInstanceConfig {
        ComputeInstance_ShieldedInstanceConfig {
            enable_integrity_monitoring: core::default::Default::default(),
            enable_secure_boot: core::default::Default::default(),
            enable_vtpm: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstance_ShieldedInstanceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstance_ShieldedInstanceConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInstance_ShieldedInstanceConfigRef {
        ComputeInstance_ShieldedInstanceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstance_ShieldedInstanceConfigRef {
    #[allow(dead_code)]
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
pub struct ComputeInstance_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeInstance_Timeouts {
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

impl terrars::ToListMappable for ComputeInstance_Timeouts {
    type O = terrars::BlockAssignable<ComputeInstance_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstance_Timeouts {}

impl BuildComputeInstance_Timeouts {
    pub fn build(self) -> ComputeInstance_Timeouts {
        ComputeInstance_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstance_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstance_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInstance_TimeoutsRef {
        ComputeInstance_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstance_TimeoutsRef {
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
pub struct ComputeInstance_WorkloadIdentityConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_certificate_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl ComputeInstance_WorkloadIdentityConfig {
    #[doc = "Set the field `identity`.\nIdentity SPIFFE id."]
    pub fn set_identity(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.identity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `identity_certificate_enabled`.\nSpecifies whether identity certificates are enabled."]
    pub fn set_identity_certificate_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.identity_certificate_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeInstance_WorkloadIdentityConfig {
    type O = terrars::BlockAssignable<ComputeInstance_WorkloadIdentityConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeInstance_WorkloadIdentityConfig {}

impl BuildComputeInstance_WorkloadIdentityConfig {
    pub fn build(self) -> ComputeInstance_WorkloadIdentityConfig {
        ComputeInstance_WorkloadIdentityConfig {
            identity: core::default::Default::default(),
            identity_certificate_enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeInstance_WorkloadIdentityConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeInstance_WorkloadIdentityConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeInstance_WorkloadIdentityConfigRef {
        ComputeInstance_WorkloadIdentityConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeInstance_WorkloadIdentityConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `identity` after provisioning.\nIdentity SPIFFE id."]
    pub fn identity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_certificate_enabled` after provisioning.\nSpecifies whether identity certificates are enabled."]
    pub fn identity_certificate_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_certificate_enabled", self.base))
    }
}
