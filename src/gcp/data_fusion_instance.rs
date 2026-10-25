#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataFusionInstanceData_ {
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
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
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
    patch_revision: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_instance: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerators: std::option::Option<std::vec::Vec<DataFusionInstance_Accelerators>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    crypto_key_config: std::option::Option<std::vec::Vec<DataFusionInstance_CryptoKeyConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_publish_config: std::option::Option<std::vec::Vec<DataFusionInstance_EventPublishConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_policy: std::option::Option<std::vec::Vec<DataFusionInstance_MaintenancePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_config: std::option::Option<std::vec::Vec<DataFusionInstance_NetworkConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataFusionInstance_Timeouts>,
    dynamic: DataFusionInstanceDynamic,
}

struct DataFusionInstance_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataFusionInstanceData_>,
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

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `patch_revision`.\nCurrent patch revision of the Data Fusion."]
    pub fn set_patch_revision(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().patch_revision = std::option::Option::Some(v.into());
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

    #[doc =
        "Set the field `tags`.\nA map of resource manager tags.\nResource manager tag keys and values have the same definition as resource manager tags.\nKeys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/{tag_value_id}.\nThe field is ignored (both PUT & PATCH) when empty."]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataFusionInstance_Accelerators>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataFusionInstance_CryptoKeyConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataFusionInstance_EventPublishConfig>>,
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

    #[doc = "Set the field `maintenance_policy`.\n"]
    pub fn set_maintenance_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataFusionInstance_MaintenancePolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().maintenance_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.maintenance_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_config`.\n"]
    pub fn set_network_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataFusionInstance_NetworkConfig>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<DataFusionInstance_Timeouts>) -> Self {
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
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
        "Get a reference to the value of field `maintenance_events` after provisioning.\nThe maintenance events for this instance."]
    pub fn maintenance_events(&self) -> terrars::ListRef<DataFusionInstance_MaintenanceEventsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_events", self.extract_ref()))
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
        "Get a reference to the value of field `patch_revision` after provisioning.\nCurrent patch revision of the Data Fusion."]
    pub fn patch_revision(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.patch_revision", self.extract_ref()))
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
        "Get a reference to the value of field `tags` after provisioning.\nA map of resource manager tags.\nResource manager tag keys and values have the same definition as resource manager tags.\nKeys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/{tag_value_id}.\nThe field is ignored (both PUT & PATCH) when empty."]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
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
    pub fn accelerators(&self) -> terrars::ListRef<DataFusionInstance_AcceleratorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accelerators", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `crypto_key_config` after provisioning.\n"]
    pub fn crypto_key_config(&self) -> terrars::ListRef<DataFusionInstance_CryptoKeyConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.crypto_key_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_publish_config` after provisioning.\n"]
    pub fn event_publish_config(&self) -> terrars::ListRef<DataFusionInstance_EventPublishConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_publish_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> terrars::ListRef<DataFusionInstance_MaintenancePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> terrars::ListRef<DataFusionInstance_NetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataFusionInstance_TimeoutsRef {
        DataFusionInstance_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
            data: std::cell::RefCell::new(DataFusionInstanceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                dataproc_service_account: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                enable_rbac: core::default::Default::default(),
                enable_stackdriver_logging: core::default::Default::default(),
                enable_stackdriver_monitoring: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: self.name,
                options: core::default::Default::default(),
                patch_revision: core::default::Default::default(),
                private_instance: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                type_: self.type_,
                version: core::default::Default::default(),
                zone: core::default::Default::default(),
                accelerators: core::default::Default::default(),
                crypto_key_config: core::default::Default::default(),
                event_publish_config: core::default::Default::default(),
                maintenance_policy: core::default::Default::default(),
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
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
        "Get a reference to the value of field `maintenance_events` after provisioning.\nThe maintenance events for this instance."]
    pub fn maintenance_events(&self) -> terrars::ListRef<DataFusionInstance_MaintenanceEventsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_events", self.extract_ref()))
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
        "Get a reference to the value of field `patch_revision` after provisioning.\nCurrent patch revision of the Data Fusion."]
    pub fn patch_revision(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.patch_revision", self.extract_ref()))
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
        "Get a reference to the value of field `tags` after provisioning.\nA map of resource manager tags.\nResource manager tag keys and values have the same definition as resource manager tags.\nKeys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/{tag_value_id}.\nThe field is ignored (both PUT & PATCH) when empty."]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
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
    pub fn accelerators(&self) -> terrars::ListRef<DataFusionInstance_AcceleratorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accelerators", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `crypto_key_config` after provisioning.\n"]
    pub fn crypto_key_config(&self) -> terrars::ListRef<DataFusionInstance_CryptoKeyConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.crypto_key_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_publish_config` after provisioning.\n"]
    pub fn event_publish_config(&self) -> terrars::ListRef<DataFusionInstance_EventPublishConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_publish_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_policy` after provisioning.\n"]
    pub fn maintenance_policy(&self) -> terrars::ListRef<DataFusionInstance_MaintenancePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_config` after provisioning.\n"]
    pub fn network_config(&self) -> terrars::ListRef<DataFusionInstance_NetworkConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataFusionInstance_TimeoutsRef {
        DataFusionInstance_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataFusionInstanceDynamic {
    accelerators: std::option::Option<terrars::DynamicBlock<DataFusionInstance_Accelerators>>,
    crypto_key_config: std::option::Option<terrars::DynamicBlock<DataFusionInstance_CryptoKeyConfig>>,
    event_publish_config: std::option::Option<terrars::DynamicBlock<DataFusionInstance_EventPublishConfig>>,
    maintenance_policy: std::option::Option<terrars::DynamicBlock<DataFusionInstance_MaintenancePolicy>>,
    network_config: std::option::Option<terrars::DynamicBlock<DataFusionInstance_NetworkConfig>>,
}

#[derive(Serialize)]
pub struct DataFusionInstance_MaintenanceEvents {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataFusionInstance_MaintenanceEvents {
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

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataFusionInstance_MaintenanceEvents {
    type O = terrars::BlockAssignable<DataFusionInstance_MaintenanceEvents>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFusionInstance_MaintenanceEvents {}

impl BuildDataFusionInstance_MaintenanceEvents {
    pub fn build(self) -> DataFusionInstance_MaintenanceEvents {
        DataFusionInstance_MaintenanceEvents {
            end_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataFusionInstance_MaintenanceEventsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFusionInstance_MaintenanceEventsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataFusionInstance_MaintenanceEventsRef {
        DataFusionInstance_MaintenanceEventsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFusionInstance_MaintenanceEventsRef {
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

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFusionInstance_Accelerators {
    accelerator_type: terrars::PrimField<std::string::String>,
    state: terrars::PrimField<std::string::String>,
}

impl DataFusionInstance_Accelerators { }

impl terrars::ToListMappable for DataFusionInstance_Accelerators {
    type O = terrars::BlockAssignable<DataFusionInstance_Accelerators>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFusionInstance_Accelerators {
    #[doc =
        "The type of an accelator for a CDF instance. Possible values: [\"CDC\", \"HEALTHCARE\", \"CCAI_INSIGHTS\"]"]
    pub accelerator_type: terrars::PrimField<std::string::String>,
    #[doc = "The type of an accelator for a CDF instance. Possible values: [\"ENABLED\", \"DISABLED\"]"]
    pub state: terrars::PrimField<std::string::String>,
}

impl BuildDataFusionInstance_Accelerators {
    pub fn build(self) -> DataFusionInstance_Accelerators {
        DataFusionInstance_Accelerators {
            accelerator_type: self.accelerator_type,
            state: self.state,
        }
    }
}

#[allow(dead_code)]
pub struct DataFusionInstance_AcceleratorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFusionInstance_AcceleratorsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataFusionInstance_AcceleratorsRef {
        DataFusionInstance_AcceleratorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFusionInstance_AcceleratorsRef {
    #[allow(dead_code)]
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
pub struct DataFusionInstance_CryptoKeyConfig {
    key_reference: terrars::PrimField<std::string::String>,
}

impl DataFusionInstance_CryptoKeyConfig { }

impl terrars::ToListMappable for DataFusionInstance_CryptoKeyConfig {
    type O = terrars::BlockAssignable<DataFusionInstance_CryptoKeyConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFusionInstance_CryptoKeyConfig {
    #[doc =
        "The name of the key which is used to encrypt/decrypt customer data. For key in Cloud KMS, the key should be in the format of projects/*/locations/*/keyRings/*/cryptoKeys/*."]
    pub key_reference: terrars::PrimField<std::string::String>,
}

impl BuildDataFusionInstance_CryptoKeyConfig {
    pub fn build(self) -> DataFusionInstance_CryptoKeyConfig {
        DataFusionInstance_CryptoKeyConfig { key_reference: self.key_reference }
    }
}

#[allow(dead_code)]
pub struct DataFusionInstance_CryptoKeyConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFusionInstance_CryptoKeyConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataFusionInstance_CryptoKeyConfigRef {
        DataFusionInstance_CryptoKeyConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFusionInstance_CryptoKeyConfigRef {
    #[allow(dead_code)]
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
pub struct DataFusionInstance_EventPublishConfig {
    enabled: terrars::PrimField<bool>,
    topic: terrars::PrimField<std::string::String>,
}

impl DataFusionInstance_EventPublishConfig { }

impl terrars::ToListMappable for DataFusionInstance_EventPublishConfig {
    type O = terrars::BlockAssignable<DataFusionInstance_EventPublishConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFusionInstance_EventPublishConfig {
    #[doc = "Option to enable Event Publishing."]
    pub enabled: terrars::PrimField<bool>,
    #[doc = "The resource name of the Pub/Sub topic. Format: projects/{projectId}/topics/{topic_id}"]
    pub topic: terrars::PrimField<std::string::String>,
}

impl BuildDataFusionInstance_EventPublishConfig {
    pub fn build(self) -> DataFusionInstance_EventPublishConfig {
        DataFusionInstance_EventPublishConfig {
            enabled: self.enabled,
            topic: self.topic,
        }
    }
}

#[allow(dead_code)]
pub struct DataFusionInstance_EventPublishConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFusionInstance_EventPublishConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataFusionInstance_EventPublishConfigRef {
        DataFusionInstance_EventPublishConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFusionInstance_EventPublishConfigRef {
    #[allow(dead_code)]
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
pub struct DataFusionInstance_MaintenancePolicy_MaintenanceWindow_RecurringTimeWindow_Window {
    end_time: terrars::PrimField<std::string::String>,
    start_time: terrars::PrimField<std::string::String>,
}

impl DataFusionInstance_MaintenancePolicy_MaintenanceWindow_RecurringTimeWindow_Window { }

impl terrars::ToListMappable for DataFusionInstance_MaintenancePolicy_MaintenanceWindow_RecurringTimeWindow_Window {
    type O =
        terrars::BlockAssignable<DataFusionInstance_MaintenancePolicy_MaintenanceWindow_RecurringTimeWindow_Window>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFusionInstance_MaintenancePolicy_MaintenanceWindow_RecurringTimeWindow_Window {
    #[doc = "The end time of the time window provided in RFC 3339 format."]
    pub end_time: terrars::PrimField<std::string::String>,
    #[doc = "The start time of the time window provided in RFC 3339 format."]
    pub start_time: terrars::PrimField<std::string::String>,
}

impl BuildDataFusionInstance_MaintenancePolicy_MaintenanceWindow_RecurringTimeWindow_Window {
    pub fn build(self) -> DataFusionInstance_MaintenancePolicy_MaintenanceWindow_RecurringTimeWindow_Window {
        DataFusionInstance_MaintenancePolicy_MaintenanceWindow_RecurringTimeWindow_Window {
            end_time: self.end_time,
            start_time: self.start_time,
        }
    }
}

#[allow(dead_code)]
pub struct DataFusionInstance_MaintenancePolicy_MaintenanceWindow_RecurringTimeWindow_WindowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFusionInstance_MaintenancePolicy_MaintenanceWindow_RecurringTimeWindow_WindowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataFusionInstance_MaintenancePolicy_MaintenanceWindow_RecurringTimeWindow_WindowRef {
        DataFusionInstance_MaintenancePolicy_MaintenanceWindow_RecurringTimeWindow_WindowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFusionInstance_MaintenancePolicy_MaintenanceWindow_RecurringTimeWindow_WindowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `end_time` after provisioning.\nThe end time of the time window provided in RFC 3339 format."]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `start_time` after provisioning.\nThe start time of the time window provided in RFC 3339 format."]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataFusionInstance_MaintenancePolicy_MaintenanceWindow_RecurringTimeWindowDynamic {
    window: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataFusionInstance_MaintenancePolicy_MaintenanceWindow_RecurringTimeWindow_Window>,
    >,
}

#[derive(Serialize)]
pub struct DataFusionInstance_MaintenancePolicy_MaintenanceWindow_RecurringTimeWindow {
    recurrence: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    window: std
    ::option
    ::Option<std::vec::Vec<DataFusionInstance_MaintenancePolicy_MaintenanceWindow_RecurringTimeWindow_Window>>,
    dynamic: DataFusionInstance_MaintenancePolicy_MaintenanceWindow_RecurringTimeWindowDynamic,
}

impl DataFusionInstance_MaintenancePolicy_MaintenanceWindow_RecurringTimeWindow {
    #[doc = "Set the field `window`.\n"]
    pub fn set_window(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataFusionInstance_MaintenancePolicy_MaintenanceWindow_RecurringTimeWindow_Window,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.window = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.window = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataFusionInstance_MaintenancePolicy_MaintenanceWindow_RecurringTimeWindow {
    type O = terrars::BlockAssignable<DataFusionInstance_MaintenancePolicy_MaintenanceWindow_RecurringTimeWindow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFusionInstance_MaintenancePolicy_MaintenanceWindow_RecurringTimeWindow {
    #[doc =
        "An RRULE with format RFC-5545 for how this window reccurs. They go on for the span of time between the start and end time. The only supported FREQ value is \"WEEKLY\". To have something repeat every weekday, use: \"FREQ=WEEKLY;BYDAY=MO,TU,WE,TH,FR\"."]
    pub recurrence: terrars::PrimField<std::string::String>,
}

impl BuildDataFusionInstance_MaintenancePolicy_MaintenanceWindow_RecurringTimeWindow {
    pub fn build(self) -> DataFusionInstance_MaintenancePolicy_MaintenanceWindow_RecurringTimeWindow {
        DataFusionInstance_MaintenancePolicy_MaintenanceWindow_RecurringTimeWindow {
            recurrence: self.recurrence,
            window: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataFusionInstance_MaintenancePolicy_MaintenanceWindow_RecurringTimeWindowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFusionInstance_MaintenancePolicy_MaintenanceWindow_RecurringTimeWindowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataFusionInstance_MaintenancePolicy_MaintenanceWindow_RecurringTimeWindowRef {
        DataFusionInstance_MaintenancePolicy_MaintenanceWindow_RecurringTimeWindowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFusionInstance_MaintenancePolicy_MaintenanceWindow_RecurringTimeWindowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `recurrence` after provisioning.\nAn RRULE with format RFC-5545 for how this window reccurs. They go on for the span of time between the start and end time. The only supported FREQ value is \"WEEKLY\". To have something repeat every weekday, use: \"FREQ=WEEKLY;BYDAY=MO,TU,WE,TH,FR\"."]
    pub fn recurrence(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recurrence", self.base))
    }

    #[doc = "Get a reference to the value of field `window` after provisioning.\n"]
    pub fn window(
        &self,
    ) -> terrars::ListRef<DataFusionInstance_MaintenancePolicy_MaintenanceWindow_RecurringTimeWindow_WindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.window", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataFusionInstance_MaintenancePolicy_MaintenanceWindowDynamic {
    recurring_time_window: std
    ::option
    ::Option<terrars::DynamicBlock<DataFusionInstance_MaintenancePolicy_MaintenanceWindow_RecurringTimeWindow>>,
}

#[derive(Serialize)]
pub struct DataFusionInstance_MaintenancePolicy_MaintenanceWindow {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recurring_time_window: std
    ::option
    ::Option<std::vec::Vec<DataFusionInstance_MaintenancePolicy_MaintenanceWindow_RecurringTimeWindow>>,
    dynamic: DataFusionInstance_MaintenancePolicy_MaintenanceWindowDynamic,
}

impl DataFusionInstance_MaintenancePolicy_MaintenanceWindow {
    #[doc = "Set the field `recurring_time_window`.\n"]
    pub fn set_recurring_time_window(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataFusionInstance_MaintenancePolicy_MaintenanceWindow_RecurringTimeWindow>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.recurring_time_window = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.recurring_time_window = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataFusionInstance_MaintenancePolicy_MaintenanceWindow {
    type O = terrars::BlockAssignable<DataFusionInstance_MaintenancePolicy_MaintenanceWindow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFusionInstance_MaintenancePolicy_MaintenanceWindow {}

impl BuildDataFusionInstance_MaintenancePolicy_MaintenanceWindow {
    pub fn build(self) -> DataFusionInstance_MaintenancePolicy_MaintenanceWindow {
        DataFusionInstance_MaintenancePolicy_MaintenanceWindow {
            recurring_time_window: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataFusionInstance_MaintenancePolicy_MaintenanceWindowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFusionInstance_MaintenancePolicy_MaintenanceWindowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataFusionInstance_MaintenancePolicy_MaintenanceWindowRef {
        DataFusionInstance_MaintenancePolicy_MaintenanceWindowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFusionInstance_MaintenancePolicy_MaintenanceWindowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `recurring_time_window` after provisioning.\n"]
    pub fn recurring_time_window(
        &self,
    ) -> terrars::ListRef<DataFusionInstance_MaintenancePolicy_MaintenanceWindow_RecurringTimeWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recurring_time_window", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataFusionInstance_MaintenancePolicyDynamic {
    maintenance_window: std
    ::option
    ::Option<terrars::DynamicBlock<DataFusionInstance_MaintenancePolicy_MaintenanceWindow>>,
}

#[derive(Serialize)]
pub struct DataFusionInstance_MaintenancePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_window: std::option::Option<std::vec::Vec<DataFusionInstance_MaintenancePolicy_MaintenanceWindow>>,
    dynamic: DataFusionInstance_MaintenancePolicyDynamic,
}

impl DataFusionInstance_MaintenancePolicy {
    #[doc = "Set the field `maintenance_window`.\n"]
    pub fn set_maintenance_window(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataFusionInstance_MaintenancePolicy_MaintenanceWindow>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.maintenance_window = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.maintenance_window = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataFusionInstance_MaintenancePolicy {
    type O = terrars::BlockAssignable<DataFusionInstance_MaintenancePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFusionInstance_MaintenancePolicy {}

impl BuildDataFusionInstance_MaintenancePolicy {
    pub fn build(self) -> DataFusionInstance_MaintenancePolicy {
        DataFusionInstance_MaintenancePolicy {
            maintenance_window: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataFusionInstance_MaintenancePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFusionInstance_MaintenancePolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataFusionInstance_MaintenancePolicyRef {
        DataFusionInstance_MaintenancePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFusionInstance_MaintenancePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `maintenance_window` after provisioning.\n"]
    pub fn maintenance_window(&self) -> terrars::ListRef<DataFusionInstance_MaintenancePolicy_MaintenanceWindowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_window", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFusionInstance_NetworkConfig_PrivateServiceConnectConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_attachment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unreachable_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataFusionInstance_NetworkConfig_PrivateServiceConnectConfig {
    #[doc =
        "Set the field `network_attachment`.\nOptional. The reference to the network attachment used to establish private connectivity.\nIt will be of the form projects/{project-id}/regions/{region}/networkAttachments/{network-attachment-id}.\nThis is required only when using connection type PRIVATE_SERVICE_CONNECT_INTERFACES."]
    pub fn set_network_attachment(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.network_attachment = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `unreachable_cidr_block`.\nOptional. Input only. The CIDR block to which the CDF instance can't route traffic to in the consumer project VPC.\nThe size of this block should be at least /25. This range should not overlap with the primary address range of any subnetwork used by the network attachment.\nThis range can be used for other purposes in the consumer VPC as long as there is no requirement for CDF to reach destinations using these addresses.\nIf this value is not provided, the server chooses a non RFC 1918 address range. The format of this field is governed by RFC 4632."]
    pub fn set_unreachable_cidr_block(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.unreachable_cidr_block = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataFusionInstance_NetworkConfig_PrivateServiceConnectConfig {
    type O = terrars::BlockAssignable<DataFusionInstance_NetworkConfig_PrivateServiceConnectConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFusionInstance_NetworkConfig_PrivateServiceConnectConfig {}

impl BuildDataFusionInstance_NetworkConfig_PrivateServiceConnectConfig {
    pub fn build(self) -> DataFusionInstance_NetworkConfig_PrivateServiceConnectConfig {
        DataFusionInstance_NetworkConfig_PrivateServiceConnectConfig {
            network_attachment: core::default::Default::default(),
            unreachable_cidr_block: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataFusionInstance_NetworkConfig_PrivateServiceConnectConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFusionInstance_NetworkConfig_PrivateServiceConnectConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataFusionInstance_NetworkConfig_PrivateServiceConnectConfigRef {
        DataFusionInstance_NetworkConfig_PrivateServiceConnectConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFusionInstance_NetworkConfig_PrivateServiceConnectConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `effective_unreachable_cidr_block` after provisioning.\nOutput only. The CIDR block to which the CDF instance can't route traffic to in the consumer project VPC.\nThe size of this block is /25. The format of this field is governed by RFC 4632."]
    pub fn effective_unreachable_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effective_unreachable_cidr_block", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network_attachment` after provisioning.\nOptional. The reference to the network attachment used to establish private connectivity.\nIt will be of the form projects/{project-id}/regions/{region}/networkAttachments/{network-attachment-id}.\nThis is required only when using connection type PRIVATE_SERVICE_CONNECT_INTERFACES."]
    pub fn network_attachment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_attachment", self.base))
    }

    #[doc =
        "Get a reference to the value of field `unreachable_cidr_block` after provisioning.\nOptional. Input only. The CIDR block to which the CDF instance can't route traffic to in the consumer project VPC.\nThe size of this block should be at least /25. This range should not overlap with the primary address range of any subnetwork used by the network attachment.\nThis range can be used for other purposes in the consumer VPC as long as there is no requirement for CDF to reach destinations using these addresses.\nIf this value is not provided, the server chooses a non RFC 1918 address range. The format of this field is governed by RFC 4632."]
    pub fn unreachable_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unreachable_cidr_block", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataFusionInstance_NetworkConfigDynamic {
    private_service_connect_config: std
    ::option
    ::Option<terrars::DynamicBlock<DataFusionInstance_NetworkConfig_PrivateServiceConnectConfig>>,
}

