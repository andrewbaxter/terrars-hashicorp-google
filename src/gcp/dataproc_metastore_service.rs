use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataprocMetastoreServiceData {
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
    tier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_config: std::option::Option<std::vec::Vec<DataprocMetastoreServiceEncryptionConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hive_metastore_config: std::option::Option<std::vec::Vec<DataprocMetastoreServiceHiveMetastoreConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_window: std::option::Option<std::vec::Vec<DataprocMetastoreServiceMaintenanceWindowEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_config: std::option::Option<std::vec::Vec<DataprocMetastoreServiceNetworkConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scaling_config: std::option::Option<std::vec::Vec<DataprocMetastoreServiceScalingConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    telemetry_config: std::option::Option<std::vec::Vec<DataprocMetastoreServiceTelemetryConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataprocMetastoreServiceTimeoutsEl>,
    dynamic: DataprocMetastoreServiceDynamic,
}

struct DataprocMetastoreService_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataprocMetastoreServiceData>,
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

    #[doc = "Set the field `tier`.\nThe tier of the service. Possible values: [\"DEVELOPER\", \"ENTERPRISE\"]"]
    pub fn set_tier(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_config`.\n"]
    pub fn set_encryption_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocMetastoreServiceEncryptionConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocMetastoreServiceHiveMetastoreConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocMetastoreServiceMaintenanceWindowEl>>,
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

    #[doc = "Set the field `network_config`.\n"]
    pub fn set_network_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocMetastoreServiceNetworkConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocMetastoreServiceScalingConfigEl>>,
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

    #[doc = "Set the field `telemetry_config`.\n"]
    pub fn set_telemetry_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocMetastoreServiceTelemetryConfigEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<DataprocMetastoreServiceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> terrars::ListRef<DataprocMetastoreServiceEncryptionConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hive_metastore_config` after provisioning.\n"]
    pub fn hive_metastore_config(&self) -> terrars::ListRef<DataprocMetastoreServiceHiveMetastoreConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hive_metastore_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> terrars::ListRef<DataprocMetastoreServiceMaintenanceWindowElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> terrars::ListRef<DataprocMetastoreServiceNetworkConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scaling_config` after provisioning.\n"]
    pub fn scaling_config(&self) -> terrars::ListRef<DataprocMetastoreServiceScalingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scaling_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `telemetry_config` after provisioning.\n"]
    pub fn telemetry_config(&self) -> terrars::ListRef<DataprocMetastoreServiceTelemetryConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.telemetry_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataprocMetastoreServiceTimeoutsElRef {
        DataprocMetastoreServiceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
            data: std::cell::RefCell::new(DataprocMetastoreServiceData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                database_type: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: core::default::Default::default(),
                network: core::default::Default::default(),
                port: core::default::Default::default(),
                project: core::default::Default::default(),
                release_channel: core::default::Default::default(),
                service_id: self.service_id,
                tier: core::default::Default::default(),
                encryption_config: core::default::Default::default(),
                hive_metastore_config: core::default::Default::default(),
                maintenance_window: core::default::Default::default(),
                network_config: core::default::Default::default(),
                scaling_config: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `encryption_config` after provisioning.\n"]
    pub fn encryption_config(&self) -> terrars::ListRef<DataprocMetastoreServiceEncryptionConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hive_metastore_config` after provisioning.\n"]
    pub fn hive_metastore_config(&self) -> terrars::ListRef<DataprocMetastoreServiceHiveMetastoreConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hive_metastore_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> terrars::ListRef<DataprocMetastoreServiceMaintenanceWindowElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_window", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> terrars::ListRef<DataprocMetastoreServiceNetworkConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scaling_config` after provisioning.\n"]
    pub fn scaling_config(&self) -> terrars::ListRef<DataprocMetastoreServiceScalingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scaling_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `telemetry_config` after provisioning.\n"]
    pub fn telemetry_config(&self) -> terrars::ListRef<DataprocMetastoreServiceTelemetryConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.telemetry_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataprocMetastoreServiceTimeoutsElRef {
        DataprocMetastoreServiceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataprocMetastoreServiceEncryptionConfigEl {
    kms_key: terrars::PrimField<std::string::String>,
}

impl DataprocMetastoreServiceEncryptionConfigEl { }

impl terrars::ToListMappable for DataprocMetastoreServiceEncryptionConfigEl {
    type O = terrars::BlockAssignable<DataprocMetastoreServiceEncryptionConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocMetastoreServiceEncryptionConfigEl {
    #[doc =
        "The fully qualified customer provided Cloud KMS key name to use for customer data encryption.\nUse the following format: 'projects/([^/]+)/locations/([^/]+)/keyRings/([^/]+)/cryptoKeys/([^/]+)'"]
    pub kms_key: terrars::PrimField<std::string::String>,
}

impl BuildDataprocMetastoreServiceEncryptionConfigEl {
    pub fn build(self) -> DataprocMetastoreServiceEncryptionConfigEl {
        DataprocMetastoreServiceEncryptionConfigEl { kms_key: self.kms_key }
    }
}

pub struct DataprocMetastoreServiceEncryptionConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocMetastoreServiceEncryptionConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocMetastoreServiceEncryptionConfigElRef {
        DataprocMetastoreServiceEncryptionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocMetastoreServiceEncryptionConfigElRef {
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
pub struct DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabEl {
    cloud_secret: terrars::PrimField<std::string::String>,
}

impl DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabEl { }

impl terrars::ToListMappable for DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabEl {
    type O = terrars::BlockAssignable<DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabEl {
    #[doc =
        "The relative resource name of a Secret Manager secret version, in the following form:\n\n\"projects/{projectNumber}/secrets/{secret_id}/versions/{version_id}\"."]
    pub cloud_secret: terrars::PrimField<std::string::String>,
}

impl BuildDataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabEl {
    pub fn build(self) -> DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabEl {
        DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabEl { cloud_secret: self.cloud_secret }
    }
}

pub struct DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabElRef {
        DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabElRef {
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
struct DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElDynamic {
    keytab: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabEl>>,
}

#[derive(Serialize)]
pub struct DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigEl {
    krb5_config_gcs_uri: terrars::PrimField<std::string::String>,
    principal: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keytab: std::option::Option<std::vec::Vec<DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabEl>>,
    dynamic: DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElDynamic,
}

impl DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigEl {
    #[doc = "Set the field `keytab`.\n"]
    pub fn set_keytab(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabEl>,
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

impl terrars::ToListMappable for DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigEl {
    type O = terrars::BlockAssignable<DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigEl {
    #[doc =
        "A Cloud Storage URI that specifies the path to a krb5.conf file. It is of the form gs://{bucket_name}/path/to/krb5.conf, although the file does not need to be named krb5.conf explicitly."]
    pub krb5_config_gcs_uri: terrars::PrimField<std::string::String>,
    #[doc =
        "A Kerberos principal that exists in the both the keytab the KDC to authenticate as. A typical principal is of the form \"primary/instance@REALM\", but there is no exact format."]
    pub principal: terrars::PrimField<std::string::String>,
}

impl BuildDataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigEl {
    pub fn build(self) -> DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigEl {
        DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigEl {
            krb5_config_gcs_uri: self.krb5_config_gcs_uri,
            principal: self.principal,
            keytab: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElRef {
        DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElRef {
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
    pub fn keytab(&self) -> terrars::ListRef<DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElKeytabElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.keytab", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataprocMetastoreServiceHiveMetastoreConfigElDynamic {
    kerberos_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigEl>>,
}

#[derive(Serialize)]
pub struct DataprocMetastoreServiceHiveMetastoreConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config_overrides: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kerberos_config: std::option::Option<std::vec::Vec<DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigEl>>,
    dynamic: DataprocMetastoreServiceHiveMetastoreConfigElDynamic,
}

impl DataprocMetastoreServiceHiveMetastoreConfigEl {
    #[doc =
        "Set the field `config_overrides`.\nA mapping of Hive metastore configuration key-value pairs to apply to the Hive metastore (configured in hive-site.xml).\nThe mappings override system defaults (some keys cannot be overridden)"]
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
                    ::Into<terrars::BlockAssignable<DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigEl>>,
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

impl terrars::ToListMappable for DataprocMetastoreServiceHiveMetastoreConfigEl {
    type O = terrars::BlockAssignable<DataprocMetastoreServiceHiveMetastoreConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocMetastoreServiceHiveMetastoreConfigEl {
    #[doc = "The Hive metastore schema version."]
    pub version: terrars::PrimField<std::string::String>,
}

impl BuildDataprocMetastoreServiceHiveMetastoreConfigEl {
    pub fn build(self) -> DataprocMetastoreServiceHiveMetastoreConfigEl {
        DataprocMetastoreServiceHiveMetastoreConfigEl {
            config_overrides: core::default::Default::default(),
            version: self.version,
            kerberos_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataprocMetastoreServiceHiveMetastoreConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocMetastoreServiceHiveMetastoreConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocMetastoreServiceHiveMetastoreConfigElRef {
        DataprocMetastoreServiceHiveMetastoreConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocMetastoreServiceHiveMetastoreConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `config_overrides` after provisioning.\nA mapping of Hive metastore configuration key-value pairs to apply to the Hive metastore (configured in hive-site.xml).\nThe mappings override system defaults (some keys cannot be overridden)"]
    pub fn config_overrides(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.config_overrides", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\nThe Hive metastore schema version."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc = "Get a reference to the value of field `kerberos_config` after provisioning.\n"]
    pub fn kerberos_config(&self) -> terrars::ListRef<DataprocMetastoreServiceHiveMetastoreConfigElKerberosConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kerberos_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocMetastoreServiceMaintenanceWindowEl {
    day_of_week: terrars::PrimField<std::string::String>,
    hour_of_day: terrars::PrimField<f64>,
}

impl DataprocMetastoreServiceMaintenanceWindowEl { }

impl terrars::ToListMappable for DataprocMetastoreServiceMaintenanceWindowEl {
    type O = terrars::BlockAssignable<DataprocMetastoreServiceMaintenanceWindowEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocMetastoreServiceMaintenanceWindowEl {
    #[doc =
        "The day of week, when the window starts. Possible values: [\"MONDAY\", \"TUESDAY\", \"WEDNESDAY\", \"THURSDAY\", \"FRIDAY\", \"SATURDAY\", \"SUNDAY\"]"]
    pub day_of_week: terrars::PrimField<std::string::String>,
    #[doc = "The hour of day (0-23) when the window starts."]
    pub hour_of_day: terrars::PrimField<f64>,
}

impl BuildDataprocMetastoreServiceMaintenanceWindowEl {
    pub fn build(self) -> DataprocMetastoreServiceMaintenanceWindowEl {
        DataprocMetastoreServiceMaintenanceWindowEl {
            day_of_week: self.day_of_week,
            hour_of_day: self.hour_of_day,
        }
    }
}

pub struct DataprocMetastoreServiceMaintenanceWindowElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocMetastoreServiceMaintenanceWindowElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocMetastoreServiceMaintenanceWindowElRef {
        DataprocMetastoreServiceMaintenanceWindowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocMetastoreServiceMaintenanceWindowElRef {
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
pub struct DataprocMetastoreServiceNetworkConfigElConsumersEl {
    subnetwork: terrars::PrimField<std::string::String>,
}

impl DataprocMetastoreServiceNetworkConfigElConsumersEl { }

impl terrars::ToListMappable for DataprocMetastoreServiceNetworkConfigElConsumersEl {
    type O = terrars::BlockAssignable<DataprocMetastoreServiceNetworkConfigElConsumersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocMetastoreServiceNetworkConfigElConsumersEl {
    #[doc =
        "The subnetwork of the customer project from which an IP address is reserved and used as the Dataproc Metastore service's endpoint.\nIt is accessible to hosts in the subnet and to all hosts in a subnet in the same region and same network.\nThere must be at least one IP address available in the subnet's primary range. The subnet is specified in the following form:\n'projects/{projectNumber}/regions/{region_id}/subnetworks/{subnetwork_id}"]
    pub subnetwork: terrars::PrimField<std::string::String>,
}

impl BuildDataprocMetastoreServiceNetworkConfigElConsumersEl {
    pub fn build(self) -> DataprocMetastoreServiceNetworkConfigElConsumersEl {
        DataprocMetastoreServiceNetworkConfigElConsumersEl { subnetwork: self.subnetwork }
    }
}

pub struct DataprocMetastoreServiceNetworkConfigElConsumersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocMetastoreServiceNetworkConfigElConsumersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataprocMetastoreServiceNetworkConfigElConsumersElRef {
        DataprocMetastoreServiceNetworkConfigElConsumersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocMetastoreServiceNetworkConfigElConsumersElRef {
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
struct DataprocMetastoreServiceNetworkConfigElDynamic {
    consumers: std::option::Option<terrars::DynamicBlock<DataprocMetastoreServiceNetworkConfigElConsumersEl>>,
}

#[derive(Serialize)]
pub struct DataprocMetastoreServiceNetworkConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumers: std::option::Option<std::vec::Vec<DataprocMetastoreServiceNetworkConfigElConsumersEl>>,
    dynamic: DataprocMetastoreServiceNetworkConfigElDynamic,
}

impl DataprocMetastoreServiceNetworkConfigEl {
    #[doc = "Set the field `consumers`.\n"]
    pub fn set_consumers(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataprocMetastoreServiceNetworkConfigElConsumersEl>>,
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

impl terrars::ToListMappable for DataprocMetastoreServiceNetworkConfigEl {
    type O = terrars::BlockAssignable<DataprocMetastoreServiceNetworkConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocMetastoreServiceNetworkConfigEl {}

impl BuildDataprocMetastoreServiceNetworkConfigEl {
    pub fn build(self) -> DataprocMetastoreServiceNetworkConfigEl {
        DataprocMetastoreServiceNetworkConfigEl {
            consumers: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataprocMetastoreServiceNetworkConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocMetastoreServiceNetworkConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocMetastoreServiceNetworkConfigElRef {
        DataprocMetastoreServiceNetworkConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocMetastoreServiceNetworkConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `consumers` after provisioning.\n"]
    pub fn consumers(&self) -> terrars::ListRef<DataprocMetastoreServiceNetworkConfigElConsumersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.consumers", self.base))
    }
}

#[derive(Serialize)]
pub struct DataprocMetastoreServiceScalingConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_size: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scaling_factor: std::option::Option<terrars::PrimField<f64>>,
}

impl DataprocMetastoreServiceScalingConfigEl {
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
}

impl terrars::ToListMappable for DataprocMetastoreServiceScalingConfigEl {
    type O = terrars::BlockAssignable<DataprocMetastoreServiceScalingConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocMetastoreServiceScalingConfigEl {}

impl BuildDataprocMetastoreServiceScalingConfigEl {
    pub fn build(self) -> DataprocMetastoreServiceScalingConfigEl {
        DataprocMetastoreServiceScalingConfigEl {
            instance_size: core::default::Default::default(),
            scaling_factor: core::default::Default::default(),
        }
    }
}

pub struct DataprocMetastoreServiceScalingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocMetastoreServiceScalingConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocMetastoreServiceScalingConfigElRef {
        DataprocMetastoreServiceScalingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocMetastoreServiceScalingConfigElRef {
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
}

#[derive(Serialize)]
pub struct DataprocMetastoreServiceTelemetryConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_format: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocMetastoreServiceTelemetryConfigEl {
    #[doc =
        "Set the field `log_format`.\nThe output format of the Dataproc Metastore service's logs. Default value: \"JSON\" Possible values: [\"LEGACY\", \"JSON\"]"]
    pub fn set_log_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_format = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataprocMetastoreServiceTelemetryConfigEl {
    type O = terrars::BlockAssignable<DataprocMetastoreServiceTelemetryConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocMetastoreServiceTelemetryConfigEl {}

impl BuildDataprocMetastoreServiceTelemetryConfigEl {
    pub fn build(self) -> DataprocMetastoreServiceTelemetryConfigEl {
        DataprocMetastoreServiceTelemetryConfigEl { log_format: core::default::Default::default() }
    }
}

pub struct DataprocMetastoreServiceTelemetryConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocMetastoreServiceTelemetryConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocMetastoreServiceTelemetryConfigElRef {
        DataprocMetastoreServiceTelemetryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocMetastoreServiceTelemetryConfigElRef {
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
pub struct DataprocMetastoreServiceTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataprocMetastoreServiceTimeoutsEl {
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

impl terrars::ToListMappable for DataprocMetastoreServiceTimeoutsEl {
    type O = terrars::BlockAssignable<DataprocMetastoreServiceTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataprocMetastoreServiceTimeoutsEl {}

impl BuildDataprocMetastoreServiceTimeoutsEl {
    pub fn build(self) -> DataprocMetastoreServiceTimeoutsEl {
        DataprocMetastoreServiceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct DataprocMetastoreServiceTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataprocMetastoreServiceTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataprocMetastoreServiceTimeoutsElRef {
        DataprocMetastoreServiceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataprocMetastoreServiceTimeoutsElRef {
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
struct DataprocMetastoreServiceDynamic {
    encryption_config: std::option::Option<terrars::DynamicBlock<DataprocMetastoreServiceEncryptionConfigEl>>,
    hive_metastore_config: std::option::Option<terrars::DynamicBlock<DataprocMetastoreServiceHiveMetastoreConfigEl>>,
    maintenance_window: std::option::Option<terrars::DynamicBlock<DataprocMetastoreServiceMaintenanceWindowEl>>,
    network_config: std::option::Option<terrars::DynamicBlock<DataprocMetastoreServiceNetworkConfigEl>>,
    scaling_config: std::option::Option<terrars::DynamicBlock<DataprocMetastoreServiceScalingConfigEl>>,
    telemetry_config: std::option::Option<terrars::DynamicBlock<DataprocMetastoreServiceTelemetryConfigEl>>,
}
