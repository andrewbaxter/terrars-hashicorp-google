use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataDataprocMetastoreServiceData {
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
    data: std::cell::RefCell<DataDataprocMetastoreServiceData>,
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
        "Get a reference to the value of field `database_type` after provisioning.\nThe database type that the Metastore service stores its data. Default value: \"MYSQL\" Possible values: [\"MYSQL\", \"SPANNER\"]"]
    pub fn database_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `encryption_config` after provisioning.\nInformation used to configure the Dataproc Metastore service to encrypt\ncustomer data at rest."]
    pub fn encryption_config(&self) -> terrars::ListRef<DataDataprocMetastoreServiceEncryptionConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `endpoint_uri` after provisioning.\nThe URI of the endpoint used to access the metastore service."]
    pub fn endpoint_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hive_metastore_config` after provisioning.\nConfiguration information specific to running Hive metastore software as the metastore service."]
    pub fn hive_metastore_config(&self) -> terrars::ListRef<DataDataprocMetastoreServiceHiveMetastoreConfigElRef> {
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
    pub fn maintenance_window(&self) -> terrars::ListRef<DataDataprocMetastoreServiceMaintenanceWindowElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_window", self.extract_ref()))
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
    pub fn network_config(&self) -> terrars::ListRef<DataDataprocMetastoreServiceNetworkConfigElRef> {
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
    pub fn scaling_config(&self) -> terrars::ListRef<DataDataprocMetastoreServiceScalingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scaling_config", self.extract_ref()))
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
        "Get a reference to the value of field `telemetry_config` after provisioning.\nThe configuration specifying telemetry settings for the Dataproc Metastore service. If unspecified defaults to JSON."]
    pub fn telemetry_config(&self) -> terrars::ListRef<DataDataprocMetastoreServiceTelemetryConfigElRef> {
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
            data: std::cell::RefCell::new(DataDataprocMetastoreServiceData {
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
        "Get a reference to the value of field `database_type` after provisioning.\nThe database type that the Metastore service stores its data. Default value: \"MYSQL\" Possible values: [\"MYSQL\", \"SPANNER\"]"]
    pub fn database_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `encryption_config` after provisioning.\nInformation used to configure the Dataproc Metastore service to encrypt\ncustomer data at rest."]
    pub fn encryption_config(&self) -> terrars::ListRef<DataDataprocMetastoreServiceEncryptionConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `endpoint_uri` after provisioning.\nThe URI of the endpoint used to access the metastore service."]
    pub fn endpoint_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `hive_metastore_config` after provisioning.\nConfiguration information specific to running Hive metastore software as the metastore service."]
    pub fn hive_metastore_config(&self) -> terrars::ListRef<DataDataprocMetastoreServiceHiveMetastoreConfigElRef> {
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
    pub fn maintenance_window(&self) -> terrars::ListRef<DataDataprocMetastoreServiceMaintenanceWindowElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_window", self.extract_ref()))
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
    pub fn network_config(&self) -> terrars::ListRef<DataDataprocMetastoreServiceNetworkConfigElRef> {
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
    pub fn scaling_config(&self) -> terrars::ListRef<DataDataprocMetastoreServiceScalingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scaling_config", self.extract_ref()))
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
        "Get a reference to the value of field `telemetry_config` after provisioning.\nThe configuration specifying telemetry settings for the Dataproc Metastore service. If unspecified defaults to JSON."]
    pub fn telemetry_config(&self) -> terrars::ListRef<DataDataprocMetastoreServiceTelemetryConfigElRef> {
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
}

#[derive(Serialize)]
pub struct DataDataprocMetastoreServiceEncryptionConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDataprocMetastoreServiceEncryptionConfigEl {
    #[doc = "Set the field `kms_key`.\n"]
    pub fn set_kms_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDataprocMetastoreServiceEncryptionConfigEl {
    type O = terrars::BlockAssignable<DataDataprocMetastoreServiceEncryptionConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDataprocMetastoreServiceEncryptionConfigEl {}

impl BuildDataDataprocMetastoreServiceEncryptionConfigEl {
    pub fn build(self) -> DataDataprocMetastoreServiceEncryptionConfigEl {
        DataDataprocMetastoreServiceEncryptionConfigEl { kms_key: core::default::Default::default() }
    }
}

pub struct DataDataprocMetastoreServiceEncryptionConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataprocMetastoreServiceEncryptionConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDataprocMetastoreServiceEncryptionConfigElRef {
        DataDataprocMetastoreServiceEncryptionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDataprocMetastoreServiceEncryptionConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\n"]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_secret: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabEl {
    #[doc = "Set the field `cloud_secret`.\n"]
    pub fn set_cloud_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cloud_secret = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabEl {
    type O = terrars::BlockAssignable<DataDataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabEl {}

impl BuildDataDataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabEl {
    pub fn build(self) -> DataDataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabEl {
        DataDataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabEl {
            cloud_secret: core::default::Default::default(),
        }
    }
}

pub struct DataDataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabElRef {
        DataDataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_secret` after provisioning.\n"]
    pub fn cloud_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_secret", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keytab: std
    ::option
    ::Option<terrars::ListField<DataDataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    krb5_config_gcs_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principal: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigEl {
    #[doc = "Set the field `keytab`.\n"]
    pub fn set_keytab(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataDataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabEl>,
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

impl terrars::ToListMappable for DataDataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigEl {
    type O = terrars::BlockAssignable<DataDataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigEl {}

impl BuildDataDataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigEl {
    pub fn build(self) -> DataDataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigEl {
        DataDataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigEl {
            keytab: core::default::Default::default(),
            krb5_config_gcs_uri: core::default::Default::default(),
            principal: core::default::Default::default(),
        }
    }
}

pub struct DataDataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElRef {
        DataDataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `keytab` after provisioning.\n"]
    pub fn keytab(
        &self,
    ) -> terrars::ListRef<DataDataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabElRef> {
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
pub struct DataDataprocMetastoreServiceHiveMetastoreConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config_overrides: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kerberos_config: std
    ::option
    ::Option<terrars::ListField<DataDataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDataprocMetastoreServiceHiveMetastoreConfigEl {
    #[doc = "Set the field `config_overrides`.\n"]
    pub fn set_config_overrides(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.config_overrides = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kerberos_config`.\n"]
    pub fn set_kerberos_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataDataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigEl>>,
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

impl terrars::ToListMappable for DataDataprocMetastoreServiceHiveMetastoreConfigEl {
    type O = terrars::BlockAssignable<DataDataprocMetastoreServiceHiveMetastoreConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDataprocMetastoreServiceHiveMetastoreConfigEl {}

impl BuildDataDataprocMetastoreServiceHiveMetastoreConfigEl {
    pub fn build(self) -> DataDataprocMetastoreServiceHiveMetastoreConfigEl {
        DataDataprocMetastoreServiceHiveMetastoreConfigEl {
            config_overrides: core::default::Default::default(),
            kerberos_config: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct DataDataprocMetastoreServiceHiveMetastoreConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataprocMetastoreServiceHiveMetastoreConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDataprocMetastoreServiceHiveMetastoreConfigElRef {
        DataDataprocMetastoreServiceHiveMetastoreConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDataprocMetastoreServiceHiveMetastoreConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `config_overrides` after provisioning.\n"]
    pub fn config_overrides(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.config_overrides", self.base))
    }

    #[doc = "Get a reference to the value of field `kerberos_config` after provisioning.\n"]
    pub fn kerberos_config(
        &self,
    ) -> terrars::ListRef<DataDataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kerberos_config", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDataprocMetastoreServiceMaintenanceWindowEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day_of_week: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hour_of_day: std::option::Option<terrars::PrimField<f64>>,
}

impl DataDataprocMetastoreServiceMaintenanceWindowEl {
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

impl terrars::ToListMappable for DataDataprocMetastoreServiceMaintenanceWindowEl {
    type O = terrars::BlockAssignable<DataDataprocMetastoreServiceMaintenanceWindowEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDataprocMetastoreServiceMaintenanceWindowEl {}

impl BuildDataDataprocMetastoreServiceMaintenanceWindowEl {
    pub fn build(self) -> DataDataprocMetastoreServiceMaintenanceWindowEl {
        DataDataprocMetastoreServiceMaintenanceWindowEl {
            day_of_week: core::default::Default::default(),
            hour_of_day: core::default::Default::default(),
        }
    }
}

pub struct DataDataprocMetastoreServiceMaintenanceWindowElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataprocMetastoreServiceMaintenanceWindowElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDataprocMetastoreServiceMaintenanceWindowElRef {
        DataDataprocMetastoreServiceMaintenanceWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDataprocMetastoreServiceMaintenanceWindowElRef {
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
pub struct DataDataprocMetastoreServiceNetworkConfigElConsumersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDataprocMetastoreServiceNetworkConfigElConsumersEl {
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

impl terrars::ToListMappable for DataDataprocMetastoreServiceNetworkConfigElConsumersEl {
    type O = terrars::BlockAssignable<DataDataprocMetastoreServiceNetworkConfigElConsumersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDataprocMetastoreServiceNetworkConfigElConsumersEl {}

impl BuildDataDataprocMetastoreServiceNetworkConfigElConsumersEl {
    pub fn build(self) -> DataDataprocMetastoreServiceNetworkConfigElConsumersEl {
        DataDataprocMetastoreServiceNetworkConfigElConsumersEl {
            endpoint_uri: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
        }
    }
}

pub struct DataDataprocMetastoreServiceNetworkConfigElConsumersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataprocMetastoreServiceNetworkConfigElConsumersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDataprocMetastoreServiceNetworkConfigElConsumersElRef {
        DataDataprocMetastoreServiceNetworkConfigElConsumersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDataprocMetastoreServiceNetworkConfigElConsumersElRef {
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
pub struct DataDataprocMetastoreServiceNetworkConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumers: std::option::Option<terrars::ListField<DataDataprocMetastoreServiceNetworkConfigElConsumersEl>>,
}

impl DataDataprocMetastoreServiceNetworkConfigEl {
    #[doc = "Set the field `consumers`.\n"]
    pub fn set_consumers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataDataprocMetastoreServiceNetworkConfigElConsumersEl>>,
    ) -> Self {
        self.consumers = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDataprocMetastoreServiceNetworkConfigEl {
    type O = terrars::BlockAssignable<DataDataprocMetastoreServiceNetworkConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDataprocMetastoreServiceNetworkConfigEl {}

impl BuildDataDataprocMetastoreServiceNetworkConfigEl {
    pub fn build(self) -> DataDataprocMetastoreServiceNetworkConfigEl {
        DataDataprocMetastoreServiceNetworkConfigEl { consumers: core::default::Default::default() }
    }
}

pub struct DataDataprocMetastoreServiceNetworkConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataprocMetastoreServiceNetworkConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDataprocMetastoreServiceNetworkConfigElRef {
        DataDataprocMetastoreServiceNetworkConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDataprocMetastoreServiceNetworkConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `consumers` after provisioning.\n"]
    pub fn consumers(&self) -> terrars::ListRef<DataDataprocMetastoreServiceNetworkConfigElConsumersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.consumers", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDataprocMetastoreServiceScalingConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_size: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scaling_factor: std::option::Option<terrars::PrimField<f64>>,
}

impl DataDataprocMetastoreServiceScalingConfigEl {
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

impl terrars::ToListMappable for DataDataprocMetastoreServiceScalingConfigEl {
    type O = terrars::BlockAssignable<DataDataprocMetastoreServiceScalingConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDataprocMetastoreServiceScalingConfigEl {}

impl BuildDataDataprocMetastoreServiceScalingConfigEl {
    pub fn build(self) -> DataDataprocMetastoreServiceScalingConfigEl {
        DataDataprocMetastoreServiceScalingConfigEl {
            instance_size: core::default::Default::default(),
            scaling_factor: core::default::Default::default(),
        }
    }
}

pub struct DataDataprocMetastoreServiceScalingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataprocMetastoreServiceScalingConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDataprocMetastoreServiceScalingConfigElRef {
        DataDataprocMetastoreServiceScalingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDataprocMetastoreServiceScalingConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
pub struct DataDataprocMetastoreServiceTelemetryConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_format: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDataprocMetastoreServiceTelemetryConfigEl {
    #[doc = "Set the field `log_format`.\n"]
    pub fn set_log_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_format = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDataprocMetastoreServiceTelemetryConfigEl {
    type O = terrars::BlockAssignable<DataDataprocMetastoreServiceTelemetryConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDataprocMetastoreServiceTelemetryConfigEl {}

impl BuildDataDataprocMetastoreServiceTelemetryConfigEl {
    pub fn build(self) -> DataDataprocMetastoreServiceTelemetryConfigEl {
        DataDataprocMetastoreServiceTelemetryConfigEl { log_format: core::default::Default::default() }
    }
}

pub struct DataDataprocMetastoreServiceTelemetryConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDataprocMetastoreServiceTelemetryConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDataprocMetastoreServiceTelemetryConfigElRef {
        DataDataprocMetastoreServiceTelemetryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDataprocMetastoreServiceTelemetryConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_format` after provisioning.\n"]
    pub fn log_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_format", self.base))
    }
}
