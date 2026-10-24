use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataFusionInstanceData {
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
    dataproc_service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_rbac: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_stackdriver_logging: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_stackdriver_monitoring: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    options: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_instance: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerators: std::option::Option<std::vec::Vec<DataFusionInstanceAcceleratorsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    crypto_key_config: std::option::Option<std::vec::Vec<DataFusionInstanceCryptoKeyConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_publish_config: std::option::Option<std::vec::Vec<DataFusionInstanceEventPublishConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_config: std::option::Option<std::vec::Vec<DataFusionInstanceNetworkConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataFusionInstanceTimeoutsEl>,
    dynamic: DataFusionInstanceDynamic,
}

struct DataFusionInstance_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataFusionInstanceData>,
}

#[derive(Clone)]
pub struct DataFusionInstance(std::rc::Rc<DataFusionInstance_>);

impl DataFusionInstance {
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
        "Set the field `dataproc_service_account`.\nUser-managed service account to set on Dataproc when Cloud Data Fusion creates Dataproc to run data processing pipelines."]
    pub fn set_dataproc_service_account(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().dataproc_service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nAn optional description of the instance."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nDisplay name for an instance."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_rbac`.\nOption to enable granular role-based access control."]
    pub fn set_enable_rbac(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_rbac = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_stackdriver_logging`.\nOption to enable Stackdriver Logging."]
    pub fn set_enable_stackdriver_logging(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_stackdriver_logging = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_stackdriver_monitoring`.\nOption to enable Stackdriver Monitoring."]
    pub fn set_enable_stackdriver_monitoring(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_stackdriver_monitoring = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nThe resource labels for instance to use to annotate any related underlying resources,\nsuch as Compute Engine VMs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `options`.\nMap of additional options used to configure the behavior of Data Fusion instance."]
    pub fn set_options(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().options = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `private_instance`.\nSpecifies whether the Data Fusion instance should be private. If set to\ntrue, all Data Fusion nodes will have private IP addresses and will not be\nable to access the public internet."]
    pub fn set_private_instance(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().private_instance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nThe region of the Data Fusion instance."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\nCurrent version of the Data Fusion."]
    pub fn set_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `zone`.\nName of the zone in which the Data Fusion instance will be created. Only DEVELOPER instances use this field."]
    pub fn set_zone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `accelerators`.\n"]
    pub fn set_accelerators(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataFusionInstanceAcceleratorsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().accelerators = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.accelerators = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `crypto_key_config`.\n"]
    pub fn set_crypto_key_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataFusionInstanceCryptoKeyConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().crypto_key_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.crypto_key_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `event_publish_config`.\n"]
    pub fn set_event_publish_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataFusionInstanceEventPublishConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().event_publish_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.event_publish_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_config`.\n"]
    pub fn set_network_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataFusionInstanceNetworkConfigEl>>,
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DataFusionInstanceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `api_endpoint` after provisioning.\nEndpoint on which the REST APIs is accessible."]
    pub fn api_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time the instance was created in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dataproc_service_account` after provisioning.\nUser-managed service account to set on Dataproc when Cloud Data Fusion creates Dataproc to run data processing pipelines."]
    pub fn dataproc_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataproc_service_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of the instance."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name for an instance."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_rbac` after provisioning.\nOption to enable granular role-based access control."]
    pub fn enable_rbac(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_rbac", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_stackdriver_logging` after provisioning.\nOption to enable Stackdriver Logging."]
    pub fn enable_stackdriver_logging(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_stackdriver_logging", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_stackdriver_monitoring` after provisioning.\nOption to enable Stackdriver Monitoring."]
    pub fn enable_stackdriver_monitoring(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_stackdriver_monitoring", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `gcs_bucket` after provisioning.\nCloud Storage bucket generated by Data Fusion in the customer project."]
    pub fn gcs_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcs_bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe resource labels for instance to use to annotate any related underlying resources,\nsuch as Compute Engine VMs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe ID of the instance or a fully qualified identifier for the instance."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `options` after provisioning.\nMap of additional options used to configure the behavior of Data Fusion instance."]
    pub fn options(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.options", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `p4_service_account` after provisioning.\nP4 service account for the customer project."]
    pub fn p4_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.p4_service_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `private_instance` after provisioning.\nSpecifies whether the Data Fusion instance should be private. If set to\ntrue, all Data Fusion nodes will have private IP addresses and will not be\nable to access the public internet."]
    pub fn private_instance(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_instance", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region of the Data Fusion instance."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_endpoint` after provisioning.\nEndpoint on which the Data Fusion UI and REST APIs are accessible."]
    pub fn service_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe current state of this Data Fusion instance.\n- CREATING: Instance is being created\n- RUNNING: Instance is running and ready for requests\n- FAILED: Instance creation failed\n- DELETING: Instance is being deleted\n- UPGRADING: Instance is being upgraded\n- RESTARTING: Instance is being restarted"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state_message` after provisioning.\nAdditional information about the current state of this Data Fusion instance if available."]
    pub fn state_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_message", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tenant_project_id` after provisioning.\nThe name of the tenant project."]
    pub fn tenant_project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenant_project_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nRepresents the type of Data Fusion instance. Each type is configured with\nthe default settings for processing and memory.\n- BASIC: Basic Data Fusion instance. In Basic type, the user will be able to create data pipelines\nusing point and click UI. However, there are certain limitations, such as fewer number\nof concurrent pipelines, no support for streaming pipelines, etc.\n- ENTERPRISE: Enterprise Data Fusion instance. In Enterprise type, the user will have more features\navailable, such as support for streaming pipelines, higher number of concurrent pipelines, etc.\n- DEVELOPER: Developer Data Fusion instance. In Developer type, the user will have all features available but\nwith restrictive capabilities. This is to help enterprises design and develop their data ingestion and integration\npipelines at low cost. Possible values: [\"BASIC\", \"ENTERPRISE\", \"DEVELOPER\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time the instance was last updated in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\nCurrent version of the Data Fusion."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nName of the zone in which the Data Fusion instance will be created. Only DEVELOPER instances use this field."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `accelerators` after provisioning.\n"]
    pub fn accelerators(&self) -> terrars::ListRef<DataFusionInstanceAcceleratorsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accelerators", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `crypto_key_config` after provisioning.\n"]
    pub fn crypto_key_config(&self) -> terrars::ListRef<DataFusionInstanceCryptoKeyConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.crypto_key_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_publish_config` after provisioning.\n"]
    pub fn event_publish_config(&self) -> terrars::ListRef<DataFusionInstanceEventPublishConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_publish_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> terrars::ListRef<DataFusionInstanceNetworkConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataFusionInstanceTimeoutsElRef {
        DataFusionInstanceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DataFusionInstance {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DataFusionInstance { }

impl terrars::ToListMappable for DataFusionInstance {
    type O = terrars::ListRef<DataFusionInstanceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DataFusionInstance_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_data_fusion_instance".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataFusionInstance {
    pub tf_id: std::string::String,
    #[doc = "The ID of the instance or a fully qualified identifier for the instance."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "Represents the type of Data Fusion instance. Each type is configured with\nthe default settings for processing and memory.\n- BASIC: Basic Data Fusion instance. In Basic type, the user will be able to create data pipelines\nusing point and click UI. However, there are certain limitations, such as fewer number\nof concurrent pipelines, no support for streaming pipelines, etc.\n- ENTERPRISE: Enterprise Data Fusion instance. In Enterprise type, the user will have more features\navailable, such as support for streaming pipelines, higher number of concurrent pipelines, etc.\n- DEVELOPER: Developer Data Fusion instance. In Developer type, the user will have all features available but\nwith restrictive capabilities. This is to help enterprises design and develop their data ingestion and integration\npipelines at low cost. Possible values: [\"BASIC\", \"ENTERPRISE\", \"DEVELOPER\"]"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildDataFusionInstance {
    pub fn build(self, stack: &mut terrars::Stack) -> DataFusionInstance {
        let out = DataFusionInstance(std::rc::Rc::new(DataFusionInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataFusionInstanceData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                dataproc_service_account: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                enable_rbac: core::default::Default::default(),
                enable_stackdriver_logging: core::default::Default::default(),
                enable_stackdriver_monitoring: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: self.name,
                options: core::default::Default::default(),
                private_instance: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                type_: self.type_,
                version: core::default::Default::default(),
                zone: core::default::Default::default(),
                accelerators: core::default::Default::default(),
                crypto_key_config: core::default::Default::default(),
                event_publish_config: core::default::Default::default(),
                network_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DataFusionInstanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFusionInstanceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataFusionInstanceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `api_endpoint` after provisioning.\nEndpoint on which the REST APIs is accessible."]
    pub fn api_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time the instance was created in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dataproc_service_account` after provisioning.\nUser-managed service account to set on Dataproc when Cloud Data Fusion creates Dataproc to run data processing pipelines."]
    pub fn dataproc_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataproc_service_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of the instance."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name for an instance."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_rbac` after provisioning.\nOption to enable granular role-based access control."]
    pub fn enable_rbac(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_rbac", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_stackdriver_logging` after provisioning.\nOption to enable Stackdriver Logging."]
    pub fn enable_stackdriver_logging(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_stackdriver_logging", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_stackdriver_monitoring` after provisioning.\nOption to enable Stackdriver Monitoring."]
    pub fn enable_stackdriver_monitoring(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_stackdriver_monitoring", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `gcs_bucket` after provisioning.\nCloud Storage bucket generated by Data Fusion in the customer project."]
    pub fn gcs_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcs_bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe resource labels for instance to use to annotate any related underlying resources,\nsuch as Compute Engine VMs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe ID of the instance or a fully qualified identifier for the instance."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `options` after provisioning.\nMap of additional options used to configure the behavior of Data Fusion instance."]
    pub fn options(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.options", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `p4_service_account` after provisioning.\nP4 service account for the customer project."]
    pub fn p4_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.p4_service_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `private_instance` after provisioning.\nSpecifies whether the Data Fusion instance should be private. If set to\ntrue, all Data Fusion nodes will have private IP addresses and will not be\nable to access the public internet."]
    pub fn private_instance(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_instance", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region of the Data Fusion instance."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_endpoint` after provisioning.\nEndpoint on which the Data Fusion UI and REST APIs are accessible."]
    pub fn service_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe current state of this Data Fusion instance.\n- CREATING: Instance is being created\n- RUNNING: Instance is running and ready for requests\n- FAILED: Instance creation failed\n- DELETING: Instance is being deleted\n- UPGRADING: Instance is being upgraded\n- RESTARTING: Instance is being restarted"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state_message` after provisioning.\nAdditional information about the current state of this Data Fusion instance if available."]
    pub fn state_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_message", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tenant_project_id` after provisioning.\nThe name of the tenant project."]
    pub fn tenant_project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenant_project_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nRepresents the type of Data Fusion instance. Each type is configured with\nthe default settings for processing and memory.\n- BASIC: Basic Data Fusion instance. In Basic type, the user will be able to create data pipelines\nusing point and click UI. However, there are certain limitations, such as fewer number\nof concurrent pipelines, no support for streaming pipelines, etc.\n- ENTERPRISE: Enterprise Data Fusion instance. In Enterprise type, the user will have more features\navailable, such as support for streaming pipelines, higher number of concurrent pipelines, etc.\n- DEVELOPER: Developer Data Fusion instance. In Developer type, the user will have all features available but\nwith restrictive capabilities. This is to help enterprises design and develop their data ingestion and integration\npipelines at low cost. Possible values: [\"BASIC\", \"ENTERPRISE\", \"DEVELOPER\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time the instance was last updated in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\nCurrent version of the Data Fusion."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `zone` after provisioning.\nName of the zone in which the Data Fusion instance will be created. Only DEVELOPER instances use this field."]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `accelerators` after provisioning.\n"]
    pub fn accelerators(&self) -> terrars::ListRef<DataFusionInstanceAcceleratorsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accelerators", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `crypto_key_config` after provisioning.\n"]
    pub fn crypto_key_config(&self) -> terrars::ListRef<DataFusionInstanceCryptoKeyConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.crypto_key_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_publish_config` after provisioning.\n"]
    pub fn event_publish_config(&self) -> terrars::ListRef<DataFusionInstanceEventPublishConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_publish_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> terrars::ListRef<DataFusionInstanceNetworkConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataFusionInstanceTimeoutsElRef {
        DataFusionInstanceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataFusionInstanceAcceleratorsEl {
    accelerator_type: terrars::PrimField<std::string::String>,
    state: terrars::PrimField<std::string::String>,
}

impl DataFusionInstanceAcceleratorsEl { }

impl terrars::ToListMappable for DataFusionInstanceAcceleratorsEl {
    type O = terrars::BlockAssignable<DataFusionInstanceAcceleratorsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFusionInstanceAcceleratorsEl {
    #[doc =
        "The type of an accelator for a CDF instance. Possible values: [\"CDC\", \"HEALTHCARE\", \"CCAI_INSIGHTS\"]"]
    pub accelerator_type: terrars::PrimField<std::string::String>,
    #[doc = "The type of an accelator for a CDF instance. Possible values: [\"ENABLED\", \"DISABLED\"]"]
    pub state: terrars::PrimField<std::string::String>,
}

impl BuildDataFusionInstanceAcceleratorsEl {
    pub fn build(self) -> DataFusionInstanceAcceleratorsEl {
        DataFusionInstanceAcceleratorsEl {
            accelerator_type: self.accelerator_type,
            state: self.state,
        }
    }
}

pub struct DataFusionInstanceAcceleratorsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFusionInstanceAcceleratorsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataFusionInstanceAcceleratorsElRef {
        DataFusionInstanceAcceleratorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFusionInstanceAcceleratorsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `accelerator_type` after provisioning.\nThe type of an accelator for a CDF instance. Possible values: [\"CDC\", \"HEALTHCARE\", \"CCAI_INSIGHTS\"]"]
    pub fn accelerator_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe type of an accelator for a CDF instance. Possible values: [\"ENABLED\", \"DISABLED\"]"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFusionInstanceCryptoKeyConfigEl {
    key_reference: terrars::PrimField<std::string::String>,
}

impl DataFusionInstanceCryptoKeyConfigEl { }

impl terrars::ToListMappable for DataFusionInstanceCryptoKeyConfigEl {
    type O = terrars::BlockAssignable<DataFusionInstanceCryptoKeyConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFusionInstanceCryptoKeyConfigEl {
    #[doc =
        "The name of the key which is used to encrypt/decrypt customer data. For key in Cloud KMS, the key should be in the format of projects/*/locations/*/keyRings/*/cryptoKeys/*."]
    pub key_reference: terrars::PrimField<std::string::String>,
}

impl BuildDataFusionInstanceCryptoKeyConfigEl {
    pub fn build(self) -> DataFusionInstanceCryptoKeyConfigEl {
        DataFusionInstanceCryptoKeyConfigEl { key_reference: self.key_reference }
    }
}

pub struct DataFusionInstanceCryptoKeyConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFusionInstanceCryptoKeyConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataFusionInstanceCryptoKeyConfigElRef {
        DataFusionInstanceCryptoKeyConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFusionInstanceCryptoKeyConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `key_reference` after provisioning.\nThe name of the key which is used to encrypt/decrypt customer data. For key in Cloud KMS, the key should be in the format of projects/*/locations/*/keyRings/*/cryptoKeys/*."]
    pub fn key_reference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_reference", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFusionInstanceEventPublishConfigEl {
    enabled: terrars::PrimField<bool>,
    topic: terrars::PrimField<std::string::String>,
}

impl DataFusionInstanceEventPublishConfigEl { }

impl terrars::ToListMappable for DataFusionInstanceEventPublishConfigEl {
    type O = terrars::BlockAssignable<DataFusionInstanceEventPublishConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFusionInstanceEventPublishConfigEl {
    #[doc = "Option to enable Event Publishing."]
    pub enabled: terrars::PrimField<bool>,
    #[doc = "The resource name of the Pub/Sub topic. Format: projects/{projectId}/topics/{topic_id}"]
    pub topic: terrars::PrimField<std::string::String>,
}

impl BuildDataFusionInstanceEventPublishConfigEl {
    pub fn build(self) -> DataFusionInstanceEventPublishConfigEl {
        DataFusionInstanceEventPublishConfigEl {
            enabled: self.enabled,
            topic: self.topic,
        }
    }
}

pub struct DataFusionInstanceEventPublishConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFusionInstanceEventPublishConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataFusionInstanceEventPublishConfigElRef {
        DataFusionInstanceEventPublishConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFusionInstanceEventPublishConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nOption to enable Event Publishing."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `topic` after provisioning.\nThe resource name of the Pub/Sub topic. Format: projects/{projectId}/topics/{topic_id}"]
    pub fn topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFusionInstanceNetworkConfigEl {
    ip_allocation: terrars::PrimField<std::string::String>,
    network: terrars::PrimField<std::string::String>,
}

impl DataFusionInstanceNetworkConfigEl { }

impl terrars::ToListMappable for DataFusionInstanceNetworkConfigEl {
    type O = terrars::BlockAssignable<DataFusionInstanceNetworkConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFusionInstanceNetworkConfigEl {
    #[doc =
        "The IP range in CIDR notation to use for the managed Data Fusion instance\nnodes. This range must not overlap with any other ranges used in the Data Fusion instance network."]
    pub ip_allocation: terrars::PrimField<std::string::String>,
    #[doc =
        "Name of the network in the project with which the tenant project\nwill be peered for executing pipelines. In case of shared VPC where the network resides in another host\nproject the network should specified in the form of projects/{host-project-id}/global/networks/{network}"]
    pub network: terrars::PrimField<std::string::String>,
}

impl BuildDataFusionInstanceNetworkConfigEl {
    pub fn build(self) -> DataFusionInstanceNetworkConfigEl {
        DataFusionInstanceNetworkConfigEl {
            ip_allocation: self.ip_allocation,
            network: self.network,
        }
    }
}

pub struct DataFusionInstanceNetworkConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFusionInstanceNetworkConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataFusionInstanceNetworkConfigElRef {
        DataFusionInstanceNetworkConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFusionInstanceNetworkConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ip_allocation` after provisioning.\nThe IP range in CIDR notation to use for the managed Data Fusion instance\nnodes. This range must not overlap with any other ranges used in the Data Fusion instance network."]
    pub fn ip_allocation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_allocation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nName of the network in the project with which the tenant project\nwill be peered for executing pipelines. In case of shared VPC where the network resides in another host\nproject the network should specified in the form of projects/{host-project-id}/global/networks/{network}"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFusionInstanceTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataFusionInstanceTimeoutsEl {
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

impl terrars::ToListMappable for DataFusionInstanceTimeoutsEl {
    type O = terrars::BlockAssignable<DataFusionInstanceTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFusionInstanceTimeoutsEl {}

impl BuildDataFusionInstanceTimeoutsEl {
    pub fn build(self) -> DataFusionInstanceTimeoutsEl {
        DataFusionInstanceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct DataFusionInstanceTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFusionInstanceTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataFusionInstanceTimeoutsElRef {
        DataFusionInstanceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFusionInstanceTimeoutsElRef {
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
struct DataFusionInstanceDynamic {
    accelerators: std::option::Option<terrars::DynamicBlock<DataFusionInstanceAcceleratorsEl>>,
    crypto_key_config: std::option::Option<terrars::DynamicBlock<DataFusionInstanceCryptoKeyConfigEl>>,
    event_publish_config: std::option::Option<terrars::DynamicBlock<DataFusionInstanceEventPublishConfigEl>>,
    network_config: std::option::Option<terrars::DynamicBlock<DataFusionInstanceNetworkConfigEl>>,
}