#[derive(Serialize)]
pub struct DataFusionInstance_NetworkConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_allocation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_service_connect_config: std
    ::option
    ::Option<std::vec::Vec<DataFusionInstance_NetworkConfig_PrivateServiceConnectConfig>>,
    dynamic: DataFusionInstance_NetworkConfigDynamic,
}

impl DataFusionInstance_NetworkConfig {
    #[doc =
        "Set the field `connection_type`.\nOptional. Type of connection for establishing private IP connectivity between the Data Fusion customer project VPC and\nthe corresponding tenant project from a predefined list of available connection modes.\nIf this field is unspecified for a private instance, VPC peering is used. Possible values: [\"VPC_PEERING\", \"PRIVATE_SERVICE_CONNECT_INTERFACES\"]"]
    pub fn set_connection_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ip_allocation`.\nThe IP range in CIDR notation to use for the managed Data Fusion instance\nnodes. This range must not overlap with any other ranges used in the Data Fusion instance network."]
    pub fn set_ip_allocation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_allocation = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `network`.\nName of the network in the project with which the tenant project\nwill be peered for executing pipelines. In case of shared VPC where the network resides in another host\nproject the network should specified in the form of projects/{host-project-id}/global/networks/{network}"]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_service_connect_config`.\n"]
    pub fn set_private_service_connect_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataFusionInstance_NetworkConfig_PrivateServiceConnectConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.private_service_connect_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.private_service_connect_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataFusionInstance_NetworkConfig {
    type O = terrars::BlockAssignable<DataFusionInstance_NetworkConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFusionInstance_NetworkConfig {}

impl BuildDataFusionInstance_NetworkConfig {
    pub fn build(self) -> DataFusionInstance_NetworkConfig {
        DataFusionInstance_NetworkConfig {
            connection_type: core::default::Default::default(),
            ip_allocation: core::default::Default::default(),
            network: core::default::Default::default(),
            private_service_connect_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataFusionInstance_NetworkConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFusionInstance_NetworkConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataFusionInstance_NetworkConfigRef {
        DataFusionInstance_NetworkConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFusionInstance_NetworkConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `connection_type` after provisioning.\nOptional. Type of connection for establishing private IP connectivity between the Data Fusion customer project VPC and\nthe corresponding tenant project from a predefined list of available connection modes.\nIf this field is unspecified for a private instance, VPC peering is used. Possible values: [\"VPC_PEERING\", \"PRIVATE_SERVICE_CONNECT_INTERFACES\"]"]
    pub fn connection_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_type", self.base))
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

    #[doc = "Get a reference to the value of field `private_service_connect_config` after provisioning.\n"]
    pub fn private_service_connect_config(
        &self,
    ) -> terrars::ListRef<DataFusionInstance_NetworkConfig_PrivateServiceConnectConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_service_connect_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFusionInstance_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataFusionInstance_Timeouts {
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

impl terrars::ToListMappable for DataFusionInstance_Timeouts {
    type O = terrars::BlockAssignable<DataFusionInstance_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFusionInstance_Timeouts {}

impl BuildDataFusionInstance_Timeouts {
    pub fn build(self) -> DataFusionInstance_Timeouts {
        DataFusionInstance_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataFusionInstance_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFusionInstance_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataFusionInstance_TimeoutsRef {
        DataFusionInstance_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFusionInstance_TimeoutsRef {
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
