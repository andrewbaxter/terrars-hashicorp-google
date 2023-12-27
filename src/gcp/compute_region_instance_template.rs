use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeRegionInstanceTemplateData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    can_ip_forward: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    machine_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata_startup_script: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_cpu_platform: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_policies: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_machine_features: Option<Vec<ComputeRegionInstanceTemplateAdvancedMachineFeaturesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confidential_instance_config: Option<Vec<ComputeRegionInstanceTemplateConfidentialInstanceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk: Option<Vec<ComputeRegionInstanceTemplateDiskEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guest_accelerator: Option<Vec<ComputeRegionInstanceTemplateGuestAcceleratorEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_interface: Option<Vec<ComputeRegionInstanceTemplateNetworkInterfaceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_performance_config: Option<Vec<ComputeRegionInstanceTemplateNetworkPerformanceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reservation_affinity: Option<Vec<ComputeRegionInstanceTemplateReservationAffinityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scheduling: Option<Vec<ComputeRegionInstanceTemplateSchedulingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_account: Option<Vec<ComputeRegionInstanceTemplateServiceAccountEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    shielded_instance_config: Option<Vec<ComputeRegionInstanceTemplateShieldedInstanceConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeRegionInstanceTemplateTimeoutsEl>,
    dynamic: ComputeRegionInstanceTemplateDynamic,
}

struct ComputeRegionInstanceTemplate_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeRegionInstanceTemplateData>,
}

#[derive(Clone)]
pub struct ComputeRegionInstanceTemplate(Rc<ComputeRegionInstanceTemplate_>);

impl ComputeRegionInstanceTemplate {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `can_ip_forward`.\nWhether to allow sending and receiving of packets with non-matching source or destination IPs. This defaults to false."]
    pub fn set_can_ip_forward(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().can_ip_forward = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nA brief description of this resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `instance_description`.\nA description of the instance."]
    pub fn set_instance_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().instance_description = Some(v.into());
        self
    }

    #[doc= "Set the field `labels`.\nA set of key/value label pairs to assign to instances created from this template,\n\t\t\t\t\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }

    #[doc= "Set the field `metadata`.\nMetadata key/value pairs to make available from within instances created from this template."]
    pub fn set_metadata(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().metadata = Some(v.into());
        self
    }

    #[doc= "Set the field `metadata_startup_script`.\nAn alternative to using the startup-script metadata key, mostly to match the compute_instance resource. This replaces the startup-script metadata key on the created instance and thus the two mechanisms are not allowed to be used simultaneously."]
    pub fn set_metadata_startup_script(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().metadata_startup_script = Some(v.into());
        self
    }

