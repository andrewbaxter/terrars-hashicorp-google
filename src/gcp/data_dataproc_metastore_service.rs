#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataDataprocMetastoreServiceData_ {
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
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    service_id: terrars::PrimField<std::string::String>,
}

struct DataDataprocMetastoreService_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataDataprocMetastoreServiceData_>,
}

#[derive(Clone)]
pub struct DataDataprocMetastoreService(std::rc::Rc<DataDataprocMetastoreService_>);

impl DataDataprocMetastoreService {
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
        "Get a reference to the value of field `encryption_config` after provisioning.\nInformation used to configure the Dataproc Metastore service to encrypt\ncustomer data at rest."]
    pub fn encryption_config(&self) -> terrars::ListRef<DataDataprocMetastoreService_EncryptionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `endpoint_uri` after provisioning.\nThe URI of the endpoint used to access the metastore service."]
    pub fn endpoint_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hive_metastore_config` after provisioning.\nConfiguration information specific to running Hive metastore software as the metastore service."]
    pub fn hive_metastore_config(&self) -> terrars::ListRef<DataDataprocMetastoreService_HiveMetastoreConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hive_metastore_config", self.extract_ref()))
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
        "Get a reference to the value of field `maintenance_window` after provisioning.\nThe one hour maintenance window of the metastore service.\nThis specifies when the service can be restarted for maintenance purposes in UTC time.\nMaintenance window is not needed for services with the 'SPANNER' database type."]
    pub fn maintenance_window(&self) -> terrars::ListRef<DataDataprocMetastoreService_MaintenanceWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_window", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `metadata_integration` after provisioning.\nThe setting that defines how metastore metadata should be integrated with external services and systems."]
    pub fn metadata_integration(&self) -> terrars::ListRef<DataDataprocMetastoreService_MetadataIntegrationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata_integration", self.extract_ref()))
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
        "Get a reference to the value of field `network_config` after provisioning.\nThe configuration specifying the network settings for the Dataproc Metastore service."]
    pub fn network_config(&self) -> terrars::ListRef<DataDataprocMetastoreService_NetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
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
        "Get a reference to the value of field `scaling_config` after provisioning.\nRepresents the scaling configuration of a metastore service."]
    pub fn scaling_config(&self) -> terrars::ListRef<DataDataprocMetastoreService_ScalingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scaling_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scheduled_backup` after provisioning.\nThe configuration of scheduled backup for the metastore service."]
    pub fn scheduled_backup(&self) -> terrars::ListRef<DataDataprocMetastoreService_ScheduledBackupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scheduled_backup", self.extract_ref()))
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
        "Get a reference to the value of field `telemetry_config` after provisioning.\nThe configuration specifying telemetry settings for the Dataproc Metastore service. If unspecified defaults to JSON."]
    pub fn telemetry_config(&self) -> terrars::ListRef<DataDataprocMetastoreService_TelemetryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.telemetry_config", self.extract_ref()))
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
}

