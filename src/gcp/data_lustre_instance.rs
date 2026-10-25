#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataLustreInstanceData_ {
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
    instance_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataLustreInstance_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataLustreInstanceData_>,
}

#[derive(Clone)]
pub struct DataLustreInstance(std::rc::Rc<DataLustreInstance_>);

impl DataLustreInstance {
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

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone`.\nZone of Lustre instance"]
    pub fn set_zone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `access_rules_options` after provisioning.\nIP-based access rules for the Managed Lustre instance. These options\ndefine the root user squash configuration."]
    pub fn access_rules_options(&self) -> terrars::ListRef<DataLustreInstance_AccessRulesOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_rules_options", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `available_version` after provisioning.\nThe version this instance can be upgraded to, if one is available. Empty\nwhen the instance is already running the newest release."]
    pub fn available_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `capacity_gib` after provisioning.\nThe storage capacity of the instance in gibibytes (GiB). Allowed values\nare from '9000' to '7632000', depending on the 'perUnitStorageThroughput'.\nSee [Performance tiers and maximum storage\ncapacities](https://cloud.google.com/managed-lustre/docs/create-instance#performance-tiers)\nfor specific minimums, maximums, and step sizes for each performance tier."]
    pub fn capacity_gib(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_gib", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the instance was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA user-readable description of the instance."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dynamic_tier_options` after provisioning.\nDynamic tier options for a Managed Lustre instance."]
    pub fn dynamic_tier_options(&self) -> terrars::ListRef<DataLustreInstance_DynamicTierOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dynamic_tier_options", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_version` after provisioning.\nThe version of Managed Lustre software that this instance is currently\nrunning."]
    pub fn effective_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effective_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `filesystem` after provisioning.\nThe filesystem name for this instance. This name is used by client-side\ntools, including when mounting the instance. Must be eight characters or\nless and can only contain letters and numbers."]
    pub fn filesystem(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filesystem", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gke_support_enabled` after provisioning.\nIndicates whether you want to enable support for GKE clients. By default,\nGKE clients are not supported."]
    pub fn gke_support_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gke_support_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_id` after provisioning.\nThe name of the Managed Lustre instance.\n\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter."]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key` after provisioning.\nThe Cloud KMS key name to use for data encryption.\nIf not set, the instance will use Google-managed encryption keys.\nIf set, the instance will use customer-managed encryption keys.\nThe key must be in the same region as the instance.\nThe key format is:\nprojects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{key}"]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maintenance_policy` after provisioning.\nDefines a maintenance policy for a resource."]
    pub fn maintenance_policy(&self) -> terrars::ListRef<DataLustreInstance_MaintenancePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mount_point` after provisioning.\nMount point of the instance in the format 'IP_ADDRESS@tcp:/FILESYSTEM'."]
    pub fn mount_point(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mount_point", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the instance."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe full name of the VPC network to which the instance is connected.\nMust be in the format\n'projects/{project_id}/global/networks/{network_name}'."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `per_unit_storage_throughput` after provisioning.\nThe throughput of the instance in MBps per TiB. Valid values are 125, 250,\n500, 1000.\nSee [Performance tiers and maximum storage\ncapacities](https://cloud.google.com/managed-lustre/docs/create-instance#performance-tiers)\nfor more information.\n\nIf the instance is using the Dynamic tier, this field must not be set or\nmust be set to zero."]
    pub fn per_unit_storage_throughput(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.per_unit_storage_throughput", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `placement_policy` after provisioning.\nThe placement policy name for the instance in the format of\nprojects/{project}/locations/{location}/resourcePolicies/{resource_policy}"]
    pub fn placement_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.placement_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe state of the instance.\nPossible values:\nACTIVE\nCREATING\nDELETING\nUPGRADING\nREPAIRING\nSTOPPED\nUPDATING\nSUSPENDED"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state_reason` after provisioning.\nThe reason why the instance is in a certain state (e.g. SUSPENDED)."]
    pub fn state_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_reason", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_version` after provisioning.\nThe version to upgrade this instance to. Set this to the value reported in\n'availableVersion', or to 'latest' to move to the newest version available\nat the time of the upgrade.\nThis field cannot be set when the instance is created; new instances are\nalways provisioned from the current release. It also cannot be changed in\nthe same operation as 'capacityGib' or 'maintenancePolicy', and the\ninstance must be ACTIVE and outside of the hour preceding a scheduled\nmaintenance window.\nThe API clears this field once the upgrade finishes, so it always reads\nback as empty on an idle instance."]
    pub fn target_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nUnique ID of the resource.\nThis is unrelated to the access rules which allow specifying the root\nsquash uid."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `upcoming_maintenance_schedule` after provisioning.\nRepresents a scheduled maintenance event."]
    pub fn upcoming_maintenance_schedule(&self) -> terrars::ListRef<DataLustreInstance_UpcomingMaintenanceScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upcoming_maintenance_schedule", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when the instance was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone` after provisioning.\nZone of Lustre instance"]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }
}