    #[doc= "Set the field `min_cpu_platform`.\nSpecifies a minimum CPU platform. Applicable values are the friendly names of CPU platforms, such as Intel Haswell or Intel Skylake."]
    pub fn set_min_cpu_platform(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().min_cpu_platform = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\nThe name of the instance template. If you leave this blank, Terraform will auto-generate a unique name."]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc= "Set the field `name_prefix`.\nCreates a unique name beginning with the specified prefix. Conflicts with name."]
    pub fn set_name_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name_prefix = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `region`.\nThe region in which the instance template is located. If it is not provided, the provider region is used."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc= "Set the field `resource_policies`.\nA list of self_links of resource policies to attach to the instance. Currently a max of 1 resource policy is supported."]
    pub fn set_resource_policies(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().resource_policies = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\nTags to attach to the instance."]
    pub fn set_tags(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Set the field `advanced_machine_features`.\n"]
    pub fn set_advanced_machine_features(
        self,
        v: impl Into<BlockAssignable<ComputeRegionInstanceTemplateAdvancedMachineFeaturesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().advanced_machine_features = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.advanced_machine_features = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `confidential_instance_config`.\n"]
    pub fn set_confidential_instance_config(
        self,
        v: impl Into<BlockAssignable<ComputeRegionInstanceTemplateConfidentialInstanceConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().confidential_instance_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.confidential_instance_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `disk`.\n"]
    pub fn set_disk(self, v: impl Into<BlockAssignable<ComputeRegionInstanceTemplateDiskEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().disk = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.disk = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `guest_accelerator`.\n"]
    pub fn set_guest_accelerator(
        self,
        v: impl Into<BlockAssignable<ComputeRegionInstanceTemplateGuestAcceleratorEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().guest_accelerator = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.guest_accelerator = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `network_interface`.\n"]
    pub fn set_network_interface(
        self,
        v: impl Into<BlockAssignable<ComputeRegionInstanceTemplateNetworkInterfaceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_interface = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_interface = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `network_performance_config`.\n"]
    pub fn set_network_performance_config(
        self,
        v: impl Into<BlockAssignable<ComputeRegionInstanceTemplateNetworkPerformanceConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_performance_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_performance_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `reservation_affinity`.\n"]
    pub fn set_reservation_affinity(
        self,
        v: impl Into<BlockAssignable<ComputeRegionInstanceTemplateReservationAffinityEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().reservation_affinity = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.reservation_affinity = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `scheduling`.\n"]
    pub fn set_scheduling(self, v: impl Into<BlockAssignable<ComputeRegionInstanceTemplateSchedulingEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scheduling = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scheduling = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `service_account`.\n"]
    pub fn set_service_account(
        self,
        v: impl Into<BlockAssignable<ComputeRegionInstanceTemplateServiceAccountEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().service_account = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.service_account = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `shielded_instance_config`.\n"]
    pub fn set_shielded_instance_config(
        self,
        v: impl Into<BlockAssignable<ComputeRegionInstanceTemplateShieldedInstanceConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().shielded_instance_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.shielded_instance_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeRegionInstanceTemplateTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `can_ip_forward` after provisioning.\nWhether to allow sending and receiving of packets with non-matching source or destination IPs. This defaults to false."]
    pub fn can_ip_forward(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.can_ip_forward", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA brief description of this resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `instance_description` after provisioning.\nA description of the instance."]
    pub fn instance_description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to instances created from this template,\n\t\t\t\t\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `machine_type` after provisioning.\nThe machine type to create. To create a machine with a custom type (such as extended memory), format the value like custom-VCPUS-MEM_IN_MB like custom-6-20480 for 6 vCPU and 20GB of RAM."]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `metadata` after provisioning.\nMetadata key/value pairs to make available from within instances created from this template."]
    pub fn metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.metadata", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `metadata_fingerprint` after provisioning.\nThe unique fingerprint of the metadata."]
    pub fn metadata_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metadata_fingerprint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `metadata_startup_script` after provisioning.\nAn alternative to using the startup-script metadata key, mostly to match the compute_instance resource. This replaces the startup-script metadata key on the created instance and thus the two mechanisms are not allowed to be used simultaneously."]
    pub fn metadata_startup_script(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metadata_startup_script", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `min_cpu_platform` after provisioning.\nSpecifies a minimum CPU platform. Applicable values are the friendly names of CPU platforms, such as Intel Haswell or Intel Skylake."]
    pub fn min_cpu_platform(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_cpu_platform", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the instance template. If you leave this blank, Terraform will auto-generate a unique name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name_prefix` after provisioning.\nCreates a unique name beginning with the specified prefix. Conflicts with name."]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nThe region in which the instance template is located. If it is not provided, the provider region is used."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `resource_policies` after provisioning.\nA list of self_links of resource policies to attach to the instance. Currently a max of 1 resource policy is supported."]
    pub fn resource_policies(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.resource_policies", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\nTags to attach to the instance."]
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

    #[doc= "Get a reference to the value of field `advanced_machine_features` after provisioning.\n"]
    pub fn advanced_machine_features(&self) -> ListRef<ComputeRegionInstanceTemplateAdvancedMachineFeaturesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.advanced_machine_features", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `confidential_instance_config` after provisioning.\n"]
    pub fn confidential_instance_config(&self) -> ListRef<ComputeRegionInstanceTemplateConfidentialInstanceConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.confidential_instance_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disk` after provisioning.\n"]
    pub fn disk(&self) -> ListRef<ComputeRegionInstanceTemplateDiskElRef> {
        ListRef::new(self.shared().clone(), format!("{}.disk", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `guest_accelerator` after provisioning.\n"]
    pub fn guest_accelerator(&self) -> ListRef<ComputeRegionInstanceTemplateGuestAcceleratorElRef> {
        ListRef::new(self.shared().clone(), format!("{}.guest_accelerator", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network_interface` after provisioning.\n"]
    pub fn network_interface(&self) -> ListRef<ComputeRegionInstanceTemplateNetworkInterfaceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_interface", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network_performance_config` after provisioning.\n"]
    pub fn network_performance_config(&self) -> ListRef<ComputeRegionInstanceTemplateNetworkPerformanceConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_performance_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `reservation_affinity` after provisioning.\n"]
    pub fn reservation_affinity(&self) -> ListRef<ComputeRegionInstanceTemplateReservationAffinityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.reservation_affinity", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `scheduling` after provisioning.\n"]
    pub fn scheduling(&self) -> ListRef<ComputeRegionInstanceTemplateSchedulingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.scheduling", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service_account` after provisioning.\n"]
    pub fn service_account(&self) -> ListRef<ComputeRegionInstanceTemplateServiceAccountElRef> {
        ListRef::new(self.shared().clone(), format!("{}.service_account", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `shielded_instance_config` after provisioning.\n"]
    pub fn shielded_instance_config(&self) -> ListRef<ComputeRegionInstanceTemplateShieldedInstanceConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.shielded_instance_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRegionInstanceTemplateTimeoutsElRef {
        ComputeRegionInstanceTemplateTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for ComputeRegionInstanceTemplate {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ComputeRegionInstanceTemplate { }

impl ToListMappable for ComputeRegionInstanceTemplate {
    type O = ListRef<ComputeRegionInstanceTemplateRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ComputeRegionInstanceTemplate_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_region_instance_template".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeRegionInstanceTemplate {
    pub tf_id: String,
    #[doc= "The machine type to create. To create a machine with a custom type (such as extended memory), format the value like custom-VCPUS-MEM_IN_MB like custom-6-20480 for 6 vCPU and 20GB of RAM."]
    pub machine_type: PrimField<String>,
}

impl BuildComputeRegionInstanceTemplate {
    pub fn build(self, stack: &mut Stack) -> ComputeRegionInstanceTemplate {
        let out = ComputeRegionInstanceTemplate(Rc::new(ComputeRegionInstanceTemplate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeRegionInstanceTemplateData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                can_ip_forward: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_description: core::default::Default::default(),
                labels: core::default::Default::default(),
                machine_type: self.machine_type,
                metadata: core::default::Default::default(),
                metadata_startup_script: core::default::Default::default(),
                min_cpu_platform: core::default::Default::default(),
                name: core::default::Default::default(),
                name_prefix: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                resource_policies: core::default::Default::default(),
                tags: core::default::Default::default(),
                advanced_machine_features: core::default::Default::default(),
                confidential_instance_config: core::default::Default::default(),
                disk: core::default::Default::default(),
                guest_accelerator: core::default::Default::default(),
                network_interface: core::default::Default::default(),
                network_performance_config: core::default::Default::default(),
                reservation_affinity: core::default::Default::default(),
                scheduling: core::default::Default::default(),
                service_account: core::default::Default::default(),
                shielded_instance_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeRegionInstanceTemplateRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeRegionInstanceTemplateRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeRegionInstanceTemplateRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `can_ip_forward` after provisioning.\nWhether to allow sending and receiving of packets with non-matching source or destination IPs. This defaults to false."]
    pub fn can_ip_forward(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.can_ip_forward", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA brief description of this resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `instance_description` after provisioning.\nA description of the instance."]
    pub fn instance_description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to instances created from this template,\n\t\t\t\t\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `machine_type` after provisioning.\nThe machine type to create. To create a machine with a custom type (such as extended memory), format the value like custom-VCPUS-MEM_IN_MB like custom-6-20480 for 6 vCPU and 20GB of RAM."]
    pub fn machine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `metadata` after provisioning.\nMetadata key/value pairs to make available from within instances created from this template."]
    pub fn metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.metadata", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `metadata_fingerprint` after provisioning.\nThe unique fingerprint of the metadata."]
    pub fn metadata_fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metadata_fingerprint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `metadata_startup_script` after provisioning.\nAn alternative to using the startup-script metadata key, mostly to match the compute_instance resource. This replaces the startup-script metadata key on the created instance and thus the two mechanisms are not allowed to be used simultaneously."]
    pub fn metadata_startup_script(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metadata_startup_script", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `min_cpu_platform` after provisioning.\nSpecifies a minimum CPU platform. Applicable values are the friendly names of CPU platforms, such as Intel Haswell or Intel Skylake."]
    pub fn min_cpu_platform(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_cpu_platform", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the instance template. If you leave this blank, Terraform will auto-generate a unique name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name_prefix` after provisioning.\nCreates a unique name beginning with the specified prefix. Conflicts with name."]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nThe region in which the instance template is located. If it is not provided, the provider region is used."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `resource_policies` after provisioning.\nA list of self_links of resource policies to attach to the instance. Currently a max of 1 resource policy is supported."]
    pub fn resource_policies(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.resource_policies", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\nTags to attach to the instance."]
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

    #[doc= "Get a reference to the value of field `advanced_machine_features` after provisioning.\n"]
    pub fn advanced_machine_features(&self) -> ListRef<ComputeRegionInstanceTemplateAdvancedMachineFeaturesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.advanced_machine_features", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `confidential_instance_config` after provisioning.\n"]
    pub fn confidential_instance_config(&self) -> ListRef<ComputeRegionInstanceTemplateConfidentialInstanceConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.confidential_instance_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disk` after provisioning.\n"]
    pub fn disk(&self) -> ListRef<ComputeRegionInstanceTemplateDiskElRef> {
        ListRef::new(self.shared().clone(), format!("{}.disk", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `guest_accelerator` after provisioning.\n"]
    pub fn guest_accelerator(&self) -> ListRef<ComputeRegionInstanceTemplateGuestAcceleratorElRef> {
        ListRef::new(self.shared().clone(), format!("{}.guest_accelerator", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network_interface` after provisioning.\n"]
    pub fn network_interface(&self) -> ListRef<ComputeRegionInstanceTemplateNetworkInterfaceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_interface", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network_performance_config` after provisioning.\n"]
    pub fn network_performance_config(&self) -> ListRef<ComputeRegionInstanceTemplateNetworkPerformanceConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_performance_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `reservation_affinity` after provisioning.\n"]
    pub fn reservation_affinity(&self) -> ListRef<ComputeRegionInstanceTemplateReservationAffinityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.reservation_affinity", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `scheduling` after provisioning.\n"]
    pub fn scheduling(&self) -> ListRef<ComputeRegionInstanceTemplateSchedulingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.scheduling", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service_account` after provisioning.\n"]
    pub fn service_account(&self) -> ListRef<ComputeRegionInstanceTemplateServiceAccountElRef> {
        ListRef::new(self.shared().clone(), format!("{}.service_account", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `shielded_instance_config` after provisioning.\n"]
    pub fn shielded_instance_config(&self) -> ListRef<ComputeRegionInstanceTemplateShieldedInstanceConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.shielded_instance_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeRegionInstanceTemplateTimeoutsElRef {
        ComputeRegionInstanceTemplateTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceTemplateAdvancedMachineFeaturesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_nested_virtualization: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threads_per_core: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visible_core_count: Option<PrimField<f64>>,
}

impl ComputeRegionInstanceTemplateAdvancedMachineFeaturesEl {
    #[doc= "Set the field `enable_nested_virtualization`.\nWhether to enable nested virtualization or not."]
    pub fn set_enable_nested_virtualization(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_nested_virtualization = Some(v.into());
        self
    }

    #[doc= "Set the field `threads_per_core`.\nThe number of threads per physical core. To disable simultaneous multithreading (SMT) set this to 1. If unset, the maximum number of threads supported per core by the underlying processor is assumed."]
    pub fn set_threads_per_core(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.threads_per_core = Some(v.into());
        self
    }

    #[doc= "Set the field `visible_core_count`.\nThe number of physical cores to expose to an instance. Multiply by the number of threads per core to compute the total number of virtual CPUs to expose to the instance. If unset, the number of cores is inferred from the instance\\'s nominal CPU count and the underlying platform\\'s SMT width."]
    pub fn set_visible_core_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.visible_core_count = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeRegionInstanceTemplateAdvancedMachineFeaturesEl {
    type O = BlockAssignable<ComputeRegionInstanceTemplateAdvancedMachineFeaturesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceTemplateAdvancedMachineFeaturesEl {}

impl BuildComputeRegionInstanceTemplateAdvancedMachineFeaturesEl {
    pub fn build(self) -> ComputeRegionInstanceTemplateAdvancedMachineFeaturesEl {
        ComputeRegionInstanceTemplateAdvancedMachineFeaturesEl {
            enable_nested_virtualization: core::default::Default::default(),
            threads_per_core: core::default::Default::default(),
            visible_core_count: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionInstanceTemplateAdvancedMachineFeaturesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeRegionInstanceTemplateAdvancedMachineFeaturesElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionInstanceTemplateAdvancedMachineFeaturesElRef {
        ComputeRegionInstanceTemplateAdvancedMachineFeaturesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceTemplateAdvancedMachineFeaturesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enable_nested_virtualization` after provisioning.\nWhether to enable nested virtualization or not."]
    pub fn enable_nested_virtualization(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_nested_virtualization", self.base))
    }

    #[doc= "Get a reference to the value of field `threads_per_core` after provisioning.\nThe number of threads per physical core. To disable simultaneous multithreading (SMT) set this to 1. If unset, the maximum number of threads supported per core by the underlying processor is assumed."]
    pub fn threads_per_core(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.threads_per_core", self.base))
    }

    #[doc= "Get a reference to the value of field `visible_core_count` after provisioning.\nThe number of physical cores to expose to an instance. Multiply by the number of threads per core to compute the total number of virtual CPUs to expose to the instance. If unset, the number of cores is inferred from the instance\\'s nominal CPU count and the underlying platform\\'s SMT width."]
    pub fn visible_core_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.visible_core_count", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceTemplateConfidentialInstanceConfigEl {
    enable_confidential_compute: PrimField<bool>,
}

impl ComputeRegionInstanceTemplateConfidentialInstanceConfigEl { }

impl ToListMappable for ComputeRegionInstanceTemplateConfidentialInstanceConfigEl {
    type O = BlockAssignable<ComputeRegionInstanceTemplateConfidentialInstanceConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceTemplateConfidentialInstanceConfigEl {
    #[doc= "Defines whether the instance should have confidential compute enabled."]
    pub enable_confidential_compute: PrimField<bool>,
}

impl BuildComputeRegionInstanceTemplateConfidentialInstanceConfigEl {
    pub fn build(self) -> ComputeRegionInstanceTemplateConfidentialInstanceConfigEl {
        ComputeRegionInstanceTemplateConfidentialInstanceConfigEl {
            enable_confidential_compute: self.enable_confidential_compute,
        }
    }
}

pub struct ComputeRegionInstanceTemplateConfidentialInstanceConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeRegionInstanceTemplateConfidentialInstanceConfigElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionInstanceTemplateConfidentialInstanceConfigElRef {
        ComputeRegionInstanceTemplateConfidentialInstanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceTemplateConfidentialInstanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enable_confidential_compute` after provisioning.\nDefines whether the instance should have confidential compute enabled."]
    pub fn enable_confidential_compute(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_confidential_compute", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceTemplateDiskElDiskEncryptionKeyEl {
    kms_key_self_link: PrimField<String>,
}

impl ComputeRegionInstanceTemplateDiskElDiskEncryptionKeyEl { }

impl ToListMappable for ComputeRegionInstanceTemplateDiskElDiskEncryptionKeyEl {
    type O = BlockAssignable<ComputeRegionInstanceTemplateDiskElDiskEncryptionKeyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceTemplateDiskElDiskEncryptionKeyEl {
    #[doc= "The self link of the encryption key that is stored in Google Cloud KMS."]
    pub kms_key_self_link: PrimField<String>,
}

impl BuildComputeRegionInstanceTemplateDiskElDiskEncryptionKeyEl {
    pub fn build(self) -> ComputeRegionInstanceTemplateDiskElDiskEncryptionKeyEl {
        ComputeRegionInstanceTemplateDiskElDiskEncryptionKeyEl { kms_key_self_link: self.kms_key_self_link }
    }
}

pub struct ComputeRegionInstanceTemplateDiskElDiskEncryptionKeyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeRegionInstanceTemplateDiskElDiskEncryptionKeyElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionInstanceTemplateDiskElDiskEncryptionKeyElRef {
        ComputeRegionInstanceTemplateDiskElDiskEncryptionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceTemplateDiskElDiskEncryptionKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `kms_key_self_link` after provisioning.\nThe self link of the encryption key that is stored in Google Cloud KMS."]
    pub fn kms_key_self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_self_link", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceTemplateDiskElSourceImageEncryptionKeyEl {
    kms_key_self_link: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_service_account: Option<PrimField<String>>,
}

impl ComputeRegionInstanceTemplateDiskElSourceImageEncryptionKeyEl {
    #[doc= "Set the field `kms_key_service_account`.\nThe service account being used for the encryption\nrequest for the given KMS key. If absent, the Compute\nEngine default service account is used."]
    pub fn set_kms_key_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_service_account = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeRegionInstanceTemplateDiskElSourceImageEncryptionKeyEl {
    type O = BlockAssignable<ComputeRegionInstanceTemplateDiskElSourceImageEncryptionKeyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceTemplateDiskElSourceImageEncryptionKeyEl {
    #[doc= "The self link of the encryption key that is stored in\nGoogle Cloud KMS."]
    pub kms_key_self_link: PrimField<String>,
}

impl BuildComputeRegionInstanceTemplateDiskElSourceImageEncryptionKeyEl {
    pub fn build(self) -> ComputeRegionInstanceTemplateDiskElSourceImageEncryptionKeyEl {
        ComputeRegionInstanceTemplateDiskElSourceImageEncryptionKeyEl {
            kms_key_self_link: self.kms_key_self_link,
            kms_key_service_account: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionInstanceTemplateDiskElSourceImageEncryptionKeyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeRegionInstanceTemplateDiskElSourceImageEncryptionKeyElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionInstanceTemplateDiskElSourceImageEncryptionKeyElRef {
        ComputeRegionInstanceTemplateDiskElSourceImageEncryptionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceTemplateDiskElSourceImageEncryptionKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `kms_key_self_link` after provisioning.\nThe self link of the encryption key that is stored in\nGoogle Cloud KMS."]
    pub fn kms_key_self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_self_link", self.base))
    }

    #[doc= "Get a reference to the value of field `kms_key_service_account` after provisioning.\nThe service account being used for the encryption\nrequest for the given KMS key. If absent, the Compute\nEngine default service account is used."]
    pub fn kms_key_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_service_account", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl {
    kms_key_self_link: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_service_account: Option<PrimField<String>>,
}

impl ComputeRegionInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl {
    #[doc= "Set the field `kms_key_service_account`.\nThe service account being used for the encryption\nrequest for the given KMS key. If absent, the Compute\nEngine default service account is used."]
    pub fn set_kms_key_service_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_service_account = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeRegionInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl {
    type O = BlockAssignable<ComputeRegionInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl {
    #[doc= "The self link of the encryption key that is stored in\nGoogle Cloud KMS."]
    pub kms_key_self_link: PrimField<String>,
}

impl BuildComputeRegionInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl {
    pub fn build(self) -> ComputeRegionInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl {
        ComputeRegionInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl {
            kms_key_self_link: self.kms_key_self_link,
            kms_key_service_account: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionInstanceTemplateDiskElSourceSnapshotEncryptionKeyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeRegionInstanceTemplateDiskElSourceSnapshotEncryptionKeyElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionInstanceTemplateDiskElSourceSnapshotEncryptionKeyElRef {
        ComputeRegionInstanceTemplateDiskElSourceSnapshotEncryptionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceTemplateDiskElSourceSnapshotEncryptionKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `kms_key_self_link` after provisioning.\nThe self link of the encryption key that is stored in\nGoogle Cloud KMS."]
    pub fn kms_key_self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_self_link", self.base))
    }

    #[doc= "Get a reference to the value of field `kms_key_service_account` after provisioning.\nThe service account being used for the encryption\nrequest for the given KMS key. If absent, the Compute\nEngine default service account is used."]
    pub fn kms_key_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_service_account", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionInstanceTemplateDiskElDynamic {
    disk_encryption_key: Option<DynamicBlock<ComputeRegionInstanceTemplateDiskElDiskEncryptionKeyEl>>,
    source_image_encryption_key: Option<DynamicBlock<ComputeRegionInstanceTemplateDiskElSourceImageEncryptionKeyEl>>,
    source_snapshot_encryption_key: Option<
        DynamicBlock<ComputeRegionInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl>,
    >,
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceTemplateDiskEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_delete: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    boot: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_size_gb: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interface: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provisioned_iops: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_policies: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_image: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_snapshot: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_encryption_key: Option<Vec<ComputeRegionInstanceTemplateDiskElDiskEncryptionKeyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_image_encryption_key: Option<Vec<ComputeRegionInstanceTemplateDiskElSourceImageEncryptionKeyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_snapshot_encryption_key: Option<Vec<ComputeRegionInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl>>,
    dynamic: ComputeRegionInstanceTemplateDiskElDynamic,
}

impl ComputeRegionInstanceTemplateDiskEl {
    #[doc= "Set the field `auto_delete`.\nWhether or not the disk should be auto-deleted. This defaults to true."]
    pub fn set_auto_delete(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.auto_delete = Some(v.into());
        self
    }

    #[doc= "Set the field `boot`.\nIndicates that this is a boot disk."]
    pub fn set_boot(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.boot = Some(v.into());
        self
    }

    #[doc= "Set the field `device_name`.\nA unique device name that is reflected into the /dev/ tree of a Linux operating system running within the instance. If not specified, the server chooses a default device name to apply to this disk."]
    pub fn set_device_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.device_name = Some(v.into());
        self
    }

    #[doc= "Set the field `disk_name`.\nName of the disk. When not provided, this defaults to the name of the instance."]
    pub fn set_disk_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_name = Some(v.into());
        self
    }

    #[doc= "Set the field `disk_size_gb`.\nThe size of the image in gigabytes. If not specified, it will inherit the size of its base image. For SCRATCH disks, the size must be one of 375 or 3000 GB, with a default of 375 GB."]
    pub fn set_disk_size_gb(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.disk_size_gb = Some(v.into());
        self
    }

    #[doc= "Set the field `disk_type`.\nThe Google Compute Engine disk type. Such as \"pd-ssd\", \"local-ssd\", \"pd-balanced\" or \"pd-standard\"."]
    pub fn set_disk_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk_type = Some(v.into());
        self
    }

    #[doc= "Set the field `interface`.\nSpecifies the disk interface to use for attaching this disk."]
    pub fn set_interface(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.interface = Some(v.into());
        self
    }

    #[doc= "Set the field `labels`.\nA set of key/value label pairs to assign to disks,"]
    pub fn set_labels(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.labels = Some(v.into());
        self
    }

    #[doc= "Set the field `mode`.\nThe mode in which to attach this disk, either READ_WRITE or READ_ONLY. If you are attaching or creating a boot disk, this must read-write mode."]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }

    #[doc= "Set the field `provisioned_iops`.\nIndicates how many IOPS to provision for the disk. This sets the number of I/O operations per second that the disk can handle. Values must be between 10,000 and 120,000. For more details, see the [Extreme persistent disk documentation](https://cloud.google.com/compute/docs/disks/extreme-persistent-disk)."]
    pub fn set_provisioned_iops(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.provisioned_iops = Some(v.into());
        self
    }

    #[doc= "Set the field `resource_policies`.\nA list (short name or id) of resource policies to attach to this disk. Currently a max of 1 resource policy is supported."]
    pub fn set_resource_policies(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.resource_policies = Some(v.into());
        self
    }

    #[doc= "Set the field `source`.\nThe name (not self_link) of the disk (such as those managed by google_compute_disk) to attach. ~> Note: Either source or source_image is required when creating a new instance except for when creating a local SSD."]
    pub fn set_source(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source = Some(v.into());
        self
    }

    #[doc= "Set the field `source_image`.\nThe image from which to initialize this disk. This can be one of: the image's self_link, projects/{project}/global/images/{image}, projects/{project}/global/images/family/{family}, global/images/{image}, global/images/family/{family}, family/{family}, {project}/{family}, {project}/{image}, {family}, or {image}. ~> Note: Either source or source_image is required when creating a new instance except for when creating a local SSD."]
    pub fn set_source_image(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_image = Some(v.into());
        self
    }

    #[doc= "Set the field `source_snapshot`.\nThe source snapshot to create this disk. When creating\na new instance, one of initializeParams.sourceSnapshot,\ninitializeParams.sourceImage, or disks.source is\nrequired except for local SSD."]
    pub fn set_source_snapshot(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_snapshot = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\nThe type of Google Compute Engine disk, can be either \"SCRATCH\" or \"PERSISTENT\"."]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc= "Set the field `disk_encryption_key`.\n"]
    pub fn set_disk_encryption_key(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionInstanceTemplateDiskElDiskEncryptionKeyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.disk_encryption_key = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.disk_encryption_key = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `source_image_encryption_key`.\n"]
    pub fn set_source_image_encryption_key(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionInstanceTemplateDiskElSourceImageEncryptionKeyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source_image_encryption_key = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source_image_encryption_key = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `source_snapshot_encryption_key`.\n"]
    pub fn set_source_snapshot_encryption_key(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source_snapshot_encryption_key = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source_snapshot_encryption_key = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeRegionInstanceTemplateDiskEl {
    type O = BlockAssignable<ComputeRegionInstanceTemplateDiskEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceTemplateDiskEl {}

impl BuildComputeRegionInstanceTemplateDiskEl {
    pub fn build(self) -> ComputeRegionInstanceTemplateDiskEl {
        ComputeRegionInstanceTemplateDiskEl {
            auto_delete: core::default::Default::default(),
            boot: core::default::Default::default(),
            device_name: core::default::Default::default(),
            disk_name: core::default::Default::default(),
            disk_size_gb: core::default::Default::default(),
            disk_type: core::default::Default::default(),
            interface: core::default::Default::default(),
            labels: core::default::Default::default(),
            mode: core::default::Default::default(),
            provisioned_iops: core::default::Default::default(),
            resource_policies: core::default::Default::default(),
            source: core::default::Default::default(),
            source_image: core::default::Default::default(),
            source_snapshot: core::default::Default::default(),
            type_: core::default::Default::default(),
            disk_encryption_key: core::default::Default::default(),
            source_image_encryption_key: core::default::Default::default(),
            source_snapshot_encryption_key: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeRegionInstanceTemplateDiskElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeRegionInstanceTemplateDiskElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionInstanceTemplateDiskElRef {
        ComputeRegionInstanceTemplateDiskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceTemplateDiskElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `auto_delete` after provisioning.\nWhether or not the disk should be auto-deleted. This defaults to true."]
    pub fn auto_delete(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_delete", self.base))
    }

    #[doc= "Get a reference to the value of field `boot` after provisioning.\nIndicates that this is a boot disk."]
    pub fn boot(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.boot", self.base))
    }

    #[doc= "Get a reference to the value of field `device_name` after provisioning.\nA unique device name that is reflected into the /dev/ tree of a Linux operating system running within the instance. If not specified, the server chooses a default device name to apply to this disk."]
    pub fn device_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }

    #[doc= "Get a reference to the value of field `disk_name` after provisioning.\nName of the disk. When not provided, this defaults to the name of the instance."]
    pub fn disk_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_name", self.base))
    }

    #[doc= "Get a reference to the value of field `disk_size_gb` after provisioning.\nThe size of the image in gigabytes. If not specified, it will inherit the size of its base image. For SCRATCH disks, the size must be one of 375 or 3000 GB, with a default of 375 GB."]
    pub fn disk_size_gb(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }

    #[doc= "Get a reference to the value of field `disk_type` after provisioning.\nThe Google Compute Engine disk type. Such as \"pd-ssd\", \"local-ssd\", \"pd-balanced\" or \"pd-standard\"."]
    pub fn disk_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }

    #[doc= "Get a reference to the value of field `interface` after provisioning.\nSpecifies the disk interface to use for attaching this disk."]
    pub fn interface(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.interface", self.base))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to disks,"]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc= "Get a reference to the value of field `mode` after provisioning.\nThe mode in which to attach this disk, either READ_WRITE or READ_ONLY. If you are attaching or creating a boot disk, this must read-write mode."]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc= "Get a reference to the value of field `provisioned_iops` after provisioning.\nIndicates how many IOPS to provision for the disk. This sets the number of I/O operations per second that the disk can handle. Values must be between 10,000 and 120,000. For more details, see the [Extreme persistent disk documentation](https://cloud.google.com/compute/docs/disks/extreme-persistent-disk)."]
    pub fn provisioned_iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.provisioned_iops", self.base))
    }

    #[doc= "Get a reference to the value of field `resource_policies` after provisioning.\nA list (short name or id) of resource policies to attach to this disk. Currently a max of 1 resource policy is supported."]
    pub fn resource_policies(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.resource_policies", self.base))
    }

    #[doc= "Get a reference to the value of field `source` after provisioning.\nThe name (not self_link) of the disk (such as those managed by google_compute_disk) to attach. ~> Note: Either source or source_image is required when creating a new instance except for when creating a local SSD."]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc= "Get a reference to the value of field `source_image` after provisioning.\nThe image from which to initialize this disk. This can be one of: the image's self_link, projects/{project}/global/images/{image}, projects/{project}/global/images/family/{family}, global/images/{image}, global/images/family/{family}, family/{family}, {project}/{family}, {project}/{image}, {family}, or {image}. ~> Note: Either source or source_image is required when creating a new instance except for when creating a local SSD."]
    pub fn source_image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_image", self.base))
    }

    #[doc= "Get a reference to the value of field `source_snapshot` after provisioning.\nThe source snapshot to create this disk. When creating\na new instance, one of initializeParams.sourceSnapshot,\ninitializeParams.sourceImage, or disks.source is\nrequired except for local SSD."]
    pub fn source_snapshot(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_snapshot", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe type of Google Compute Engine disk, can be either \"SCRATCH\" or \"PERSISTENT\"."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `disk_encryption_key` after provisioning.\n"]
    pub fn disk_encryption_key(&self) -> ListRef<ComputeRegionInstanceTemplateDiskElDiskEncryptionKeyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.disk_encryption_key", self.base))
    }

    #[doc= "Get a reference to the value of field `source_image_encryption_key` after provisioning.\n"]
    pub fn source_image_encryption_key(
        &self,
    ) -> ListRef<ComputeRegionInstanceTemplateDiskElSourceImageEncryptionKeyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source_image_encryption_key", self.base))
    }

    #[doc= "Get a reference to the value of field `source_snapshot_encryption_key` after provisioning.\n"]
    pub fn source_snapshot_encryption_key(
        &self,
    ) -> ListRef<ComputeRegionInstanceTemplateDiskElSourceSnapshotEncryptionKeyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source_snapshot_encryption_key", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceTemplateGuestAcceleratorEl {
    count: PrimField<f64>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl ComputeRegionInstanceTemplateGuestAcceleratorEl { }

impl ToListMappable for ComputeRegionInstanceTemplateGuestAcceleratorEl {
    type O = BlockAssignable<ComputeRegionInstanceTemplateGuestAcceleratorEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceTemplateGuestAcceleratorEl {
    #[doc= "The number of the guest accelerator cards exposed to this instance."]
    pub count: PrimField<f64>,
    #[doc= "The accelerator type resource to expose to this instance. E.g. nvidia-tesla-k80."]
    pub type_: PrimField<String>,
}

impl BuildComputeRegionInstanceTemplateGuestAcceleratorEl {
    pub fn build(self) -> ComputeRegionInstanceTemplateGuestAcceleratorEl {
        ComputeRegionInstanceTemplateGuestAcceleratorEl {
            count: self.count,
            type_: self.type_,
        }
    }
}

pub struct ComputeRegionInstanceTemplateGuestAcceleratorElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeRegionInstanceTemplateGuestAcceleratorElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionInstanceTemplateGuestAcceleratorElRef {
        ComputeRegionInstanceTemplateGuestAcceleratorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceTemplateGuestAcceleratorElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `count` after provisioning.\nThe number of the guest accelerator cards exposed to this instance."]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe accelerator type resource to expose to this instance. E.g. nvidia-tesla-k80."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceTemplateNetworkInterfaceElAccessConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nat_ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_tier: Option<PrimField<String>>,
}

impl ComputeRegionInstanceTemplateNetworkInterfaceElAccessConfigEl {
    #[doc= "Set the field `nat_ip`.\nThe IP address that will be 1:1 mapped to the instance's network ip. If not given, one will be generated."]
    pub fn set_nat_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nat_ip = Some(v.into());
        self
    }

    #[doc= "Set the field `network_tier`.\nThe networking tier used for configuring this instance template. This field can take the following values: PREMIUM, STANDARD, FIXED_STANDARD. If this field is not specified, it is assumed to be PREMIUM."]
    pub fn set_network_tier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_tier = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeRegionInstanceTemplateNetworkInterfaceElAccessConfigEl {
    type O = BlockAssignable<ComputeRegionInstanceTemplateNetworkInterfaceElAccessConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceTemplateNetworkInterfaceElAccessConfigEl {}

impl BuildComputeRegionInstanceTemplateNetworkInterfaceElAccessConfigEl {
    pub fn build(self) -> ComputeRegionInstanceTemplateNetworkInterfaceElAccessConfigEl {
        ComputeRegionInstanceTemplateNetworkInterfaceElAccessConfigEl {
            nat_ip: core::default::Default::default(),
            network_tier: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionInstanceTemplateNetworkInterfaceElAccessConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeRegionInstanceTemplateNetworkInterfaceElAccessConfigElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionInstanceTemplateNetworkInterfaceElAccessConfigElRef {
        ComputeRegionInstanceTemplateNetworkInterfaceElAccessConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceTemplateNetworkInterfaceElAccessConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `nat_ip` after provisioning.\nThe IP address that will be 1:1 mapped to the instance's network ip. If not given, one will be generated."]
    pub fn nat_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.nat_ip", self.base))
    }

    #[doc= "Get a reference to the value of field `network_tier` after provisioning.\nThe networking tier used for configuring this instance template. This field can take the following values: PREMIUM, STANDARD, FIXED_STANDARD. If this field is not specified, it is assumed to be PREMIUM."]
    pub fn network_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_tier", self.base))
    }

    #[doc= "Get a reference to the value of field `public_ptr_domain_name` after provisioning.\nThe DNS domain name for the public PTR record.The DNS domain name for the public PTR record."]
    pub fn public_ptr_domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_ptr_domain_name", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceTemplateNetworkInterfaceElAliasIpRangeEl {
    ip_cidr_range: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnetwork_range_name: Option<PrimField<String>>,
}

impl ComputeRegionInstanceTemplateNetworkInterfaceElAliasIpRangeEl {
    #[doc= "Set the field `subnetwork_range_name`.\nThe subnetwork secondary range name specifying the secondary range from which to allocate the IP CIDR range for this alias IP range. If left unspecified, the primary range of the subnetwork will be used."]
    pub fn set_subnetwork_range_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnetwork_range_name = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeRegionInstanceTemplateNetworkInterfaceElAliasIpRangeEl {
    type O = BlockAssignable<ComputeRegionInstanceTemplateNetworkInterfaceElAliasIpRangeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceTemplateNetworkInterfaceElAliasIpRangeEl {
    #[doc= "The IP CIDR range represented by this alias IP range. This IP CIDR range must belong to the specified subnetwork and cannot contain IP addresses reserved by system or used by other network interfaces. At the time of writing only a netmask (e.g. /24) may be supplied, with a CIDR format resulting in an API error."]
    pub ip_cidr_range: PrimField<String>,
}

impl BuildComputeRegionInstanceTemplateNetworkInterfaceElAliasIpRangeEl {
    pub fn build(self) -> ComputeRegionInstanceTemplateNetworkInterfaceElAliasIpRangeEl {
        ComputeRegionInstanceTemplateNetworkInterfaceElAliasIpRangeEl {
            ip_cidr_range: self.ip_cidr_range,
            subnetwork_range_name: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionInstanceTemplateNetworkInterfaceElAliasIpRangeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeRegionInstanceTemplateNetworkInterfaceElAliasIpRangeElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionInstanceTemplateNetworkInterfaceElAliasIpRangeElRef {
        ComputeRegionInstanceTemplateNetworkInterfaceElAliasIpRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceTemplateNetworkInterfaceElAliasIpRangeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `ip_cidr_range` after provisioning.\nThe IP CIDR range represented by this alias IP range. This IP CIDR range must belong to the specified subnetwork and cannot contain IP addresses reserved by system or used by other network interfaces. At the time of writing only a netmask (e.g. /24) may be supplied, with a CIDR format resulting in an API error."]
    pub fn ip_cidr_range(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_cidr_range", self.base))
    }

    #[doc= "Get a reference to the value of field `subnetwork_range_name` after provisioning.\nThe subnetwork secondary range name specifying the secondary range from which to allocate the IP CIDR range for this alias IP range. If left unspecified, the primary range of the subnetwork will be used."]
    pub fn subnetwork_range_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnetwork_range_name", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl {
    network_tier: PrimField<String>,
}

impl ComputeRegionInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl { }

impl ToListMappable for ComputeRegionInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl {
    type O = BlockAssignable<ComputeRegionInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl {
    #[doc= "The service-level to be provided for IPv6 traffic when the subnet has an external subnet. Only PREMIUM tier is valid for IPv6"]
    pub network_tier: PrimField<String>,
}

impl BuildComputeRegionInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl {
    pub fn build(self) -> ComputeRegionInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl {
        ComputeRegionInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl { network_tier: self.network_tier }
    }
}

pub struct ComputeRegionInstanceTemplateNetworkInterfaceElIpv6AccessConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeRegionInstanceTemplateNetworkInterfaceElIpv6AccessConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionInstanceTemplateNetworkInterfaceElIpv6AccessConfigElRef {
        ComputeRegionInstanceTemplateNetworkInterfaceElIpv6AccessConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceTemplateNetworkInterfaceElIpv6AccessConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `external_ipv6` after provisioning.\nThe first IPv6 address of the external IPv6 range associated with this instance, prefix length is stored in externalIpv6PrefixLength in ipv6AccessConfig. The field is output only, an IPv6 address from a subnetwork associated with the instance will be allocated dynamically."]
    pub fn external_ipv6(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.external_ipv6", self.base))
    }

    #[doc= "Get a reference to the value of field `external_ipv6_prefix_length` after provisioning.\nThe prefix length of the external IPv6 range."]
    pub fn external_ipv6_prefix_length(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.external_ipv6_prefix_length", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of this access configuration."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `network_tier` after provisioning.\nThe service-level to be provided for IPv6 traffic when the subnet has an external subnet. Only PREMIUM tier is valid for IPv6"]
    pub fn network_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_tier", self.base))
    }

    #[doc= "Get a reference to the value of field `public_ptr_domain_name` after provisioning.\nThe domain name to be used when creating DNSv6 records for the external IPv6 ranges."]
    pub fn public_ptr_domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.public_ptr_domain_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionInstanceTemplateNetworkInterfaceElDynamic {
    access_config: Option<DynamicBlock<ComputeRegionInstanceTemplateNetworkInterfaceElAccessConfigEl>>,
    alias_ip_range: Option<DynamicBlock<ComputeRegionInstanceTemplateNetworkInterfaceElAliasIpRangeEl>>,
    ipv6_access_config: Option<DynamicBlock<ComputeRegionInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl>>,
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceTemplateNetworkInterfaceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    internal_ipv6_prefix_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_address: Option<PrimField<String>>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    access_config: Option<Vec<ComputeRegionInstanceTemplateNetworkInterfaceElAccessConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alias_ip_range: Option<Vec<ComputeRegionInstanceTemplateNetworkInterfaceElAliasIpRangeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6_access_config: Option<Vec<ComputeRegionInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl>>,
    dynamic: ComputeRegionInstanceTemplateNetworkInterfaceElDynamic,
}

impl ComputeRegionInstanceTemplateNetworkInterfaceEl {
    #[doc= "Set the field `internal_ipv6_prefix_length`.\nThe prefix length of the primary internal IPv6 range."]
    pub fn set_internal_ipv6_prefix_length(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.internal_ipv6_prefix_length = Some(v.into());
        self
    }

    #[doc= "Set the field `ipv6_address`.\nAn IPv6 internal network address for this network interface. If not specified, Google Cloud will automatically assign an internal IPv6 address from the instance's subnetwork."]
    pub fn set_ipv6_address(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ipv6_address = Some(v.into());
        self
    }

    #[doc= "Set the field `network`.\nThe name or self_link of the network to attach this interface to. Use network attribute for Legacy or Auto subnetted networks and subnetwork for custom subnetted networks."]
    pub fn set_network(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network = Some(v.into());
        self
    }

    #[doc= "Set the field `network_ip`.\nThe private IP address to assign to the instance. If empty, the address will be automatically assigned."]
    pub fn set_network_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.network_ip = Some(v.into());
        self
    }

    #[doc= "Set the field `nic_type`.\nThe type of vNIC to be used on this interface. Possible values:GVNIC, VIRTIO_NET"]
    pub fn set_nic_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.nic_type = Some(v.into());
        self
    }

    #[doc= "Set the field `queue_count`.\nThe networking queue count that's specified by users for the network interface. Both Rx and Tx queues will be set to this number. It will be empty if not specified."]
    pub fn set_queue_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.queue_count = Some(v.into());
        self
    }

    #[doc= "Set the field `stack_type`.\nThe stack type for this network interface to identify whether the IPv6 feature is enabled or not. If not specified, IPV4_ONLY will be used."]
    pub fn set_stack_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.stack_type = Some(v.into());
        self
    }

    #[doc= "Set the field `subnetwork`.\nThe name of the subnetwork to attach this interface to. The subnetwork must exist in the same region this instance will be created in. Either network or subnetwork must be provided."]
    pub fn set_subnetwork(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnetwork = Some(v.into());
        self
    }

    #[doc= "Set the field `subnetwork_project`.\nThe ID of the project in which the subnetwork belongs. If it is not provided, the provider project is used."]
    pub fn set_subnetwork_project(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnetwork_project = Some(v.into());
        self
    }

    #[doc= "Set the field `access_config`.\n"]
    pub fn set_access_config(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionInstanceTemplateNetworkInterfaceElAccessConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.access_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.access_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `alias_ip_range`.\n"]
    pub fn set_alias_ip_range(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionInstanceTemplateNetworkInterfaceElAliasIpRangeEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.alias_ip_range = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.alias_ip_range = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `ipv6_access_config`.\n"]
    pub fn set_ipv6_access_config(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ipv6_access_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ipv6_access_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeRegionInstanceTemplateNetworkInterfaceEl {
    type O = BlockAssignable<ComputeRegionInstanceTemplateNetworkInterfaceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceTemplateNetworkInterfaceEl {}

impl BuildComputeRegionInstanceTemplateNetworkInterfaceEl {
    pub fn build(self) -> ComputeRegionInstanceTemplateNetworkInterfaceEl {
        ComputeRegionInstanceTemplateNetworkInterfaceEl {
            internal_ipv6_prefix_length: core::default::Default::default(),
            ipv6_address: core::default::Default::default(),
            network: core::default::Default::default(),
            network_ip: core::default::Default::default(),
            nic_type: core::default::Default::default(),
            queue_count: core::default::Default::default(),
            stack_type: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
            subnetwork_project: core::default::Default::default(),
            access_config: core::default::Default::default(),
            alias_ip_range: core::default::Default::default(),
            ipv6_access_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeRegionInstanceTemplateNetworkInterfaceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeRegionInstanceTemplateNetworkInterfaceElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionInstanceTemplateNetworkInterfaceElRef {
        ComputeRegionInstanceTemplateNetworkInterfaceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceTemplateNetworkInterfaceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `internal_ipv6_prefix_length` after provisioning.\nThe prefix length of the primary internal IPv6 range."]
    pub fn internal_ipv6_prefix_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.internal_ipv6_prefix_length", self.base))
    }

    #[doc= "Get a reference to the value of field `ipv6_access_type` after provisioning.\nOne of EXTERNAL, INTERNAL to indicate whether the IP can be accessed from the Internet. This field is always inherited from its subnetwork."]
    pub fn ipv6_access_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_access_type", self.base))
    }

    #[doc= "Get a reference to the value of field `ipv6_address` after provisioning.\nAn IPv6 internal network address for this network interface. If not specified, Google Cloud will automatically assign an internal IPv6 address from the instance's subnetwork."]
    pub fn ipv6_address(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_address", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the network_interface."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `network` after provisioning.\nThe name or self_link of the network to attach this interface to. Use network attribute for Legacy or Auto subnetted networks and subnetwork for custom subnetted networks."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc= "Get a reference to the value of field `network_ip` after provisioning.\nThe private IP address to assign to the instance. If empty, the address will be automatically assigned."]
    pub fn network_ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_ip", self.base))
    }

    #[doc= "Get a reference to the value of field `nic_type` after provisioning.\nThe type of vNIC to be used on this interface. Possible values:GVNIC, VIRTIO_NET"]
    pub fn nic_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.nic_type", self.base))
    }

    #[doc= "Get a reference to the value of field `queue_count` after provisioning.\nThe networking queue count that's specified by users for the network interface. Both Rx and Tx queues will be set to this number. It will be empty if not specified."]
    pub fn queue_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.queue_count", self.base))
    }

    #[doc= "Get a reference to the value of field `stack_type` after provisioning.\nThe stack type for this network interface to identify whether the IPv6 feature is enabled or not. If not specified, IPV4_ONLY will be used."]
    pub fn stack_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stack_type", self.base))
    }

    #[doc= "Get a reference to the value of field `subnetwork` after provisioning.\nThe name of the subnetwork to attach this interface to. The subnetwork must exist in the same region this instance will be created in. Either network or subnetwork must be provided."]
    pub fn subnetwork(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }

    #[doc= "Get a reference to the value of field `subnetwork_project` after provisioning.\nThe ID of the project in which the subnetwork belongs. If it is not provided, the provider project is used."]
    pub fn subnetwork_project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnetwork_project", self.base))
    }

    #[doc= "Get a reference to the value of field `access_config` after provisioning.\n"]
    pub fn access_config(&self) -> ListRef<ComputeRegionInstanceTemplateNetworkInterfaceElAccessConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.access_config", self.base))
    }

    #[doc= "Get a reference to the value of field `alias_ip_range` after provisioning.\n"]
    pub fn alias_ip_range(&self) -> ListRef<ComputeRegionInstanceTemplateNetworkInterfaceElAliasIpRangeElRef> {
        ListRef::new(self.shared().clone(), format!("{}.alias_ip_range", self.base))
    }

    #[doc= "Get a reference to the value of field `ipv6_access_config` after provisioning.\n"]
    pub fn ipv6_access_config(&self) -> ListRef<ComputeRegionInstanceTemplateNetworkInterfaceElIpv6AccessConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ipv6_access_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceTemplateNetworkPerformanceConfigEl {
    total_egress_bandwidth_tier: PrimField<String>,
}

impl ComputeRegionInstanceTemplateNetworkPerformanceConfigEl { }

impl ToListMappable for ComputeRegionInstanceTemplateNetworkPerformanceConfigEl {
    type O = BlockAssignable<ComputeRegionInstanceTemplateNetworkPerformanceConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceTemplateNetworkPerformanceConfigEl {
    #[doc= "The egress bandwidth tier to enable. Possible values:TIER_1, DEFAULT"]
    pub total_egress_bandwidth_tier: PrimField<String>,
}

impl BuildComputeRegionInstanceTemplateNetworkPerformanceConfigEl {
    pub fn build(self) -> ComputeRegionInstanceTemplateNetworkPerformanceConfigEl {
        ComputeRegionInstanceTemplateNetworkPerformanceConfigEl {
            total_egress_bandwidth_tier: self.total_egress_bandwidth_tier,
        }
    }
}

pub struct ComputeRegionInstanceTemplateNetworkPerformanceConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeRegionInstanceTemplateNetworkPerformanceConfigElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionInstanceTemplateNetworkPerformanceConfigElRef {
        ComputeRegionInstanceTemplateNetworkPerformanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceTemplateNetworkPerformanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `total_egress_bandwidth_tier` after provisioning.\nThe egress bandwidth tier to enable. Possible values:TIER_1, DEFAULT"]
    pub fn total_egress_bandwidth_tier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.total_egress_bandwidth_tier", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceTemplateReservationAffinityElSpecificReservationEl {
    key: PrimField<String>,
    values: ListField<PrimField<String>>,
}

impl ComputeRegionInstanceTemplateReservationAffinityElSpecificReservationEl { }

impl ToListMappable for ComputeRegionInstanceTemplateReservationAffinityElSpecificReservationEl {
    type O = BlockAssignable<ComputeRegionInstanceTemplateReservationAffinityElSpecificReservationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceTemplateReservationAffinityElSpecificReservationEl {
    #[doc= "Corresponds to the label key of a reservation resource. To target a SPECIFIC_RESERVATION by name, specify compute.googleapis.com/reservation-name as the key and specify the name of your reservation as the only value."]
    pub key: PrimField<String>,
    #[doc= "Corresponds to the label values of a reservation resource."]
    pub values: ListField<PrimField<String>>,
}

impl BuildComputeRegionInstanceTemplateReservationAffinityElSpecificReservationEl {
    pub fn build(self) -> ComputeRegionInstanceTemplateReservationAffinityElSpecificReservationEl {
        ComputeRegionInstanceTemplateReservationAffinityElSpecificReservationEl {
            key: self.key,
            values: self.values,
        }
    }
}

pub struct ComputeRegionInstanceTemplateReservationAffinityElSpecificReservationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeRegionInstanceTemplateReservationAffinityElSpecificReservationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionInstanceTemplateReservationAffinityElSpecificReservationElRef {
        ComputeRegionInstanceTemplateReservationAffinityElSpecificReservationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceTemplateReservationAffinityElSpecificReservationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\nCorresponds to the label key of a reservation resource. To target a SPECIFIC_RESERVATION by name, specify compute.googleapis.com/reservation-name as the key and specify the name of your reservation as the only value."]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc= "Get a reference to the value of field `values` after provisioning.\nCorresponds to the label values of a reservation resource."]
    pub fn values(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionInstanceTemplateReservationAffinityElDynamic {
    specific_reservation: Option<
        DynamicBlock<ComputeRegionInstanceTemplateReservationAffinityElSpecificReservationEl>,
    >,
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceTemplateReservationAffinityEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    specific_reservation: Option<Vec<ComputeRegionInstanceTemplateReservationAffinityElSpecificReservationEl>>,
    dynamic: ComputeRegionInstanceTemplateReservationAffinityElDynamic,
}

impl ComputeRegionInstanceTemplateReservationAffinityEl {
    #[doc= "Set the field `specific_reservation`.\n"]
    pub fn set_specific_reservation(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionInstanceTemplateReservationAffinityElSpecificReservationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.specific_reservation = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.specific_reservation = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeRegionInstanceTemplateReservationAffinityEl {
    type O = BlockAssignable<ComputeRegionInstanceTemplateReservationAffinityEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceTemplateReservationAffinityEl {
    #[doc= "The type of reservation from which this instance can consume resources."]
    pub type_: PrimField<String>,
}

impl BuildComputeRegionInstanceTemplateReservationAffinityEl {
    pub fn build(self) -> ComputeRegionInstanceTemplateReservationAffinityEl {
        ComputeRegionInstanceTemplateReservationAffinityEl {
            type_: self.type_,
            specific_reservation: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeRegionInstanceTemplateReservationAffinityElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeRegionInstanceTemplateReservationAffinityElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionInstanceTemplateReservationAffinityElRef {
        ComputeRegionInstanceTemplateReservationAffinityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceTemplateReservationAffinityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe type of reservation from which this instance can consume resources."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `specific_reservation` after provisioning.\n"]
    pub fn specific_reservation(
        &self,
    ) -> ListRef<ComputeRegionInstanceTemplateReservationAffinityElSpecificReservationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.specific_reservation", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    nanos: Option<PrimField<f64>>,
    seconds: PrimField<f64>,
}

impl ComputeRegionInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl {
    #[doc= "Set the field `nanos`.\nSpan of time that's a fraction of a second at nanosecond\nresolution. Durations less than one second are represented\nwith a 0 seconds field and a positive nanos field. Must\nbe from 0 to 999,999,999 inclusive."]
    pub fn set_nanos(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.nanos = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeRegionInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl {
    type O = BlockAssignable<ComputeRegionInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl {
    #[doc= "Span of time at a resolution of a second.\nMust be from 0 to 315,576,000,000 inclusive."]
    pub seconds: PrimField<f64>,
}

impl BuildComputeRegionInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl {
    pub fn build(self) -> ComputeRegionInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl {
        ComputeRegionInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl {
            nanos: core::default::Default::default(),
            seconds: self.seconds,
        }
    }
}

pub struct ComputeRegionInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeRegionInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeRegionInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutElRef {
        ComputeRegionInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `nanos` after provisioning.\nSpan of time that's a fraction of a second at nanosecond\nresolution. Durations less than one second are represented\nwith a 0 seconds field and a positive nanos field. Must\nbe from 0 to 999,999,999 inclusive."]
    pub fn nanos(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc= "Get a reference to the value of field `seconds` after provisioning.\nSpan of time at a resolution of a second.\nMust be from 0 to 315,576,000,000 inclusive."]
    pub fn seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceTemplateSchedulingElNodeAffinitiesEl {
    key: PrimField<String>,
    operator: PrimField<String>,
    values: SetField<PrimField<String>>,
}

impl ComputeRegionInstanceTemplateSchedulingElNodeAffinitiesEl { }

impl ToListMappable for ComputeRegionInstanceTemplateSchedulingElNodeAffinitiesEl {
    type O = BlockAssignable<ComputeRegionInstanceTemplateSchedulingElNodeAffinitiesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceTemplateSchedulingElNodeAffinitiesEl {
    #[doc= ""]
    pub key: PrimField<String>,
    #[doc= ""]
    pub operator: PrimField<String>,
    #[doc= ""]
    pub values: SetField<PrimField<String>>,
}

impl BuildComputeRegionInstanceTemplateSchedulingElNodeAffinitiesEl {
    pub fn build(self) -> ComputeRegionInstanceTemplateSchedulingElNodeAffinitiesEl {
        ComputeRegionInstanceTemplateSchedulingElNodeAffinitiesEl {
            key: self.key,
            operator: self.operator,
            values: self.values,
        }
    }
}

pub struct ComputeRegionInstanceTemplateSchedulingElNodeAffinitiesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeRegionInstanceTemplateSchedulingElNodeAffinitiesElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionInstanceTemplateSchedulingElNodeAffinitiesElRef {
        ComputeRegionInstanceTemplateSchedulingElNodeAffinitiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceTemplateSchedulingElNodeAffinitiesElRef {
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

#[derive(Serialize, Default)]
struct ComputeRegionInstanceTemplateSchedulingElDynamic {
    local_ssd_recovery_timeout: Option<
        DynamicBlock<ComputeRegionInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl>,
    >,
    node_affinities: Option<DynamicBlock<ComputeRegionInstanceTemplateSchedulingElNodeAffinitiesEl>>,
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceTemplateSchedulingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_restart: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_termination_action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_node_cpus: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_host_maintenance: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preemptible: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provisioning_model: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    local_ssd_recovery_timeout: Option<Vec<ComputeRegionInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_affinities: Option<Vec<ComputeRegionInstanceTemplateSchedulingElNodeAffinitiesEl>>,
    dynamic: ComputeRegionInstanceTemplateSchedulingElDynamic,
}

impl ComputeRegionInstanceTemplateSchedulingEl {
    #[doc= "Set the field `automatic_restart`.\nSpecifies whether the instance should be automatically restarted if it is terminated by Compute Engine (not terminated by a user). This defaults to true."]
    pub fn set_automatic_restart(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.automatic_restart = Some(v.into());
        self
    }

    #[doc= "Set the field `instance_termination_action`.\nSpecifies the action GCE should take when SPOT VM is preempted."]
    pub fn set_instance_termination_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_termination_action = Some(v.into());
        self
    }

    #[doc= "Set the field `min_node_cpus`.\nMinimum number of cpus for the instance."]
    pub fn set_min_node_cpus(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.min_node_cpus = Some(v.into());
        self
    }

    #[doc= "Set the field `on_host_maintenance`.\nDefines the maintenance behavior for this instance."]
    pub fn set_on_host_maintenance(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.on_host_maintenance = Some(v.into());
        self
    }

    #[doc= "Set the field `preemptible`.\nAllows instance to be preempted. This defaults to false."]
    pub fn set_preemptible(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.preemptible = Some(v.into());
        self
    }

    #[doc= "Set the field `provisioning_model`.\nWhether the instance is spot. If this is set as SPOT."]
    pub fn set_provisioning_model(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.provisioning_model = Some(v.into());
        self
    }

    #[doc= "Set the field `local_ssd_recovery_timeout`.\n"]
    pub fn set_local_ssd_recovery_timeout(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.local_ssd_recovery_timeout = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.local_ssd_recovery_timeout = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `node_affinities`.\n"]
    pub fn set_node_affinities(
        mut self,
        v: impl Into<BlockAssignable<ComputeRegionInstanceTemplateSchedulingElNodeAffinitiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.node_affinities = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.node_affinities = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeRegionInstanceTemplateSchedulingEl {
    type O = BlockAssignable<ComputeRegionInstanceTemplateSchedulingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceTemplateSchedulingEl {}

impl BuildComputeRegionInstanceTemplateSchedulingEl {
    pub fn build(self) -> ComputeRegionInstanceTemplateSchedulingEl {
        ComputeRegionInstanceTemplateSchedulingEl {
            automatic_restart: core::default::Default::default(),
            instance_termination_action: core::default::Default::default(),
            min_node_cpus: core::default::Default::default(),
            on_host_maintenance: core::default::Default::default(),
            preemptible: core::default::Default::default(),
            provisioning_model: core::default::Default::default(),
            local_ssd_recovery_timeout: core::default::Default::default(),
            node_affinities: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeRegionInstanceTemplateSchedulingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeRegionInstanceTemplateSchedulingElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionInstanceTemplateSchedulingElRef {
        ComputeRegionInstanceTemplateSchedulingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceTemplateSchedulingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `automatic_restart` after provisioning.\nSpecifies whether the instance should be automatically restarted if it is terminated by Compute Engine (not terminated by a user). This defaults to true."]
    pub fn automatic_restart(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.automatic_restart", self.base))
    }

    #[doc= "Get a reference to the value of field `instance_termination_action` after provisioning.\nSpecifies the action GCE should take when SPOT VM is preempted."]
    pub fn instance_termination_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_termination_action", self.base))
    }

    #[doc= "Get a reference to the value of field `min_node_cpus` after provisioning.\nMinimum number of cpus for the instance."]
    pub fn min_node_cpus(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_node_cpus", self.base))
    }

    #[doc= "Get a reference to the value of field `on_host_maintenance` after provisioning.\nDefines the maintenance behavior for this instance."]
    pub fn on_host_maintenance(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.on_host_maintenance", self.base))
    }

    #[doc= "Get a reference to the value of field `preemptible` after provisioning.\nAllows instance to be preempted. This defaults to false."]
    pub fn preemptible(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.preemptible", self.base))
    }

    #[doc= "Get a reference to the value of field `provisioning_model` after provisioning.\nWhether the instance is spot. If this is set as SPOT."]
    pub fn provisioning_model(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.provisioning_model", self.base))
    }

    #[doc= "Get a reference to the value of field `local_ssd_recovery_timeout` after provisioning.\n"]
    pub fn local_ssd_recovery_timeout(
        &self,
    ) -> ListRef<ComputeRegionInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutElRef> {
        ListRef::new(self.shared().clone(), format!("{}.local_ssd_recovery_timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceTemplateServiceAccountEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<PrimField<String>>,
    scopes: SetField<PrimField<String>>,
}

impl ComputeRegionInstanceTemplateServiceAccountEl {
    #[doc= "Set the field `email`.\nThe service account e-mail address. If not given, the default Google Compute Engine service account is used."]
    pub fn set_email(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.email = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeRegionInstanceTemplateServiceAccountEl {
    type O = BlockAssignable<ComputeRegionInstanceTemplateServiceAccountEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceTemplateServiceAccountEl {
    #[doc= "A list of service scopes. Both OAuth2 URLs and gcloud short names are supported. To allow full access to all Cloud APIs, use the cloud-platform scope."]
    pub scopes: SetField<PrimField<String>>,
}

impl BuildComputeRegionInstanceTemplateServiceAccountEl {
    pub fn build(self) -> ComputeRegionInstanceTemplateServiceAccountEl {
        ComputeRegionInstanceTemplateServiceAccountEl {
            email: core::default::Default::default(),
            scopes: self.scopes,
        }
    }
}

pub struct ComputeRegionInstanceTemplateServiceAccountElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeRegionInstanceTemplateServiceAccountElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionInstanceTemplateServiceAccountElRef {
        ComputeRegionInstanceTemplateServiceAccountElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceTemplateServiceAccountElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `email` after provisioning.\nThe service account e-mail address. If not given, the default Google Compute Engine service account is used."]
    pub fn email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc= "Get a reference to the value of field `scopes` after provisioning.\nA list of service scopes. Both OAuth2 URLs and gcloud short names are supported. To allow full access to all Cloud APIs, use the cloud-platform scope."]
    pub fn scopes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceTemplateShieldedInstanceConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_integrity_monitoring: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_secure_boot: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_vtpm: Option<PrimField<bool>>,
}

impl ComputeRegionInstanceTemplateShieldedInstanceConfigEl {
    #[doc= "Set the field `enable_integrity_monitoring`.\nCompare the most recent boot measurements to the integrity policy baseline and return a pair of pass/fail results depending on whether they match or not. Defaults to true."]
    pub fn set_enable_integrity_monitoring(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_integrity_monitoring = Some(v.into());
        self
    }

    #[doc= "Set the field `enable_secure_boot`.\nVerify the digital signature of all boot components, and halt the boot process if signature verification fails. Defaults to false."]
    pub fn set_enable_secure_boot(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_secure_boot = Some(v.into());
        self
    }

    #[doc= "Set the field `enable_vtpm`.\nUse a virtualized trusted platform module, which is a specialized computer chip you can use to encrypt objects like keys and certificates. Defaults to true."]
    pub fn set_enable_vtpm(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_vtpm = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeRegionInstanceTemplateShieldedInstanceConfigEl {
    type O = BlockAssignable<ComputeRegionInstanceTemplateShieldedInstanceConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceTemplateShieldedInstanceConfigEl {}

impl BuildComputeRegionInstanceTemplateShieldedInstanceConfigEl {
    pub fn build(self) -> ComputeRegionInstanceTemplateShieldedInstanceConfigEl {
        ComputeRegionInstanceTemplateShieldedInstanceConfigEl {
            enable_integrity_monitoring: core::default::Default::default(),
            enable_secure_boot: core::default::Default::default(),
            enable_vtpm: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionInstanceTemplateShieldedInstanceConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeRegionInstanceTemplateShieldedInstanceConfigElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionInstanceTemplateShieldedInstanceConfigElRef {
        ComputeRegionInstanceTemplateShieldedInstanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceTemplateShieldedInstanceConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enable_integrity_monitoring` after provisioning.\nCompare the most recent boot measurements to the integrity policy baseline and return a pair of pass/fail results depending on whether they match or not. Defaults to true."]
    pub fn enable_integrity_monitoring(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_integrity_monitoring", self.base))
    }

    #[doc= "Get a reference to the value of field `enable_secure_boot` after provisioning.\nVerify the digital signature of all boot components, and halt the boot process if signature verification fails. Defaults to false."]
    pub fn enable_secure_boot(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_secure_boot", self.base))
    }

    #[doc= "Get a reference to the value of field `enable_vtpm` after provisioning.\nUse a virtualized trusted platform module, which is a specialized computer chip you can use to encrypt objects like keys and certificates. Defaults to true."]
    pub fn enable_vtpm(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_vtpm", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeRegionInstanceTemplateTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl ComputeRegionInstanceTemplateTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc= "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeRegionInstanceTemplateTimeoutsEl {
    type O = BlockAssignable<ComputeRegionInstanceTemplateTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeRegionInstanceTemplateTimeoutsEl {}

impl BuildComputeRegionInstanceTemplateTimeoutsEl {
    pub fn build(self) -> ComputeRegionInstanceTemplateTimeoutsEl {
        ComputeRegionInstanceTemplateTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct ComputeRegionInstanceTemplateTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeRegionInstanceTemplateTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeRegionInstanceTemplateTimeoutsElRef {
        ComputeRegionInstanceTemplateTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeRegionInstanceTemplateTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc= "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeRegionInstanceTemplateDynamic {
    advanced_machine_features: Option<DynamicBlock<ComputeRegionInstanceTemplateAdvancedMachineFeaturesEl>>,
    confidential_instance_config: Option<DynamicBlock<ComputeRegionInstanceTemplateConfidentialInstanceConfigEl>>,
    disk: Option<DynamicBlock<ComputeRegionInstanceTemplateDiskEl>>,
    guest_accelerator: Option<DynamicBlock<ComputeRegionInstanceTemplateGuestAcceleratorEl>>,
    network_interface: Option<DynamicBlock<ComputeRegionInstanceTemplateNetworkInterfaceEl>>,
    network_performance_config: Option<DynamicBlock<ComputeRegionInstanceTemplateNetworkPerformanceConfigEl>>,
    reservation_affinity: Option<DynamicBlock<ComputeRegionInstanceTemplateReservationAffinityEl>>,
    scheduling: Option<DynamicBlock<ComputeRegionInstanceTemplateSchedulingEl>>,
    service_account: Option<DynamicBlock<ComputeRegionInstanceTemplateServiceAccountEl>>,
    shielded_instance_config: Option<DynamicBlock<ComputeRegionInstanceTemplateShieldedInstanceConfigEl>>,
}
