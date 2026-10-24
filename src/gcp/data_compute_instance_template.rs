use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataComputeInstanceTemplateData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    most_recent: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    self_link_unique: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataComputeInstanceTemplate_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataComputeInstanceTemplateData>,
}

#[derive(Clone)]
pub struct DataComputeInstanceTemplate(std::rc::Rc<DataComputeInstanceTemplate_>);

impl DataComputeInstanceTemplate {
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

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `most_recent`.\n"]
    pub fn set_most_recent(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().most_recent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `name`.\nThe name of the instance template. If you leave this blank, Terraform will auto-generate a unique name."]
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

    #[doc = "Set the field `self_link_unique`.\n"]
    pub fn set_self_link_unique(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().self_link_unique = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `advanced_machine_features` after provisioning.\nControls for advanced machine-related behavior features."]
    pub fn advanced_machine_features(&self) -> terrars::ListRef<DataComputeInstanceTemplateAdvancedMachineFeaturesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_machine_features", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `can_ip_forward` after provisioning.\nWhether to allow sending and receiving of packets with non-matching source or destination IPs. This defaults to false."]
    pub fn can_ip_forward(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.can_ip_forward", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `confidential_instance_config` after provisioning.\nThe Confidential VM config being used by the instance. on_host_maintenance has to be set to TERMINATE or this will fail to create."]
    pub fn confidential_instance_config(
        &self,
    ) -> terrars::ListRef<DataComputeInstanceTemplateConfidentialInstanceConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.confidential_instance_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA brief description of this resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disk` after provisioning.\nDisks to attach to instances created from this template. This can be specified multiple times for multiple disks."]
    pub fn disk(&self) -> terrars::ListRef<DataComputeInstanceTemplateDiskElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disk", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `guest_accelerator` after provisioning.\nList of the type and count of accelerator cards attached to the instance."]
    pub fn guest_accelerator(&self) -> terrars::ListRef<DataComputeInstanceTemplateGuestAcceleratorElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.guest_accelerator", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_description` after provisioning.\nA description of the instance."]
    pub fn instance_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to instances created from this template.\n\t\t\t\t\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `machine_type` after provisioning.\nThe machine type to create. To create a machine with a custom type (such as extended memory), format the value like custom-VCPUS-MEM_IN_MB like custom-6-20480 for 6 vCPU and 20GB of RAM."]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `metadata` after provisioning.\nMetadata key/value pairs to make available from within instances created from this template."]
    pub fn metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.metadata", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `metadata_fingerprint` after provisioning.\nThe unique fingerprint of the metadata."]
    pub fn metadata_fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata_fingerprint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `metadata_startup_script` after provisioning.\nAn alternative to using the startup-script metadata key, mostly to match the compute_instance resource. This replaces the startup-script metadata key on the created instance and thus the two mechanisms are not allowed to be used simultaneously."]
    pub fn metadata_startup_script(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata_startup_script", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `min_cpu_platform` after provisioning.\nSpecifies a minimum CPU platform. Applicable values are the friendly names of CPU platforms, such as Intel Haswell or Intel Skylake."]
    pub fn min_cpu_platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_cpu_platform", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `most_recent` after provisioning.\n"]
    pub fn most_recent(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.most_recent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the instance template. If you leave this blank, Terraform will auto-generate a unique name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name_prefix` after provisioning.\nCreates a unique name beginning with the specified prefix. Conflicts with name."]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network_interface` after provisioning.\nNetworks to attach to instances created from this template. This can be specified multiple times for multiple networks."]
    pub fn network_interface(&self) -> terrars::ListRef<DataComputeInstanceTemplateNetworkInterfaceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_interface", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network_performance_config` after provisioning.\nConfigures network performance settings for the instance. If not specified, the instance will be created with its default network performance configuration."]
    pub fn network_performance_config(
        &self,
    ) -> terrars::ListRef<DataComputeInstanceTemplateNetworkPerformanceConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_performance_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nAn instance template is a global resource that is not bound to a zone or a region. However, you can still specify some regional resources in an instance template, which restricts the template to the region where that resource resides. For example, a custom subnetwork resource is tied to a specific region. Defaults to the region of the Provider if no value is given."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reservation_affinity` after provisioning.\nSpecifies the reservations that this instance can consume from."]
    pub fn reservation_affinity(&self) -> terrars::ListRef<DataComputeInstanceTemplateReservationAffinityElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reservation_affinity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_policies` after provisioning.\nA list of self_links of resource policies to attach to the instance. Currently a max of 1 resource policy is supported."]
    pub fn resource_policies(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_policies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scheduling` after provisioning.\nThe scheduling strategy to use."]
    pub fn scheduling(&self) -> terrars::ListRef<DataComputeInstanceTemplateSchedulingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scheduling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link_unique` after provisioning.\n"]
    pub fn self_link_unique(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link_unique", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nService account to attach to the instance."]
    pub fn service_account(&self) -> terrars::ListRef<DataComputeInstanceTemplateServiceAccountElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shielded_instance_config` after provisioning.\nEnable Shielded VM on this instance. Shielded VM provides verifiable integrity to prevent against malware and rootkits. Defaults to disabled. Note: shielded_instance_config can only be used with boot images with shielded vm support."]
    pub fn shielded_instance_config(&self) -> terrars::ListRef<DataComputeInstanceTemplateShieldedInstanceConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shielded_instance_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\nTags to attach to the instance."]
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
}