impl terrars::Referable for DataLustreInstance {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataLustreInstance { }

impl terrars::ToListMappable for DataLustreInstance {
    type O = terrars::ListRef<DataLustreInstanceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataLustreInstance_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_lustre_instance".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataLustreInstance {
    pub tf_id: std::string::String,
    #[doc =
        "The name of the Managed Lustre instance.\n\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter."]
    pub instance_id: terrars::PrimField<std::string::String>,
}

impl BuildDataLustreInstance {
    pub fn build(self, stack: &mut terrars::Stack) -> DataLustreInstance {
        let out = DataLustreInstance(std::rc::Rc::new(DataLustreInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataLustreInstanceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                project: core::default::Default::default(),
                zone: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataLustreInstanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLustreInstanceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataLustreInstanceRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `access_rules_options` after provisioning.\nIP-based access rules for the Managed Lustre instance. These options\ndefine the root user squash configuration."]
    pub fn access_rules_options(&self) -> terrars::ListRef<DataLustreInstance_AccessRulesOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_rules_options", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `available_version` after provisioning.\nThe version this instance can be upgraded to, if one is available. Empty\nwhen the instance is already running the newest release."]
    pub fn available_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `capacity_gib` after provisioning.\nThe storage capacity of the instance in gibibytes (GiB). Allowed values\nare from '9000' to '7632000', depending on the 'perUnitStorageThroughput'.\nSee [Performance tiers and maximum storage\ncapacities](https://cloud.google.com/managed-lustre/docs/create-instance#performance-tiers)\nfor specific minimums, maximums, and step sizes for each performance tier."]
    pub fn capacity_gib(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_gib", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the instance was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA user-readable description of the instance."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dynamic_tier_options` after provisioning.\nDynamic tier options for a Managed Lustre instance."]
    pub fn dynamic_tier_options(&self) -> terrars::ListRef<DataLustreInstance_DynamicTierOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dynamic_tier_options", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_version` after provisioning.\nThe version of Managed Lustre software that this instance is currently\nrunning."]
    pub fn effective_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effective_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `filesystem` after provisioning.\nThe filesystem name for this instance. This name is used by client-side\ntools, including when mounting the instance. Must be eight characters or\nless and can only contain letters and numbers."]
    pub fn filesystem(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filesystem", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gke_support_enabled` after provisioning.\nIndicates whether you want to enable support for GKE clients. By default,\nGKE clients are not supported."]
    pub fn gke_support_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gke_support_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_id` after provisioning.\nThe name of the Managed Lustre instance.\n\n* Must contain only lowercase letters, numbers, and hyphens.\n* Must start with a letter.\n* Must be between 1-63 characters.\n* Must end with a number or a letter."]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key` after provisioning.\nThe Cloud KMS key name to use for data encryption.\nIf not set, the instance will use Google-managed encryption keys.\nIf set, the instance will use customer-managed encryption keys.\nThe key must be in the same region as the instance.\nThe key format is:\nprojects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{key}"]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `maintenance_policy` after provisioning.\nDefines a maintenance policy for a resource."]
    pub fn maintenance_policy(&self) -> terrars::ListRef<DataLustreInstance_MaintenancePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `mount_point` after provisioning.\nMount point of the instance in the format 'IP_ADDRESS@tcp:/FILESYSTEM'."]
    pub fn mount_point(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mount_point", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the instance."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe full name of the VPC network to which the instance is connected.\nMust be in the format\n'projects/{project_id}/global/networks/{network_name}'."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `per_unit_storage_throughput` after provisioning.\nThe throughput of the instance in MBps per TiB. Valid values are 125, 250,\n500, 1000.\nSee [Performance tiers and maximum storage\ncapacities](https://cloud.google.com/managed-lustre/docs/create-instance#performance-tiers)\nfor more information.\n\nIf the instance is using the Dynamic tier, this field must not be set or\nmust be set to zero."]
    pub fn per_unit_storage_throughput(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.per_unit_storage_throughput", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `placement_policy` after provisioning.\nThe placement policy name for the instance in the format of\nprojects/{project}/locations/{location}/resourcePolicies/{resource_policy}"]
    pub fn placement_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.placement_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe state of the instance.\nPossible values:\nACTIVE\nCREATING\nDELETING\nUPGRADING\nREPAIRING\nSTOPPED\nUPDATING\nSUSPENDED"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state_reason` after provisioning.\nThe reason why the instance is in a certain state (e.g. SUSPENDED)."]
    pub fn state_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_reason", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_version` after provisioning.\nThe version to upgrade this instance to. Set this to the value reported in\n'availableVersion', or to 'latest' to move to the newest version available\nat the time of the upgrade.\nThis field cannot be set when the instance is created; new instances are\nalways provisioned from the current release. It also cannot be changed in\nthe same operation as 'capacityGib' or 'maintenancePolicy', and the\ninstance must be ACTIVE and outside of the hour preceding a scheduled\nmaintenance window.\nThe API clears this field once the upgrade finishes, so it always reads\nback as empty on an idle instance."]
    pub fn target_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nUnique ID of the resource.\nThis is unrelated to the access rules which allow specifying the root\nsquash uid."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `upcoming_maintenance_schedule` after provisioning.\nRepresents a scheduled maintenance event."]
    pub fn upcoming_maintenance_schedule(&self) -> terrars::ListRef<DataLustreInstance_UpcomingMaintenanceScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upcoming_maintenance_schedule", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when the instance was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone` after provisioning.\nZone of Lustre instance"]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataLustreInstance_AccessRulesOptions_AccessRules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_address_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    squash_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLustreInstance_AccessRulesOptions_AccessRules {
    #[doc = "Set the field `ip_address_ranges`.\n"]
    pub fn set_ip_address_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ip_address_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `squash_mode`.\n"]
    pub fn set_squash_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.squash_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLustreInstance_AccessRulesOptions_AccessRules {
    type O = terrars::BlockAssignable<DataLustreInstance_AccessRulesOptions_AccessRules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLustreInstance_AccessRulesOptions_AccessRules {}

impl BuildDataLustreInstance_AccessRulesOptions_AccessRules {
    pub fn build(self) -> DataLustreInstance_AccessRulesOptions_AccessRules {
        DataLustreInstance_AccessRulesOptions_AccessRules {
            ip_address_ranges: core::default::Default::default(),
            name: core::default::Default::default(),
            squash_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLustreInstance_AccessRulesOptions_AccessRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLustreInstance_AccessRulesOptions_AccessRulesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLustreInstance_AccessRulesOptions_AccessRulesRef {
        DataLustreInstance_AccessRulesOptions_AccessRulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLustreInstance_AccessRulesOptions_AccessRulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip_address_ranges` after provisioning.\n"]
    pub fn ip_address_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_address_ranges", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `squash_mode` after provisioning.\n"]
    pub fn squash_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.squash_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLustreInstance_AccessRulesOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_rules: std::option::Option<terrars::ListField<DataLustreInstance_AccessRulesOptions_AccessRules>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_squash_gid: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_squash_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_squash_uid: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLustreInstance_AccessRulesOptions {
    #[doc = "Set the field `access_rules`.\n"]
    pub fn set_access_rules(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataLustreInstance_AccessRulesOptions_AccessRules>>,
    ) -> Self {
        self.access_rules = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_squash_gid`.\n"]
    pub fn set_default_squash_gid(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.default_squash_gid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_squash_mode`.\n"]
    pub fn set_default_squash_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.default_squash_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_squash_uid`.\n"]
    pub fn set_default_squash_uid(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.default_squash_uid = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLustreInstance_AccessRulesOptions {
    type O = terrars::BlockAssignable<DataLustreInstance_AccessRulesOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLustreInstance_AccessRulesOptions {}

impl BuildDataLustreInstance_AccessRulesOptions {
    pub fn build(self) -> DataLustreInstance_AccessRulesOptions {
        DataLustreInstance_AccessRulesOptions {
            access_rules: core::default::Default::default(),
            default_squash_gid: core::default::Default::default(),
            default_squash_mode: core::default::Default::default(),
            default_squash_uid: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLustreInstance_AccessRulesOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLustreInstance_AccessRulesOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLustreInstance_AccessRulesOptionsRef {
        DataLustreInstance_AccessRulesOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLustreInstance_AccessRulesOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_rules` after provisioning.\n"]
    pub fn access_rules(&self) -> terrars::ListRef<DataLustreInstance_AccessRulesOptions_AccessRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_rules", self.base))
    }

    #[doc = "Get a reference to the value of field `default_squash_gid` after provisioning.\n"]
    pub fn default_squash_gid(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_squash_gid", self.base))
    }

    #[doc = "Get a reference to the value of field `default_squash_mode` after provisioning.\n"]
    pub fn default_squash_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_squash_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `default_squash_uid` after provisioning.\n"]
    pub fn default_squash_uid(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_squash_uid", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLustreInstance_DynamicTierOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLustreInstance_DynamicTierOptions {
    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLustreInstance_DynamicTierOptions {
    type O = terrars::BlockAssignable<DataLustreInstance_DynamicTierOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLustreInstance_DynamicTierOptions {}

impl BuildDataLustreInstance_DynamicTierOptions {
    pub fn build(self) -> DataLustreInstance_DynamicTierOptions {
        DataLustreInstance_DynamicTierOptions { mode: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataLustreInstance_DynamicTierOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLustreInstance_DynamicTierOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLustreInstance_DynamicTierOptionsRef {
        DataLustreInstance_DynamicTierOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLustreInstance_DynamicTierOptionsRef {
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
pub struct DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_EndDate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    month: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    year: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_EndDate {
    #[doc = "Set the field `day`.\n"]
    pub fn set_day(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.day = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `month`.\n"]
    pub fn set_month(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.month = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `year`.\n"]
    pub fn set_year(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.year = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_EndDate {
    type O = terrars::BlockAssignable<DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_EndDate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_EndDate {}

impl BuildDataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_EndDate {
    pub fn build(self) -> DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_EndDate {
        DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_EndDate {
            day: core::default::Default::default(),
            month: core::default::Default::default(),
            year: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_EndDateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_EndDateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_EndDateRef {
        DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_EndDateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_EndDateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `day` after provisioning.\n"]
    pub fn day(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc = "Get a reference to the value of field `month` after provisioning.\n"]
    pub fn month(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.month", self.base))
    }

    #[doc = "Get a reference to the value of field `year` after provisioning.\n"]
    pub fn year(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.year", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_StartDate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    month: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    year: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_StartDate {
    #[doc = "Set the field `day`.\n"]
    pub fn set_day(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.day = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `month`.\n"]
    pub fn set_month(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.month = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `year`.\n"]
    pub fn set_year(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.year = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_StartDate {
    type O = terrars::BlockAssignable<DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_StartDate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_StartDate {}

impl BuildDataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_StartDate {
    pub fn build(self) -> DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_StartDate {
        DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_StartDate {
            day: core::default::Default::default(),
            month: core::default::Default::default(),
            year: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_StartDateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_StartDateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_StartDateRef {
        DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_StartDateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_StartDateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `day` after provisioning.\n"]
    pub fn day(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day", self.base))
    }

    #[doc = "Get a reference to the value of field `month` after provisioning.\n"]
    pub fn month(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.month", self.base))
    }

    #[doc = "Get a reference to the value of field `year` after provisioning.\n"]
    pub fn year(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.year", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_Time {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_Time {
    #[doc = "Set the field `hours`.\n"]
    pub fn set_hours(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hours = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minutes`.\n"]
    pub fn set_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minutes = std::option::Option::Some(v.into());
        self
    }

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

impl terrars::ToListMappable for DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_Time {
    type O = terrars::BlockAssignable<DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_Time>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_Time {}

impl BuildDataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_Time {
    pub fn build(self) -> DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_Time {
        DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_Time {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_TimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_TimeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_TimeRef {
        DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_TimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_TimeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hours` after provisioning.\n"]
    pub fn hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }

    #[doc = "Get a reference to the value of field `minutes` after provisioning.\n"]
    pub fn minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
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
pub struct DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_date: std
    ::option
    ::Option<terrars::ListField<DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_EndDate>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_date: std
    ::option
    ::Option<terrars::ListField<DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_StartDate>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time: std::option::Option<terrars::ListField<DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_Time>>,
}

impl DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow {
    #[doc = "Set the field `end_date`.\n"]
    pub fn set_end_date(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_EndDate>,
                    >,
    ) -> Self {
        self.end_date = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_date`.\n"]
    pub fn set_start_date(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_StartDate>,
                    >,
    ) -> Self {
        self.start_date = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `time`.\n"]
    pub fn set_time(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_Time>>,
    ) -> Self {
        self.time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow {
    type O = terrars::BlockAssignable<DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow {}

impl BuildDataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow {
    pub fn build(self) -> DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow {
        DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow {
            end_date: core::default::Default::default(),
            start_date: core::default::Default::default(),
            time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindowRef {
        DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_date` after provisioning.\n"]
    pub fn end_date(
        &self,
    ) -> terrars::ListRef<DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_EndDateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.end_date", self.base))
    }

    #[doc = "Get a reference to the value of field `start_date` after provisioning.\n"]
    pub fn start_date(
        &self,
    ) -> terrars::ListRef<DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_StartDateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start_date", self.base))
    }

    #[doc = "Get a reference to the value of field `time` after provisioning.\n"]
    pub fn time(&self) -> terrars::ListRef<DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow_TimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows_StartTime {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nanos: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows_StartTime {
    #[doc = "Set the field `hours`.\n"]
    pub fn set_hours(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hours = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minutes`.\n"]
    pub fn set_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minutes = std::option::Option::Some(v.into());
        self
    }

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

impl terrars::ToListMappable for DataLustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows_StartTime {
    type O = terrars::BlockAssignable<DataLustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows_StartTime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows_StartTime {}

impl BuildDataLustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows_StartTime {
    pub fn build(self) -> DataLustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows_StartTime {
        DataLustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows_StartTime {
            hours: core::default::Default::default(),
            minutes: core::default::Default::default(),
            nanos: core::default::Default::default(),
            seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows_StartTimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows_StartTimeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows_StartTimeRef {
        DataLustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows_StartTimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows_StartTimeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hours` after provisioning.\n"]
    pub fn hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hours", self.base))
    }

    #[doc = "Get a reference to the value of field `minutes` after provisioning.\n"]
    pub fn minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minutes", self.base))
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
pub struct DataLustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day_of_week: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std
    ::option
    ::Option<terrars::ListField<DataLustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows_StartTime>>,
}

impl DataLustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows {
    #[doc = "Set the field `day_of_week`.\n"]
    pub fn set_day_of_week(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.day_of_week = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataLustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows_StartTime>,
                    >,
    ) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows {
    type O = terrars::BlockAssignable<DataLustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows {}

impl BuildDataLustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows {
    pub fn build(self) -> DataLustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows {
        DataLustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows {
            day_of_week: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLustreInstance_MaintenancePolicy_WeeklyMaintenanceWindowsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLustreInstance_MaintenancePolicy_WeeklyMaintenanceWindowsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLustreInstance_MaintenancePolicy_WeeklyMaintenanceWindowsRef {
        DataLustreInstance_MaintenancePolicy_WeeklyMaintenanceWindowsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLustreInstance_MaintenancePolicy_WeeklyMaintenanceWindowsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `day_of_week` after provisioning.\n"]
    pub fn day_of_week(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day_of_week", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(
        &self,
    ) -> terrars::ListRef<DataLustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows_StartTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLustreInstance_MaintenancePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_exclusion_window: std
    ::option
    ::Option<terrars::ListField<DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weekly_maintenance_windows: std
    ::option
    ::Option<terrars::ListField<DataLustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows>>,
}

impl DataLustreInstance_MaintenancePolicy {
    #[doc = "Set the field `maintenance_exclusion_window`.\n"]
    pub fn set_maintenance_exclusion_window(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindow>>,
    ) -> Self {
        self.maintenance_exclusion_window = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `weekly_maintenance_windows`.\n"]
    pub fn set_weekly_maintenance_windows(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataLustreInstance_MaintenancePolicy_WeeklyMaintenanceWindows>>,
    ) -> Self {
        self.weekly_maintenance_windows = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLustreInstance_MaintenancePolicy {
    type O = terrars::BlockAssignable<DataLustreInstance_MaintenancePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLustreInstance_MaintenancePolicy {}

impl BuildDataLustreInstance_MaintenancePolicy {
    pub fn build(self) -> DataLustreInstance_MaintenancePolicy {
        DataLustreInstance_MaintenancePolicy {
            maintenance_exclusion_window: core::default::Default::default(),
            weekly_maintenance_windows: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLustreInstance_MaintenancePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLustreInstance_MaintenancePolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLustreInstance_MaintenancePolicyRef {
        DataLustreInstance_MaintenancePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLustreInstance_MaintenancePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `maintenance_exclusion_window` after provisioning.\n"]
    pub fn maintenance_exclusion_window(
        &self,
    ) -> terrars::ListRef<DataLustreInstance_MaintenancePolicy_MaintenanceExclusionWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_exclusion_window", self.base))
    }

    #[doc = "Get a reference to the value of field `weekly_maintenance_windows` after provisioning.\n"]
    pub fn weekly_maintenance_windows(
        &self,
    ) -> terrars::ListRef<DataLustreInstance_MaintenancePolicy_WeeklyMaintenanceWindowsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weekly_maintenance_windows", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLustreInstance_UpcomingMaintenanceSchedule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLustreInstance_UpcomingMaintenanceSchedule {
    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLustreInstance_UpcomingMaintenanceSchedule {
    type O = terrars::BlockAssignable<DataLustreInstance_UpcomingMaintenanceSchedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLustreInstance_UpcomingMaintenanceSchedule {}

impl BuildDataLustreInstance_UpcomingMaintenanceSchedule {
    pub fn build(self) -> DataLustreInstance_UpcomingMaintenanceSchedule {
        DataLustreInstance_UpcomingMaintenanceSchedule {
            end_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLustreInstance_UpcomingMaintenanceScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLustreInstance_UpcomingMaintenanceScheduleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLustreInstance_UpcomingMaintenanceScheduleRef {
        DataLustreInstance_UpcomingMaintenanceScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLustreInstance_UpcomingMaintenanceScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}