impl terrars::Referable for DataDataprocMetastoreService {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataDataprocMetastoreService { }

impl terrars::ToListMappable for DataDataprocMetastoreService {
    type O = terrars::ListRef<DataDataprocMetastoreServiceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataDataprocMetastoreService_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_dataproc_metastore_service".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataDataprocMetastoreService {
    pub tf_id: std::string::String,
    #[doc = "The location where the metastore service should reside.\nThe default value is 'global'."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "The ID of the metastore service. The id must contain only letters (a-z, A-Z), numbers (0-9), underscores (_),\nand hyphens (-). Cannot begin or end with underscore or hyphen. Must consist of between\n3 and 63 characters."]
    pub service_id: terrars::PrimField<std::string::String>,
}

impl BuildDataDataprocMetastoreService {
    pub fn build(self, stack: &mut terrars::Stack) -> DataDataprocMetastoreService {
        let out = DataDataprocMetastoreService(std::rc::Rc::new(DataDataprocMetastoreService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataDataprocMetastoreServiceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                service_id: self.service_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataDataprocMetastoreServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataprocMetastoreServiceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataDataprocMetastoreServiceRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
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
        "Get a reference to the value of field `encryption_config` after provisioning.\nInformation used to configure the Dataproc Metastore service to encrypt\ncustomer data at rest."]
    pub fn encryption_config(&self) -> terrars::ListRef<DataDataprocMetastoreService_EncryptionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `endpoint_uri` after provisioning.\nThe URI of the endpoint used to access the metastore service."]
    pub fn endpoint_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hive_metastore_config` after provisioning.\nConfiguration information specific to running Hive metastore software as the metastore service."]
    pub fn hive_metastore_config(&self) -> terrars::ListRef<DataDataprocMetastoreService_HiveMetastoreConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hive_metastore_config", self.extract_ref()))
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
        "Get a reference to the value of field `maintenance_window` after provisioning.\nThe one hour maintenance window of the metastore service.\nThis specifies when the service can be restarted for maintenance purposes in UTC time.\nMaintenance window is not needed for services with the 'SPANNER' database type."]
    pub fn maintenance_window(&self) -> terrars::ListRef<DataDataprocMetastoreService_MaintenanceWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_window", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `metadata_integration` after provisioning.\nThe setting that defines how metastore metadata should be integrated with external services and systems."]
    pub fn metadata_integration(&self) -> terrars::ListRef<DataDataprocMetastoreService_MetadataIntegrationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata_integration", self.extract_ref()))
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
        "Get a reference to the value of field `network_config` after provisioning.\nThe configuration specifying the network settings for the Dataproc Metastore service."]
    pub fn network_config(&self) -> terrars::ListRef<DataDataprocMetastoreService_NetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
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
        "Get a reference to the value of field `scaling_config` after provisioning.\nRepresents the scaling configuration of a metastore service."]
    pub fn scaling_config(&self) -> terrars::ListRef<DataDataprocMetastoreService_ScalingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scaling_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `scheduled_backup` after provisioning.\nThe configuration of scheduled backup for the metastore service."]
    pub fn scheduled_backup(&self) -> terrars::ListRef<DataDataprocMetastoreService_ScheduledBackupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scheduled_backup", self.extract_ref()))
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
        "Get a reference to the value of field `telemetry_config` after provisioning.\nThe configuration specifying telemetry settings for the Dataproc Metastore service. If unspecified defaults to JSON."]
    pub fn telemetry_config(&self) -> terrars::ListRef<DataDataprocMetastoreService_TelemetryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.telemetry_config", self.extract_ref()))
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
}

#[derive(Serialize)]
pub struct DataDataprocMetastoreService_EncryptionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDataprocMetastoreService_EncryptionConfig {
    #[doc = "Set the field `kms_key`.\n"]
    pub fn set_kms_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDataprocMetastoreService_EncryptionConfig {
    type O = terrars::BlockAssignable<DataDataprocMetastoreService_EncryptionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDataprocMetastoreService_EncryptionConfig {}

impl BuildDataDataprocMetastoreService_EncryptionConfig {
    pub fn build(self) -> DataDataprocMetastoreService_EncryptionConfig {
        DataDataprocMetastoreService_EncryptionConfig { kms_key: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataDataprocMetastoreService_EncryptionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataprocMetastoreService_EncryptionConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDataprocMetastoreService_EncryptionConfigRef {
        DataDataprocMetastoreService_EncryptionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDataprocMetastoreService_EncryptionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\n"]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDataprocMetastoreService_HiveMetastoreConfig_AuxiliaryVersions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config_overrides: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDataprocMetastoreService_HiveMetastoreConfig_AuxiliaryVersions {
    #[doc = "Set the field `config_overrides`.\n"]
    pub fn set_config_overrides(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.config_overrides = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDataprocMetastoreService_HiveMetastoreConfig_AuxiliaryVersions {
    type O = terrars::BlockAssignable<DataDataprocMetastoreService_HiveMetastoreConfig_AuxiliaryVersions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDataprocMetastoreService_HiveMetastoreConfig_AuxiliaryVersions {}

impl BuildDataDataprocMetastoreService_HiveMetastoreConfig_AuxiliaryVersions {
    pub fn build(self) -> DataDataprocMetastoreService_HiveMetastoreConfig_AuxiliaryVersions {
        DataDataprocMetastoreService_HiveMetastoreConfig_AuxiliaryVersions {
            config_overrides: core::default::Default::default(),
            key: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDataprocMetastoreService_HiveMetastoreConfig_AuxiliaryVersionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataprocMetastoreService_HiveMetastoreConfig_AuxiliaryVersionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDataprocMetastoreService_HiveMetastoreConfig_AuxiliaryVersionsRef {
        DataDataprocMetastoreService_HiveMetastoreConfig_AuxiliaryVersionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDataprocMetastoreService_HiveMetastoreConfig_AuxiliaryVersionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `config_overrides` after provisioning.\n"]
    pub fn config_overrides(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.config_overrides", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDataprocMetastoreService_HiveMetastoreConfig_KerberosConfig_Keytab {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_secret: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDataprocMetastoreService_HiveMetastoreConfig_KerberosConfig_Keytab {
    #[doc = "Set the field `cloud_secret`.\n"]
    pub fn set_cloud_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cloud_secret = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDataprocMetastoreService_HiveMetastoreConfig_KerberosConfig_Keytab {
    type O = terrars::BlockAssignable<DataDataprocMetastoreService_HiveMetastoreConfig_KerberosConfig_Keytab>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDataprocMetastoreService_HiveMetastoreConfig_KerberosConfig_Keytab {}

impl BuildDataDataprocMetastoreService_HiveMetastoreConfig_KerberosConfig_Keytab {
    pub fn build(self) -> DataDataprocMetastoreService_HiveMetastoreConfig_KerberosConfig_Keytab {
        DataDataprocMetastoreService_HiveMetastoreConfig_KerberosConfig_Keytab {
            cloud_secret: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDataprocMetastoreService_HiveMetastoreConfig_KerberosConfig_KeytabRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataprocMetastoreService_HiveMetastoreConfig_KerberosConfig_KeytabRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDataprocMetastoreService_HiveMetastoreConfig_KerberosConfig_KeytabRef {
        DataDataprocMetastoreService_HiveMetastoreConfig_KerberosConfig_KeytabRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDataprocMetastoreService_HiveMetastoreConfig_KerberosConfig_KeytabRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_secret` after provisioning.\n"]
    pub fn cloud_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_secret", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDataprocMetastoreService_HiveMetastoreConfig_KerberosConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keytab: std
    ::option
    ::Option<terrars::ListField<DataDataprocMetastoreService_HiveMetastoreConfig_KerberosConfig_Keytab>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    krb5_config_gcs_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principal: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDataprocMetastoreService_HiveMetastoreConfig_KerberosConfig {
    #[doc = "Set the field `keytab`.\n"]
    pub fn set_keytab(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataDataprocMetastoreService_HiveMetastoreConfig_KerberosConfig_Keytab>,
                    >,
    ) -> Self {
        self.keytab = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `krb5_config_gcs_uri`.\n"]
    pub fn set_krb5_config_gcs_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.krb5_config_gcs_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `principal`.\n"]
    pub fn set_principal(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.principal = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDataprocMetastoreService_HiveMetastoreConfig_KerberosConfig {
    type O = terrars::BlockAssignable<DataDataprocMetastoreService_HiveMetastoreConfig_KerberosConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDataprocMetastoreService_HiveMetastoreConfig_KerberosConfig {}

impl BuildDataDataprocMetastoreService_HiveMetastoreConfig_KerberosConfig {
    pub fn build(self) -> DataDataprocMetastoreService_HiveMetastoreConfig_KerberosConfig {
        DataDataprocMetastoreService_HiveMetastoreConfig_KerberosConfig {
            keytab: core::default::Default::default(),
            krb5_config_gcs_uri: core::default::Default::default(),
            principal: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDataprocMetastoreService_HiveMetastoreConfig_KerberosConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataprocMetastoreService_HiveMetastoreConfig_KerberosConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDataprocMetastoreService_HiveMetastoreConfig_KerberosConfigRef {
        DataDataprocMetastoreService_HiveMetastoreConfig_KerberosConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDataprocMetastoreService_HiveMetastoreConfig_KerberosConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `keytab` after provisioning.\n"]
    pub fn keytab(&self) -> terrars::ListRef<DataDataprocMetastoreService_HiveMetastoreConfig_KerberosConfig_KeytabRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.keytab", self.base))
    }

    #[doc = "Get a reference to the value of field `krb5_config_gcs_uri` after provisioning.\n"]
    pub fn krb5_config_gcs_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.krb5_config_gcs_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDataprocMetastoreService_HiveMetastoreConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auxiliary_versions: std
    ::option
    ::Option<terrars::SetField<DataDataprocMetastoreService_HiveMetastoreConfig_AuxiliaryVersions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config_overrides: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kerberos_config: std
    ::option
    ::Option<terrars::ListField<DataDataprocMetastoreService_HiveMetastoreConfig_KerberosConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDataprocMetastoreService_HiveMetastoreConfig {
    #[doc = "Set the field `auxiliary_versions`.\n"]
    pub fn set_auxiliary_versions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::SetField<DataDataprocMetastoreService_HiveMetastoreConfig_AuxiliaryVersions>>,
    ) -> Self {
        self.auxiliary_versions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `config_overrides`.\n"]
    pub fn set_config_overrides(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.config_overrides = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint_protocol`.\n"]
    pub fn set_endpoint_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint_protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kerberos_config`.\n"]
    pub fn set_kerberos_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataDataprocMetastoreService_HiveMetastoreConfig_KerberosConfig>>,
    ) -> Self {
        self.kerberos_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDataprocMetastoreService_HiveMetastoreConfig {
    type O = terrars::BlockAssignable<DataDataprocMetastoreService_HiveMetastoreConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDataprocMetastoreService_HiveMetastoreConfig {}

impl BuildDataDataprocMetastoreService_HiveMetastoreConfig {
    pub fn build(self) -> DataDataprocMetastoreService_HiveMetastoreConfig {
        DataDataprocMetastoreService_HiveMetastoreConfig {
            auxiliary_versions: core::default::Default::default(),
            config_overrides: core::default::Default::default(),
            endpoint_protocol: core::default::Default::default(),
            kerberos_config: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDataprocMetastoreService_HiveMetastoreConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataprocMetastoreService_HiveMetastoreConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDataprocMetastoreService_HiveMetastoreConfigRef {
        DataDataprocMetastoreService_HiveMetastoreConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDataprocMetastoreService_HiveMetastoreConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auxiliary_versions` after provisioning.\n"]
    pub fn auxiliary_versions(
        &self,
    ) -> terrars::SetRef<DataDataprocMetastoreService_HiveMetastoreConfig_AuxiliaryVersionsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.auxiliary_versions", self.base))
    }

    #[doc = "Get a reference to the value of field `config_overrides` after provisioning.\n"]
    pub fn config_overrides(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.config_overrides", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint_protocol` after provisioning.\n"]
    pub fn endpoint_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `kerberos_config` after provisioning.\n"]
    pub fn kerberos_config(
        &self,
    ) -> terrars::ListRef<DataDataprocMetastoreService_HiveMetastoreConfig_KerberosConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kerberos_config", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDataprocMetastoreService_MaintenanceWindow {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day_of_week: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hour_of_day: std::option::Option<terrars::PrimField<f64>>,
}

impl DataDataprocMetastoreService_MaintenanceWindow {
    #[doc = "Set the field `day_of_week`.\n"]
    pub fn set_day_of_week(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.day_of_week = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hour_of_day`.\n"]
    pub fn set_hour_of_day(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.hour_of_day = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDataprocMetastoreService_MaintenanceWindow {
    type O = terrars::BlockAssignable<DataDataprocMetastoreService_MaintenanceWindow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDataprocMetastoreService_MaintenanceWindow {}

impl BuildDataDataprocMetastoreService_MaintenanceWindow {
    pub fn build(self) -> DataDataprocMetastoreService_MaintenanceWindow {
        DataDataprocMetastoreService_MaintenanceWindow {
            day_of_week: core::default::Default::default(),
            hour_of_day: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDataprocMetastoreService_MaintenanceWindowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataprocMetastoreService_MaintenanceWindowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDataprocMetastoreService_MaintenanceWindowRef {
        DataDataprocMetastoreService_MaintenanceWindowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDataprocMetastoreService_MaintenanceWindowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `day_of_week` after provisioning.\n"]
    pub fn day_of_week(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day_of_week", self.base))
    }

    #[doc = "Get a reference to the value of field `hour_of_day` after provisioning.\n"]
    pub fn hour_of_day(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hour_of_day", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDataprocMetastoreService_MetadataIntegration_DataCatalogConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataDataprocMetastoreService_MetadataIntegration_DataCatalogConfig {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDataprocMetastoreService_MetadataIntegration_DataCatalogConfig {
    type O = terrars::BlockAssignable<DataDataprocMetastoreService_MetadataIntegration_DataCatalogConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDataprocMetastoreService_MetadataIntegration_DataCatalogConfig {}

impl BuildDataDataprocMetastoreService_MetadataIntegration_DataCatalogConfig {
    pub fn build(self) -> DataDataprocMetastoreService_MetadataIntegration_DataCatalogConfig {
        DataDataprocMetastoreService_MetadataIntegration_DataCatalogConfig {
            enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDataprocMetastoreService_MetadataIntegration_DataCatalogConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataprocMetastoreService_MetadataIntegration_DataCatalogConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDataprocMetastoreService_MetadataIntegration_DataCatalogConfigRef {
        DataDataprocMetastoreService_MetadataIntegration_DataCatalogConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDataprocMetastoreService_MetadataIntegration_DataCatalogConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDataprocMetastoreService_MetadataIntegration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_catalog_config: std
    ::option
    ::Option<terrars::ListField<DataDataprocMetastoreService_MetadataIntegration_DataCatalogConfig>>,
}

impl DataDataprocMetastoreService_MetadataIntegration {
    #[doc = "Set the field `data_catalog_config`.\n"]
    pub fn set_data_catalog_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataDataprocMetastoreService_MetadataIntegration_DataCatalogConfig>>,
    ) -> Self {
        self.data_catalog_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDataprocMetastoreService_MetadataIntegration {
    type O = terrars::BlockAssignable<DataDataprocMetastoreService_MetadataIntegration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDataprocMetastoreService_MetadataIntegration {}

impl BuildDataDataprocMetastoreService_MetadataIntegration {
    pub fn build(self) -> DataDataprocMetastoreService_MetadataIntegration {
        DataDataprocMetastoreService_MetadataIntegration { data_catalog_config: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataDataprocMetastoreService_MetadataIntegrationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataprocMetastoreService_MetadataIntegrationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDataprocMetastoreService_MetadataIntegrationRef {
        DataDataprocMetastoreService_MetadataIntegrationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDataprocMetastoreService_MetadataIntegrationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_catalog_config` after provisioning.\n"]
    pub fn data_catalog_config(
        &self,
    ) -> terrars::ListRef<DataDataprocMetastoreService_MetadataIntegration_DataCatalogConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_catalog_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDataprocMetastoreService_NetworkConfig_Consumers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDataprocMetastoreService_NetworkConfig_Consumers {
    #[doc = "Set the field `endpoint_uri`.\n"]
    pub fn set_endpoint_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnetwork`.\n"]
    pub fn set_subnetwork(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnetwork = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDataprocMetastoreService_NetworkConfig_Consumers {
    type O = terrars::BlockAssignable<DataDataprocMetastoreService_NetworkConfig_Consumers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDataprocMetastoreService_NetworkConfig_Consumers {}

impl BuildDataDataprocMetastoreService_NetworkConfig_Consumers {
    pub fn build(self) -> DataDataprocMetastoreService_NetworkConfig_Consumers {
        DataDataprocMetastoreService_NetworkConfig_Consumers {
            endpoint_uri: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDataprocMetastoreService_NetworkConfig_ConsumersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataprocMetastoreService_NetworkConfig_ConsumersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDataprocMetastoreService_NetworkConfig_ConsumersRef {
        DataDataprocMetastoreService_NetworkConfig_ConsumersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDataprocMetastoreService_NetworkConfig_ConsumersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint_uri` after provisioning.\n"]
    pub fn endpoint_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `subnetwork` after provisioning.\n"]
    pub fn subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDataprocMetastoreService_NetworkConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumers: std::option::Option<terrars::ListField<DataDataprocMetastoreService_NetworkConfig_Consumers>>,
}

impl DataDataprocMetastoreService_NetworkConfig {
    #[doc = "Set the field `consumers`.\n"]
    pub fn set_consumers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataDataprocMetastoreService_NetworkConfig_Consumers>>,
    ) -> Self {
        self.consumers = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDataprocMetastoreService_NetworkConfig {
    type O = terrars::BlockAssignable<DataDataprocMetastoreService_NetworkConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDataprocMetastoreService_NetworkConfig {}

impl BuildDataDataprocMetastoreService_NetworkConfig {
    pub fn build(self) -> DataDataprocMetastoreService_NetworkConfig {
        DataDataprocMetastoreService_NetworkConfig { consumers: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataDataprocMetastoreService_NetworkConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataprocMetastoreService_NetworkConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDataprocMetastoreService_NetworkConfigRef {
        DataDataprocMetastoreService_NetworkConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDataprocMetastoreService_NetworkConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `consumers` after provisioning.\n"]
    pub fn consumers(&self) -> terrars::ListRef<DataDataprocMetastoreService_NetworkConfig_ConsumersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.consumers", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDataprocMetastoreService_ScalingConfig_AutoscalingConfig_LimitConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_scaling_factor: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_scaling_factor: std::option::Option<terrars::PrimField<f64>>,
}

impl DataDataprocMetastoreService_ScalingConfig_AutoscalingConfig_LimitConfig {
    #[doc = "Set the field `max_scaling_factor`.\n"]
    pub fn set_max_scaling_factor(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_scaling_factor = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_scaling_factor`.\n"]
    pub fn set_min_scaling_factor(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_scaling_factor = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDataprocMetastoreService_ScalingConfig_AutoscalingConfig_LimitConfig {
    type O = terrars::BlockAssignable<DataDataprocMetastoreService_ScalingConfig_AutoscalingConfig_LimitConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDataprocMetastoreService_ScalingConfig_AutoscalingConfig_LimitConfig {}

impl BuildDataDataprocMetastoreService_ScalingConfig_AutoscalingConfig_LimitConfig {
    pub fn build(self) -> DataDataprocMetastoreService_ScalingConfig_AutoscalingConfig_LimitConfig {
        DataDataprocMetastoreService_ScalingConfig_AutoscalingConfig_LimitConfig {
            max_scaling_factor: core::default::Default::default(),
            min_scaling_factor: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDataprocMetastoreService_ScalingConfig_AutoscalingConfig_LimitConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataprocMetastoreService_ScalingConfig_AutoscalingConfig_LimitConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDataprocMetastoreService_ScalingConfig_AutoscalingConfig_LimitConfigRef {
        DataDataprocMetastoreService_ScalingConfig_AutoscalingConfig_LimitConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDataprocMetastoreService_ScalingConfig_AutoscalingConfig_LimitConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_scaling_factor` after provisioning.\n"]
    pub fn max_scaling_factor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_scaling_factor", self.base))
    }

    #[doc = "Get a reference to the value of field `min_scaling_factor` after provisioning.\n"]
    pub fn min_scaling_factor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_scaling_factor", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDataprocMetastoreService_ScalingConfig_AutoscalingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_factor: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limit_config: std
    ::option
    ::Option<terrars::ListField<DataDataprocMetastoreService_ScalingConfig_AutoscalingConfig_LimitConfig>>,
}

impl DataDataprocMetastoreService_ScalingConfig_AutoscalingConfig {
    #[doc = "Set the field `autoscaling_enabled`.\n"]
    pub fn set_autoscaling_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.autoscaling_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autoscaling_factor`.\n"]
    pub fn set_autoscaling_factor(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.autoscaling_factor = std::option::Option::Some(v.into());
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
                        terrars::ListField<DataDataprocMetastoreService_ScalingConfig_AutoscalingConfig_LimitConfig>,
                    >,
    ) -> Self {
        self.limit_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDataprocMetastoreService_ScalingConfig_AutoscalingConfig {
    type O = terrars::BlockAssignable<DataDataprocMetastoreService_ScalingConfig_AutoscalingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDataprocMetastoreService_ScalingConfig_AutoscalingConfig {}

impl BuildDataDataprocMetastoreService_ScalingConfig_AutoscalingConfig {
    pub fn build(self) -> DataDataprocMetastoreService_ScalingConfig_AutoscalingConfig {
        DataDataprocMetastoreService_ScalingConfig_AutoscalingConfig {
            autoscaling_enabled: core::default::Default::default(),
            autoscaling_factor: core::default::Default::default(),
            limit_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDataprocMetastoreService_ScalingConfig_AutoscalingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataprocMetastoreService_ScalingConfig_AutoscalingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDataprocMetastoreService_ScalingConfig_AutoscalingConfigRef {
        DataDataprocMetastoreService_ScalingConfig_AutoscalingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDataprocMetastoreService_ScalingConfig_AutoscalingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `autoscaling_enabled` after provisioning.\n"]
    pub fn autoscaling_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autoscaling_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `autoscaling_factor` after provisioning.\n"]
    pub fn autoscaling_factor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autoscaling_factor", self.base))
    }

    #[doc = "Get a reference to the value of field `limit_config` after provisioning.\n"]
    pub fn limit_config(
        &self,
    ) -> terrars::ListRef<DataDataprocMetastoreService_ScalingConfig_AutoscalingConfig_LimitConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.limit_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDataprocMetastoreService_ScalingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_config: std
    ::option
    ::Option<terrars::ListField<DataDataprocMetastoreService_ScalingConfig_AutoscalingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_size: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scaling_factor: std::option::Option<terrars::PrimField<f64>>,
}

impl DataDataprocMetastoreService_ScalingConfig {
    #[doc = "Set the field `autoscaling_config`.\n"]
    pub fn set_autoscaling_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataDataprocMetastoreService_ScalingConfig_AutoscalingConfig>>,
    ) -> Self {
        self.autoscaling_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_size`.\n"]
    pub fn set_instance_size(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scaling_factor`.\n"]
    pub fn set_scaling_factor(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.scaling_factor = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDataprocMetastoreService_ScalingConfig {
    type O = terrars::BlockAssignable<DataDataprocMetastoreService_ScalingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDataprocMetastoreService_ScalingConfig {}

impl BuildDataDataprocMetastoreService_ScalingConfig {
    pub fn build(self) -> DataDataprocMetastoreService_ScalingConfig {
        DataDataprocMetastoreService_ScalingConfig {
            autoscaling_config: core::default::Default::default(),
            instance_size: core::default::Default::default(),
            scaling_factor: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDataprocMetastoreService_ScalingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataprocMetastoreService_ScalingConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataDataprocMetastoreService_ScalingConfigRef {
        DataDataprocMetastoreService_ScalingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDataprocMetastoreService_ScalingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `autoscaling_config` after provisioning.\n"]
    pub fn autoscaling_config(
        &self,
    ) -> terrars::ListRef<DataDataprocMetastoreService_ScalingConfig_AutoscalingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling_config", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_size` after provisioning.\n"]
    pub fn instance_size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_size", self.base))
    }

    #[doc = "Get a reference to the value of field `scaling_factor` after provisioning.\n"]
    pub fn scaling_factor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scaling_factor", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDataprocMetastoreService_ScheduledBackup {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cron_schedule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDataprocMetastoreService_ScheduledBackup {
    #[doc = "Set the field `backup_location`.\n"]
    pub fn set_backup_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.backup_location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cron_schedule`.\n"]
    pub fn set_cron_schedule(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cron_schedule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `time_zone`.\n"]
    pub fn set_time_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.time_zone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDataprocMetastoreService_ScheduledBackup {
    type O = terrars::BlockAssignable<DataDataprocMetastoreService_ScheduledBackup>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDataprocMetastoreService_ScheduledBackup {}

impl BuildDataDataprocMetastoreService_ScheduledBackup {
    pub fn build(self) -> DataDataprocMetastoreService_ScheduledBackup {
        DataDataprocMetastoreService_ScheduledBackup {
            backup_location: core::default::Default::default(),
            cron_schedule: core::default::Default::default(),
            enabled: core::default::Default::default(),
            time_zone: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDataprocMetastoreService_ScheduledBackupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataprocMetastoreService_ScheduledBackupRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDataprocMetastoreService_ScheduledBackupRef {
        DataDataprocMetastoreService_ScheduledBackupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDataprocMetastoreService_ScheduledBackupRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `backup_location` after provisioning.\n"]
    pub fn backup_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backup_location", self.base))
    }

    #[doc = "Get a reference to the value of field `cron_schedule` after provisioning.\n"]
    pub fn cron_schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cron_schedule", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\n"]
    pub fn time_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDataprocMetastoreService_TelemetryConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_format: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDataprocMetastoreService_TelemetryConfig {
    #[doc = "Set the field `log_format`.\n"]
    pub fn set_log_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_format = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDataprocMetastoreService_TelemetryConfig {
    type O = terrars::BlockAssignable<DataDataprocMetastoreService_TelemetryConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDataprocMetastoreService_TelemetryConfig {}

impl BuildDataDataprocMetastoreService_TelemetryConfig {
    pub fn build(self) -> DataDataprocMetastoreService_TelemetryConfig {
        DataDataprocMetastoreService_TelemetryConfig { log_format: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataDataprocMetastoreService_TelemetryConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataprocMetastoreService_TelemetryConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDataprocMetastoreService_TelemetryConfigRef {
        DataDataprocMetastoreService_TelemetryConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDataprocMetastoreService_TelemetryConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_format` after provisioning.\n"]
    pub fn log_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_format", self.base))
    }
}