impl terrars::Referable for DataComputeInstanceTemplate {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataComputeInstanceTemplate { }

impl terrars::ToListMappable for DataComputeInstanceTemplate {
    type O = terrars::ListRef<DataComputeInstanceTemplateRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataComputeInstanceTemplate_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_compute_instance_template".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataComputeInstanceTemplate {
    pub tf_id: std::string::String,
}

impl BuildDataComputeInstanceTemplate {
    pub fn build(self, stack: &mut terrars::Stack) -> DataComputeInstanceTemplate {
        let out = DataComputeInstanceTemplate(std::rc::Rc::new(DataComputeInstanceTemplate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataComputeInstanceTemplateData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                filter: core::default::Default::default(),
                id: core::default::Default::default(),
                most_recent: core::default::Default::default(),
                name: core::default::Default::default(),
                project: core::default::Default::default(),
                self_link_unique: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataComputeInstanceTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceTemplateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataComputeInstanceTemplateRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `advanced_machine_features` after provisioning.\nControls for advanced machine-related behavior features."]
    pub fn advanced_machine_features(&self) -> terrars::ListRef<DataComputeInstanceTemplateAdvancedMachineFeaturesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_machine_features", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `can_ip_forward` after provisioning.\nWhether to allow sending and receiving of packets with non-matching source or destination IPs. This defaults to false."]
    pub fn can_ip_forward(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.can_ip_forward", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `confidential_instance_config` after provisioning.\nThe Confidential VM config being used by the instance. on_host_maintenance has to be set to TERMINATE or this will fail to create."]
    pub fn confidential_instance_config(
        &self,
    ) -> terrars::ListRef<DataComputeInstanceTemplateConfidentialInstanceConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.confidential_instance_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA brief description of this resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disk` after provisioning.\nDisks to attach to instances created from this template. This can be specified multiple times for multiple disks."]
    pub fn disk(&self) -> terrars::ListRef<DataComputeInstanceTemplateDiskElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disk", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `guest_accelerator` after provisioning.\nList of the type and count of accelerator cards attached to the instance."]
    pub fn guest_accelerator(&self) -> terrars::ListRef<DataComputeInstanceTemplateGuestAcceleratorElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.guest_accelerator", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_description` after provisioning.\nA description of the instance."]
    pub fn instance_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to instances created from this template.\n\t\t\t\t\n\t\t\t\t**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\n\t\t\t\tPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `machine_type` after provisioning.\nThe machine type to create. To create a machine with a custom type (such as extended memory), format the value like custom-VCPUS-MEM_IN_MB like custom-6-20480 for 6 vCPU and 20GB of RAM."]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `metadata` after provisioning.\nMetadata key/value pairs to make available from within instances created from this template."]
    pub fn metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.metadata", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `metadata_fingerprint` after provisioning.\nThe unique fingerprint of the metadata."]
    pub fn metadata_fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata_fingerprint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `metadata_startup_script` after provisioning.\nAn alternative to using the startup-script metadata key, mostly to match the compute_instance resource. This replaces the startup-script metadata key on the created instance and thus the two mechanisms are not allowed to be used simultaneously."]
    pub fn metadata_startup_script(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata_startup_script", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `min_cpu_platform` after provisioning.\nSpecifies a minimum CPU platform. Applicable values are the friendly names of CPU platforms, such as Intel Haswell or Intel Skylake."]
    pub fn min_cpu_platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_cpu_platform", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `most_recent` after provisioning.\n"]
    pub fn most_recent(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.most_recent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the instance template. If you leave this blank, Terraform will auto-generate a unique name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name_prefix` after provisioning.\nCreates a unique name beginning with the specified prefix. Conflicts with name."]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network_interface` after provisioning.\nNetworks to attach to instances created from this template. This can be specified multiple times for multiple networks."]
    pub fn network_interface(&self) -> terrars::ListRef<DataComputeInstanceTemplateNetworkInterfaceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_interface", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network_performance_config` after provisioning.\nConfigures network performance settings for the instance. If not specified, the instance will be created with its default network performance configuration."]
    pub fn network_performance_config(
        &self,
    ) -> terrars::ListRef<DataComputeInstanceTemplateNetworkPerformanceConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_performance_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nAn instance template is a global resource that is not bound to a zone or a region. However, you can still specify some regional resources in an instance template, which restricts the template to the region where that resource resides. For example, a custom subnetwork resource is tied to a specific region. Defaults to the region of the Provider if no value is given."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reservation_affinity` after provisioning.\nSpecifies the reservations that this instance can consume from."]
    pub fn reservation_affinity(&self) -> terrars::ListRef<DataComputeInstanceTemplateReservationAffinityElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reservation_affinity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_policies` after provisioning.\nA list of self_links of resource policies to attach to the instance. Currently a max of 1 resource policy is supported."]
    pub fn resource_policies(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_policies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scheduling` after provisioning.\nThe scheduling strategy to use."]
    pub fn scheduling(&self) -> terrars::ListRef<DataComputeInstanceTemplateSchedulingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scheduling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link_unique` after provisioning.\n"]
    pub fn self_link_unique(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link_unique", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nService account to attach to the instance."]
    pub fn service_account(&self) -> terrars::ListRef<DataComputeInstanceTemplateServiceAccountElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `shielded_instance_config` after provisioning.\nEnable Shielded VM on this instance. Shielded VM provides verifiable integrity to prevent against malware and rootkits. Defaults to disabled. Note: shielded_instance_config can only be used with boot images with shielded vm support."]
    pub fn shielded_instance_config(&self) -> terrars::ListRef<DataComputeInstanceTemplateShieldedInstanceConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shielded_instance_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\nTags to attach to the instance."]
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
}

#[derive(Serialize)]
pub struct DataComputeInstanceTemplateAdvancedMachineFeaturesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_nested_virtualization: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threads_per_core: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    visible_core_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DataComputeInstanceTemplateAdvancedMachineFeaturesEl {
    #[doc = "Set the field `enable_nested_virtualization`.\n"]
    pub fn set_enable_nested_virtualization(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_nested_virtualization = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `threads_per_core`.\n"]
    pub fn set_threads_per_core(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.threads_per_core = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `visible_core_count`.\n"]
    pub fn set_visible_core_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.visible_core_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeInstanceTemplateAdvancedMachineFeaturesEl {
    type O = terrars::BlockAssignable<DataComputeInstanceTemplateAdvancedMachineFeaturesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceTemplateAdvancedMachineFeaturesEl {}

impl BuildDataComputeInstanceTemplateAdvancedMachineFeaturesEl {
    pub fn build(self) -> DataComputeInstanceTemplateAdvancedMachineFeaturesEl {
        DataComputeInstanceTemplateAdvancedMachineFeaturesEl {
            enable_nested_virtualization: core::default::Default::default(),
            threads_per_core: core::default::Default::default(),
            visible_core_count: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceTemplateAdvancedMachineFeaturesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceTemplateAdvancedMachineFeaturesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceTemplateAdvancedMachineFeaturesElRef {
        DataComputeInstanceTemplateAdvancedMachineFeaturesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceTemplateAdvancedMachineFeaturesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_nested_virtualization` after provisioning.\n"]
    pub fn enable_nested_virtualization(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_nested_virtualization", self.base))
    }

    #[doc = "Get a reference to the value of field `threads_per_core` after provisioning.\n"]
    pub fn threads_per_core(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threads_per_core", self.base))
    }

    #[doc = "Get a reference to the value of field `visible_core_count` after provisioning.\n"]
    pub fn visible_core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.visible_core_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceTemplateConfidentialInstanceConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_confidential_compute: std::option::Option<terrars::PrimField<bool>>,
}

