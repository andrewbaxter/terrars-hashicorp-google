#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataprocMetastoreServiceData_ {
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
    database_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    release_channel: std::option::Option<terrars::PrimField<std::string::String>>,
    service_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_config: std::option::Option<std::vec::Vec<DataprocMetastoreService_EncryptionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hive_metastore_config: std::option::Option<std::vec::Vec<DataprocMetastoreService_HiveMetastoreConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_window: std::option::Option<std::vec::Vec<DataprocMetastoreService_MaintenanceWindow>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata_integration: std::option::Option<std::vec::Vec<DataprocMetastoreService_MetadataIntegration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_config: std::option::Option<std::vec::Vec<DataprocMetastoreService_NetworkConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scaling_config: std::option::Option<std::vec::Vec<DataprocMetastoreService_ScalingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheduled_backup: std::option::Option<std::vec::Vec<DataprocMetastoreService_ScheduledBackup>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    telemetry_config: std::option::Option<std::vec::Vec<DataprocMetastoreService_TelemetryConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataprocMetastoreService_Timeouts>,
    dynamic: DataprocMetastoreServiceDynamic,
}

struct DataprocMetastoreService_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataprocMetastoreServiceData_>,
}

#[derive(Clone)]
pub struct DataprocMetastoreService(std::rc::Rc<DataprocMetastoreService_>);

