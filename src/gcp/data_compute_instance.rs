use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataComputeInstanceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_link: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
}

struct DataComputeInstance_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataComputeInstanceData>,
}

#[derive(Clone)]
pub struct DataComputeInstance(Rc<DataComputeInstance_>);

impl DataComputeInstance {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\nThe name of the instance. One of name or self_link must be provided."]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\nThe ID of the project in which the resource belongs. If self_link is provided, this value is ignored. If neither self_link nor project are provided, the provider project is used."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `self_link`.\nThe URI of the created resource."]
    pub fn set_self_link(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().self_link = Some(v.into());
        self
    }

    #[doc= "Set the field `zone`.\nThe zone of the instance. If self_link is provided, this value is ignored. If neither self_link nor zone are provided, the provider zone is used."]
    pub fn set_zone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().zone = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `advanced_machine_features` after provisioning.\nControls for advanced machine-related behavior features."]
    pub fn advanced_machine_features(&self) -> ListRef<DataComputeInstanceAdvancedMachineFeaturesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.advanced_machine_features", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `allow_stopping_for_update` after provisioning.\nIf true, allows Terraform to stop the instance to update its properties. If you try to update a property that requires stopping the instance without setting this field, the update will fail."]
    pub fn allow_stopping_for_update(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_stopping_for_update", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `attached_disk` after provisioning.\nList of disks attached to the instance"]
    pub fn attached_disk(&self) -> ListRef<DataComputeInstanceAttachedDiskElRef> {
        ListRef::new(self.shared().clone(), format!("{}.attached_disk", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `boot_disk` after provisioning.\nThe boot disk for the instance."]
    pub fn boot_disk(&self) -> ListRef<DataComputeInstanceBootDiskElRef> {
        ListRef::new(self.shared().clone(), format!("{}.boot_disk", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `can_ip_forward` after provisioning.\nWhether sending and receiving of packets with non-matching source or destination IPs is allowed."]
    pub fn can_ip_forward(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.can_ip_forward", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `confidential_instance_config` after provisioning.\nThe Confidential VM config being used by the instance.  on_host_maintenance has to be set to TERMINATE or this will fail to create."]
    pub fn confidential_instance_config(&self) -> ListRef<DataComputeInstanceConfidentialInstanceConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.confidential_instance_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cpu_platform` after provisioning.\nThe CPU platform used by this instance."]
    pub fn cpu_platform(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_platform", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `current_status` after provisioning.\n\n\t\t\t\t\tCurrent status of the instance.\n\t\t\t\t\tThis could be one of the following values: PROVISIONING, STAGING, RUNNING, STOPPING, SUSPENDING, SUSPENDED, REPAIRING, and TERMINATED.\n\t\t\t\t\tFor more information about the status of the instance, see [Instance life cycle](https://cloud.google.com/compute/docs/instances/instance-life-cycle)."]
    pub fn current_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.current_status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether deletion protection is enabled on this instance."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA brief description of the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `desired_status` after provisioning.\nDesired status of the instance. Either \"RUNNING\" or \"TERMINATED\"."]
    pub fn desired_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.desired_status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_display` after provisioning.\nWhether the instance has virtual displays enabled."]
    pub fn enable_display(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_display", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `guest_accelerator` after provisioning.\nList of the type and count of accelerator cards attached to the instance."]
    pub fn guest_accelerator(&self) -> ListRef<DataComputeInstanceGuestAcceleratorElRef> {
        ListRef::new(self.shared().clone(), format!("{}.guest_accelerator", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hostname` after provisioning.\nA custom hostname for the instance. Must be a fully qualified DNS name and RFC-1035-valid. Valid format is a series of labels 1-63 characters long matching the regular expression [a-z]([-a-z0-9]*[a-z0-9]), concatenated with periods. The entire hostname must not exceed 253 characters. Changing this forces a new resource to be created."]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `instance_id` after provisioning.\nThe server-assigned unique identifier of this instance."]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `label_fingerprint` after provisioning.\nThe unique fingerprint of the labels."]
    pub fn label_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.label_fingerprint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs assigned to the instance.\n\t\t\t\t\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `machine_type` after provisioning.\nThe machine type to create."]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `metadata` after provisioning.\nMetadata key/value pairs made available within the instance."]
    pub fn metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.metadata", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `metadata_fingerprint` after provisioning.\nThe unique fingerprint of the metadata."]
    pub fn metadata_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metadata_fingerprint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `metadata_startup_script` after provisioning.\nMetadata startup scripts made available within the instance."]
    pub fn metadata_startup_script(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metadata_startup_script", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `min_cpu_platform` after provisioning.\nThe minimum CPU platform specified for the VM instance."]
    pub fn min_cpu_platform(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_cpu_platform", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the instance. One of name or self_link must be provided."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network_interface` after provisioning.\nThe networks attached to the instance."]
    pub fn network_interface(&self) -> ListRef<DataComputeInstanceNetworkInterfaceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_interface", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network_performance_config` after provisioning.\nConfigures network performance settings for the instance. If not specified, the instance will be created with its default network performance configuration."]
    pub fn network_performance_config(&self) -> ListRef<DataComputeInstanceNetworkPerformanceConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_performance_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `params` after provisioning.\nStores additional params passed with the request, but not persisted as part of resource payload."]
    pub fn params(&self) -> ListRef<DataComputeInstanceParamsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.params", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If self_link is provided, this value is ignored. If neither self_link nor project are provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `reservation_affinity` after provisioning.\nSpecifies the reservations that this instance can consume from."]
    pub fn reservation_affinity(&self) -> ListRef<DataComputeInstanceReservationAffinityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.reservation_affinity", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `resource_policies` after provisioning.\nA list of self_links of resource policies to attach to the instance. Currently a max of 1 resource policy is supported."]
    pub fn resource_policies(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.resource_policies", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `scheduling` after provisioning.\nThe scheduling strategy being used by the instance."]
    pub fn scheduling(&self) -> ListRef<DataComputeInstanceSchedulingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.scheduling", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `scratch_disk` after provisioning.\nThe scratch disks attached to the instance."]
    pub fn scratch_disk(&self) -> ListRef<DataComputeInstanceScratchDiskElRef> {
        ListRef::new(self.shared().clone(), format!("{}.scratch_disk", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service_account` after provisioning.\nThe service account to attach to the instance."]
    pub fn service_account(&self) -> ListRef<DataComputeInstanceServiceAccountElRef> {
        ListRef::new(self.shared().clone(), format!("{}.service_account", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `shielded_instance_config` after provisioning.\nThe shielded vm config being used by the instance."]
    pub fn shielded_instance_config(&self) -> ListRef<DataComputeInstanceShieldedInstanceConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.shielded_instance_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\nThe list of tags attached to the instance."]
    pub fn tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_fingerprint` after provisioning.\nThe unique fingerprint of the tags."]
    pub fn tags_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tags_fingerprint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone` after provisioning.\nThe zone of the instance. If self_link is provided, this value is ignored. If neither self_link nor zone are provided, the provider zone is used."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }
}

impl Referable for DataComputeInstance {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataComputeInstance { }

impl ToListMappable for DataComputeInstance {
    type O = ListRef<DataComputeInstanceRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataComputeInstance_ {
    fn extract_datasource_type(&self) -> String {
        "google_compute_instance".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataComputeInstance {
    pub tf_id: String,
}

impl BuildDataComputeInstance {
    pub fn build(self, stack: &mut Stack) -> DataComputeInstance {
        let out = DataComputeInstance(Rc::new(DataComputeInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataComputeInstanceData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
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

pub struct DataComputeInstanceRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataComputeInstanceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataComputeInstanceRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `advanced_machine_features` after provisioning.\nControls for advanced machine-related behavior features."]
    pub fn advanced_machine_features(&self) -> ListRef<DataComputeInstanceAdvancedMachineFeaturesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.advanced_machine_features", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `allow_stopping_for_update` after provisioning.\nIf true, allows Terraform to stop the instance to update its properties. If you try to update a property that requires stopping the instance without setting this field, the update will fail."]
    pub fn allow_stopping_for_update(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.allow_stopping_for_update", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `attached_disk` after provisioning.\nList of disks attached to the instance"]
    pub fn attached_disk(&self) -> ListRef<DataComputeInstanceAttachedDiskElRef> {
        ListRef::new(self.shared().clone(), format!("{}.attached_disk", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `boot_disk` after provisioning.\nThe boot disk for the instance."]
    pub fn boot_disk(&self) -> ListRef<DataComputeInstanceBootDiskElRef> {
        ListRef::new(self.shared().clone(), format!("{}.boot_disk", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `can_ip_forward` after provisioning.\nWhether sending and receiving of packets with non-matching source or destination IPs is allowed."]
    pub fn can_ip_forward(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.can_ip_forward", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `confidential_instance_config` after provisioning.\nThe Confidential VM config being used by the instance.  on_host_maintenance has to be set to TERMINATE or this will fail to create."]
    pub fn confidential_instance_config(&self) -> ListRef<DataComputeInstanceConfidentialInstanceConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.confidential_instance_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cpu_platform` after provisioning.\nThe CPU platform used by this instance."]
    pub fn cpu_platform(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_platform", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `current_status` after provisioning.\n\n\t\t\t\t\tCurrent status of the instance.\n\t\t\t\t\tThis could be one of the following values: PROVISIONING, STAGING, RUNNING, STOPPING, SUSPENDING, SUSPENDED, REPAIRING, and TERMINATED.\n\t\t\t\t\tFor more information about the status of the instance, see [Instance life cycle](https://cloud.google.com/compute/docs/instances/instance-life-cycle)."]
    pub fn current_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.current_status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether deletion protection is enabled on this instance."]
    pub fn deletion_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA brief description of the resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `desired_status` after provisioning.\nDesired status of the instance. Either \"RUNNING\" or \"TERMINATED\"."]
    pub fn desired_status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.desired_status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_display` after provisioning.\nWhether the instance has virtual displays enabled."]
    pub fn enable_display(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_display", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `guest_accelerator` after provisioning.\nList of the type and count of accelerator cards attached to the instance."]
    pub fn guest_accelerator(&self) -> ListRef<DataComputeInstanceGuestAcceleratorElRef> {
        ListRef::new(self.shared().clone(), format!("{}.guest_accelerator", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hostname` after provisioning.\nA custom hostname for the instance. Must be a fully qualified DNS name and RFC-1035-valid. Valid format is a series of labels 1-63 characters long matching the regular expression [a-z]([-a-z0-9]*[a-z0-9]), concatenated with periods. The entire hostname must not exceed 253 characters. Changing this forces a new resource to be created."]
    pub fn hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `instance_id` after provisioning.\nThe server-assigned unique identifier of this instance."]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `label_fingerprint` after provisioning.\nThe unique fingerprint of the labels."]
    pub fn label_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.label_fingerprint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs assigned to the instance.\n\t\t\t\t\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `machine_type` after provisioning.\nThe machine type to create."]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `metadata` after provisioning.\nMetadata key/value pairs made available within the instance."]
    pub fn metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.metadata", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `metadata_fingerprint` after provisioning.\nThe unique fingerprint of the metadata."]
    pub fn metadata_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metadata_fingerprint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `metadata_startup_script` after provisioning.\nMetadata startup scripts made available within the instance."]
    pub fn metadata_startup_script(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metadata_startup_script", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `min_cpu_platform` after provisioning.\nThe minimum CPU platform specified for the VM instance."]
    pub fn min_cpu_platform(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_cpu_platform", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the instance. One of name or self_link must be provided."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network_interface` after provisioning.\nThe networks attached to the instance."]
    pub fn network_interface(&self) -> ListRef<DataComputeInstanceNetworkInterfaceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_interface", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network_performance_config` after provisioning.\nConfigures network performance settings for the instance. If not specified, the instance will be created with its default network performance configuration."]
    pub fn network_performance_config(&self) -> ListRef<DataComputeInstanceNetworkPerformanceConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_performance_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `params` after provisioning.\nStores additional params passed with the request, but not persisted as part of resource payload."]
    pub fn params(&self) -> ListRef<DataComputeInstanceParamsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.params", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If self_link is provided, this value is ignored. If neither self_link nor project are provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `reservation_affinity` after provisioning.\nSpecifies the reservations that this instance can consume from."]
    pub fn reservation_affinity(&self) -> ListRef<DataComputeInstanceReservationAffinityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.reservation_affinity", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `resource_policies` after provisioning.\nA list of self_links of resource policies to attach to the instance. Currently a max of 1 resource policy is supported."]
    pub fn resource_policies(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.resource_policies", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `scheduling` after provisioning.\nThe scheduling strategy being used by the instance."]
    pub fn scheduling(&self) -> ListRef<DataComputeInstanceSchedulingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.scheduling", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `scratch_disk` after provisioning.\nThe scratch disks attached to the instance."]
    pub fn scratch_disk(&self) -> ListRef<DataComputeInstanceScratchDiskElRef> {
        ListRef::new(self.shared().clone(), format!("{}.scratch_disk", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service_account` after provisioning.\nThe service account to attach to the instance."]
    pub fn service_account(&self) -> ListRef<DataComputeInstanceServiceAccountElRef> {
        ListRef::new(self.shared().clone(), format!("{}.service_account", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `shielded_instance_config` after provisioning.\nThe shielded vm config being used by the instance."]
    pub fn shielded_instance_config(&self) -> ListRef<DataComputeInstanceShieldedInstanceConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.shielded_instance_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\nThe list of tags attached to the instance."]
    pub fn tags(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_fingerprint` after provisioning.\nThe unique fingerprint of the tags."]
    pub fn tags_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tags_fingerprint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone` after provisioning.\nThe zone of the instance. If self_link is provided, this value is ignored. If neither self_link nor zone are provided, the provider zone is used."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceAdvancedMachineFeaturesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_nested_virtualization: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threads_per_core: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visible_core_count: Option<PrimField<f64>>,
}

impl DataComputeInstanceAdvancedMachineFeaturesEl {
    #[doc= "Set the field `enable_nested_virtualization`.\n"]
    pub fn set_enable_nested_virtualization(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_nested_virtualization = Some(v.into());
        self
    }

    #[doc= "Set the field `threads_per_core`.\n"]
    pub fn set_threads_per_core(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.threads_per_core = Some(v.into());
        self
    }

    #[doc= "Set the field `visible_core_count`.\n"]
    pub fn set_visible_core_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.visible_core_count = Some(v.into());
        self
    }
}

impl ToListMappable for DataComputeInstanceAdvancedMachineFeaturesEl {
    type O = BlockAssignable<DataComputeInstanceAdvancedMachineFeaturesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceAdvancedMachineFeaturesEl {}

impl BuildDataComputeInstanceAdvancedMachineFeaturesEl {
    pub fn build(self) -> DataComputeInstanceAdvancedMachineFeaturesEl {
        DataComputeInstanceAdvancedMachineFeaturesEl {
            enable_nested_virtualization: core::default::Default::default(),
            threads_per_core: core::default::Default::default(),
            visible_core_count: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceAdvancedMachineFeaturesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataComputeInstanceAdvancedMachineFeaturesElRef {
    fn new(shared: StackShared, base: String) -> DataComputeInstanceAdvancedMachineFeaturesElRef {
        DataComputeInstanceAdvancedMachineFeaturesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceAdvancedMachineFeaturesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enable_nested_virtualization` after provisioning.\n"]
    pub fn enable_nested_virtualization(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_nested_virtualization", self.base))
    }

    #[doc= "Get a reference to the value of field `threads_per_core` after provisioning.\n"]
    pub fn threads_per_core(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.threads_per_core", self.base))
    }

    #[doc= "Get a reference to the value of field `visible_core_count` after provisioning.\n"]
    pub fn visible_core_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.visible_core_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceAttachedDiskEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    device_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_encryption_key_raw: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_encryption_key_sha256: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_self_link: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<PrimField<String>>,
}

impl DataComputeInstanceAttachedDiskEl {
    #[doc= "Set the field `device_name`.\n"]
    pub fn set_device_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.device_name = Some(v.into());
        self
    }

    #[doc= "Set the field `disk_encryption_key_raw`.\n"]
    pub fn set_disk_encryption_key_raw(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_encryption_key_raw = Some(v.into());
        self
    }

    #[doc= "Set the field `disk_encryption_key_sha256`.\n"]
    pub fn set_disk_encryption_key_sha256(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_encryption_key_sha256 = Some(v.into());
        self
    }

    #[doc= "Set the field `kms_key_self_link`.\n"]
    pub fn set_kms_key_self_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_self_link = Some(v.into());
        self
    }

    #[doc= "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }

    #[doc= "Set the field `source`.\n"]
    pub fn set_source(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source = Some(v.into());
        self
    }
}

impl ToListMappable for DataComputeInstanceAttachedDiskEl {
    type O = BlockAssignable<DataComputeInstanceAttachedDiskEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceAttachedDiskEl {}

impl BuildDataComputeInstanceAttachedDiskEl {
    pub fn build(self) -> DataComputeInstanceAttachedDiskEl {
        DataComputeInstanceAttachedDiskEl {
            device_name: core::default::Default::default(),
            disk_encryption_key_raw: core::default::Default::default(),
            disk_encryption_key_sha256: core::default::Default::default(),
            kms_key_self_link: core::default::Default::default(),
            mode: core::default::Default::default(),
            source: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceAttachedDiskElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataComputeInstanceAttachedDiskElRef {
    fn new(shared: StackShared, base: String) -> DataComputeInstanceAttachedDiskElRef {
        DataComputeInstanceAttachedDiskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceAttachedDiskElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `device_name` after provisioning.\n"]
    pub fn device_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }

    #[doc= "Get a reference to the value of field `disk_encryption_key_raw` after provisioning.\n"]
    pub fn disk_encryption_key_raw(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_encryption_key_raw", self.base))
    }

    #[doc= "Get a reference to the value of field `disk_encryption_key_sha256` after provisioning.\n"]
    pub fn disk_encryption_key_sha256(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_encryption_key_sha256", self.base))
    }

    #[doc= "Get a reference to the value of field `kms_key_self_link` after provisioning.\n"]
    pub fn kms_key_self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_self_link", self.base))
    }

    #[doc= "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc= "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceBootDiskElInitializeParamsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_manager_tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl DataComputeInstanceBootDiskElInitializeParamsEl {
    #[doc= "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.image = Some(v.into());
        self
    }

    #[doc= "Set the field `labels`.\n"]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }

    #[doc= "Set the field `resource_manager_tags`.\n"]
    pub fn set_resource_manager_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.resource_manager_tags = Some(v.into());
        self
    }

    #[doc= "Set the field `size`.\n"]
    pub fn set_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.size = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for DataComputeInstanceBootDiskElInitializeParamsEl {
    type O = BlockAssignable<DataComputeInstanceBootDiskElInitializeParamsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceBootDiskElInitializeParamsEl {}

impl BuildDataComputeInstanceBootDiskElInitializeParamsEl {
    pub fn build(self) -> DataComputeInstanceBootDiskElInitializeParamsEl {
        DataComputeInstanceBootDiskElInitializeParamsEl {
            image: core::default::Default::default(),
            labels: core::default::Default::default(),
            resource_manager_tags: core::default::Default::default(),
            size: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceBootDiskElInitializeParamsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataComputeInstanceBootDiskElInitializeParamsElRef {
    fn new(shared: StackShared, base: String) -> DataComputeInstanceBootDiskElInitializeParamsElRef {
        DataComputeInstanceBootDiskElInitializeParamsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceBootDiskElInitializeParamsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc= "Get a reference to the value of field `resource_manager_tags` after provisioning.\n"]
    pub fn resource_manager_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.resource_manager_tags", self.base))
    }

    #[doc= "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceBootDiskEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_delete: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_encryption_key_raw: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_encryption_key_sha256: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initialize_params: Option<ListField<DataComputeInstanceBootDiskElInitializeParamsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_self_link: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<PrimField<String>>,
}

impl DataComputeInstanceBootDiskEl {
    #[doc= "Set the field `auto_delete`.\n"]
    pub fn set_auto_delete(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.auto_delete = Some(v.into());
        self
    }

    #[doc= "Set the field `device_name`.\n"]
    pub fn set_device_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.device_name = Some(v.into());
        self
    }

    #[doc= "Set the field `disk_encryption_key_raw`.\n"]
    pub fn set_disk_encryption_key_raw(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_encryption_key_raw = Some(v.into());
        self
    }

    #[doc= "Set the field `disk_encryption_key_sha256`.\n"]
    pub fn set_disk_encryption_key_sha256(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_encryption_key_sha256 = Some(v.into());
        self
    }

    #[doc= "Set the field `initialize_params`.\n"]
    pub fn set_initialize_params(
        mut self,
        v: impl Into<ListField<DataComputeInstanceBootDiskElInitializeParamsEl>>,
    ) -> Self {
        self.initialize_params = Some(v.into());
        self
    }

    #[doc= "Set the field `kms_key_self_link`.\n"]
    pub fn set_kms_key_self_link(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_self_link = Some(v.into());
        self
    }

    #[doc= "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }

    #[doc= "Set the field `source`.\n"]
    pub fn set_source(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source = Some(v.into());
        self
    }
}

impl ToListMappable for DataComputeInstanceBootDiskEl {
    type O = BlockAssignable<DataComputeInstanceBootDiskEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceBootDiskEl {}

impl BuildDataComputeInstanceBootDiskEl {
    pub fn build(self) -> DataComputeInstanceBootDiskEl {
        DataComputeInstanceBootDiskEl {
            auto_delete: core::default::Default::default(),
            device_name: core::default::Default::default(),
            disk_encryption_key_raw: core::default::Default::default(),
            disk_encryption_key_sha256: core::default::Default::default(),
            initialize_params: core::default::Default::default(),
            kms_key_self_link: core::default::Default::default(),
            mode: core::default::Default::default(),
            source: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceBootDiskElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataComputeInstanceBootDiskElRef {
    fn new(shared: StackShared, base: String) -> DataComputeInstanceBootDiskElRef {
        DataComputeInstanceBootDiskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceBootDiskElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `auto_delete` after provisioning.\n"]
    pub fn auto_delete(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_delete", self.base))
    }

    #[doc= "Get a reference to the value of field `device_name` after provisioning.\n"]
    pub fn device_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }

    #[doc= "Get a reference to the value of field `disk_encryption_key_raw` after provisioning.\n"]
    pub fn disk_encryption_key_raw(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_encryption_key_raw", self.base))
    }

    #[doc= "Get a reference to the value of field `disk_encryption_key_sha256` after provisioning.\n"]
    pub fn disk_encryption_key_sha256(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_encryption_key_sha256", self.base))
    }

    #[doc= "Get a reference to the value of field `initialize_params` after provisioning.\n"]
    pub fn initialize_params(&self) -> ListRef<DataComputeInstanceBootDiskElInitializeParamsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.initialize_params", self.base))
    }

    #[doc= "Get a reference to the value of field `kms_key_self_link` after provisioning.\n"]
    pub fn kms_key_self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_self_link", self.base))
    }

    #[doc= "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc= "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceConfidentialInstanceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_confidential_compute: Option<PrimField<bool>>,
}

impl DataComputeInstanceConfidentialInstanceConfigEl {
    #[doc= "Set the field `enable_confidential_compute`.\n"]
    pub fn set_enable_confidential_compute(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_confidential_compute = Some(v.into());
        self
    }
}

impl ToListMappable for DataComputeInstanceConfidentialInstanceConfigEl {
    type O = BlockAssignable<DataComputeInstanceConfidentialInstanceConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceConfidentialInstanceConfigEl {}

impl BuildDataComputeInstanceConfidentialInstanceConfigEl {
    pub fn build(self) -> DataComputeInstanceConfidentialInstanceConfigEl {
        DataComputeInstanceConfidentialInstanceConfigEl {
            enable_confidential_compute: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceConfidentialInstanceConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataComputeInstanceConfidentialInstanceConfigElRef {
    fn new(shared: StackShared, base: String) -> DataComputeInstanceConfidentialInstanceConfigElRef {
        DataComputeInstanceConfidentialInstanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceConfidentialInstanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enable_confidential_compute` after provisioning.\n"]
    pub fn enable_confidential_compute(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_confidential_compute", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceGuestAcceleratorEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl DataComputeInstanceGuestAcceleratorEl {
    #[doc= "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.count = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for DataComputeInstanceGuestAcceleratorEl {
    type O = BlockAssignable<DataComputeInstanceGuestAcceleratorEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceGuestAcceleratorEl {}

impl BuildDataComputeInstanceGuestAcceleratorEl {
    pub fn build(self) -> DataComputeInstanceGuestAcceleratorEl {
        DataComputeInstanceGuestAcceleratorEl {
            count: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceGuestAcceleratorElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataComputeInstanceGuestAcceleratorElRef {
    fn new(shared: StackShared, base: String) -> DataComputeInstanceGuestAcceleratorElRef {
        DataComputeInstanceGuestAcceleratorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceGuestAcceleratorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceNetworkInterfaceElAccessConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nat_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_tier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_ptr_domain_name: Option<PrimField<String>>,
}

impl DataComputeInstanceNetworkInterfaceElAccessConfigEl {
    #[doc= "Set the field `nat_ip`.\n"]
    pub fn set_nat_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nat_ip = Some(v.into());
        self
    }

    #[doc= "Set the field `network_tier`.\n"]
    pub fn set_network_tier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_tier = Some(v.into());
        self
    }

    #[doc= "Set the field `public_ptr_domain_name`.\n"]
    pub fn set_public_ptr_domain_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.public_ptr_domain_name = Some(v.into());
        self
    }
}

impl ToListMappable for DataComputeInstanceNetworkInterfaceElAccessConfigEl {
    type O = BlockAssignable<DataComputeInstanceNetworkInterfaceElAccessConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceNetworkInterfaceElAccessConfigEl {}

impl BuildDataComputeInstanceNetworkInterfaceElAccessConfigEl {
    pub fn build(self) -> DataComputeInstanceNetworkInterfaceElAccessConfigEl {
        DataComputeInstanceNetworkInterfaceElAccessConfigEl {
            nat_ip: core::default::Default::default(),
            network_tier: core::default::Default::default(),
            public_ptr_domain_name: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceNetworkInterfaceElAccessConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataComputeInstanceNetworkInterfaceElAccessConfigElRef {
    fn new(shared: StackShared, base: String) -> DataComputeInstanceNetworkInterfaceElAccessConfigElRef {
        DataComputeInstanceNetworkInterfaceElAccessConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceNetworkInterfaceElAccessConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `nat_ip` after provisioning.\n"]
    pub fn nat_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.nat_ip", self.base))
    }

    #[doc= "Get a reference to the value of field `network_tier` after provisioning.\n"]
    pub fn network_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_tier", self.base))
    }

    #[doc= "Get a reference to the value of field `public_ptr_domain_name` after provisioning.\n"]
    pub fn public_ptr_domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_ptr_domain_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceNetworkInterfaceElAliasIpRangeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_cidr_range: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork_range_name: Option<PrimField<String>>,
}

impl DataComputeInstanceNetworkInterfaceElAliasIpRangeEl {
    #[doc= "Set the field `ip_cidr_range`.\n"]
    pub fn set_ip_cidr_range(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_cidr_range = Some(v.into());
        self
    }

    #[doc= "Set the field `subnetwork_range_name`.\n"]
    pub fn set_subnetwork_range_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnetwork_range_name = Some(v.into());
        self
    }
}

impl ToListMappable for DataComputeInstanceNetworkInterfaceElAliasIpRangeEl {
    type O = BlockAssignable<DataComputeInstanceNetworkInterfaceElAliasIpRangeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceNetworkInterfaceElAliasIpRangeEl {}

impl BuildDataComputeInstanceNetworkInterfaceElAliasIpRangeEl {
    pub fn build(self) -> DataComputeInstanceNetworkInterfaceElAliasIpRangeEl {
        DataComputeInstanceNetworkInterfaceElAliasIpRangeEl {
            ip_cidr_range: core::default::Default::default(),
            subnetwork_range_name: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceNetworkInterfaceElAliasIpRangeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataComputeInstanceNetworkInterfaceElAliasIpRangeElRef {
    fn new(shared: StackShared, base: String) -> DataComputeInstanceNetworkInterfaceElAliasIpRangeElRef {
        DataComputeInstanceNetworkInterfaceElAliasIpRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceNetworkInterfaceElAliasIpRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `ip_cidr_range` after provisioning.\n"]
    pub fn ip_cidr_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_cidr_range", self.base))
    }

    #[doc= "Get a reference to the value of field `subnetwork_range_name` after provisioning.\n"]
    pub fn subnetwork_range_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnetwork_range_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceNetworkInterfaceElIpv6AccessConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    external_ipv6: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_ipv6_prefix_length: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_tier: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_ptr_domain_name: Option<PrimField<String>>,
}

impl DataComputeInstanceNetworkInterfaceElIpv6AccessConfigEl {
    #[doc= "Set the field `external_ipv6`.\n"]
    pub fn set_external_ipv6(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.external_ipv6 = Some(v.into());
        self
    }

    #[doc= "Set the field `external_ipv6_prefix_length`.\n"]
    pub fn set_external_ipv6_prefix_length(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.external_ipv6_prefix_length = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `network_tier`.\n"]
    pub fn set_network_tier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_tier = Some(v.into());
        self
    }

    #[doc= "Set the field `public_ptr_domain_name`.\n"]
    pub fn set_public_ptr_domain_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.public_ptr_domain_name = Some(v.into());
        self
    }
}

impl ToListMappable for DataComputeInstanceNetworkInterfaceElIpv6AccessConfigEl {
    type O = BlockAssignable<DataComputeInstanceNetworkInterfaceElIpv6AccessConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceNetworkInterfaceElIpv6AccessConfigEl {}

impl BuildDataComputeInstanceNetworkInterfaceElIpv6AccessConfigEl {
    pub fn build(self) -> DataComputeInstanceNetworkInterfaceElIpv6AccessConfigEl {
        DataComputeInstanceNetworkInterfaceElIpv6AccessConfigEl {
            external_ipv6: core::default::Default::default(),
            external_ipv6_prefix_length: core::default::Default::default(),
            name: core::default::Default::default(),
            network_tier: core::default::Default::default(),
            public_ptr_domain_name: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceNetworkInterfaceElIpv6AccessConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataComputeInstanceNetworkInterfaceElIpv6AccessConfigElRef {
    fn new(shared: StackShared, base: String) -> DataComputeInstanceNetworkInterfaceElIpv6AccessConfigElRef {
        DataComputeInstanceNetworkInterfaceElIpv6AccessConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceNetworkInterfaceElIpv6AccessConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `external_ipv6` after provisioning.\n"]
    pub fn external_ipv6(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.external_ipv6", self.base))
    }

    #[doc= "Get a reference to the value of field `external_ipv6_prefix_length` after provisioning.\n"]
    pub fn external_ipv6_prefix_length(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.external_ipv6_prefix_length", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `network_tier` after provisioning.\n"]
    pub fn network_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_tier", self.base))
    }

    #[doc= "Get a reference to the value of field `public_ptr_domain_name` after provisioning.\n"]
    pub fn public_ptr_domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_ptr_domain_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceNetworkInterfaceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_config: Option<ListField<DataComputeInstanceNetworkInterfaceElAccessConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alias_ip_range: Option<ListField<DataComputeInstanceNetworkInterfaceElAliasIpRangeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    internal_ipv6_prefix_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_access_config: Option<ListField<DataComputeInstanceNetworkInterfaceElIpv6AccessConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_access_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_address: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nic_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    queue_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stack_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork_project: Option<PrimField<String>>,
}

impl DataComputeInstanceNetworkInterfaceEl {
    #[doc= "Set the field `access_config`.\n"]
    pub fn set_access_config(
        mut self,
        v: impl Into<ListField<DataComputeInstanceNetworkInterfaceElAccessConfigEl>>,
    ) -> Self {
        self.access_config = Some(v.into());
        self
    }

    #[doc= "Set the field `alias_ip_range`.\n"]
    pub fn set_alias_ip_range(
        mut self,
        v: impl Into<ListField<DataComputeInstanceNetworkInterfaceElAliasIpRangeEl>>,
    ) -> Self {
        self.alias_ip_range = Some(v.into());
        self
    }

    #[doc= "Set the field `internal_ipv6_prefix_length`.\n"]
    pub fn set_internal_ipv6_prefix_length(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.internal_ipv6_prefix_length = Some(v.into());
        self
    }

    #[doc= "Set the field `ipv6_access_config`.\n"]
    pub fn set_ipv6_access_config(
        mut self,
        v: impl Into<ListField<DataComputeInstanceNetworkInterfaceElIpv6AccessConfigEl>>,
    ) -> Self {
        self.ipv6_access_config = Some(v.into());
        self
    }

    #[doc= "Set the field `ipv6_access_type`.\n"]
    pub fn set_ipv6_access_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ipv6_access_type = Some(v.into());
        self
    }

    #[doc= "Set the field `ipv6_address`.\n"]
    pub fn set_ipv6_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ipv6_address = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc= "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }

    #[doc= "Set the field `network_ip`.\n"]
    pub fn set_network_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_ip = Some(v.into());
        self
    }

    #[doc= "Set the field `nic_type`.\n"]
    pub fn set_nic_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nic_type = Some(v.into());
        self
    }

    #[doc= "Set the field `queue_count`.\n"]
    pub fn set_queue_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.queue_count = Some(v.into());
        self
    }

    #[doc= "Set the field `stack_type`.\n"]
    pub fn set_stack_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.stack_type = Some(v.into());
        self
    }

    #[doc= "Set the field `subnetwork`.\n"]
    pub fn set_subnetwork(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnetwork = Some(v.into());
        self
    }

    #[doc= "Set the field `subnetwork_project`.\n"]
    pub fn set_subnetwork_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnetwork_project = Some(v.into());
        self
    }
}

impl ToListMappable for DataComputeInstanceNetworkInterfaceEl {
    type O = BlockAssignable<DataComputeInstanceNetworkInterfaceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceNetworkInterfaceEl {}

impl BuildDataComputeInstanceNetworkInterfaceEl {
    pub fn build(self) -> DataComputeInstanceNetworkInterfaceEl {
        DataComputeInstanceNetworkInterfaceEl {
            access_config: core::default::Default::default(),
            alias_ip_range: core::default::Default::default(),
            internal_ipv6_prefix_length: core::default::Default::default(),
            ipv6_access_config: core::default::Default::default(),
            ipv6_access_type: core::default::Default::default(),
            ipv6_address: core::default::Default::default(),
            name: core::default::Default::default(),
            network: core::default::Default::default(),
            network_ip: core::default::Default::default(),
            nic_type: core::default::Default::default(),
            queue_count: core::default::Default::default(),
            stack_type: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
            subnetwork_project: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceNetworkInterfaceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataComputeInstanceNetworkInterfaceElRef {
    fn new(shared: StackShared, base: String) -> DataComputeInstanceNetworkInterfaceElRef {
        DataComputeInstanceNetworkInterfaceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceNetworkInterfaceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `access_config` after provisioning.\n"]
    pub fn access_config(&self) -> ListRef<DataComputeInstanceNetworkInterfaceElAccessConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.access_config", self.base))
    }

    #[doc= "Get a reference to the value of field `alias_ip_range` after provisioning.\n"]
    pub fn alias_ip_range(&self) -> ListRef<DataComputeInstanceNetworkInterfaceElAliasIpRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.alias_ip_range", self.base))
    }

    #[doc= "Get a reference to the value of field `internal_ipv6_prefix_length` after provisioning.\n"]
    pub fn internal_ipv6_prefix_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.internal_ipv6_prefix_length", self.base))
    }

    #[doc= "Get a reference to the value of field `ipv6_access_config` after provisioning.\n"]
    pub fn ipv6_access_config(&self) -> ListRef<DataComputeInstanceNetworkInterfaceElIpv6AccessConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ipv6_access_config", self.base))
    }

    #[doc= "Get a reference to the value of field `ipv6_access_type` after provisioning.\n"]
    pub fn ipv6_access_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_access_type", self.base))
    }

    #[doc= "Get a reference to the value of field `ipv6_address` after provisioning.\n"]
    pub fn ipv6_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_address", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc= "Get a reference to the value of field `network_ip` after provisioning.\n"]
    pub fn network_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_ip", self.base))
    }

    #[doc= "Get a reference to the value of field `nic_type` after provisioning.\n"]
    pub fn nic_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.nic_type", self.base))
    }

    #[doc= "Get a reference to the value of field `queue_count` after provisioning.\n"]
    pub fn queue_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.queue_count", self.base))
    }

    #[doc= "Get a reference to the value of field `stack_type` after provisioning.\n"]
    pub fn stack_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stack_type", self.base))
    }

    #[doc= "Get a reference to the value of field `subnetwork` after provisioning.\n"]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }

    #[doc= "Get a reference to the value of field `subnetwork_project` after provisioning.\n"]
    pub fn subnetwork_project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnetwork_project", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceNetworkPerformanceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    total_egress_bandwidth_tier: Option<PrimField<String>>,
}

impl DataComputeInstanceNetworkPerformanceConfigEl {
    #[doc= "Set the field `total_egress_bandwidth_tier`.\n"]
    pub fn set_total_egress_bandwidth_tier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.total_egress_bandwidth_tier = Some(v.into());
        self
    }
}

impl ToListMappable for DataComputeInstanceNetworkPerformanceConfigEl {
    type O = BlockAssignable<DataComputeInstanceNetworkPerformanceConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceNetworkPerformanceConfigEl {}

impl BuildDataComputeInstanceNetworkPerformanceConfigEl {
    pub fn build(self) -> DataComputeInstanceNetworkPerformanceConfigEl {
        DataComputeInstanceNetworkPerformanceConfigEl {
            total_egress_bandwidth_tier: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceNetworkPerformanceConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataComputeInstanceNetworkPerformanceConfigElRef {
    fn new(shared: StackShared, base: String) -> DataComputeInstanceNetworkPerformanceConfigElRef {
        DataComputeInstanceNetworkPerformanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceNetworkPerformanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `total_egress_bandwidth_tier` after provisioning.\n"]
    pub fn total_egress_bandwidth_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.total_egress_bandwidth_tier", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceParamsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_manager_tags: Option<RecField<PrimField<String>>>,
}

impl DataComputeInstanceParamsEl {
    #[doc= "Set the field `resource_manager_tags`.\n"]
    pub fn set_resource_manager_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.resource_manager_tags = Some(v.into());
        self
    }
}

impl ToListMappable for DataComputeInstanceParamsEl {
    type O = BlockAssignable<DataComputeInstanceParamsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceParamsEl {}

impl BuildDataComputeInstanceParamsEl {
    pub fn build(self) -> DataComputeInstanceParamsEl {
        DataComputeInstanceParamsEl { resource_manager_tags: core::default::Default::default() }
    }
}

pub struct DataComputeInstanceParamsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataComputeInstanceParamsElRef {
    fn new(shared: StackShared, base: String) -> DataComputeInstanceParamsElRef {
        DataComputeInstanceParamsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceParamsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `resource_manager_tags` after provisioning.\n"]
    pub fn resource_manager_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.resource_manager_tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceReservationAffinityElSpecificReservationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<ListField<PrimField<String>>>,
}

impl DataComputeInstanceReservationAffinityElSpecificReservationEl {
    #[doc= "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc= "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataComputeInstanceReservationAffinityElSpecificReservationEl {
    type O = BlockAssignable<DataComputeInstanceReservationAffinityElSpecificReservationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceReservationAffinityElSpecificReservationEl {}

impl BuildDataComputeInstanceReservationAffinityElSpecificReservationEl {
    pub fn build(self) -> DataComputeInstanceReservationAffinityElSpecificReservationEl {
        DataComputeInstanceReservationAffinityElSpecificReservationEl {
            key: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceReservationAffinityElSpecificReservationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataComputeInstanceReservationAffinityElSpecificReservationElRef {
    fn new(shared: StackShared, base: String) -> DataComputeInstanceReservationAffinityElSpecificReservationElRef {
        DataComputeInstanceReservationAffinityElSpecificReservationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceReservationAffinityElSpecificReservationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceReservationAffinityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    specific_reservation: Option<ListField<DataComputeInstanceReservationAffinityElSpecificReservationEl>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl DataComputeInstanceReservationAffinityEl {
    #[doc= "Set the field `specific_reservation`.\n"]
    pub fn set_specific_reservation(
        mut self,
        v: impl Into<ListField<DataComputeInstanceReservationAffinityElSpecificReservationEl>>,
    ) -> Self {
        self.specific_reservation = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for DataComputeInstanceReservationAffinityEl {
    type O = BlockAssignable<DataComputeInstanceReservationAffinityEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceReservationAffinityEl {}

impl BuildDataComputeInstanceReservationAffinityEl {
    pub fn build(self) -> DataComputeInstanceReservationAffinityEl {
        DataComputeInstanceReservationAffinityEl {
            specific_reservation: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceReservationAffinityElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataComputeInstanceReservationAffinityElRef {
    fn new(shared: StackShared, base: String) -> DataComputeInstanceReservationAffinityElRef {
        DataComputeInstanceReservationAffinityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceReservationAffinityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `specific_reservation` after provisioning.\n"]
    pub fn specific_reservation(&self) -> ListRef<DataComputeInstanceReservationAffinityElSpecificReservationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.specific_reservation", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceSchedulingElLocalSsdRecoveryTimeoutEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seconds: Option<PrimField<f64>>,
}

impl DataComputeInstanceSchedulingElLocalSsdRecoveryTimeoutEl {
    #[doc= "Set the field `nanos`.\n"]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }

    #[doc= "Set the field `seconds`.\n"]
    pub fn set_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.seconds = Some(v.into());
        self
    }
}

impl ToListMappable for DataComputeInstanceSchedulingElLocalSsdRecoveryTimeoutEl {
    type O = BlockAssignable<DataComputeInstanceSchedulingElLocalSsdRecoveryTimeoutEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceSchedulingElLocalSsdRecoveryTimeoutEl {}

impl BuildDataComputeInstanceSchedulingElLocalSsdRecoveryTimeoutEl {
    pub fn build(self) -> DataComputeInstanceSchedulingElLocalSsdRecoveryTimeoutEl {
        DataComputeInstanceSchedulingElLocalSsdRecoveryTimeoutEl {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceSchedulingElLocalSsdRecoveryTimeoutElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataComputeInstanceSchedulingElLocalSsdRecoveryTimeoutElRef {
    fn new(shared: StackShared, base: String) -> DataComputeInstanceSchedulingElLocalSsdRecoveryTimeoutElRef {
        DataComputeInstanceSchedulingElLocalSsdRecoveryTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceSchedulingElLocalSsdRecoveryTimeoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `nanos` after provisioning.\n"]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc= "Get a reference to the value of field `seconds` after provisioning.\n"]
    pub fn seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceSchedulingElNodeAffinitiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operator: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<SetField<PrimField<String>>>,
}

impl DataComputeInstanceSchedulingElNodeAffinitiesEl {
    #[doc= "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key = Some(v.into());
        self
    }

    #[doc= "Set the field `operator`.\n"]
    pub fn set_operator(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.operator = Some(v.into());
        self
    }

    #[doc= "Set the field `values`.\n"]
    pub fn set_values(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.values = Some(v.into());
        self
    }
}

impl ToListMappable for DataComputeInstanceSchedulingElNodeAffinitiesEl {
    type O = BlockAssignable<DataComputeInstanceSchedulingElNodeAffinitiesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceSchedulingElNodeAffinitiesEl {}

impl BuildDataComputeInstanceSchedulingElNodeAffinitiesEl {
    pub fn build(self) -> DataComputeInstanceSchedulingElNodeAffinitiesEl {
        DataComputeInstanceSchedulingElNodeAffinitiesEl {
            key: core::default::Default::default(),
            operator: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceSchedulingElNodeAffinitiesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataComputeInstanceSchedulingElNodeAffinitiesElRef {
    fn new(shared: StackShared, base: String) -> DataComputeInstanceSchedulingElNodeAffinitiesElRef {
        DataComputeInstanceSchedulingElNodeAffinitiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceSchedulingElNodeAffinitiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc= "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceSchedulingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_restart: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_termination_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_ssd_recovery_timeout: Option<ListField<DataComputeInstanceSchedulingElLocalSsdRecoveryTimeoutEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_node_cpus: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_affinities: Option<SetField<DataComputeInstanceSchedulingElNodeAffinitiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_host_maintenance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preemptible: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provisioning_model: Option<PrimField<String>>,
}

impl DataComputeInstanceSchedulingEl {
    #[doc= "Set the field `automatic_restart`.\n"]
    pub fn set_automatic_restart(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.automatic_restart = Some(v.into());
        self
    }

    #[doc= "Set the field `instance_termination_action`.\n"]
    pub fn set_instance_termination_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_termination_action = Some(v.into());
        self
    }

    #[doc= "Set the field `local_ssd_recovery_timeout`.\n"]
    pub fn set_local_ssd_recovery_timeout(
        mut self,
        v: impl Into<ListField<DataComputeInstanceSchedulingElLocalSsdRecoveryTimeoutEl>>,
    ) -> Self {
        self.local_ssd_recovery_timeout = Some(v.into());
        self
    }

    #[doc= "Set the field `min_node_cpus`.\n"]
    pub fn set_min_node_cpus(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_node_cpus = Some(v.into());
        self
    }

    #[doc= "Set the field `node_affinities`.\n"]
    pub fn set_node_affinities(
        mut self,
        v: impl Into<SetField<DataComputeInstanceSchedulingElNodeAffinitiesEl>>,
    ) -> Self {
        self.node_affinities = Some(v.into());
        self
    }

    #[doc= "Set the field `on_host_maintenance`.\n"]
    pub fn set_on_host_maintenance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.on_host_maintenance = Some(v.into());
        self
    }

    #[doc= "Set the field `preemptible`.\n"]
    pub fn set_preemptible(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.preemptible = Some(v.into());
        self
    }

    #[doc= "Set the field `provisioning_model`.\n"]
    pub fn set_provisioning_model(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.provisioning_model = Some(v.into());
        self
    }
}

impl ToListMappable for DataComputeInstanceSchedulingEl {
    type O = BlockAssignable<DataComputeInstanceSchedulingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceSchedulingEl {}

impl BuildDataComputeInstanceSchedulingEl {
    pub fn build(self) -> DataComputeInstanceSchedulingEl {
        DataComputeInstanceSchedulingEl {
            automatic_restart: core::default::Default::default(),
            instance_termination_action: core::default::Default::default(),
            local_ssd_recovery_timeout: core::default::Default::default(),
            min_node_cpus: core::default::Default::default(),
            node_affinities: core::default::Default::default(),
            on_host_maintenance: core::default::Default::default(),
            preemptible: core::default::Default::default(),
            provisioning_model: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceSchedulingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataComputeInstanceSchedulingElRef {
    fn new(shared: StackShared, base: String) -> DataComputeInstanceSchedulingElRef {
        DataComputeInstanceSchedulingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceSchedulingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `automatic_restart` after provisioning.\n"]
    pub fn automatic_restart(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.automatic_restart", self.base))
    }

    #[doc= "Get a reference to the value of field `instance_termination_action` after provisioning.\n"]
    pub fn instance_termination_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_termination_action", self.base))
    }

    #[doc= "Get a reference to the value of field `local_ssd_recovery_timeout` after provisioning.\n"]
    pub fn local_ssd_recovery_timeout(&self) -> ListRef<DataComputeInstanceSchedulingElLocalSsdRecoveryTimeoutElRef> {
        ListRef::new(self.shared().clone(), format!("{}.local_ssd_recovery_timeout", self.base))
    }

    #[doc= "Get a reference to the value of field `min_node_cpus` after provisioning.\n"]
    pub fn min_node_cpus(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_node_cpus", self.base))
    }

    #[doc= "Get a reference to the value of field `node_affinities` after provisioning.\n"]
    pub fn node_affinities(&self) -> SetRef<DataComputeInstanceSchedulingElNodeAffinitiesElRef> {
        SetRef::new(self.shared().clone(), format!("{}.node_affinities", self.base))
    }

    #[doc= "Get a reference to the value of field `on_host_maintenance` after provisioning.\n"]
    pub fn on_host_maintenance(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.on_host_maintenance", self.base))
    }

    #[doc= "Get a reference to the value of field `preemptible` after provisioning.\n"]
    pub fn preemptible(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.preemptible", self.base))
    }

    #[doc= "Get a reference to the value of field `provisioning_model` after provisioning.\n"]
    pub fn provisioning_model(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.provisioning_model", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceScratchDiskEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    device_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interface: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<PrimField<f64>>,
}

impl DataComputeInstanceScratchDiskEl {
    #[doc= "Set the field `device_name`.\n"]
    pub fn set_device_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.device_name = Some(v.into());
        self
    }

    #[doc= "Set the field `interface`.\n"]
    pub fn set_interface(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.interface = Some(v.into());
        self
    }

    #[doc= "Set the field `size`.\n"]
    pub fn set_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.size = Some(v.into());
        self
    }
}

impl ToListMappable for DataComputeInstanceScratchDiskEl {
    type O = BlockAssignable<DataComputeInstanceScratchDiskEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceScratchDiskEl {}

impl BuildDataComputeInstanceScratchDiskEl {
    pub fn build(self) -> DataComputeInstanceScratchDiskEl {
        DataComputeInstanceScratchDiskEl {
            device_name: core::default::Default::default(),
            interface: core::default::Default::default(),
            size: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceScratchDiskElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataComputeInstanceScratchDiskElRef {
    fn new(shared: StackShared, base: String) -> DataComputeInstanceScratchDiskElRef {
        DataComputeInstanceScratchDiskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceScratchDiskElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `device_name` after provisioning.\n"]
    pub fn device_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }

    #[doc= "Get a reference to the value of field `interface` after provisioning.\n"]
    pub fn interface(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interface", self.base))
    }

    #[doc= "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceServiceAccountEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scopes: Option<SetField<PrimField<String>>>,
}

impl DataComputeInstanceServiceAccountEl {
    #[doc= "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.email = Some(v.into());
        self
    }

    #[doc= "Set the field `scopes`.\n"]
    pub fn set_scopes(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.scopes = Some(v.into());
        self
    }
}

impl ToListMappable for DataComputeInstanceServiceAccountEl {
    type O = BlockAssignable<DataComputeInstanceServiceAccountEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceServiceAccountEl {}

impl BuildDataComputeInstanceServiceAccountEl {
    pub fn build(self) -> DataComputeInstanceServiceAccountEl {
        DataComputeInstanceServiceAccountEl {
            email: core::default::Default::default(),
            scopes: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceServiceAccountElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataComputeInstanceServiceAccountElRef {
    fn new(shared: StackShared, base: String) -> DataComputeInstanceServiceAccountElRef {
        DataComputeInstanceServiceAccountElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceServiceAccountElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc= "Get a reference to the value of field `scopes` after provisioning.\n"]
    pub fn scopes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceShieldedInstanceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_integrity_monitoring: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_secure_boot: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_vtpm: Option<PrimField<bool>>,
}

impl DataComputeInstanceShieldedInstanceConfigEl {
    #[doc= "Set the field `enable_integrity_monitoring`.\n"]
    pub fn set_enable_integrity_monitoring(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_integrity_monitoring = Some(v.into());
        self
    }

    #[doc= "Set the field `enable_secure_boot`.\n"]
    pub fn set_enable_secure_boot(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_secure_boot = Some(v.into());
        self
    }

    #[doc= "Set the field `enable_vtpm`.\n"]
    pub fn set_enable_vtpm(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_vtpm = Some(v.into());
        self
    }
}

impl ToListMappable for DataComputeInstanceShieldedInstanceConfigEl {
    type O = BlockAssignable<DataComputeInstanceShieldedInstanceConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceShieldedInstanceConfigEl {}

impl BuildDataComputeInstanceShieldedInstanceConfigEl {
    pub fn build(self) -> DataComputeInstanceShieldedInstanceConfigEl {
        DataComputeInstanceShieldedInstanceConfigEl {
            enable_integrity_monitoring: core::default::Default::default(),
            enable_secure_boot: core::default::Default::default(),
            enable_vtpm: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceShieldedInstanceConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataComputeInstanceShieldedInstanceConfigElRef {
    fn new(shared: StackShared, base: String) -> DataComputeInstanceShieldedInstanceConfigElRef {
        DataComputeInstanceShieldedInstanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceShieldedInstanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enable_integrity_monitoring` after provisioning.\n"]
    pub fn enable_integrity_monitoring(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_integrity_monitoring", self.base))
    }

    #[doc= "Get a reference to the value of field `enable_secure_boot` after provisioning.\n"]
    pub fn enable_secure_boot(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_secure_boot", self.base))
    }

    #[doc= "Get a reference to the value of field `enable_vtpm` after provisioning.\n"]
    pub fn enable_vtpm(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_vtpm", self.base))
    }
}