impl DataComputeInstanceTemplateConfidentialInstanceConfigEl {
    #[doc = "Set the field `enable_confidential_compute`.\n"]
    pub fn set_enable_confidential_compute(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_confidential_compute = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeInstanceTemplateConfidentialInstanceConfigEl {
    type O = terrars::BlockAssignable<DataComputeInstanceTemplateConfidentialInstanceConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceTemplateConfidentialInstanceConfigEl {}

impl BuildDataComputeInstanceTemplateConfidentialInstanceConfigEl {
    pub fn build(self) -> DataComputeInstanceTemplateConfidentialInstanceConfigEl {
        DataComputeInstanceTemplateConfidentialInstanceConfigEl {
            enable_confidential_compute: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceTemplateConfidentialInstanceConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceTemplateConfidentialInstanceConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceTemplateConfidentialInstanceConfigElRef {
        DataComputeInstanceTemplateConfidentialInstanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceTemplateConfidentialInstanceConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_confidential_compute` after provisioning.\n"]
    pub fn enable_confidential_compute(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_confidential_compute", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceTemplateDiskElDiskEncryptionKeyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_self_link: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeInstanceTemplateDiskElDiskEncryptionKeyEl {
    #[doc = "Set the field `kms_key_self_link`.\n"]
    pub fn set_kms_key_self_link(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_self_link = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeInstanceTemplateDiskElDiskEncryptionKeyEl {
    type O = terrars::BlockAssignable<DataComputeInstanceTemplateDiskElDiskEncryptionKeyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceTemplateDiskElDiskEncryptionKeyEl {}

impl BuildDataComputeInstanceTemplateDiskElDiskEncryptionKeyEl {
    pub fn build(self) -> DataComputeInstanceTemplateDiskElDiskEncryptionKeyEl {
        DataComputeInstanceTemplateDiskElDiskEncryptionKeyEl { kms_key_self_link: core::default::Default::default() }
    }
}

pub struct DataComputeInstanceTemplateDiskElDiskEncryptionKeyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceTemplateDiskElDiskEncryptionKeyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceTemplateDiskElDiskEncryptionKeyElRef {
        DataComputeInstanceTemplateDiskElDiskEncryptionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceTemplateDiskElDiskEncryptionKeyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_self_link` after provisioning.\n"]
    pub fn kms_key_self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_self_link", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceTemplateDiskElSourceImageEncryptionKeyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_self_link: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_service_account: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeInstanceTemplateDiskElSourceImageEncryptionKeyEl {
    #[doc = "Set the field `kms_key_self_link`.\n"]
    pub fn set_kms_key_self_link(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_self_link = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_service_account`.\n"]
    pub fn set_kms_key_service_account(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.kms_key_service_account = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeInstanceTemplateDiskElSourceImageEncryptionKeyEl {
    type O = terrars::BlockAssignable<DataComputeInstanceTemplateDiskElSourceImageEncryptionKeyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceTemplateDiskElSourceImageEncryptionKeyEl {}

impl BuildDataComputeInstanceTemplateDiskElSourceImageEncryptionKeyEl {
    pub fn build(self) -> DataComputeInstanceTemplateDiskElSourceImageEncryptionKeyEl {
        DataComputeInstanceTemplateDiskElSourceImageEncryptionKeyEl {
            kms_key_self_link: core::default::Default::default(),
            kms_key_service_account: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceTemplateDiskElSourceImageEncryptionKeyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceTemplateDiskElSourceImageEncryptionKeyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceTemplateDiskElSourceImageEncryptionKeyElRef {
        DataComputeInstanceTemplateDiskElSourceImageEncryptionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceTemplateDiskElSourceImageEncryptionKeyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_self_link` after provisioning.\n"]
    pub fn kms_key_self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_self_link", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_service_account` after provisioning.\n"]
    pub fn kms_key_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_service_account", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_self_link: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_service_account: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl {
    #[doc = "Set the field `kms_key_self_link`.\n"]
    pub fn set_kms_key_self_link(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_self_link = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_service_account`.\n"]
    pub fn set_kms_key_service_account(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.kms_key_service_account = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl {
    type O = terrars::BlockAssignable<DataComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl {}

impl BuildDataComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl {
    pub fn build(self) -> DataComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl {
        DataComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl {
            kms_key_self_link: core::default::Default::default(),
            kms_key_service_account: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyElRef {
        DataComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_self_link` after provisioning.\n"]
    pub fn kms_key_self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_self_link", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_service_account` after provisioning.\n"]
    pub fn kms_key_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_service_account", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceTemplateDiskEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_delete: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_encryption_key: std::option::Option<terrars::ListField<DataComputeInstanceTemplateDiskElDiskEncryptionKeyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_size_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interface: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisioned_iops: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_policies: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_image: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_image_encryption_key: std
    ::option
    ::Option<terrars::ListField<DataComputeInstanceTemplateDiskElSourceImageEncryptionKeyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_snapshot: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_snapshot_encryption_key: std
    ::option
    ::Option<terrars::ListField<DataComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeInstanceTemplateDiskEl {
    #[doc = "Set the field `auto_delete`.\n"]
    pub fn set_auto_delete(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.auto_delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `boot`.\n"]
    pub fn set_boot(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.boot = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_name`.\n"]
    pub fn set_device_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.device_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk_encryption_key`.\n"]
    pub fn set_disk_encryption_key(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeInstanceTemplateDiskElDiskEncryptionKeyEl>>,
    ) -> Self {
        self.disk_encryption_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk_name`.\n"]
    pub fn set_disk_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk_size_gb`.\n"]
    pub fn set_disk_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.disk_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk_type`.\n"]
    pub fn set_disk_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interface`.\n"]
    pub fn set_interface(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.interface = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `provisioned_iops`.\n"]
    pub fn set_provisioned_iops(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.provisioned_iops = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_policies`.\n"]
    pub fn set_resource_policies(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resource_policies = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_image`.\n"]
    pub fn set_source_image(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_image = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_image_encryption_key`.\n"]
    pub fn set_source_image_encryption_key(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeInstanceTemplateDiskElSourceImageEncryptionKeyEl>>,
    ) -> Self {
        self.source_image_encryption_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_snapshot`.\n"]
    pub fn set_source_snapshot(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_snapshot = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_snapshot_encryption_key`.\n"]
    pub fn set_source_snapshot_encryption_key(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyEl>>,
    ) -> Self {
        self.source_snapshot_encryption_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeInstanceTemplateDiskEl {
    type O = terrars::BlockAssignable<DataComputeInstanceTemplateDiskEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceTemplateDiskEl {}

impl BuildDataComputeInstanceTemplateDiskEl {
    pub fn build(self) -> DataComputeInstanceTemplateDiskEl {
        DataComputeInstanceTemplateDiskEl {
            auto_delete: core::default::Default::default(),
            boot: core::default::Default::default(),
            device_name: core::default::Default::default(),
            disk_encryption_key: core::default::Default::default(),
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
            source_image_encryption_key: core::default::Default::default(),
            source_snapshot: core::default::Default::default(),
            source_snapshot_encryption_key: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceTemplateDiskElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceTemplateDiskElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeInstanceTemplateDiskElRef {
        DataComputeInstanceTemplateDiskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceTemplateDiskElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_delete` after provisioning.\n"]
    pub fn auto_delete(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_delete", self.base))
    }

    #[doc = "Get a reference to the value of field `boot` after provisioning.\n"]
    pub fn boot(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boot", self.base))
    }

    #[doc = "Get a reference to the value of field `device_name` after provisioning.\n"]
    pub fn device_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }

    #[doc = "Get a reference to the value of field `disk_encryption_key` after provisioning.\n"]
    pub fn disk_encryption_key(&self) -> terrars::ListRef<DataComputeInstanceTemplateDiskElDiskEncryptionKeyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disk_encryption_key", self.base))
    }

    #[doc = "Get a reference to the value of field `disk_name` after provisioning.\n"]
    pub fn disk_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_name", self.base))
    }

    #[doc = "Get a reference to the value of field `disk_size_gb` after provisioning.\n"]
    pub fn disk_size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `disk_type` after provisioning.\n"]
    pub fn disk_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }

    #[doc = "Get a reference to the value of field `interface` after provisioning.\n"]
    pub fn interface(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interface", self.base))
    }

    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `provisioned_iops` after provisioning.\n"]
    pub fn provisioned_iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioned_iops", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_policies` after provisioning.\n"]
    pub fn resource_policies(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_policies", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc = "Get a reference to the value of field `source_image` after provisioning.\n"]
    pub fn source_image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_image", self.base))
    }

    #[doc = "Get a reference to the value of field `source_image_encryption_key` after provisioning.\n"]
    pub fn source_image_encryption_key(
        &self,
    ) -> terrars::ListRef<DataComputeInstanceTemplateDiskElSourceImageEncryptionKeyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_image_encryption_key", self.base))
    }

    #[doc = "Get a reference to the value of field `source_snapshot` after provisioning.\n"]
    pub fn source_snapshot(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_snapshot", self.base))
    }

    #[doc = "Get a reference to the value of field `source_snapshot_encryption_key` after provisioning.\n"]
    pub fn source_snapshot_encryption_key(
        &self,
    ) -> terrars::ListRef<DataComputeInstanceTemplateDiskElSourceSnapshotEncryptionKeyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_snapshot_encryption_key", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceTemplateGuestAcceleratorEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeInstanceTemplateGuestAcceleratorEl {
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

impl terrars::ToListMappable for DataComputeInstanceTemplateGuestAcceleratorEl {
    type O = terrars::BlockAssignable<DataComputeInstanceTemplateGuestAcceleratorEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceTemplateGuestAcceleratorEl {}

impl BuildDataComputeInstanceTemplateGuestAcceleratorEl {
    pub fn build(self) -> DataComputeInstanceTemplateGuestAcceleratorEl {
        DataComputeInstanceTemplateGuestAcceleratorEl {
            count: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceTemplateGuestAcceleratorElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceTemplateGuestAcceleratorElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceTemplateGuestAcceleratorElRef {
        DataComputeInstanceTemplateGuestAcceleratorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceTemplateGuestAcceleratorElRef {
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
pub struct DataComputeInstanceTemplateNetworkInterfaceElAccessConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nat_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_tier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_ptr_domain_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeInstanceTemplateNetworkInterfaceElAccessConfigEl {
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

impl terrars::ToListMappable for DataComputeInstanceTemplateNetworkInterfaceElAccessConfigEl {
    type O = terrars::BlockAssignable<DataComputeInstanceTemplateNetworkInterfaceElAccessConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceTemplateNetworkInterfaceElAccessConfigEl {}

impl BuildDataComputeInstanceTemplateNetworkInterfaceElAccessConfigEl {
    pub fn build(self) -> DataComputeInstanceTemplateNetworkInterfaceElAccessConfigEl {
        DataComputeInstanceTemplateNetworkInterfaceElAccessConfigEl {
            nat_ip: core::default::Default::default(),
            network_tier: core::default::Default::default(),
            public_ptr_domain_name: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceTemplateNetworkInterfaceElAccessConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceTemplateNetworkInterfaceElAccessConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceTemplateNetworkInterfaceElAccessConfigElRef {
        DataComputeInstanceTemplateNetworkInterfaceElAccessConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceTemplateNetworkInterfaceElAccessConfigElRef {
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
pub struct DataComputeInstanceTemplateNetworkInterfaceElAliasIpRangeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_cidr_range: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork_range_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeInstanceTemplateNetworkInterfaceElAliasIpRangeEl {
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

impl terrars::ToListMappable for DataComputeInstanceTemplateNetworkInterfaceElAliasIpRangeEl {
    type O = terrars::BlockAssignable<DataComputeInstanceTemplateNetworkInterfaceElAliasIpRangeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceTemplateNetworkInterfaceElAliasIpRangeEl {}

impl BuildDataComputeInstanceTemplateNetworkInterfaceElAliasIpRangeEl {
    pub fn build(self) -> DataComputeInstanceTemplateNetworkInterfaceElAliasIpRangeEl {
        DataComputeInstanceTemplateNetworkInterfaceElAliasIpRangeEl {
            ip_cidr_range: core::default::Default::default(),
            subnetwork_range_name: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceTemplateNetworkInterfaceElAliasIpRangeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceTemplateNetworkInterfaceElAliasIpRangeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceTemplateNetworkInterfaceElAliasIpRangeElRef {
        DataComputeInstanceTemplateNetworkInterfaceElAliasIpRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceTemplateNetworkInterfaceElAliasIpRangeElRef {
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
pub struct DataComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_ipv6: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_ipv6_prefix_length: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_tier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_ptr_domain_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl {
    #[doc = "Set the field `external_ipv6`.\n"]
    pub fn set_external_ipv6(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.external_ipv6 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `external_ipv6_prefix_length`.\n"]
    pub fn set_external_ipv6_prefix_length(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.external_ipv6_prefix_length = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for DataComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl {
    type O = terrars::BlockAssignable<DataComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl {}

impl BuildDataComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl {
    pub fn build(self) -> DataComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl {
        DataComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl {
            external_ipv6: core::default::Default::default(),
            external_ipv6_prefix_length: core::default::Default::default(),
            name: core::default::Default::default(),
            network_tier: core::default::Default::default(),
            public_ptr_domain_name: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigElRef {
        DataComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `external_ipv6` after provisioning.\n"]
    pub fn external_ipv6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_ipv6", self.base))
    }

    #[doc = "Get a reference to the value of field `external_ipv6_prefix_length` after provisioning.\n"]
    pub fn external_ipv6_prefix_length(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_ipv6_prefix_length", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
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
pub struct DataComputeInstanceTemplateNetworkInterfaceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_config: std::option::Option<terrars::ListField<DataComputeInstanceTemplateNetworkInterfaceElAccessConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alias_ip_range: std::option::Option<terrars::ListField<DataComputeInstanceTemplateNetworkInterfaceElAliasIpRangeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internal_ipv6_prefix_length: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_access_config: std
    ::option
    ::Option<terrars::ListField<DataComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_access_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
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
}

impl DataComputeInstanceTemplateNetworkInterfaceEl {
    #[doc = "Set the field `access_config`.\n"]
    pub fn set_access_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeInstanceTemplateNetworkInterfaceElAccessConfigEl>>,
    ) -> Self {
        self.access_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `alias_ip_range`.\n"]
    pub fn set_alias_ip_range(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeInstanceTemplateNetworkInterfaceElAliasIpRangeEl>>,
    ) -> Self {
        self.alias_ip_range = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `internal_ipv6_prefix_length`.\n"]
    pub fn set_internal_ipv6_prefix_length(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.internal_ipv6_prefix_length = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_access_config`.\n"]
    pub fn set_ipv6_access_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigEl>>,
    ) -> Self {
        self.ipv6_access_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_access_type`.\n"]
    pub fn set_ipv6_access_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ipv6_access_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_address`.\n"]
    pub fn set_ipv6_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ipv6_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_ip`.\n"]
    pub fn set_network_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nic_type`.\n"]
    pub fn set_nic_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.nic_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `queue_count`.\n"]
    pub fn set_queue_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.queue_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stack_type`.\n"]
    pub fn set_stack_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.stack_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnetwork`.\n"]
    pub fn set_subnetwork(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnetwork = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnetwork_project`.\n"]
    pub fn set_subnetwork_project(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.subnetwork_project = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeInstanceTemplateNetworkInterfaceEl {
    type O = terrars::BlockAssignable<DataComputeInstanceTemplateNetworkInterfaceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceTemplateNetworkInterfaceEl {}

impl BuildDataComputeInstanceTemplateNetworkInterfaceEl {
    pub fn build(self) -> DataComputeInstanceTemplateNetworkInterfaceEl {
        DataComputeInstanceTemplateNetworkInterfaceEl {
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

pub struct DataComputeInstanceTemplateNetworkInterfaceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceTemplateNetworkInterfaceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceTemplateNetworkInterfaceElRef {
        DataComputeInstanceTemplateNetworkInterfaceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceTemplateNetworkInterfaceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_config` after provisioning.\n"]
    pub fn access_config(&self) -> terrars::ListRef<DataComputeInstanceTemplateNetworkInterfaceElAccessConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_config", self.base))
    }

    #[doc = "Get a reference to the value of field `alias_ip_range` after provisioning.\n"]
    pub fn alias_ip_range(&self) -> terrars::ListRef<DataComputeInstanceTemplateNetworkInterfaceElAliasIpRangeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alias_ip_range", self.base))
    }

    #[doc = "Get a reference to the value of field `internal_ipv6_prefix_length` after provisioning.\n"]
    pub fn internal_ipv6_prefix_length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.internal_ipv6_prefix_length", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv6_access_config` after provisioning.\n"]
    pub fn ipv6_access_config(
        &self,
    ) -> terrars::ListRef<DataComputeInstanceTemplateNetworkInterfaceElIpv6AccessConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ipv6_access_config", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv6_access_type` after provisioning.\n"]
    pub fn ipv6_access_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_access_type", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv6_address` after provisioning.\n"]
    pub fn ipv6_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_address", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc = "Get a reference to the value of field `network_ip` after provisioning.\n"]
    pub fn network_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `nic_type` after provisioning.\n"]
    pub fn nic_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nic_type", self.base))
    }

    #[doc = "Get a reference to the value of field `queue_count` after provisioning.\n"]
    pub fn queue_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queue_count", self.base))
    }

    #[doc = "Get a reference to the value of field `stack_type` after provisioning.\n"]
    pub fn stack_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stack_type", self.base))
    }

    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\n"]
    pub fn subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }

    #[doc = "Get a reference to the value of field `subnetwork_project` after provisioning.\n"]
    pub fn subnetwork_project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork_project", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceTemplateNetworkPerformanceConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_egress_bandwidth_tier: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeInstanceTemplateNetworkPerformanceConfigEl {
    #[doc = "Set the field `total_egress_bandwidth_tier`.\n"]
    pub fn set_total_egress_bandwidth_tier(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.total_egress_bandwidth_tier = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeInstanceTemplateNetworkPerformanceConfigEl {
    type O = terrars::BlockAssignable<DataComputeInstanceTemplateNetworkPerformanceConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceTemplateNetworkPerformanceConfigEl {}

impl BuildDataComputeInstanceTemplateNetworkPerformanceConfigEl {
    pub fn build(self) -> DataComputeInstanceTemplateNetworkPerformanceConfigEl {
        DataComputeInstanceTemplateNetworkPerformanceConfigEl {
            total_egress_bandwidth_tier: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceTemplateNetworkPerformanceConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceTemplateNetworkPerformanceConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceTemplateNetworkPerformanceConfigElRef {
        DataComputeInstanceTemplateNetworkPerformanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceTemplateNetworkPerformanceConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `total_egress_bandwidth_tier` after provisioning.\n"]
    pub fn total_egress_bandwidth_tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_egress_bandwidth_tier", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceTemplateReservationAffinityElSpecificReservationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataComputeInstanceTemplateReservationAffinityElSpecificReservationEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeInstanceTemplateReservationAffinityElSpecificReservationEl {
    type O = terrars::BlockAssignable<DataComputeInstanceTemplateReservationAffinityElSpecificReservationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceTemplateReservationAffinityElSpecificReservationEl {}

impl BuildDataComputeInstanceTemplateReservationAffinityElSpecificReservationEl {
    pub fn build(self) -> DataComputeInstanceTemplateReservationAffinityElSpecificReservationEl {
        DataComputeInstanceTemplateReservationAffinityElSpecificReservationEl {
            key: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceTemplateReservationAffinityElSpecificReservationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceTemplateReservationAffinityElSpecificReservationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceTemplateReservationAffinityElSpecificReservationElRef {
        DataComputeInstanceTemplateReservationAffinityElSpecificReservationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceTemplateReservationAffinityElSpecificReservationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceTemplateReservationAffinityEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    specific_reservation: std
    ::option
    ::Option<terrars::ListField<DataComputeInstanceTemplateReservationAffinityElSpecificReservationEl>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeInstanceTemplateReservationAffinityEl {
    #[doc = "Set the field `specific_reservation`.\n"]
    pub fn set_specific_reservation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataComputeInstanceTemplateReservationAffinityElSpecificReservationEl>>,
    ) -> Self {
        self.specific_reservation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeInstanceTemplateReservationAffinityEl {
    type O = terrars::BlockAssignable<DataComputeInstanceTemplateReservationAffinityEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceTemplateReservationAffinityEl {}

impl BuildDataComputeInstanceTemplateReservationAffinityEl {
    pub fn build(self) -> DataComputeInstanceTemplateReservationAffinityEl {
        DataComputeInstanceTemplateReservationAffinityEl {
            specific_reservation: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceTemplateReservationAffinityElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceTemplateReservationAffinityElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceTemplateReservationAffinityElRef {
        DataComputeInstanceTemplateReservationAffinityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceTemplateReservationAffinityElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `specific_reservation` after provisioning.\n"]
    pub fn specific_reservation(
        &self,
    ) -> terrars::ListRef<DataComputeInstanceTemplateReservationAffinityElSpecificReservationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.specific_reservation", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl DataComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl {
    #[doc = "Set the field `nanos`.\n"]
    pub fn set_nanos(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.nanos = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `seconds`.\n"]
    pub fn set_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl {
    type O = terrars::BlockAssignable<DataComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl {}

impl BuildDataComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl {
    pub fn build(self) -> DataComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl {
        DataComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl {
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutElRef {
        DataComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `nanos` after provisioning.\n"]
    pub fn nanos(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nanos", self.base))
    }

    #[doc = "Get a reference to the value of field `seconds` after provisioning.\n"]
    pub fn seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceTemplateSchedulingElNodeAffinitiesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataComputeInstanceTemplateSchedulingElNodeAffinitiesEl {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operator`.\n"]
    pub fn set_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeInstanceTemplateSchedulingElNodeAffinitiesEl {
    type O = terrars::BlockAssignable<DataComputeInstanceTemplateSchedulingElNodeAffinitiesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceTemplateSchedulingElNodeAffinitiesEl {}

impl BuildDataComputeInstanceTemplateSchedulingElNodeAffinitiesEl {
    pub fn build(self) -> DataComputeInstanceTemplateSchedulingElNodeAffinitiesEl {
        DataComputeInstanceTemplateSchedulingElNodeAffinitiesEl {
            key: core::default::Default::default(),
            operator: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceTemplateSchedulingElNodeAffinitiesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceTemplateSchedulingElNodeAffinitiesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceTemplateSchedulingElNodeAffinitiesElRef {
        DataComputeInstanceTemplateSchedulingElNodeAffinitiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceTemplateSchedulingElNodeAffinitiesElRef {
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
pub struct DataComputeInstanceTemplateSchedulingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automatic_restart: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_termination_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_ssd_recovery_timeout: std
    ::option
    ::Option<terrars::ListField<DataComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_node_cpus: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_affinities: std::option::Option<terrars::SetField<DataComputeInstanceTemplateSchedulingElNodeAffinitiesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_host_maintenance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preemptible: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provisioning_model: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeInstanceTemplateSchedulingEl {
    #[doc = "Set the field `automatic_restart`.\n"]
    pub fn set_automatic_restart(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.automatic_restart = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_termination_action`.\n"]
    pub fn set_instance_termination_action(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.instance_termination_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_ssd_recovery_timeout`.\n"]
    pub fn set_local_ssd_recovery_timeout(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutEl>>,
    ) -> Self {
        self.local_ssd_recovery_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_node_cpus`.\n"]
    pub fn set_min_node_cpus(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_node_cpus = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_affinities`.\n"]
    pub fn set_node_affinities(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataComputeInstanceTemplateSchedulingElNodeAffinitiesEl>>,
    ) -> Self {
        self.node_affinities = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `on_host_maintenance`.\n"]
    pub fn set_on_host_maintenance(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.on_host_maintenance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preemptible`.\n"]
    pub fn set_preemptible(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.preemptible = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `provisioning_model`.\n"]
    pub fn set_provisioning_model(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.provisioning_model = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeInstanceTemplateSchedulingEl {
    type O = terrars::BlockAssignable<DataComputeInstanceTemplateSchedulingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceTemplateSchedulingEl {}

impl BuildDataComputeInstanceTemplateSchedulingEl {
    pub fn build(self) -> DataComputeInstanceTemplateSchedulingEl {
        DataComputeInstanceTemplateSchedulingEl {
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

pub struct DataComputeInstanceTemplateSchedulingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceTemplateSchedulingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeInstanceTemplateSchedulingElRef {
        DataComputeInstanceTemplateSchedulingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceTemplateSchedulingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `automatic_restart` after provisioning.\n"]
    pub fn automatic_restart(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automatic_restart", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_termination_action` after provisioning.\n"]
    pub fn instance_termination_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_termination_action", self.base))
    }

    #[doc = "Get a reference to the value of field `local_ssd_recovery_timeout` after provisioning.\n"]
    pub fn local_ssd_recovery_timeout(
        &self,
    ) -> terrars::ListRef<DataComputeInstanceTemplateSchedulingElLocalSsdRecoveryTimeoutElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.local_ssd_recovery_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `min_node_cpus` after provisioning.\n"]
    pub fn min_node_cpus(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_node_cpus", self.base))
    }

    #[doc = "Get a reference to the value of field `node_affinities` after provisioning.\n"]
    pub fn node_affinities(&self) -> terrars::SetRef<DataComputeInstanceTemplateSchedulingElNodeAffinitiesElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.node_affinities", self.base))
    }

    #[doc = "Get a reference to the value of field `on_host_maintenance` after provisioning.\n"]
    pub fn on_host_maintenance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.on_host_maintenance", self.base))
    }

    #[doc = "Get a reference to the value of field `preemptible` after provisioning.\n"]
    pub fn preemptible(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preemptible", self.base))
    }

    #[doc = "Get a reference to the value of field `provisioning_model` after provisioning.\n"]
    pub fn provisioning_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioning_model", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeInstanceTemplateServiceAccountEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scopes: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataComputeInstanceTemplateServiceAccountEl {
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

impl terrars::ToListMappable for DataComputeInstanceTemplateServiceAccountEl {
    type O = terrars::BlockAssignable<DataComputeInstanceTemplateServiceAccountEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceTemplateServiceAccountEl {}

impl BuildDataComputeInstanceTemplateServiceAccountEl {
    pub fn build(self) -> DataComputeInstanceTemplateServiceAccountEl {
        DataComputeInstanceTemplateServiceAccountEl {
            email: core::default::Default::default(),
            scopes: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceTemplateServiceAccountElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceTemplateServiceAccountElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceTemplateServiceAccountElRef {
        DataComputeInstanceTemplateServiceAccountElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceTemplateServiceAccountElRef {
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
pub struct DataComputeInstanceTemplateShieldedInstanceConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_integrity_monitoring: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_secure_boot: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_vtpm: std::option::Option<terrars::PrimField<bool>>,
}

impl DataComputeInstanceTemplateShieldedInstanceConfigEl {
    #[doc = "Set the field `enable_integrity_monitoring`.\n"]
    pub fn set_enable_integrity_monitoring(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_integrity_monitoring = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_secure_boot`.\n"]
    pub fn set_enable_secure_boot(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_secure_boot = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_vtpm`.\n"]
    pub fn set_enable_vtpm(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_vtpm = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeInstanceTemplateShieldedInstanceConfigEl {
    type O = terrars::BlockAssignable<DataComputeInstanceTemplateShieldedInstanceConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeInstanceTemplateShieldedInstanceConfigEl {}

impl BuildDataComputeInstanceTemplateShieldedInstanceConfigEl {
    pub fn build(self) -> DataComputeInstanceTemplateShieldedInstanceConfigEl {
        DataComputeInstanceTemplateShieldedInstanceConfigEl {
            enable_integrity_monitoring: core::default::Default::default(),
            enable_secure_boot: core::default::Default::default(),
            enable_vtpm: core::default::Default::default(),
        }
    }
}

pub struct DataComputeInstanceTemplateShieldedInstanceConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeInstanceTemplateShieldedInstanceConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeInstanceTemplateShieldedInstanceConfigElRef {
        DataComputeInstanceTemplateShieldedInstanceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeInstanceTemplateShieldedInstanceConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_integrity_monitoring` after provisioning.\n"]
    pub fn enable_integrity_monitoring(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_integrity_monitoring", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_secure_boot` after provisioning.\n"]
    pub fn enable_secure_boot(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_secure_boot", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_vtpm` after provisioning.\n"]
    pub fn enable_vtpm(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_vtpm", self.base))
    }
}