impl DataprocMetastoreService {
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
        "Set the field `database_type`.\nThe database type that the Metastore service stores its data. Default value: \"MYSQL\" Possible values: [\"MYSQL\", \"SPANNER\"]"]
    pub fn set_database_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().database_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_protection`.\nIndicates if the dataproc metastore should be protected against accidental deletions."]
    pub fn set_deletion_protection(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nUser-defined labels for the metastore service.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `location`.\nThe location where the metastore service should reside.\nThe default value is 'global'."]
    pub fn set_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `network`.\nThe relative resource name of the VPC network on which the instance can be accessed. It is specified in the following form:\n\n\"projects/{projectNumber}/global/networks/{network_id}\"."]
    pub fn set_network(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\nThe TCP port at which the metastore service is reached. Default: 9083."]
    pub fn set_port(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `release_channel`.\nThe release channel of the service. If unspecified, defaults to 'STABLE'. Default value: \"STABLE\" Possible values: [\"CANARY\", \"STABLE\"]"]
    pub fn set_release_channel(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().release_channel = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tags`.\nA map of resource manager tags.\nResource manager tag keys and values have the same definition as resource manager tags.\nKeys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/{tag_value_id}."]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tier`.\nThe tier of the service. Possible values: [\"DEVELOPER\", \"ENTERPRISE\"]"]
    pub fn set_tier(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocMetastoreService_EncryptionConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `hive_metastore_config`.\n"]
    pub fn set_hive_metastore_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocMetastoreService_HiveMetastoreConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().hive_metastore_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.hive_metastore_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `maintenance_window`.\n"]
    pub fn set_maintenance_window(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocMetastoreService_MaintenanceWindow>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().maintenance_window = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.maintenance_window = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `metadata_integration`.\n"]
    pub fn set_metadata_integration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocMetastoreService_MetadataIntegration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().metadata_integration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.metadata_integration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_config`.\n"]
    pub fn set_network_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocMetastoreService_NetworkConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scaling_config`.\n"]
    pub fn set_scaling_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocMetastoreService_ScalingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scaling_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scaling_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scheduled_backup`.\n"]
    pub fn set_scheduled_backup(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocMetastoreService_ScheduledBackup>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scheduled_backup = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scheduled_backup = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `telemetry_config`.\n"]
    pub fn set_telemetry_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocMetastoreService_TelemetryConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().telemetry_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.telemetry_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DataprocMetastoreService_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `artifact_gcs_uri` after provisioning.\nA Cloud Storage URI (starting with gs://) that specifies where artifacts related to the metastore service are stored."]
    pub fn artifact_gcs_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.artifact_gcs_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time when the metastore service was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `database_type` after provisioning.\nThe database type that the Metastore service stores its data. Default value: \"MYSQL\" Possible values: [\"MYSQL\", \"SPANNER\"]"]
    pub fn database_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection` after provisioning.\nIndicates if the dataproc metastore should be protected against accidental deletions."]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `endpoint_uri` after provisioning.\nThe URI of the endpoint used to access the metastore service."]
    pub fn endpoint_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels for the metastore service.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location where the metastore service should reside.\nThe default value is 'global'."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe relative resource name of the metastore service."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe relative resource name of the VPC network on which the instance can be accessed. It is specified in the following form:\n\n\"projects/{projectNumber}/global/networks/{network_id}\"."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe TCP port at which the metastore service is reached. Default: 9083."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `release_channel` after provisioning.\nThe release channel of the service. If unspecified, defaults to 'STABLE'. Default value: \"STABLE\" Possible values: [\"CANARY\", \"STABLE\"]"]
    pub fn release_channel(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.release_channel", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_id` after provisioning.\nThe ID of the metastore service. The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_),\nand hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of between\n3 and 63 characters."]
    pub fn service_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe current state of the metastore service."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state_message` after provisioning.\nAdditional information about the current state of the metastore service, if available."]
    pub fn state_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_message", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nA map of resource manager tags.\nResource manager tag keys and values have the same definition as resource manager tags.\nKeys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/{tag_value_id}."]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tier` after provisioning.\nThe tier of the service. Possible values: [\"DEVELOPER\", \"ENTERPRISE\"]"]
    pub fn tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tier", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nThe globally unique resource identifier of the metastore service."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time when the metastore service was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> terrars::ListRef<DataprocMetastoreService_EncryptionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hive_metastore_config` after provisioning.\n"]
    pub fn hive_metastore_config(&self) -> terrars::ListRef<DataprocMetastoreService_HiveMetastoreConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hive_metastore_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> terrars::ListRef<DataprocMetastoreService_MaintenanceWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metadata_integration` after provisioning.\n"]
    pub fn metadata_integration(&self) -> terrars::ListRef<DataprocMetastoreService_MetadataIntegrationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata_integration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> terrars::ListRef<DataprocMetastoreService_NetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scaling_config` after provisioning.\n"]
    pub fn scaling_config(&self) -> terrars::ListRef<DataprocMetastoreService_ScalingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scaling_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scheduled_backup` after provisioning.\n"]
    pub fn scheduled_backup(&self) -> terrars::ListRef<DataprocMetastoreService_ScheduledBackupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scheduled_backup", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `telemetry_config` after provisioning.\n"]
    pub fn telemetry_config(&self) -> terrars::ListRef<DataprocMetastoreService_TelemetryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.telemetry_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataprocMetastoreService_TimeoutsRef {
        DataprocMetastoreService_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DataprocMetastoreService {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DataprocMetastoreService { }

impl terrars::ToListMappable for DataprocMetastoreService {
    type O = terrars::ListRef<DataprocMetastoreServiceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DataprocMetastoreService_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_dataproc_metastore_service".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataprocMetastoreService {
    pub tf_id: std::string::String,
    #[doc =
        "The ID of the metastore service. The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_),\nand hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of between\n3 and 63 characters."]
    pub service_id: terrars::PrimField<std::string::String>,
}

impl BuildDataprocMetastoreService {
    pub fn build(self, stack: &mut terrars::Stack) -> DataprocMetastoreService {
        let out = DataprocMetastoreService(std::rc::Rc::new(DataprocMetastoreService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataprocMetastoreServiceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                database_type: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: core::default::Default::default(),
                network: core::default::Default::default(),
                port: core::default::Default::default(),
                project: core::default::Default::default(),
                release_channel: core::default::Default::default(),
                service_id: self.service_id,
                tags: core::default::Default::default(),
                tier: core::default::Default::default(),
                encryption_config: core::default::Default::default(),
                hive_metastore_config: core::default::Default::default(),
                maintenance_window: core::default::Default::default(),
                metadata_integration: core::default::Default::default(),
                network_config: core::default::Default::default(),
                scaling_config: core::default::Default::default(),
                scheduled_backup: core::default::Default::default(),
                telemetry_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DataprocMetastoreServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocMetastoreServiceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataprocMetastoreServiceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `artifact_gcs_uri` after provisioning.\nA Cloud Storage URI (starting with gs://) that specifies where artifacts related to the metastore service are stored."]
    pub fn artifact_gcs_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.artifact_gcs_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The time when the metastore service was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `database_type` after provisioning.\nThe database type that the Metastore service stores its data. Default value: \"MYSQL\" Possible values: [\"MYSQL\", \"SPANNER\"]"]
    pub fn database_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection` after provisioning.\nIndicates if the dataproc metastore should be protected against accidental deletions."]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `endpoint_uri` after provisioning.\nThe URI of the endpoint used to access the metastore service."]
    pub fn endpoint_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUser-defined labels for the metastore service.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location where the metastore service should reside.\nThe default value is 'global'."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe relative resource name of the metastore service."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe relative resource name of the VPC network on which the instance can be accessed. It is specified in the following form:\n\n\"projects/{projectNumber}/global/networks/{network_id}\"."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe TCP port at which the metastore service is reached. Default: 9083."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `release_channel` after provisioning.\nThe release channel of the service. If unspecified, defaults to 'STABLE'. Default value: \"STABLE\" Possible values: [\"CANARY\", \"STABLE\"]"]
    pub fn release_channel(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.release_channel", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_id` after provisioning.\nThe ID of the metastore service. The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_),\nand hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of between\n3 and 63 characters."]
    pub fn service_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe current state of the metastore service."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state_message` after provisioning.\nAdditional information about the current state of the metastore service, if available."]
    pub fn state_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_message", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nA map of resource manager tags.\nResource manager tag keys and values have the same definition as resource manager tags.\nKeys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/{tag_value_id}."]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tier` after provisioning.\nThe tier of the service. Possible values: [\"DEVELOPER\", \"ENTERPRISE\"]"]
    pub fn tier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tier", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nThe globally unique resource identifier of the metastore service."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time when the metastore service was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> terrars::ListRef<DataprocMetastoreService_EncryptionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hive_metastore_config` after provisioning.\n"]
    pub fn hive_metastore_config(&self) -> terrars::ListRef<DataprocMetastoreService_HiveMetastoreConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hive_metastore_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> terrars::ListRef<DataprocMetastoreService_MaintenanceWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metadata_integration` after provisioning.\n"]
    pub fn metadata_integration(&self) -> terrars::ListRef<DataprocMetastoreService_MetadataIntegrationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata_integration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> terrars::ListRef<DataprocMetastoreService_NetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scaling_config` after provisioning.\n"]
    pub fn scaling_config(&self) -> terrars::ListRef<DataprocMetastoreService_ScalingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scaling_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scheduled_backup` after provisioning.\n"]
    pub fn scheduled_backup(&self) -> terrars::ListRef<DataprocMetastoreService_ScheduledBackupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scheduled_backup", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `telemetry_config` after provisioning.\n"]
    pub fn telemetry_config(&self) -> terrars::ListRef<DataprocMetastoreService_TelemetryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.telemetry_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataprocMetastoreService_TimeoutsRef {
        DataprocMetastoreService_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataprocMetastoreServiceDynamic {
    encryption_config: std::option::Option<terrars::DynamicBlock<DataprocMetastoreService_EncryptionConfig>>,
    hive_metastore_config: std::option::Option<terrars::DynamicBlock<DataprocMetastoreService_HiveMetastoreConfig>>,
    maintenance_window: std::option::Option<terrars::DynamicBlock<DataprocMetastoreService_MaintenanceWindow>>,
    metadata_integration: std::option::Option<terrars::DynamicBlock<DataprocMetastoreService_MetadataIntegration>>,
    network_config: std::option::Option<terrars::DynamicBlock<DataprocMetastoreService_NetworkConfig>>,
    scaling_config: std::option::Option<terrars::DynamicBlock<DataprocMetastoreService_ScalingConfig>>,
    scheduled_backup: std::option::Option<terrars::DynamicBlock<DataprocMetastoreService_ScheduledBackup>>,
    telemetry_config: std::option::Option<terrars::DynamicBlock<DataprocMetastoreService_TelemetryConfig>>,
}

#[derive(Serialize)]
pub struct DataprocMetastoreService_EncryptionConfig {
    kms_key: terrars::PrimField<std::string::String>,
}

impl DataprocMetastoreService_EncryptionConfig { }

impl terrars::ToListMappable for DataprocMetastoreService_EncryptionConfig {
    type O = terrars::BlockAssignable<DataprocMetastoreService_EncryptionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocMetastoreService_EncryptionConfig {
    #[doc =
        "The fully qualified customer provided Cloud KMS key name to use for customer data encryption.\nUse the following format: 'projects/([^/]+)/locations/([^/]+)/keyRings/([^/]+)/cryptoKeys/([^/]+)'"]
    pub kms_key: terrars::PrimField<std::string::String>,
}

impl BuildDataprocMetastoreService_EncryptionConfig {
    pub fn build(self) -> DataprocMetastoreService_EncryptionConfig {
        DataprocMetastoreService_EncryptionConfig { kms_key: self.kms_key }
    }
}

#[allow(dead_code)]
pub struct DataprocMetastoreService_EncryptionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocMetastoreService_EncryptionConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocMetastoreService_EncryptionConfigRef {
        DataprocMetastoreService_EncryptionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocMetastoreService_EncryptionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key` after provisioning.\nThe fully qualified customer provided Cloud KMS key name to use for customer data encryption.\nUse the following format: 'projects/([^/]+)/locations/([^/]+)/keyRings/([^/]+)/cryptoKeys/([^/]+)'"]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocMetastoreService_HiveMetastoreConfig_AuxiliaryVersions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config_overrides: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    key: terrars::PrimField<std::string::String>,
    version: terrars::PrimField<std::string::String>,
}

impl DataprocMetastoreService_HiveMetastoreConfig_AuxiliaryVersions {
    #[doc =
        "Set the field `config_overrides`.\nA mapping of Hive metastore configuration key-value pairs to apply to the auxiliary Hive metastore (configured in hive-site.xml) in addition to the primary version's overrides.\nIf keys are present in both the auxiliary version's overrides and the primary version's overrides, the value from the auxiliary version's overrides takes precedence."]
    pub fn set_config_overrides(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.config_overrides = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocMetastoreService_HiveMetastoreConfig_AuxiliaryVersions {
    type O = terrars::BlockAssignable<DataprocMetastoreService_HiveMetastoreConfig_AuxiliaryVersions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocMetastoreService_HiveMetastoreConfig_AuxiliaryVersions {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc =
        "The Hive metastore version of the auxiliary service. It must be less than the primary Hive metastore service's version."]
    pub version: terrars::PrimField<std::string::String>,
}

impl BuildDataprocMetastoreService_HiveMetastoreConfig_AuxiliaryVersions {
    pub fn build(self) -> DataprocMetastoreService_HiveMetastoreConfig_AuxiliaryVersions {
        DataprocMetastoreService_HiveMetastoreConfig_AuxiliaryVersions {
            config_overrides: core::default::Default::default(),
            key: self.key,
            version: self.version,
        }
    }
}

#[allow(dead_code)]
pub struct DataprocMetastoreService_HiveMetastoreConfig_AuxiliaryVersionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocMetastoreService_HiveMetastoreConfig_AuxiliaryVersionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocMetastoreService_HiveMetastoreConfig_AuxiliaryVersionsRef {
        DataprocMetastoreService_HiveMetastoreConfig_AuxiliaryVersionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocMetastoreService_HiveMetastoreConfig_AuxiliaryVersionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `config_overrides` after provisioning.\nA mapping of Hive metastore configuration key-value pairs to apply to the auxiliary Hive metastore (configured in hive-site.xml) in addition to the primary version's overrides.\nIf keys are present in both the auxiliary version's overrides and the primary version's overrides, the value from the auxiliary version's overrides takes precedence."]
    pub fn config_overrides(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.config_overrides", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nThe Hive metastore version of the auxiliary service. It must be less than the primary Hive metastore service's version."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocMetastoreService_HiveMetastoreConfig_KerberosConfig_Keytab {
    cloud_secret: terrars::PrimField<std::string::String>,
}

impl DataprocMetastoreService_HiveMetastoreConfig_KerberosConfig_Keytab { }

impl terrars::ToListMappable for DataprocMetastoreService_HiveMetastoreConfig_KerberosConfig_Keytab {
    type O = terrars::BlockAssignable<DataprocMetastoreService_HiveMetastoreConfig_KerberosConfig_Keytab>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocMetastoreService_HiveMetastoreConfig_KerberosConfig_Keytab {
    #[doc =
        "The relative resource name of a Secret Manager secret version, in the following form:\n\n\"projects/{projectNumber}/secrets/{secret_id}/versions/{version_id}\"."]
    pub cloud_secret: terrars::PrimField<std::string::String>,
}

impl BuildDataprocMetastoreService_HiveMetastoreConfig_KerberosConfig_Keytab {
    pub fn build(self) -> DataprocMetastoreService_HiveMetastoreConfig_KerberosConfig_Keytab {
        DataprocMetastoreService_HiveMetastoreConfig_KerberosConfig_Keytab { cloud_secret: self.cloud_secret }
    }
}

#[allow(dead_code)]
pub struct DataprocMetastoreService_HiveMetastoreConfig_KerberosConfig_KeytabRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocMetastoreService_HiveMetastoreConfig_KerberosConfig_KeytabRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocMetastoreService_HiveMetastoreConfig_KerberosConfig_KeytabRef {
        DataprocMetastoreService_HiveMetastoreConfig_KerberosConfig_KeytabRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocMetastoreService_HiveMetastoreConfig_KerberosConfig_KeytabRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cloud_secret` after provisioning.\nThe relative resource name of a Secret Manager secret version, in the following form:\n\n\"projects/{projectNumber}/secrets/{secret_id}/versions/{version_id}\"."]
    pub fn cloud_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_secret", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocMetastoreService_HiveMetastoreConfig_KerberosConfigDynamic {
    keytab: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocMetastoreService_HiveMetastoreConfig_KerberosConfig_Keytab>>,
}

#[derive(Serialize)]
pub struct DataprocMetastoreService_HiveMetastoreConfig_KerberosConfig {
    krb5_config_gcs_uri: terrars::PrimField<std::string::String>,
    principal: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keytab: std::option::Option<std::vec::Vec<DataprocMetastoreService_HiveMetastoreConfig_KerberosConfig_Keytab>>,
    dynamic: DataprocMetastoreService_HiveMetastoreConfig_KerberosConfigDynamic,
}

impl DataprocMetastoreService_HiveMetastoreConfig_KerberosConfig {
    #[doc = "Set the field `keytab`.\n"]
    pub fn set_keytab(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataprocMetastoreService_HiveMetastoreConfig_KerberosConfig_Keytab>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.keytab = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.keytab = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocMetastoreService_HiveMetastoreConfig_KerberosConfig {
    type O = terrars::BlockAssignable<DataprocMetastoreService_HiveMetastoreConfig_KerberosConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocMetastoreService_HiveMetastoreConfig_KerberosConfig {
    #[doc =
        "A Cloud Storage URI that specifies the path to a krb5.conf file. It is of the form gs://{bucket_name}/path/to/krb5.conf, although the file does not need to be named krb5.conf explicitly."]
    pub krb5_config_gcs_uri: terrars::PrimField<std::string::String>,
    #[doc =
        "A Kerberos principal that exists in the both the keytab the KDC to authenticate as. A typical principal is of the form \"primary/instance@REALM\", but there is no exact format."]
    pub principal: terrars::PrimField<std::string::String>,
}

impl BuildDataprocMetastoreService_HiveMetastoreConfig_KerberosConfig {
    pub fn build(self) -> DataprocMetastoreService_HiveMetastoreConfig_KerberosConfig {
        DataprocMetastoreService_HiveMetastoreConfig_KerberosConfig {
            krb5_config_gcs_uri: self.krb5_config_gcs_uri,
            principal: self.principal,
            keytab: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocMetastoreService_HiveMetastoreConfig_KerberosConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocMetastoreService_HiveMetastoreConfig_KerberosConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocMetastoreService_HiveMetastoreConfig_KerberosConfigRef {
        DataprocMetastoreService_HiveMetastoreConfig_KerberosConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocMetastoreService_HiveMetastoreConfig_KerberosConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `krb5_config_gcs_uri` after provisioning.\nA Cloud Storage URI that specifies the path to a krb5.conf file. It is of the form gs://{bucket_name}/path/to/krb5.conf, although the file does not need to be named krb5.conf explicitly."]
    pub fn krb5_config_gcs_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.krb5_config_gcs_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `principal` after provisioning.\nA Kerberos principal that exists in the both the keytab the KDC to authenticate as. A typical principal is of the form \"primary/instance@REALM\", but there is no exact format."]
    pub fn principal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal", self.base))
    }

    #[doc = "Get a reference to the value of field `keytab` after provisioning.\n"]
    pub fn keytab(&self) -> terrars::ListRef<DataprocMetastoreService_HiveMetastoreConfig_KerberosConfig_KeytabRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.keytab", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocMetastoreService_HiveMetastoreConfigDynamic {
    auxiliary_versions: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocMetastoreService_HiveMetastoreConfig_AuxiliaryVersions>>,
    kerberos_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocMetastoreService_HiveMetastoreConfig_KerberosConfig>>,
}

#[derive(Serialize)]
pub struct DataprocMetastoreService_HiveMetastoreConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config_overrides: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auxiliary_versions: std
    ::option
    ::Option<std::vec::Vec<DataprocMetastoreService_HiveMetastoreConfig_AuxiliaryVersions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kerberos_config: std::option::Option<std::vec::Vec<DataprocMetastoreService_HiveMetastoreConfig_KerberosConfig>>,
    dynamic: DataprocMetastoreService_HiveMetastoreConfigDynamic,
}

impl DataprocMetastoreService_HiveMetastoreConfig {
    #[doc =
        "Set the field `config_overrides`.\nA mapping of Hive metastore configuration key-value pairs to apply to the Hive metastore (configured in hive-site.xml).\nThe mappings override system defaults (some keys cannot be overridden)"]
    pub fn set_config_overrides(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.config_overrides = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `endpoint_protocol`.\nThe protocol to use for the metastore service endpoint. If unspecified, defaults to 'THRIFT'. Default value: \"THRIFT\" Possible values: [\"THRIFT\", \"GRPC\"]"]
    pub fn set_endpoint_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint_protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auxiliary_versions`.\n"]
    pub fn set_auxiliary_versions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataprocMetastoreService_HiveMetastoreConfig_AuxiliaryVersions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.auxiliary_versions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.auxiliary_versions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kerberos_config`.\n"]
    pub fn set_kerberos_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataprocMetastoreService_HiveMetastoreConfig_KerberosConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kerberos_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kerberos_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocMetastoreService_HiveMetastoreConfig {
    type O = terrars::BlockAssignable<DataprocMetastoreService_HiveMetastoreConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocMetastoreService_HiveMetastoreConfig {
    #[doc = "The Hive metastore schema version."]
    pub version: terrars::PrimField<std::string::String>,
}

impl BuildDataprocMetastoreService_HiveMetastoreConfig {
    pub fn build(self) -> DataprocMetastoreService_HiveMetastoreConfig {
        DataprocMetastoreService_HiveMetastoreConfig {
            config_overrides: core::default::Default::default(),
            endpoint_protocol: core::default::Default::default(),
            version: self.version,
            auxiliary_versions: core::default::Default::default(),
            kerberos_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocMetastoreService_HiveMetastoreConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocMetastoreService_HiveMetastoreConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocMetastoreService_HiveMetastoreConfigRef {
        DataprocMetastoreService_HiveMetastoreConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocMetastoreService_HiveMetastoreConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `config_overrides` after provisioning.\nA mapping of Hive metastore configuration key-value pairs to apply to the Hive metastore (configured in hive-site.xml).\nThe mappings override system defaults (some keys cannot be overridden)"]
    pub fn config_overrides(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.config_overrides", self.base))
    }

    #[doc =
        "Get a reference to the value of field `endpoint_protocol` after provisioning.\nThe protocol to use for the metastore service endpoint. If unspecified, defaults to 'THRIFT'. Default value: \"THRIFT\" Possible values: [\"THRIFT\", \"GRPC\"]"]
    pub fn endpoint_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\nThe Hive metastore schema version."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc = "Get a reference to the value of field `kerberos_config` after provisioning.\n"]
    pub fn kerberos_config(&self) -> terrars::ListRef<DataprocMetastoreService_HiveMetastoreConfig_KerberosConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kerberos_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocMetastoreService_MaintenanceWindow {
    day_of_week: terrars::PrimField<std::string::String>,
    hour_of_day: terrars::PrimField<f64>,
}

impl DataprocMetastoreService_MaintenanceWindow { }

impl terrars::ToListMappable for DataprocMetastoreService_MaintenanceWindow {
    type O = terrars::BlockAssignable<DataprocMetastoreService_MaintenanceWindow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocMetastoreService_MaintenanceWindow {
    #[doc =
        "The day of week, when the window starts. Possible values: [\"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub day_of_week: terrars::PrimField<std::string::String>,
    #[doc = "The hour of day (0-23) when the window starts."]
    pub hour_of_day: terrars::PrimField<f64>,
}

impl BuildDataprocMetastoreService_MaintenanceWindow {
    pub fn build(self) -> DataprocMetastoreService_MaintenanceWindow {
        DataprocMetastoreService_MaintenanceWindow {
            day_of_week: self.day_of_week,
            hour_of_day: self.hour_of_day,
        }
    }
}

#[allow(dead_code)]
pub struct DataprocMetastoreService_MaintenanceWindowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocMetastoreService_MaintenanceWindowRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocMetastoreService_MaintenanceWindowRef {
        DataprocMetastoreService_MaintenanceWindowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocMetastoreService_MaintenanceWindowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `day_of_week` after provisioning.\nThe day of week, when the window starts. Possible values: [\"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub fn day_of_week(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day_of_week", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hour_of_day` after provisioning.\nThe hour of day (0-23) when the window starts."]
    pub fn hour_of_day(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hour_of_day", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocMetastoreService_MetadataIntegration_DataCatalogConfig {
    enabled: terrars::PrimField<bool>,
}

impl DataprocMetastoreService_MetadataIntegration_DataCatalogConfig { }

impl terrars::ToListMappable for DataprocMetastoreService_MetadataIntegration_DataCatalogConfig {
    type O = terrars::BlockAssignable<DataprocMetastoreService_MetadataIntegration_DataCatalogConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocMetastoreService_MetadataIntegration_DataCatalogConfig {
    #[doc =
        "Defines whether the metastore metadata should be synced to Data Catalog. The default value is to disable syncing metastore metadata to Data Catalog."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildDataprocMetastoreService_MetadataIntegration_DataCatalogConfig {
    pub fn build(self) -> DataprocMetastoreService_MetadataIntegration_DataCatalogConfig {
        DataprocMetastoreService_MetadataIntegration_DataCatalogConfig { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct DataprocMetastoreService_MetadataIntegration_DataCatalogConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocMetastoreService_MetadataIntegration_DataCatalogConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocMetastoreService_MetadataIntegration_DataCatalogConfigRef {
        DataprocMetastoreService_MetadataIntegration_DataCatalogConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocMetastoreService_MetadataIntegration_DataCatalogConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nDefines whether the metastore metadata should be synced to Data Catalog. The default value is to disable syncing metastore metadata to Data Catalog."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocMetastoreService_MetadataIntegrationDynamic {
    data_catalog_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocMetastoreService_MetadataIntegration_DataCatalogConfig>>,
}

#[derive(Serialize)]
pub struct DataprocMetastoreService_MetadataIntegration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_catalog_config: std
    ::option
    ::Option<std::vec::Vec<DataprocMetastoreService_MetadataIntegration_DataCatalogConfig>>,
    dynamic: DataprocMetastoreService_MetadataIntegrationDynamic,
}

impl DataprocMetastoreService_MetadataIntegration {
    #[doc = "Set the field `data_catalog_config`.\n"]
    pub fn set_data_catalog_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataprocMetastoreService_MetadataIntegration_DataCatalogConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.data_catalog_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.data_catalog_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocMetastoreService_MetadataIntegration {
    type O = terrars::BlockAssignable<DataprocMetastoreService_MetadataIntegration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocMetastoreService_MetadataIntegration {}

impl BuildDataprocMetastoreService_MetadataIntegration {
    pub fn build(self) -> DataprocMetastoreService_MetadataIntegration {
        DataprocMetastoreService_MetadataIntegration {
            data_catalog_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocMetastoreService_MetadataIntegrationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocMetastoreService_MetadataIntegrationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocMetastoreService_MetadataIntegrationRef {
        DataprocMetastoreService_MetadataIntegrationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocMetastoreService_MetadataIntegrationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_catalog_config` after provisioning.\n"]
    pub fn data_catalog_config(
        &self,
    ) -> terrars::ListRef<DataprocMetastoreService_MetadataIntegration_DataCatalogConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_catalog_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocMetastoreService_NetworkConfig_Consumers {
    subnetwork: terrars::PrimField<std::string::String>,
}

impl DataprocMetastoreService_NetworkConfig_Consumers { }

impl terrars::ToListMappable for DataprocMetastoreService_NetworkConfig_Consumers {
    type O = terrars::BlockAssignable<DataprocMetastoreService_NetworkConfig_Consumers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocMetastoreService_NetworkConfig_Consumers {
    #[doc =
        "The subnetwork of the customer project from which an IP address is reserved and used as the Dataproc Metastore service's endpoint.\nIt is accessible to hosts in the subnet and to all hosts in a subnet in the same region and same network.\nThere must be at least one IP address available in the subnet's primary range. The subnet is specified in the following form:\n'projects/{projectNumber}/regions/{region_id}/subnetworks/{subnetwork_id}"]
    pub subnetwork: terrars::PrimField<std::string::String>,
}

impl BuildDataprocMetastoreService_NetworkConfig_Consumers {
    pub fn build(self) -> DataprocMetastoreService_NetworkConfig_Consumers {
        DataprocMetastoreService_NetworkConfig_Consumers { subnetwork: self.subnetwork }
    }
}

#[allow(dead_code)]
pub struct DataprocMetastoreService_NetworkConfig_ConsumersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocMetastoreService_NetworkConfig_ConsumersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocMetastoreService_NetworkConfig_ConsumersRef {
        DataprocMetastoreService_NetworkConfig_ConsumersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocMetastoreService_NetworkConfig_ConsumersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `endpoint_uri` after provisioning.\nThe URI of the endpoint used to access the metastore service."]
    pub fn endpoint_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subnetwork` after provisioning.\nThe subnetwork of the customer project from which an IP address is reserved and used as the Dataproc Metastore service's endpoint.\nIt is accessible to hosts in the subnet and to all hosts in a subnet in the same region and same network.\nThere must be at least one IP address available in the subnet's primary range. The subnet is specified in the following form:\n'projects/{projectNumber}/regions/{region_id}/subnetworks/{subnetwork_id}"]
    pub fn subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocMetastoreService_NetworkConfigDynamic {
    consumers: std::option::Option<terrars::DynamicBlock<DataprocMetastoreService_NetworkConfig_Consumers>>,
}

#[derive(Serialize)]
pub struct DataprocMetastoreService_NetworkConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumers: std::option::Option<std::vec::Vec<DataprocMetastoreService_NetworkConfig_Consumers>>,
    dynamic: DataprocMetastoreService_NetworkConfigDynamic,
}

impl DataprocMetastoreService_NetworkConfig {
    #[doc = "Set the field `consumers`.\n"]
    pub fn set_consumers(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocMetastoreService_NetworkConfig_Consumers>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.consumers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.consumers = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocMetastoreService_NetworkConfig {
    type O = terrars::BlockAssignable<DataprocMetastoreService_NetworkConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocMetastoreService_NetworkConfig {}

impl BuildDataprocMetastoreService_NetworkConfig {
    pub fn build(self) -> DataprocMetastoreService_NetworkConfig {
        DataprocMetastoreService_NetworkConfig {
            consumers: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocMetastoreService_NetworkConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocMetastoreService_NetworkConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocMetastoreService_NetworkConfigRef {
        DataprocMetastoreService_NetworkConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocMetastoreService_NetworkConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `consumers` after provisioning.\n"]
    pub fn consumers(&self) -> terrars::ListRef<DataprocMetastoreService_NetworkConfig_ConsumersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.consumers", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocMetastoreService_ScalingConfig_AutoscalingConfig_LimitConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_scaling_factor: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_scaling_factor: std::option::Option<terrars::PrimField<f64>>,
}

impl DataprocMetastoreService_ScalingConfig_AutoscalingConfig_LimitConfig {
    #[doc =
        "Set the field `max_scaling_factor`.\nThe maximum scaling factor that the service will autoscale to. The default value is 6.0."]
    pub fn set_max_scaling_factor(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_scaling_factor = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_scaling_factor`.\nThe minimum scaling factor that the service will autoscale to. The default value is 0.1."]
    pub fn set_min_scaling_factor(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_scaling_factor = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocMetastoreService_ScalingConfig_AutoscalingConfig_LimitConfig {
    type O = terrars::BlockAssignable<DataprocMetastoreService_ScalingConfig_AutoscalingConfig_LimitConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocMetastoreService_ScalingConfig_AutoscalingConfig_LimitConfig {}

impl BuildDataprocMetastoreService_ScalingConfig_AutoscalingConfig_LimitConfig {
    pub fn build(self) -> DataprocMetastoreService_ScalingConfig_AutoscalingConfig_LimitConfig {
        DataprocMetastoreService_ScalingConfig_AutoscalingConfig_LimitConfig {
            max_scaling_factor: core::default::Default::default(),
            min_scaling_factor: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocMetastoreService_ScalingConfig_AutoscalingConfig_LimitConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocMetastoreService_ScalingConfig_AutoscalingConfig_LimitConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocMetastoreService_ScalingConfig_AutoscalingConfig_LimitConfigRef {
        DataprocMetastoreService_ScalingConfig_AutoscalingConfig_LimitConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocMetastoreService_ScalingConfig_AutoscalingConfig_LimitConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_scaling_factor` after provisioning.\nThe maximum scaling factor that the service will autoscale to. The default value is 6.0."]
    pub fn max_scaling_factor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_scaling_factor", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_scaling_factor` after provisioning.\nThe minimum scaling factor that the service will autoscale to. The default value is 0.1."]
    pub fn min_scaling_factor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_scaling_factor", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocMetastoreService_ScalingConfig_AutoscalingConfigDynamic {
    limit_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocMetastoreService_ScalingConfig_AutoscalingConfig_LimitConfig>>,
}

#[derive(Serialize)]
pub struct DataprocMetastoreService_ScalingConfig_AutoscalingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limit_config: std
    ::option
    ::Option<std::vec::Vec<DataprocMetastoreService_ScalingConfig_AutoscalingConfig_LimitConfig>>,
    dynamic: DataprocMetastoreService_ScalingConfig_AutoscalingConfigDynamic,
}

impl DataprocMetastoreService_ScalingConfig_AutoscalingConfig {
    #[doc =
        "Set the field `autoscaling_enabled`.\nDefines whether autoscaling is enabled. The default value is false."]
    pub fn set_autoscaling_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.autoscaling_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `limit_config`.\n"]
    pub fn set_limit_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataprocMetastoreService_ScalingConfig_AutoscalingConfig_LimitConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.limit_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.limit_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocMetastoreService_ScalingConfig_AutoscalingConfig {
    type O = terrars::BlockAssignable<DataprocMetastoreService_ScalingConfig_AutoscalingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocMetastoreService_ScalingConfig_AutoscalingConfig {}

impl BuildDataprocMetastoreService_ScalingConfig_AutoscalingConfig {
    pub fn build(self) -> DataprocMetastoreService_ScalingConfig_AutoscalingConfig {
        DataprocMetastoreService_ScalingConfig_AutoscalingConfig {
            autoscaling_enabled: core::default::Default::default(),
            limit_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocMetastoreService_ScalingConfig_AutoscalingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocMetastoreService_ScalingConfig_AutoscalingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocMetastoreService_ScalingConfig_AutoscalingConfigRef {
        DataprocMetastoreService_ScalingConfig_AutoscalingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocMetastoreService_ScalingConfig_AutoscalingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `autoscaling_enabled` after provisioning.\nDefines whether autoscaling is enabled. The default value is false."]
    pub fn autoscaling_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autoscaling_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `autoscaling_factor` after provisioning.\nOutput only. The scaling factor of a service with autoscaling enabled."]
    pub fn autoscaling_factor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autoscaling_factor", self.base))
    }

    #[doc = "Get a reference to the value of field `limit_config` after provisioning.\n"]
    pub fn limit_config(
        &self,
    ) -> terrars::ListRef<DataprocMetastoreService_ScalingConfig_AutoscalingConfig_LimitConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.limit_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocMetastoreService_ScalingConfigDynamic {
    autoscaling_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocMetastoreService_ScalingConfig_AutoscalingConfig>>,
}

#[derive(Serialize)]
pub struct DataprocMetastoreService_ScalingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_size: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scaling_factor: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_config: std::option::Option<std::vec::Vec<DataprocMetastoreService_ScalingConfig_AutoscalingConfig>>,
    dynamic: DataprocMetastoreService_ScalingConfigDynamic,
}

impl DataprocMetastoreService_ScalingConfig {
    #[doc =
        "Set the field `instance_size`.\nMetastore instance sizes. Possible values: [\"EXTRA_SMALL\", \"SMALL\", \"MEDIUM\", \"LARGE\", \"EXTRA_LARGE\"]"]
    pub fn set_instance_size(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_size = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `scaling_factor`.\nScaling factor, in increments of 0.1 for values less than 1.0, and increments of 1.0 for values greater than 1.0."]
    pub fn set_scaling_factor(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.scaling_factor = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autoscaling_config`.\n"]
    pub fn set_autoscaling_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataprocMetastoreService_ScalingConfig_AutoscalingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.autoscaling_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.autoscaling_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataprocMetastoreService_ScalingConfig {
    type O = terrars::BlockAssignable<DataprocMetastoreService_ScalingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocMetastoreService_ScalingConfig {}

impl BuildDataprocMetastoreService_ScalingConfig {
    pub fn build(self) -> DataprocMetastoreService_ScalingConfig {
        DataprocMetastoreService_ScalingConfig {
            instance_size: core::default::Default::default(),
            scaling_factor: core::default::Default::default(),
            autoscaling_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocMetastoreService_ScalingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocMetastoreService_ScalingConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocMetastoreService_ScalingConfigRef {
        DataprocMetastoreService_ScalingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocMetastoreService_ScalingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `instance_size` after provisioning.\nMetastore instance sizes. Possible values: [\"EXTRA_SMALL\", \"SMALL\", \"MEDIUM\", \"LARGE\", \"EXTRA_LARGE\"]"]
    pub fn instance_size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_size", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scaling_factor` after provisioning.\nScaling factor, in increments of 0.1 for values less than 1.0, and increments of 1.0 for values greater than 1.0."]
    pub fn scaling_factor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scaling_factor", self.base))
    }

    #[doc = "Get a reference to the value of field `autoscaling_config` after provisioning.\n"]
    pub fn autoscaling_config(&self) -> terrars::ListRef<DataprocMetastoreService_ScalingConfig_AutoscalingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocMetastoreService_ScheduledBackup {
    backup_location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cron_schedule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocMetastoreService_ScheduledBackup {
    #[doc =
        "Set the field `cron_schedule`.\nThe scheduled interval in Cron format, see https://en.wikipedia.org/wiki/Cron The default is empty: scheduled backup is not enabled. Must be specified to enable scheduled backups."]
    pub fn set_cron_schedule(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cron_schedule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\nDefines whether the scheduled backup is enabled. The default value is false."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `time_zone`.\nSpecifies the time zone to be used when interpreting cronSchedule. Must be a time zone name from the time zone database (https://en.wikipedia.org/wiki/List_of_tz_database_time_zones), e.g. America/Los_Angeles or Africa/Abidjan. If left unspecified, the default is UTC."]
    pub fn set_time_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.time_zone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocMetastoreService_ScheduledBackup {
    type O = terrars::BlockAssignable<DataprocMetastoreService_ScheduledBackup>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocMetastoreService_ScheduledBackup {
    #[doc =
        "A Cloud Storage URI of a folder, in the format gs://<bucket_name>/<path_inside_bucket>. A sub-folder <backup_folder> containing backup files will be stored below it."]
    pub backup_location: terrars::PrimField<std::string::String>,
}

impl BuildDataprocMetastoreService_ScheduledBackup {
    pub fn build(self) -> DataprocMetastoreService_ScheduledBackup {
        DataprocMetastoreService_ScheduledBackup {
            backup_location: self.backup_location,
            cron_schedule: core::default::Default::default(),
            enabled: core::default::Default::default(),
            time_zone: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocMetastoreService_ScheduledBackupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocMetastoreService_ScheduledBackupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocMetastoreService_ScheduledBackupRef {
        DataprocMetastoreService_ScheduledBackupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocMetastoreService_ScheduledBackupRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `backup_location` after provisioning.\nA Cloud Storage URI of a folder, in the format gs://<bucket_name>/<path_inside_bucket>. A sub-folder <backup_folder> containing backup files will be stored below it."]
    pub fn backup_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_location", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cron_schedule` after provisioning.\nThe scheduled interval in Cron format, see https://en.wikipedia.org/wiki/Cron The default is empty: scheduled backup is not enabled. Must be specified to enable scheduled backups."]
    pub fn cron_schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cron_schedule", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nDefines whether the scheduled backup is enabled. The default value is false."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `time_zone` after provisioning.\nSpecifies the time zone to be used when interpreting cronSchedule. Must be a time zone name from the time zone database (https://en.wikipedia.org/wiki/List_of_tz_database_time_zones), e.g. America/Los_Angeles or Africa/Abidjan. If left unspecified, the default is UTC."]
    pub fn time_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocMetastoreService_TelemetryConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_format: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocMetastoreService_TelemetryConfig {
    #[doc =
        "Set the field `log_format`.\nThe output format of the Dataproc Metastore service's logs. Default value: \"JSON\" Possible values: [\"LEGACY\", \"JSON\"]"]
    pub fn set_log_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_format = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocMetastoreService_TelemetryConfig {
    type O = terrars::BlockAssignable<DataprocMetastoreService_TelemetryConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocMetastoreService_TelemetryConfig {}

impl BuildDataprocMetastoreService_TelemetryConfig {
    pub fn build(self) -> DataprocMetastoreService_TelemetryConfig {
        DataprocMetastoreService_TelemetryConfig { log_format: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataprocMetastoreService_TelemetryConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocMetastoreService_TelemetryConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocMetastoreService_TelemetryConfigRef {
        DataprocMetastoreService_TelemetryConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocMetastoreService_TelemetryConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `log_format` after provisioning.\nThe output format of the Dataproc Metastore service's logs. Default value: \"JSON\" Possible values: [\"LEGACY\", \"JSON\"]"]
    pub fn log_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_format", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocMetastoreService_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocMetastoreService_Timeouts {
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

impl terrars::ToListMappable for DataprocMetastoreService_Timeouts {
    type O = terrars::BlockAssignable<DataprocMetastoreService_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocMetastoreService_Timeouts {}

impl BuildDataprocMetastoreService_Timeouts {
    pub fn build(self) -> DataprocMetastoreService_Timeouts {
        DataprocMetastoreService_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataprocMetastoreService_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocMetastoreService_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocMetastoreService_TimeoutsRef {
        DataprocMetastoreService_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocMetastoreService_TimeoutsRef {
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
