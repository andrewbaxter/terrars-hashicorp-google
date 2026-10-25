#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct IntegrationConnectorsConnectionData_ {
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
    connector_version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eventing_enablement_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suspended: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_config: std::option::Option<std::vec::Vec<IntegrationConnectorsConnection_AuthConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config_variable: std::option::Option<std::vec::Vec<IntegrationConnectorsConnection_ConfigVariable>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_config: std::option::Option<std::vec::Vec<IntegrationConnectorsConnection_DestinationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eventing_config: std::option::Option<std::vec::Vec<IntegrationConnectorsConnection_EventingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lock_config: std::option::Option<std::vec::Vec<IntegrationConnectorsConnection_LockConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_config: std::option::Option<std::vec::Vec<IntegrationConnectorsConnection_LogConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_config: std::option::Option<std::vec::Vec<IntegrationConnectorsConnection_NodeConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_config: std::option::Option<std::vec::Vec<IntegrationConnectorsConnection_SslConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<IntegrationConnectorsConnection_Timeouts>,
    dynamic: IntegrationConnectorsConnectionDynamic,
}

struct IntegrationConnectorsConnection_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<IntegrationConnectorsConnectionData_>,
}

#[derive(Clone)]
pub struct IntegrationConnectorsConnection(std::rc::Rc<IntegrationConnectorsConnection_>);

impl IntegrationConnectorsConnection {
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
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nAn arbitrary description for the Connection."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `eventing_enablement_type`.\nEventing enablement type. Will be nil if eventing is not enabled. Possible values: [\"EVENTING_AND_CONNECTION\", \"ONLY_EVENTING\"]"]
    pub fn set_eventing_enablement_type(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().eventing_enablement_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nResource labels to represent user provided metadata.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_account`.\nService account needed for runtime plane to access Google Cloud resources."]
    pub fn set_service_account(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `suspended`.\nSuspended indicates if a user has suspended a connection or not."]
    pub fn set_suspended(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().suspended = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auth_config`.\n"]
    pub fn set_auth_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IntegrationConnectorsConnection_AuthConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auth_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auth_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `config_variable`.\n"]
    pub fn set_config_variable(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IntegrationConnectorsConnection_ConfigVariable>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().config_variable = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.config_variable = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `destination_config`.\n"]
    pub fn set_destination_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IntegrationConnectorsConnection_DestinationConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `eventing_config`.\n"]
    pub fn set_eventing_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IntegrationConnectorsConnection_EventingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().eventing_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.eventing_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lock_config`.\n"]
    pub fn set_lock_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IntegrationConnectorsConnection_LockConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lock_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.lock_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `log_config`.\n"]
    pub fn set_log_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IntegrationConnectorsConnection_LogConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().log_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.log_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `node_config`.\n"]
    pub fn set_node_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IntegrationConnectorsConnection_NodeConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().node_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.node_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ssl_config`.\n"]
    pub fn set_ssl_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IntegrationConnectorsConnection_SslConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ssl_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ssl_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<IntegrationConnectorsConnection_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `connection_revision` after provisioning.\nConnection revision. This field is only updated when the connection is created or updated by User."]
    pub fn connection_revision(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_revision", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `connector_version` after provisioning.\nconnectorVersion of the Connector."]
    pub fn connector_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connector_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `connector_version_infra_config` after provisioning.\nThis configuration provides infra configs like rate limit threshold which need to be configurable for every connector version."]
    pub fn connector_version_infra_config(
        &self,
    ) -> terrars::ListRef<IntegrationConnectorsConnection_ConnectorVersionInfraConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.connector_version_infra_config", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `connector_version_launch_stage` after provisioning.\nFlag to mark the version indicating the launch stage."]
    pub fn connector_version_launch_stage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.connector_version_launch_stage", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime the Namespace was created in UTC."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn arbitrary description for the Connection."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `eventing_enablement_type` after provisioning.\nEventing enablement type. Will be nil if eventing is not enabled. Possible values: [\"EVENTING_AND_CONNECTION\", \"ONLY_EVENTING\"]"]
    pub fn eventing_enablement_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.eventing_enablement_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `eventing_runtime_data` after provisioning.\nEventing Runtime Data."]
    pub fn eventing_runtime_data(&self) -> terrars::ListRef<IntegrationConnectorsConnection_EventingRuntimeDataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.eventing_runtime_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nResource labels to represent user provided metadata.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nLocation in which Connection needs to be created."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of Connection needs to be created."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nService account needed for runtime plane to access Google Cloud resources."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_directory` after provisioning.\nThe name of the Service Directory service name. Used for Private Harpoon to resolve the ILB address.\ne.g. \"projects/cloud-connectors-e2e-testing/locations/us-central1/namespaces/istio-system/services/istio-ingressgateway-connectors\""]
    pub fn service_directory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_directory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nStatus of the Integration Connector."]
    pub fn status(&self) -> terrars::ListRef<IntegrationConnectorsConnection_StatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subscription_type` after provisioning.\nThis subscription type enum states the subscription type of the project."]
    pub fn subscription_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscription_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `suspended` after provisioning.\nSuspended indicates if a user has suspended a connection or not."]
    pub fn suspended(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suspended", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime the Namespace was updated in UTC."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auth_config` after provisioning.\n"]
    pub fn auth_config(&self) -> terrars::ListRef<IntegrationConnectorsConnection_AuthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auth_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config_variable` after provisioning.\n"]
    pub fn config_variable(&self) -> terrars::ListRef<IntegrationConnectorsConnection_ConfigVariableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config_variable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_config` after provisioning.\n"]
    pub fn destination_config(&self) -> terrars::ListRef<IntegrationConnectorsConnection_DestinationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `eventing_config` after provisioning.\n"]
    pub fn eventing_config(&self) -> terrars::ListRef<IntegrationConnectorsConnection_EventingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.eventing_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lock_config` after provisioning.\n"]
    pub fn lock_config(&self) -> terrars::ListRef<IntegrationConnectorsConnection_LockConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lock_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_config` after provisioning.\n"]
    pub fn log_config(&self) -> terrars::ListRef<IntegrationConnectorsConnection_LogConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> terrars::ListRef<IntegrationConnectorsConnection_NodeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ssl_config` after provisioning.\n"]
    pub fn ssl_config(&self) -> terrars::ListRef<IntegrationConnectorsConnection_SslConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssl_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IntegrationConnectorsConnection_TimeoutsRef {
        IntegrationConnectorsConnection_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for IntegrationConnectorsConnection {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for IntegrationConnectorsConnection { }

impl terrars::ToListMappable for IntegrationConnectorsConnection {
    type O = terrars::ListRef<IntegrationConnectorsConnectionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for IntegrationConnectorsConnection_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_integration_connectors_connection".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIntegrationConnectorsConnection {
    pub tf_id: std::string::String,
    #[doc = "connectorVersion of the Connector."]
    pub connector_version: terrars::PrimField<std::string::String>,
    #[doc = "Location in which Connection needs to be created."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "Name of Connection needs to be created."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnection {
    pub fn build(self, stack: &mut terrars::Stack) -> IntegrationConnectorsConnection {
        let out = IntegrationConnectorsConnection(std::rc::Rc::new(IntegrationConnectorsConnection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(IntegrationConnectorsConnectionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                connector_version: self.connector_version,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                eventing_enablement_type: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                service_account: core::default::Default::default(),
                suspended: core::default::Default::default(),
                auth_config: core::default::Default::default(),
                config_variable: core::default::Default::default(),
                destination_config: core::default::Default::default(),
                eventing_config: core::default::Default::default(),
                lock_config: core::default::Default::default(),
                log_config: core::default::Default::default(),
                node_config: core::default::Default::default(),
                ssl_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IntegrationConnectorsConnectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl IntegrationConnectorsConnectionRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `connection_revision` after provisioning.\nConnection revision. This field is only updated when the connection is created or updated by User."]
    pub fn connection_revision(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_revision", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `connector_version` after provisioning.\nconnectorVersion of the Connector."]
    pub fn connector_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connector_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `connector_version_infra_config` after provisioning.\nThis configuration provides infra configs like rate limit threshold which need to be configurable for every connector version."]
    pub fn connector_version_infra_config(
        &self,
    ) -> terrars::ListRef<IntegrationConnectorsConnection_ConnectorVersionInfraConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.connector_version_infra_config", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `connector_version_launch_stage` after provisioning.\nFlag to mark the version indicating the launch stage."]
    pub fn connector_version_launch_stage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.connector_version_launch_stage", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime the Namespace was created in UTC."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn arbitrary description for the Connection."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `eventing_enablement_type` after provisioning.\nEventing enablement type. Will be nil if eventing is not enabled. Possible values: [\"EVENTING_AND_CONNECTION\", \"ONLY_EVENTING\"]"]
    pub fn eventing_enablement_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.eventing_enablement_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `eventing_runtime_data` after provisioning.\nEventing Runtime Data."]
    pub fn eventing_runtime_data(&self) -> terrars::ListRef<IntegrationConnectorsConnection_EventingRuntimeDataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.eventing_runtime_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nResource labels to represent user provided metadata.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nLocation in which Connection needs to be created."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of Connection needs to be created."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nService account needed for runtime plane to access Google Cloud resources."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_directory` after provisioning.\nThe name of the Service Directory service name. Used for Private Harpoon to resolve the ILB address.\ne.g. \"projects/cloud-connectors-e2e-testing/locations/us-central1/namespaces/istio-system/services/istio-ingressgateway-connectors\""]
    pub fn service_directory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_directory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nStatus of the Integration Connector."]
    pub fn status(&self) -> terrars::ListRef<IntegrationConnectorsConnection_StatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subscription_type` after provisioning.\nThis subscription type enum states the subscription type of the project."]
    pub fn subscription_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscription_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `suspended` after provisioning.\nSuspended indicates if a user has suspended a connection or not."]
    pub fn suspended(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suspended", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime the Namespace was updated in UTC."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auth_config` after provisioning.\n"]
    pub fn auth_config(&self) -> terrars::ListRef<IntegrationConnectorsConnection_AuthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auth_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config_variable` after provisioning.\n"]
    pub fn config_variable(&self) -> terrars::ListRef<IntegrationConnectorsConnection_ConfigVariableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config_variable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_config` after provisioning.\n"]
    pub fn destination_config(&self) -> terrars::ListRef<IntegrationConnectorsConnection_DestinationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `eventing_config` after provisioning.\n"]
    pub fn eventing_config(&self) -> terrars::ListRef<IntegrationConnectorsConnection_EventingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.eventing_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lock_config` after provisioning.\n"]
    pub fn lock_config(&self) -> terrars::ListRef<IntegrationConnectorsConnection_LockConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lock_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_config` after provisioning.\n"]
    pub fn log_config(&self) -> terrars::ListRef<IntegrationConnectorsConnection_LogConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> terrars::ListRef<IntegrationConnectorsConnection_NodeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ssl_config` after provisioning.\n"]
    pub fn ssl_config(&self) -> terrars::ListRef<IntegrationConnectorsConnection_SslConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssl_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IntegrationConnectorsConnection_TimeoutsRef {
        IntegrationConnectorsConnection_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct IntegrationConnectorsConnectionDynamic {
    auth_config: std::option::Option<terrars::DynamicBlock<IntegrationConnectorsConnection_AuthConfig>>,
    config_variable: std::option::Option<terrars::DynamicBlock<IntegrationConnectorsConnection_ConfigVariable>>,
    destination_config: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnection_DestinationConfig>>,
    eventing_config: std::option::Option<terrars::DynamicBlock<IntegrationConnectorsConnection_EventingConfig>>,
    lock_config: std::option::Option<terrars::DynamicBlock<IntegrationConnectorsConnection_LockConfig>>,
    log_config: std::option::Option<terrars::DynamicBlock<IntegrationConnectorsConnection_LogConfig>>,
    node_config: std::option::Option<terrars::DynamicBlock<IntegrationConnectorsConnection_NodeConfig>>,
    ssl_config: std::option::Option<terrars::DynamicBlock<IntegrationConnectorsConnection_SslConfig>>,
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnection_ConnectorVersionInfraConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ratelimit_threshold: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IntegrationConnectorsConnection_ConnectorVersionInfraConfig {
    #[doc = "Set the field `ratelimit_threshold`.\n"]
    pub fn set_ratelimit_threshold(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ratelimit_threshold = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IntegrationConnectorsConnection_ConnectorVersionInfraConfig {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnection_ConnectorVersionInfraConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnection_ConnectorVersionInfraConfig {}

impl BuildIntegrationConnectorsConnection_ConnectorVersionInfraConfig {
    pub fn build(self) -> IntegrationConnectorsConnection_ConnectorVersionInfraConfig {
        IntegrationConnectorsConnection_ConnectorVersionInfraConfig {
            ratelimit_threshold: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationConnectorsConnection_ConnectorVersionInfraConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnection_ConnectorVersionInfraConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnection_ConnectorVersionInfraConfigRef {
        IntegrationConnectorsConnection_ConnectorVersionInfraConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnection_ConnectorVersionInfraConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ratelimit_threshold` after provisioning.\n"]
    pub fn ratelimit_threshold(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ratelimit_threshold", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnection_EventingRuntimeData_Status {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IntegrationConnectorsConnection_EventingRuntimeData_Status {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IntegrationConnectorsConnection_EventingRuntimeData_Status {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnection_EventingRuntimeData_Status>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnection_EventingRuntimeData_Status {}

impl BuildIntegrationConnectorsConnection_EventingRuntimeData_Status {
    pub fn build(self) -> IntegrationConnectorsConnection_EventingRuntimeData_Status {
        IntegrationConnectorsConnection_EventingRuntimeData_Status {
            description: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationConnectorsConnection_EventingRuntimeData_StatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnection_EventingRuntimeData_StatusRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnection_EventingRuntimeData_StatusRef {
        IntegrationConnectorsConnection_EventingRuntimeData_StatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnection_EventingRuntimeData_StatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnection_EventingRuntimeData {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    events_listener_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::ListField<IntegrationConnectorsConnection_EventingRuntimeData_Status>>,
}

impl IntegrationConnectorsConnection_EventingRuntimeData {
    #[doc = "Set the field `events_listener_endpoint`.\n"]
    pub fn set_events_listener_endpoint(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.events_listener_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(
        mut self,
        v: impl std::convert::Into<terrars::ListField<IntegrationConnectorsConnection_EventingRuntimeData_Status>>,
    ) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IntegrationConnectorsConnection_EventingRuntimeData {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnection_EventingRuntimeData>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnection_EventingRuntimeData {}

impl BuildIntegrationConnectorsConnection_EventingRuntimeData {
    pub fn build(self) -> IntegrationConnectorsConnection_EventingRuntimeData {
        IntegrationConnectorsConnection_EventingRuntimeData {
            events_listener_endpoint: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationConnectorsConnection_EventingRuntimeDataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnection_EventingRuntimeDataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnection_EventingRuntimeDataRef {
        IntegrationConnectorsConnection_EventingRuntimeDataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnection_EventingRuntimeDataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `events_listener_endpoint` after provisioning.\n"]
    pub fn events_listener_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.events_listener_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::ListRef<IntegrationConnectorsConnection_EventingRuntimeData_StatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnection_Status {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IntegrationConnectorsConnection_Status {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IntegrationConnectorsConnection_Status {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnection_Status>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnection_Status {}

impl BuildIntegrationConnectorsConnection_Status {
    pub fn build(self) -> IntegrationConnectorsConnection_Status {
        IntegrationConnectorsConnection_Status {
            description: core::default::Default::default(),
            state: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationConnectorsConnection_StatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnection_StatusRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IntegrationConnectorsConnection_StatusRef {
        IntegrationConnectorsConnection_StatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnection_StatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnection_ConfigVariable_EncryptionKeyValue {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl IntegrationConnectorsConnection_ConfigVariable_EncryptionKeyValue {
    #[doc =
        "Set the field `kms_key_name`.\nThe [KMS key name] with which the content of the Operation is encrypted. The\nexpected format: projects/*/locations/*/keyRings/*/cryptoKeys/*.\nWill be empty string if google managed."]
    pub fn set_kms_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IntegrationConnectorsConnection_ConfigVariable_EncryptionKeyValue {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnection_ConfigVariable_EncryptionKeyValue>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnection_ConfigVariable_EncryptionKeyValue {
    #[doc = "Type of Encryption Key Possible values: [\"GOOGLE_MANAGED\", \"CUSTOMER_MANAGED\"]"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnection_ConfigVariable_EncryptionKeyValue {
    pub fn build(self) -> IntegrationConnectorsConnection_ConfigVariable_EncryptionKeyValue {
        IntegrationConnectorsConnection_ConfigVariable_EncryptionKeyValue {
            kms_key_name: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationConnectorsConnection_ConfigVariable_EncryptionKeyValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnection_ConfigVariable_EncryptionKeyValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnection_ConfigVariable_EncryptionKeyValueRef {
        IntegrationConnectorsConnection_ConfigVariable_EncryptionKeyValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnection_ConfigVariable_EncryptionKeyValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nThe [KMS key name] with which the content of the Operation is encrypted. The\nexpected format: projects/*/locations/*/keyRings/*/cryptoKeys/*.\nWill be empty string if google managed."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nType of Encryption Key Possible values: [\"GOOGLE_MANAGED\", \"CUSTOMER_MANAGED\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnection_ConfigVariable_SecretValue {
    secret_version: terrars::PrimField<std::string::String>,
}

impl IntegrationConnectorsConnection_ConfigVariable_SecretValue { }

impl terrars::ToListMappable for IntegrationConnectorsConnection_ConfigVariable_SecretValue {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnection_ConfigVariable_SecretValue>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnection_ConfigVariable_SecretValue {
    #[doc = "Secret version of Secret Value for Config variable."]
    pub secret_version: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnection_ConfigVariable_SecretValue {
    pub fn build(self) -> IntegrationConnectorsConnection_ConfigVariable_SecretValue {
        IntegrationConnectorsConnection_ConfigVariable_SecretValue { secret_version: self.secret_version }
    }
}

#[allow(dead_code)]
pub struct IntegrationConnectorsConnection_ConfigVariable_SecretValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnection_ConfigVariable_SecretValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnection_ConfigVariable_SecretValueRef {
        IntegrationConnectorsConnection_ConfigVariable_SecretValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnection_ConfigVariable_SecretValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `secret_version` after provisioning.\nSecret version of Secret Value for Config variable."]
    pub fn secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_version", self.base))
    }
}

#[derive(Serialize, Default)]
struct IntegrationConnectorsConnection_AuthConfig_AdditionalVariableDynamic {
    encryption_key_value: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnection_ConfigVariable_EncryptionKeyValue>>,
    secret_value: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnection_ConfigVariable_SecretValue>>,
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnection_AuthConfig_AdditionalVariable {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boolean_value: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    integer_value: std::option::Option<terrars::PrimField<f64>>,
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_key_value: std
    ::option
    ::Option<std::vec::Vec<IntegrationConnectorsConnection_ConfigVariable_EncryptionKeyValue>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_value: std::option::Option<std::vec::Vec<IntegrationConnectorsConnection_ConfigVariable_SecretValue>>,
    dynamic: IntegrationConnectorsConnection_AuthConfig_AdditionalVariableDynamic,
}

impl IntegrationConnectorsConnection_AuthConfig_AdditionalVariable {
    #[doc = "Set the field `boolean_value`.\nBoolean Value of configVariable."]
    pub fn set_boolean_value(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.boolean_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `integer_value`.\nInteger Value of configVariable."]
    pub fn set_integer_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.integer_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `string_value`.\nString Value of configVariabley."]
    pub fn set_string_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.string_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_key_value`.\n"]
    pub fn set_encryption_key_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<IntegrationConnectorsConnection_ConfigVariable_EncryptionKeyValue>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.encryption_key_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption_key_value = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `secret_value`.\n"]
    pub fn set_secret_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<IntegrationConnectorsConnection_ConfigVariable_SecretValue>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.secret_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.secret_value = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IntegrationConnectorsConnection_AuthConfig_AdditionalVariable {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnection_AuthConfig_AdditionalVariable>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnection_AuthConfig_AdditionalVariable {
    #[doc = "Key for the configVariable"]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnection_AuthConfig_AdditionalVariable {
    pub fn build(self) -> IntegrationConnectorsConnection_AuthConfig_AdditionalVariable {
        IntegrationConnectorsConnection_AuthConfig_AdditionalVariable {
            boolean_value: core::default::Default::default(),
            integer_value: core::default::Default::default(),
            key: self.key,
            string_value: core::default::Default::default(),
            encryption_key_value: core::default::Default::default(),
            secret_value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationConnectorsConnection_AuthConfig_AdditionalVariableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnection_AuthConfig_AdditionalVariableRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnection_AuthConfig_AdditionalVariableRef {
        IntegrationConnectorsConnection_AuthConfig_AdditionalVariableRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnection_AuthConfig_AdditionalVariableRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `boolean_value` after provisioning.\nBoolean Value of configVariable."]
    pub fn boolean_value(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boolean_value", self.base))
    }

    #[doc =
        "Get a reference to the value of field `integer_value` after provisioning.\nInteger Value of configVariable."]
    pub fn integer_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integer_value", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\nKey for the configVariable"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `string_value` after provisioning.\nString Value of configVariabley."]
    pub fn string_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.string_value", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_key_value` after provisioning.\n"]
    pub fn encryption_key_value(
        &self,
    ) -> terrars::ListRef<IntegrationConnectorsConnection_ConfigVariable_EncryptionKeyValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_key_value", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_value` after provisioning.\n"]
    pub fn secret_value(&self) -> terrars::ListRef<IntegrationConnectorsConnection_ConfigVariable_SecretValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_value", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnection_AuthConfig_Oauth2AuthCodeFlow_ClientSecret {
    secret_version: terrars::PrimField<std::string::String>,
}

impl IntegrationConnectorsConnection_AuthConfig_Oauth2AuthCodeFlow_ClientSecret { }

impl terrars::ToListMappable for IntegrationConnectorsConnection_AuthConfig_Oauth2AuthCodeFlow_ClientSecret {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnection_AuthConfig_Oauth2AuthCodeFlow_ClientSecret>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnection_AuthConfig_Oauth2AuthCodeFlow_ClientSecret {
    #[doc = "The resource name of the secret version in the format,\nformat as: projects/*/secrets/*/versions/*."]
    pub secret_version: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnection_AuthConfig_Oauth2AuthCodeFlow_ClientSecret {
    pub fn build(self) -> IntegrationConnectorsConnection_AuthConfig_Oauth2AuthCodeFlow_ClientSecret {
        IntegrationConnectorsConnection_AuthConfig_Oauth2AuthCodeFlow_ClientSecret {
            secret_version: self.secret_version,
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationConnectorsConnection_AuthConfig_Oauth2AuthCodeFlow_ClientSecretRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnection_AuthConfig_Oauth2AuthCodeFlow_ClientSecretRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnection_AuthConfig_Oauth2AuthCodeFlow_ClientSecretRef {
        IntegrationConnectorsConnection_AuthConfig_Oauth2AuthCodeFlow_ClientSecretRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnection_AuthConfig_Oauth2AuthCodeFlow_ClientSecretRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `secret_version` after provisioning.\nThe resource name of the secret version in the format,\nformat as: projects/*/secrets/*/versions/*."]
    pub fn secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_version", self.base))
    }
}

#[derive(Serialize, Default)]
struct IntegrationConnectorsConnection_AuthConfig_Oauth2AuthCodeFlowDynamic {
    client_secret: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnection_AuthConfig_Oauth2AuthCodeFlow_ClientSecret>>,
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnection_AuthConfig_Oauth2AuthCodeFlow {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_pkce: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scopes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std
    ::option
    ::Option<std::vec::Vec<IntegrationConnectorsConnection_AuthConfig_Oauth2AuthCodeFlow_ClientSecret>>,
    dynamic: IntegrationConnectorsConnection_AuthConfig_Oauth2AuthCodeFlowDynamic,
}

impl IntegrationConnectorsConnection_AuthConfig_Oauth2AuthCodeFlow {
    #[doc = "Set the field `auth_uri`.\nAuth URL for Authorization Code Flow."]
    pub fn set_auth_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_id`.\nClient ID for user-provided OAuth app."]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_pkce`.\nWhether to enable PKCE when the user performs the auth code flow."]
    pub fn set_enable_pkce(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_pkce = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scopes`.\nScopes the connection will request when the user performs the auth code flow."]
    pub fn set_scopes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.scopes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret`.\n"]
    pub fn set_client_secret(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<IntegrationConnectorsConnection_AuthConfig_Oauth2AuthCodeFlow_ClientSecret>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.client_secret = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.client_secret = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IntegrationConnectorsConnection_AuthConfig_Oauth2AuthCodeFlow {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnection_AuthConfig_Oauth2AuthCodeFlow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnection_AuthConfig_Oauth2AuthCodeFlow {}

impl BuildIntegrationConnectorsConnection_AuthConfig_Oauth2AuthCodeFlow {
    pub fn build(self) -> IntegrationConnectorsConnection_AuthConfig_Oauth2AuthCodeFlow {
        IntegrationConnectorsConnection_AuthConfig_Oauth2AuthCodeFlow {
            auth_uri: core::default::Default::default(),
            client_id: core::default::Default::default(),
            enable_pkce: core::default::Default::default(),
            scopes: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationConnectorsConnection_AuthConfig_Oauth2AuthCodeFlowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnection_AuthConfig_Oauth2AuthCodeFlowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnection_AuthConfig_Oauth2AuthCodeFlowRef {
        IntegrationConnectorsConnection_AuthConfig_Oauth2AuthCodeFlowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnection_AuthConfig_Oauth2AuthCodeFlowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `auth_uri` after provisioning.\nAuth URL for Authorization Code Flow."]
    pub fn auth_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_id` after provisioning.\nClient ID for user-provided OAuth app."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_pkce` after provisioning.\nWhether to enable PKCE when the user performs the auth code flow."]
    pub fn enable_pkce(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_pkce", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scopes` after provisioning.\nScopes the connection will request when the user performs the auth code flow."]
    pub fn scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(
        &self,
    ) -> terrars::ListRef<IntegrationConnectorsConnection_AuthConfig_Oauth2AuthCodeFlow_ClientSecretRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }
}

#[derive(Serialize, Default)]
struct IntegrationConnectorsConnection_AuthConfig_Oauth2ClientCredentialsDynamic {
    client_secret: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnection_AuthConfig_Oauth2AuthCodeFlow_ClientSecret>>,
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnection_AuthConfig_Oauth2ClientCredentials {
    client_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std
    ::option
    ::Option<std::vec::Vec<IntegrationConnectorsConnection_AuthConfig_Oauth2AuthCodeFlow_ClientSecret>>,
    dynamic: IntegrationConnectorsConnection_AuthConfig_Oauth2ClientCredentialsDynamic,
}

impl IntegrationConnectorsConnection_AuthConfig_Oauth2ClientCredentials {
    #[doc = "Set the field `client_secret`.\n"]
    pub fn set_client_secret(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<IntegrationConnectorsConnection_AuthConfig_Oauth2AuthCodeFlow_ClientSecret>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.client_secret = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.client_secret = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IntegrationConnectorsConnection_AuthConfig_Oauth2ClientCredentials {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnection_AuthConfig_Oauth2ClientCredentials>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnection_AuthConfig_Oauth2ClientCredentials {
    #[doc = "Secret version of Password for Authentication."]
    pub client_id: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnection_AuthConfig_Oauth2ClientCredentials {
    pub fn build(self) -> IntegrationConnectorsConnection_AuthConfig_Oauth2ClientCredentials {
        IntegrationConnectorsConnection_AuthConfig_Oauth2ClientCredentials {
            client_id: self.client_id,
            client_secret: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationConnectorsConnection_AuthConfig_Oauth2ClientCredentialsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnection_AuthConfig_Oauth2ClientCredentialsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnection_AuthConfig_Oauth2ClientCredentialsRef {
        IntegrationConnectorsConnection_AuthConfig_Oauth2ClientCredentialsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnection_AuthConfig_Oauth2ClientCredentialsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `client_id` after provisioning.\nSecret version of Password for Authentication."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(
        &self,
    ) -> terrars::ListRef<IntegrationConnectorsConnection_AuthConfig_Oauth2AuthCodeFlow_ClientSecretRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer_ClientKey {
    secret_version: terrars::PrimField<std::string::String>,
}

impl IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer_ClientKey { }

impl terrars::ToListMappable for IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer_ClientKey {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer_ClientKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer_ClientKey {
    #[doc = "The resource name of the secret version in the format,\nformat as: projects/*/secrets/*/versions/*."]
    pub secret_version: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer_ClientKey {
    pub fn build(self) -> IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer_ClientKey {
        IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer_ClientKey { secret_version: self.secret_version }
    }
}

#[allow(dead_code)]
pub struct IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer_ClientKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer_ClientKeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer_ClientKeyRef {
        IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer_ClientKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer_ClientKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `secret_version` after provisioning.\nThe resource name of the secret version in the format,\nformat as: projects/*/secrets/*/versions/*."]
    pub fn secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_version", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer_JwtClaims {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audience: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    issuer: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer_JwtClaims {
    #[doc = "Set the field `audience`.\nValue for the \"aud\" claim."]
    pub fn set_audience(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.audience = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `issuer`.\nValue for the \"iss\" claim."]
    pub fn set_issuer(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.issuer = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subject`.\nValue for the \"sub\" claim."]
    pub fn set_subject(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subject = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer_JwtClaims {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer_JwtClaims>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer_JwtClaims {}

impl BuildIntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer_JwtClaims {
    pub fn build(self) -> IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer_JwtClaims {
        IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer_JwtClaims {
            audience: core::default::Default::default(),
            issuer: core::default::Default::default(),
            subject: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer_JwtClaimsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer_JwtClaimsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer_JwtClaimsRef {
        IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer_JwtClaimsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer_JwtClaimsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audience` after provisioning.\nValue for the \"aud\" claim."]
    pub fn audience(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audience", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\nValue for the \"iss\" claim."]
    pub fn issuer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }

    #[doc = "Get a reference to the value of field `subject` after provisioning.\nValue for the \"sub\" claim."]
    pub fn subject(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subject", self.base))
    }
}

#[derive(Serialize, Default)]
struct IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearerDynamic {
    client_key: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer_ClientKey>>,
    jwt_claims: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer_JwtClaims>>,
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_key: std::option::Option<std::vec::Vec<IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer_ClientKey>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jwt_claims: std::option::Option<std::vec::Vec<IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer_JwtClaims>>,
    dynamic: IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearerDynamic,
}

impl IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer {
    #[doc = "Set the field `client_key`.\n"]
    pub fn set_client_key(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer_ClientKey>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.client_key = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.client_key = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `jwt_claims`.\n"]
    pub fn set_jwt_claims(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer_JwtClaims>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.jwt_claims = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.jwt_claims = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer {}

impl BuildIntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer {
    pub fn build(self) -> IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer {
        IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer {
            client_key: core::default::Default::default(),
            jwt_claims: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearerRef {
        IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_key` after provisioning.\n"]
    pub fn client_key(
        &self,
    ) -> terrars::ListRef<IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer_ClientKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_key", self.base))
    }

    #[doc = "Get a reference to the value of field `jwt_claims` after provisioning.\n"]
    pub fn jwt_claims(
        &self,
    ) -> terrars::ListRef<IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer_JwtClaimsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jwt_claims", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnection_AuthConfig_SshPublicKey_SshClientCert {
    secret_version: terrars::PrimField<std::string::String>,
}

impl IntegrationConnectorsConnection_AuthConfig_SshPublicKey_SshClientCert { }

impl terrars::ToListMappable for IntegrationConnectorsConnection_AuthConfig_SshPublicKey_SshClientCert {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnection_AuthConfig_SshPublicKey_SshClientCert>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnection_AuthConfig_SshPublicKey_SshClientCert {
    #[doc = "The resource name of the secret version in the format,\nformat as: projects/*/secrets/*/versions/*."]
    pub secret_version: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnection_AuthConfig_SshPublicKey_SshClientCert {
    pub fn build(self) -> IntegrationConnectorsConnection_AuthConfig_SshPublicKey_SshClientCert {
        IntegrationConnectorsConnection_AuthConfig_SshPublicKey_SshClientCert { secret_version: self.secret_version }
    }
}

#[allow(dead_code)]
pub struct IntegrationConnectorsConnection_AuthConfig_SshPublicKey_SshClientCertRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnection_AuthConfig_SshPublicKey_SshClientCertRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnection_AuthConfig_SshPublicKey_SshClientCertRef {
        IntegrationConnectorsConnection_AuthConfig_SshPublicKey_SshClientCertRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnection_AuthConfig_SshPublicKey_SshClientCertRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `secret_version` after provisioning.\nThe resource name of the secret version in the format,\nformat as: projects/*/secrets/*/versions/*."]
    pub fn secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_version", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnection_AuthConfig_SshPublicKey_SshClientCertPass {
    secret_version: terrars::PrimField<std::string::String>,
}

impl IntegrationConnectorsConnection_AuthConfig_SshPublicKey_SshClientCertPass { }

impl terrars::ToListMappable for IntegrationConnectorsConnection_AuthConfig_SshPublicKey_SshClientCertPass {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnection_AuthConfig_SshPublicKey_SshClientCertPass>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnection_AuthConfig_SshPublicKey_SshClientCertPass {
    #[doc = "The resource name of the secret version in the format,\nformat as: projects/*/secrets/*/versions/*."]
    pub secret_version: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnection_AuthConfig_SshPublicKey_SshClientCertPass {
    pub fn build(self) -> IntegrationConnectorsConnection_AuthConfig_SshPublicKey_SshClientCertPass {
        IntegrationConnectorsConnection_AuthConfig_SshPublicKey_SshClientCertPass {
            secret_version: self.secret_version,
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationConnectorsConnection_AuthConfig_SshPublicKey_SshClientCertPassRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnection_AuthConfig_SshPublicKey_SshClientCertPassRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnection_AuthConfig_SshPublicKey_SshClientCertPassRef {
        IntegrationConnectorsConnection_AuthConfig_SshPublicKey_SshClientCertPassRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnection_AuthConfig_SshPublicKey_SshClientCertPassRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `secret_version` after provisioning.\nThe resource name of the secret version in the format,\nformat as: projects/*/secrets/*/versions/*."]
    pub fn secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_version", self.base))
    }
}

#[derive(Serialize, Default)]
struct IntegrationConnectorsConnection_AuthConfig_SshPublicKeyDynamic {
    ssh_client_cert: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnection_AuthConfig_SshPublicKey_SshClientCert>>,
    ssh_client_cert_pass: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnection_AuthConfig_SshPublicKey_SshClientCertPass>>,
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnection_AuthConfig_SshPublicKey {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cert_type: std::option::Option<terrars::PrimField<std::string::String>>,
    username: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssh_client_cert: std
    ::option
    ::Option<std::vec::Vec<IntegrationConnectorsConnection_AuthConfig_SshPublicKey_SshClientCert>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssh_client_cert_pass: std
    ::option
    ::Option<std::vec::Vec<IntegrationConnectorsConnection_AuthConfig_SshPublicKey_SshClientCertPass>>,
    dynamic: IntegrationConnectorsConnection_AuthConfig_SshPublicKeyDynamic,
}

impl IntegrationConnectorsConnection_AuthConfig_SshPublicKey {
    #[doc = "Set the field `cert_type`.\nFormat of SSH Client cert."]
    pub fn set_cert_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cert_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssh_client_cert`.\n"]
    pub fn set_ssh_client_cert(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<IntegrationConnectorsConnection_AuthConfig_SshPublicKey_SshClientCert>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ssh_client_cert = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ssh_client_cert = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ssh_client_cert_pass`.\n"]
    pub fn set_ssh_client_cert_pass(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<IntegrationConnectorsConnection_AuthConfig_SshPublicKey_SshClientCertPass>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ssh_client_cert_pass = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ssh_client_cert_pass = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IntegrationConnectorsConnection_AuthConfig_SshPublicKey {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnection_AuthConfig_SshPublicKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnection_AuthConfig_SshPublicKey {
    #[doc = "The user account used to authenticate."]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnection_AuthConfig_SshPublicKey {
    pub fn build(self) -> IntegrationConnectorsConnection_AuthConfig_SshPublicKey {
        IntegrationConnectorsConnection_AuthConfig_SshPublicKey {
            cert_type: core::default::Default::default(),
            username: self.username,
            ssh_client_cert: core::default::Default::default(),
            ssh_client_cert_pass: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationConnectorsConnection_AuthConfig_SshPublicKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnection_AuthConfig_SshPublicKeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnection_AuthConfig_SshPublicKeyRef {
        IntegrationConnectorsConnection_AuthConfig_SshPublicKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnection_AuthConfig_SshPublicKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cert_type` after provisioning.\nFormat of SSH Client cert."]
    pub fn cert_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cert_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `username` after provisioning.\nThe user account used to authenticate."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }

    #[doc = "Get a reference to the value of field `ssh_client_cert` after provisioning.\n"]
    pub fn ssh_client_cert(
        &self,
    ) -> terrars::ListRef<IntegrationConnectorsConnection_AuthConfig_SshPublicKey_SshClientCertRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssh_client_cert", self.base))
    }

    #[doc = "Get a reference to the value of field `ssh_client_cert_pass` after provisioning.\n"]
    pub fn ssh_client_cert_pass(
        &self,
    ) -> terrars::ListRef<IntegrationConnectorsConnection_AuthConfig_SshPublicKey_SshClientCertPassRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssh_client_cert_pass", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnection_AuthConfig_UserPassword_Password {
    secret_version: terrars::PrimField<std::string::String>,
}

impl IntegrationConnectorsConnection_AuthConfig_UserPassword_Password { }

impl terrars::ToListMappable for IntegrationConnectorsConnection_AuthConfig_UserPassword_Password {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnection_AuthConfig_UserPassword_Password>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnection_AuthConfig_UserPassword_Password {
    #[doc = "The resource name of the secret version in the format,\nformat as: projects/*/secrets/*/versions/*."]
    pub secret_version: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnection_AuthConfig_UserPassword_Password {
    pub fn build(self) -> IntegrationConnectorsConnection_AuthConfig_UserPassword_Password {
        IntegrationConnectorsConnection_AuthConfig_UserPassword_Password { secret_version: self.secret_version }
    }
}

#[allow(dead_code)]
pub struct IntegrationConnectorsConnection_AuthConfig_UserPassword_PasswordRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnection_AuthConfig_UserPassword_PasswordRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnection_AuthConfig_UserPassword_PasswordRef {
        IntegrationConnectorsConnection_AuthConfig_UserPassword_PasswordRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnection_AuthConfig_UserPassword_PasswordRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `secret_version` after provisioning.\nThe resource name of the secret version in the format,\nformat as: projects/*/secrets/*/versions/*."]
    pub fn secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_version", self.base))
    }
}

#[derive(Serialize, Default)]
struct IntegrationConnectorsConnection_AuthConfig_UserPasswordDynamic {
    password: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnection_AuthConfig_UserPassword_Password>>,
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnection_AuthConfig_UserPassword {
    username: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<std::vec::Vec<IntegrationConnectorsConnection_AuthConfig_UserPassword_Password>>,
    dynamic: IntegrationConnectorsConnection_AuthConfig_UserPasswordDynamic,
}

impl IntegrationConnectorsConnection_AuthConfig_UserPassword {
    #[doc = "Set the field `password`.\n"]
    pub fn set_password(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<IntegrationConnectorsConnection_AuthConfig_UserPassword_Password>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.password = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.password = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IntegrationConnectorsConnection_AuthConfig_UserPassword {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnection_AuthConfig_UserPassword>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnection_AuthConfig_UserPassword {
    #[doc = "Username for Authentication."]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnection_AuthConfig_UserPassword {
    pub fn build(self) -> IntegrationConnectorsConnection_AuthConfig_UserPassword {
        IntegrationConnectorsConnection_AuthConfig_UserPassword {
            username: self.username,
            password: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationConnectorsConnection_AuthConfig_UserPasswordRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnection_AuthConfig_UserPasswordRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnection_AuthConfig_UserPasswordRef {
        IntegrationConnectorsConnection_AuthConfig_UserPasswordRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnection_AuthConfig_UserPasswordRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\nUsername for Authentication."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> terrars::ListRef<IntegrationConnectorsConnection_AuthConfig_UserPassword_PasswordRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.password", self.base))
    }
}

#[derive(Serialize, Default)]
struct IntegrationConnectorsConnection_AuthConfigDynamic {
    additional_variable: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnection_AuthConfig_AdditionalVariable>>,
    oauth2_auth_code_flow: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnection_AuthConfig_Oauth2AuthCodeFlow>>,
    oauth2_client_credentials: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnection_AuthConfig_Oauth2ClientCredentials>>,
    oauth2_jwt_bearer: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer>>,
    ssh_public_key: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnection_AuthConfig_SshPublicKey>>,
    user_password: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnection_AuthConfig_UserPassword>>,
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnection_AuthConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_key: std::option::Option<terrars::PrimField<std::string::String>>,
    auth_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_variable: std
    ::option
    ::Option<std::vec::Vec<IntegrationConnectorsConnection_AuthConfig_AdditionalVariable>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth2_auth_code_flow: std
    ::option
    ::Option<std::vec::Vec<IntegrationConnectorsConnection_AuthConfig_Oauth2AuthCodeFlow>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth2_client_credentials: std
    ::option
    ::Option<std::vec::Vec<IntegrationConnectorsConnection_AuthConfig_Oauth2ClientCredentials>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth2_jwt_bearer: std::option::Option<std::vec::Vec<IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssh_public_key: std::option::Option<std::vec::Vec<IntegrationConnectorsConnection_AuthConfig_SshPublicKey>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_password: std::option::Option<std::vec::Vec<IntegrationConnectorsConnection_AuthConfig_UserPassword>>,
    dynamic: IntegrationConnectorsConnection_AuthConfigDynamic,
}

impl IntegrationConnectorsConnection_AuthConfig {
    #[doc = "Set the field `auth_key`.\nThe type of authentication configured."]
    pub fn set_auth_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `additional_variable`.\n"]
    pub fn set_additional_variable(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<IntegrationConnectorsConnection_AuthConfig_AdditionalVariable>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.additional_variable = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.additional_variable = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oauth2_auth_code_flow`.\n"]
    pub fn set_oauth2_auth_code_flow(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<IntegrationConnectorsConnection_AuthConfig_Oauth2AuthCodeFlow>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oauth2_auth_code_flow = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth2_auth_code_flow = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oauth2_client_credentials`.\n"]
    pub fn set_oauth2_client_credentials(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<IntegrationConnectorsConnection_AuthConfig_Oauth2ClientCredentials>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oauth2_client_credentials = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth2_client_credentials = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oauth2_jwt_bearer`.\n"]
    pub fn set_oauth2_jwt_bearer(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearer>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oauth2_jwt_bearer = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth2_jwt_bearer = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ssh_public_key`.\n"]
    pub fn set_ssh_public_key(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<IntegrationConnectorsConnection_AuthConfig_SshPublicKey>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ssh_public_key = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ssh_public_key = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `user_password`.\n"]
    pub fn set_user_password(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<IntegrationConnectorsConnection_AuthConfig_UserPassword>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.user_password = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.user_password = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IntegrationConnectorsConnection_AuthConfig {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnection_AuthConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnection_AuthConfig {
    #[doc =
        "authType of the Connection Possible values: [\"AUTH_TYPE_UNSPECIFIED\", \"USER_PASSWORD\", \"OAUTH2_JWT_BEARER\", \"OAUTH2_CLIENT_CREDENTIALS\", \"SSH_PUBLIC_KEY\", \"OAUTH2_AUTH_CODE_FLOW\"]"]
    pub auth_type: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnection_AuthConfig {
    pub fn build(self) -> IntegrationConnectorsConnection_AuthConfig {
        IntegrationConnectorsConnection_AuthConfig {
            auth_key: core::default::Default::default(),
            auth_type: self.auth_type,
            additional_variable: core::default::Default::default(),
            oauth2_auth_code_flow: core::default::Default::default(),
            oauth2_client_credentials: core::default::Default::default(),
            oauth2_jwt_bearer: core::default::Default::default(),
            ssh_public_key: core::default::Default::default(),
            user_password: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationConnectorsConnection_AuthConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnection_AuthConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IntegrationConnectorsConnection_AuthConfigRef {
        IntegrationConnectorsConnection_AuthConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnection_AuthConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `auth_key` after provisioning.\nThe type of authentication configured."]
    pub fn auth_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `auth_type` after provisioning.\nauthType of the Connection Possible values: [\"AUTH_TYPE_UNSPECIFIED\", \"USER_PASSWORD\", \"OAUTH2_JWT_BEARER\", \"OAUTH2_CLIENT_CREDENTIALS\", \"SSH_PUBLIC_KEY\", \"OAUTH2_AUTH_CODE_FLOW\"]"]
    pub fn auth_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_type", self.base))
    }

    #[doc = "Get a reference to the value of field `additional_variable` after provisioning.\n"]
    pub fn additional_variable(
        &self,
    ) -> terrars::ListRef<IntegrationConnectorsConnection_AuthConfig_AdditionalVariableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_variable", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth2_auth_code_flow` after provisioning.\n"]
    pub fn oauth2_auth_code_flow(
        &self,
    ) -> terrars::ListRef<IntegrationConnectorsConnection_AuthConfig_Oauth2AuthCodeFlowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth2_auth_code_flow", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth2_client_credentials` after provisioning.\n"]
    pub fn oauth2_client_credentials(
        &self,
    ) -> terrars::ListRef<IntegrationConnectorsConnection_AuthConfig_Oauth2ClientCredentialsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth2_client_credentials", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth2_jwt_bearer` after provisioning.\n"]
    pub fn oauth2_jwt_bearer(&self) -> terrars::ListRef<IntegrationConnectorsConnection_AuthConfig_Oauth2JwtBearerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth2_jwt_bearer", self.base))
    }

    #[doc = "Get a reference to the value of field `ssh_public_key` after provisioning.\n"]
    pub fn ssh_public_key(&self) -> terrars::ListRef<IntegrationConnectorsConnection_AuthConfig_SshPublicKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssh_public_key", self.base))
    }

    #[doc = "Get a reference to the value of field `user_password` after provisioning.\n"]
    pub fn user_password(&self) -> terrars::ListRef<IntegrationConnectorsConnection_AuthConfig_UserPasswordRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_password", self.base))
    }
}

#[derive(Serialize, Default)]
struct IntegrationConnectorsConnection_ConfigVariableDynamic {
    encryption_key_value: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnection_ConfigVariable_EncryptionKeyValue>>,
    secret_value: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnection_ConfigVariable_SecretValue>>,
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnection_ConfigVariable {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boolean_value: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    integer_value: std::option::Option<terrars::PrimField<f64>>,
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_key_value: std
    ::option
    ::Option<std::vec::Vec<IntegrationConnectorsConnection_ConfigVariable_EncryptionKeyValue>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_value: std::option::Option<std::vec::Vec<IntegrationConnectorsConnection_ConfigVariable_SecretValue>>,
    dynamic: IntegrationConnectorsConnection_ConfigVariableDynamic,
}

impl IntegrationConnectorsConnection_ConfigVariable {
    #[doc = "Set the field `boolean_value`.\nBoolean Value of configVariable"]
    pub fn set_boolean_value(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.boolean_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `integer_value`.\nInteger Value of configVariable"]
    pub fn set_integer_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.integer_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `string_value`.\nString Value of configVariabley"]
    pub fn set_string_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.string_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_key_value`.\n"]
    pub fn set_encryption_key_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<IntegrationConnectorsConnection_ConfigVariable_EncryptionKeyValue>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.encryption_key_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption_key_value = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `secret_value`.\n"]
    pub fn set_secret_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<IntegrationConnectorsConnection_ConfigVariable_SecretValue>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.secret_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.secret_value = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IntegrationConnectorsConnection_ConfigVariable {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnection_ConfigVariable>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnection_ConfigVariable {
    #[doc = "Key for the configVariable"]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnection_ConfigVariable {
    pub fn build(self) -> IntegrationConnectorsConnection_ConfigVariable {
        IntegrationConnectorsConnection_ConfigVariable {
            boolean_value: core::default::Default::default(),
            integer_value: core::default::Default::default(),
            key: self.key,
            string_value: core::default::Default::default(),
            encryption_key_value: core::default::Default::default(),
            secret_value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationConnectorsConnection_ConfigVariableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnection_ConfigVariableRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnection_ConfigVariableRef {
        IntegrationConnectorsConnection_ConfigVariableRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnection_ConfigVariableRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `boolean_value` after provisioning.\nBoolean Value of configVariable"]
    pub fn boolean_value(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boolean_value", self.base))
    }

    #[doc =
        "Get a reference to the value of field `integer_value` after provisioning.\nInteger Value of configVariable"]
    pub fn integer_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integer_value", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\nKey for the configVariable"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `string_value` after provisioning.\nString Value of configVariabley"]
    pub fn string_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.string_value", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_key_value` after provisioning.\n"]
    pub fn encryption_key_value(
        &self,
    ) -> terrars::ListRef<IntegrationConnectorsConnection_ConfigVariable_EncryptionKeyValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_key_value", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_value` after provisioning.\n"]
    pub fn secret_value(&self) -> terrars::ListRef<IntegrationConnectorsConnection_ConfigVariable_SecretValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_value", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnection_DestinationConfig_Destination {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_attachment: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IntegrationConnectorsConnection_DestinationConfig_Destination {
    #[doc = "Set the field `host`.\nFor publicly routable host."]
    pub fn set_host(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\nThe port is the target port number that is accepted by the destination."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_attachment`.\nPSC service attachments. Format: projects/*/regions/*/serviceAttachments/*"]
    pub fn set_service_attachment(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_attachment = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IntegrationConnectorsConnection_DestinationConfig_Destination {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnection_DestinationConfig_Destination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnection_DestinationConfig_Destination {}

impl BuildIntegrationConnectorsConnection_DestinationConfig_Destination {
    pub fn build(self) -> IntegrationConnectorsConnection_DestinationConfig_Destination {
        IntegrationConnectorsConnection_DestinationConfig_Destination {
            host: core::default::Default::default(),
            port: core::default::Default::default(),
            service_attachment: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationConnectorsConnection_DestinationConfig_DestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnection_DestinationConfig_DestinationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnection_DestinationConfig_DestinationRef {
        IntegrationConnectorsConnection_DestinationConfig_DestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnection_DestinationConfig_DestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\nFor publicly routable host."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe port is the target port number that is accepted by the destination."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_attachment` after provisioning.\nPSC service attachments. Format: projects/*/regions/*/serviceAttachments/*"]
    pub fn service_attachment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_attachment", self.base))
    }
}

#[derive(Serialize, Default)]
struct IntegrationConnectorsConnection_DestinationConfigDynamic {
    destination: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnection_DestinationConfig_Destination>>,
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnection_DestinationConfig {
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std::option::Option<std::vec::Vec<IntegrationConnectorsConnection_DestinationConfig_Destination>>,
    dynamic: IntegrationConnectorsConnection_DestinationConfigDynamic,
}

impl IntegrationConnectorsConnection_DestinationConfig {
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<IntegrationConnectorsConnection_DestinationConfig_Destination>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destination = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IntegrationConnectorsConnection_DestinationConfig {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnection_DestinationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnection_DestinationConfig {
    #[doc = "The key is the destination identifier that is supported by the Connector."]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnection_DestinationConfig {
    pub fn build(self) -> IntegrationConnectorsConnection_DestinationConfig {
        IntegrationConnectorsConnection_DestinationConfig {
            key: self.key,
            destination: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationConnectorsConnection_DestinationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnection_DestinationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnection_DestinationConfigRef {
        IntegrationConnectorsConnection_DestinationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnection_DestinationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `key` after provisioning.\nThe key is the destination identifier that is supported by the Connector."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> terrars::ListRef<IntegrationConnectorsConnection_DestinationConfig_DestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnection_SslConfig_AdditionalVariable_EncryptionKeyValue {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IntegrationConnectorsConnection_SslConfig_AdditionalVariable_EncryptionKeyValue {
    #[doc =
        "Set the field `kms_key_name`.\nThe [KMS key name] with which the content of the Operation is encrypted. The\nexpected format: projects/*/locations/*/keyRings/*/cryptoKeys/*.\nWill be empty string if google managed."]
    pub fn set_kms_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nType of Encryption Key Possible values: [\"GOOGLE_MANAGED\", \"CUSTOMER_MANAGED\"]"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IntegrationConnectorsConnection_SslConfig_AdditionalVariable_EncryptionKeyValue {
    type O =
        terrars::BlockAssignable<IntegrationConnectorsConnection_SslConfig_AdditionalVariable_EncryptionKeyValue>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnection_SslConfig_AdditionalVariable_EncryptionKeyValue {}

impl BuildIntegrationConnectorsConnection_SslConfig_AdditionalVariable_EncryptionKeyValue {
    pub fn build(self) -> IntegrationConnectorsConnection_SslConfig_AdditionalVariable_EncryptionKeyValue {
        IntegrationConnectorsConnection_SslConfig_AdditionalVariable_EncryptionKeyValue {
            kms_key_name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationConnectorsConnection_SslConfig_AdditionalVariable_EncryptionKeyValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnection_SslConfig_AdditionalVariable_EncryptionKeyValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnection_SslConfig_AdditionalVariable_EncryptionKeyValueRef {
        IntegrationConnectorsConnection_SslConfig_AdditionalVariable_EncryptionKeyValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnection_SslConfig_AdditionalVariable_EncryptionKeyValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nThe [KMS key name] with which the content of the Operation is encrypted. The\nexpected format: projects/*/locations/*/keyRings/*/cryptoKeys/*.\nWill be empty string if google managed."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nType of Encryption Key Possible values: [\"GOOGLE_MANAGED\", \"CUSTOMER_MANAGED\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct IntegrationConnectorsConnection_SslConfig_AdditionalVariableDynamic {
    encryption_key_value: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnection_SslConfig_AdditionalVariable_EncryptionKeyValue>>,
    secret_value: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnection_ConfigVariable_SecretValue>>,
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnection_SslConfig_AdditionalVariable {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boolean_value: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    integer_value: std::option::Option<terrars::PrimField<f64>>,
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_key_value: std
    ::option
    ::Option<std::vec::Vec<IntegrationConnectorsConnection_SslConfig_AdditionalVariable_EncryptionKeyValue>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_value: std::option::Option<std::vec::Vec<IntegrationConnectorsConnection_ConfigVariable_SecretValue>>,
    dynamic: IntegrationConnectorsConnection_SslConfig_AdditionalVariableDynamic,
}

impl IntegrationConnectorsConnection_SslConfig_AdditionalVariable {
    #[doc = "Set the field `boolean_value`.\nBoolean Value of configVariable."]
    pub fn set_boolean_value(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.boolean_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `integer_value`.\nInteger Value of configVariable."]
    pub fn set_integer_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.integer_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `string_value`.\nString Value of configVariabley."]
    pub fn set_string_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.string_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_key_value`.\n"]
    pub fn set_encryption_key_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            IntegrationConnectorsConnection_SslConfig_AdditionalVariable_EncryptionKeyValue,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.encryption_key_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption_key_value = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `secret_value`.\n"]
    pub fn set_secret_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<IntegrationConnectorsConnection_ConfigVariable_SecretValue>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.secret_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.secret_value = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IntegrationConnectorsConnection_SslConfig_AdditionalVariable {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnection_SslConfig_AdditionalVariable>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnection_SslConfig_AdditionalVariable {
    #[doc = "Key for the configVariable"]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnection_SslConfig_AdditionalVariable {
    pub fn build(self) -> IntegrationConnectorsConnection_SslConfig_AdditionalVariable {
        IntegrationConnectorsConnection_SslConfig_AdditionalVariable {
            boolean_value: core::default::Default::default(),
            integer_value: core::default::Default::default(),
            key: self.key,
            string_value: core::default::Default::default(),
            encryption_key_value: core::default::Default::default(),
            secret_value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationConnectorsConnection_SslConfig_AdditionalVariableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnection_SslConfig_AdditionalVariableRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnection_SslConfig_AdditionalVariableRef {
        IntegrationConnectorsConnection_SslConfig_AdditionalVariableRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnection_SslConfig_AdditionalVariableRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `boolean_value` after provisioning.\nBoolean Value of configVariable."]
    pub fn boolean_value(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boolean_value", self.base))
    }

    #[doc =
        "Get a reference to the value of field `integer_value` after provisioning.\nInteger Value of configVariable."]
    pub fn integer_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.integer_value", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\nKey for the configVariable"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `string_value` after provisioning.\nString Value of configVariabley."]
    pub fn string_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.string_value", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_key_value` after provisioning.\n"]
    pub fn encryption_key_value(
        &self,
    ) -> terrars::ListRef<IntegrationConnectorsConnection_SslConfig_AdditionalVariable_EncryptionKeyValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_key_value", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_value` after provisioning.\n"]
    pub fn secret_value(&self) -> terrars::ListRef<IntegrationConnectorsConnection_ConfigVariable_SecretValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct IntegrationConnectorsConnection_EventingConfig_AuthConfig_UserPasswordDynamic {
    password: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnection_AuthConfig_UserPassword_Password>>,
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnection_EventingConfig_AuthConfig_UserPassword {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<std::vec::Vec<IntegrationConnectorsConnection_AuthConfig_UserPassword_Password>>,
    dynamic: IntegrationConnectorsConnection_EventingConfig_AuthConfig_UserPasswordDynamic,
}

impl IntegrationConnectorsConnection_EventingConfig_AuthConfig_UserPassword {
    #[doc = "Set the field `username`.\nUsername for Authentication."]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `password`.\n"]
    pub fn set_password(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<IntegrationConnectorsConnection_AuthConfig_UserPassword_Password>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.password = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.password = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IntegrationConnectorsConnection_EventingConfig_AuthConfig_UserPassword {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnection_EventingConfig_AuthConfig_UserPassword>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnection_EventingConfig_AuthConfig_UserPassword {}

impl BuildIntegrationConnectorsConnection_EventingConfig_AuthConfig_UserPassword {
    pub fn build(self) -> IntegrationConnectorsConnection_EventingConfig_AuthConfig_UserPassword {
        IntegrationConnectorsConnection_EventingConfig_AuthConfig_UserPassword {
            username: core::default::Default::default(),
            password: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationConnectorsConnection_EventingConfig_AuthConfig_UserPasswordRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnection_EventingConfig_AuthConfig_UserPasswordRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnection_EventingConfig_AuthConfig_UserPasswordRef {
        IntegrationConnectorsConnection_EventingConfig_AuthConfig_UserPasswordRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnection_EventingConfig_AuthConfig_UserPasswordRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\nUsername for Authentication."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> terrars::ListRef<IntegrationConnectorsConnection_AuthConfig_UserPassword_PasswordRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.password", self.base))
    }
}

#[derive(Serialize, Default)]
struct IntegrationConnectorsConnection_EventingConfig_AuthConfigDynamic {
    additional_variable: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnection_SslConfig_AdditionalVariable>>,
    user_password: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnection_EventingConfig_AuthConfig_UserPassword>>,
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnection_EventingConfig_AuthConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_key: std::option::Option<terrars::PrimField<std::string::String>>,
    auth_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_variable: std::option::Option<std::vec::Vec<IntegrationConnectorsConnection_SslConfig_AdditionalVariable>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_password: std
    ::option
    ::Option<std::vec::Vec<IntegrationConnectorsConnection_EventingConfig_AuthConfig_UserPassword>>,
    dynamic: IntegrationConnectorsConnection_EventingConfig_AuthConfigDynamic,
}

impl IntegrationConnectorsConnection_EventingConfig_AuthConfig {
    #[doc = "Set the field `auth_key`.\nThe type of authentication configured."]
    pub fn set_auth_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `additional_variable`.\n"]
    pub fn set_additional_variable(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<IntegrationConnectorsConnection_SslConfig_AdditionalVariable>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.additional_variable = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.additional_variable = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `user_password`.\n"]
    pub fn set_user_password(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<IntegrationConnectorsConnection_EventingConfig_AuthConfig_UserPassword>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.user_password = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.user_password = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IntegrationConnectorsConnection_EventingConfig_AuthConfig {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnection_EventingConfig_AuthConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnection_EventingConfig_AuthConfig {
    #[doc = "authType of the Connection Possible values: [\"USER_PASSWORD\"]"]
    pub auth_type: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnection_EventingConfig_AuthConfig {
    pub fn build(self) -> IntegrationConnectorsConnection_EventingConfig_AuthConfig {
        IntegrationConnectorsConnection_EventingConfig_AuthConfig {
            auth_key: core::default::Default::default(),
            auth_type: self.auth_type,
            additional_variable: core::default::Default::default(),
            user_password: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationConnectorsConnection_EventingConfig_AuthConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnection_EventingConfig_AuthConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnection_EventingConfig_AuthConfigRef {
        IntegrationConnectorsConnection_EventingConfig_AuthConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnection_EventingConfig_AuthConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `auth_key` after provisioning.\nThe type of authentication configured."]
    pub fn auth_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `auth_type` after provisioning.\nauthType of the Connection Possible values: [\"USER_PASSWORD\"]"]
    pub fn auth_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_type", self.base))
    }

    #[doc = "Get a reference to the value of field `additional_variable` after provisioning.\n"]
    pub fn additional_variable(
        &self,
    ) -> terrars::ListRef<IntegrationConnectorsConnection_SslConfig_AdditionalVariableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_variable", self.base))
    }

    #[doc = "Get a reference to the value of field `user_password` after provisioning.\n"]
    pub fn user_password(
        &self,
    ) -> terrars::ListRef<IntegrationConnectorsConnection_EventingConfig_AuthConfig_UserPasswordRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_password", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnection_EventingConfig_RegistrationDestinationConfig_Destination {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_attachment: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IntegrationConnectorsConnection_EventingConfig_RegistrationDestinationConfig_Destination {
    #[doc = "Set the field `host`.\nHost"]
    pub fn set_host(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\nport number"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_attachment`.\nService Attachment"]
    pub fn set_service_attachment(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_attachment = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for IntegrationConnectorsConnection_EventingConfig_RegistrationDestinationConfig_Destination {
    type O =
        terrars
        ::BlockAssignable<IntegrationConnectorsConnection_EventingConfig_RegistrationDestinationConfig_Destination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnection_EventingConfig_RegistrationDestinationConfig_Destination {}

impl BuildIntegrationConnectorsConnection_EventingConfig_RegistrationDestinationConfig_Destination {
    pub fn build(self) -> IntegrationConnectorsConnection_EventingConfig_RegistrationDestinationConfig_Destination {
        IntegrationConnectorsConnection_EventingConfig_RegistrationDestinationConfig_Destination {
            host: core::default::Default::default(),
            port: core::default::Default::default(),
            service_attachment: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationConnectorsConnection_EventingConfig_RegistrationDestinationConfig_DestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnection_EventingConfig_RegistrationDestinationConfig_DestinationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnection_EventingConfig_RegistrationDestinationConfig_DestinationRef {
        IntegrationConnectorsConnection_EventingConfig_RegistrationDestinationConfig_DestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnection_EventingConfig_RegistrationDestinationConfig_DestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\nHost"]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\nport number"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `service_attachment` after provisioning.\nService Attachment"]
    pub fn service_attachment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_attachment", self.base))
    }
}

#[derive(Serialize, Default)]
struct IntegrationConnectorsConnection_EventingConfig_RegistrationDestinationConfigDynamic {
    destination: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<IntegrationConnectorsConnection_EventingConfig_RegistrationDestinationConfig_Destination>,
    >,
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnection_EventingConfig_RegistrationDestinationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std
    ::option
    ::Option<std::vec::Vec<IntegrationConnectorsConnection_EventingConfig_RegistrationDestinationConfig_Destination>>,
    dynamic: IntegrationConnectorsConnection_EventingConfig_RegistrationDestinationConfigDynamic,
}

impl IntegrationConnectorsConnection_EventingConfig_RegistrationDestinationConfig {
    #[doc = "Set the field `key`.\nKey for the connection"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            IntegrationConnectorsConnection_EventingConfig_RegistrationDestinationConfig_Destination,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destination = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IntegrationConnectorsConnection_EventingConfig_RegistrationDestinationConfig {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnection_EventingConfig_RegistrationDestinationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnection_EventingConfig_RegistrationDestinationConfig {}

impl BuildIntegrationConnectorsConnection_EventingConfig_RegistrationDestinationConfig {
    pub fn build(self) -> IntegrationConnectorsConnection_EventingConfig_RegistrationDestinationConfig {
        IntegrationConnectorsConnection_EventingConfig_RegistrationDestinationConfig {
            key: core::default::Default::default(),
            destination: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationConnectorsConnection_EventingConfig_RegistrationDestinationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnection_EventingConfig_RegistrationDestinationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnection_EventingConfig_RegistrationDestinationConfigRef {
        IntegrationConnectorsConnection_EventingConfig_RegistrationDestinationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnection_EventingConfig_RegistrationDestinationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\nKey for the connection"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(
        &self,
    ) -> terrars
    ::ListRef<IntegrationConnectorsConnection_EventingConfig_RegistrationDestinationConfig_DestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }
}

#[derive(Serialize, Default)]
struct IntegrationConnectorsConnection_EventingConfigDynamic {
    additional_variable: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnection_SslConfig_AdditionalVariable>>,
    auth_config: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnection_EventingConfig_AuthConfig>>,
    registration_destination_config: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnection_EventingConfig_RegistrationDestinationConfig>>,
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnection_EventingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enrichment_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_variable: std::option::Option<std::vec::Vec<IntegrationConnectorsConnection_SslConfig_AdditionalVariable>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_config: std::option::Option<std::vec::Vec<IntegrationConnectorsConnection_EventingConfig_AuthConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registration_destination_config: std
    ::option
    ::Option<std::vec::Vec<IntegrationConnectorsConnection_EventingConfig_RegistrationDestinationConfig>>,
    dynamic: IntegrationConnectorsConnection_EventingConfigDynamic,
}

impl IntegrationConnectorsConnection_EventingConfig {
    #[doc = "Set the field `enrichment_enabled`.\nEnrichment Enabled."]
    pub fn set_enrichment_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enrichment_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `additional_variable`.\n"]
    pub fn set_additional_variable(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<IntegrationConnectorsConnection_SslConfig_AdditionalVariable>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.additional_variable = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.additional_variable = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `auth_config`.\n"]
    pub fn set_auth_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<IntegrationConnectorsConnection_EventingConfig_AuthConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.auth_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.auth_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `registration_destination_config`.\n"]
    pub fn set_registration_destination_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            IntegrationConnectorsConnection_EventingConfig_RegistrationDestinationConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.registration_destination_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.registration_destination_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IntegrationConnectorsConnection_EventingConfig {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnection_EventingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnection_EventingConfig {}

impl BuildIntegrationConnectorsConnection_EventingConfig {
    pub fn build(self) -> IntegrationConnectorsConnection_EventingConfig {
        IntegrationConnectorsConnection_EventingConfig {
            enrichment_enabled: core::default::Default::default(),
            additional_variable: core::default::Default::default(),
            auth_config: core::default::Default::default(),
            registration_destination_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationConnectorsConnection_EventingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnection_EventingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnection_EventingConfigRef {
        IntegrationConnectorsConnection_EventingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnection_EventingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enrichment_enabled` after provisioning.\nEnrichment Enabled."]
    pub fn enrichment_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enrichment_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `additional_variable` after provisioning.\n"]
    pub fn additional_variable(
        &self,
    ) -> terrars::ListRef<IntegrationConnectorsConnection_SslConfig_AdditionalVariableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_variable", self.base))
    }

    #[doc = "Get a reference to the value of field `auth_config` after provisioning.\n"]
    pub fn auth_config(&self) -> terrars::ListRef<IntegrationConnectorsConnection_EventingConfig_AuthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auth_config", self.base))
    }

    #[doc = "Get a reference to the value of field `registration_destination_config` after provisioning.\n"]
    pub fn registration_destination_config(
        &self,
    ) -> terrars::ListRef<IntegrationConnectorsConnection_EventingConfig_RegistrationDestinationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.registration_destination_config", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnection_LockConfig {
    locked: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reason: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IntegrationConnectorsConnection_LockConfig {
    #[doc = "Set the field `reason`.\nDescribes why a connection is locked."]
    pub fn set_reason(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.reason = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IntegrationConnectorsConnection_LockConfig {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnection_LockConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnection_LockConfig {
    #[doc = "Indicates whether or not the connection is locked."]
    pub locked: terrars::PrimField<bool>,
}

impl BuildIntegrationConnectorsConnection_LockConfig {
    pub fn build(self) -> IntegrationConnectorsConnection_LockConfig {
        IntegrationConnectorsConnection_LockConfig {
            locked: self.locked,
            reason: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationConnectorsConnection_LockConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnection_LockConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IntegrationConnectorsConnection_LockConfigRef {
        IntegrationConnectorsConnection_LockConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnection_LockConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `locked` after provisioning.\nIndicates whether or not the connection is locked."]
    pub fn locked(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.locked", self.base))
    }

    #[doc = "Get a reference to the value of field `reason` after provisioning.\nDescribes why a connection is locked."]
    pub fn reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reason", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnection_LogConfig {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IntegrationConnectorsConnection_LogConfig {
    #[doc =
        "Set the field `level`.\nLog configuration level. Possible values: [\"LOG_LEVEL_UNSPECIFIED\", \"ERROR\", \"INFO\", \"DEBUG\"]"]
    pub fn set_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.level = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IntegrationConnectorsConnection_LogConfig {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnection_LogConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnection_LogConfig {
    #[doc = "Enabled represents whether logging is enabled or not for a connection."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildIntegrationConnectorsConnection_LogConfig {
    pub fn build(self) -> IntegrationConnectorsConnection_LogConfig {
        IntegrationConnectorsConnection_LogConfig {
            enabled: self.enabled,
            level: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationConnectorsConnection_LogConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnection_LogConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IntegrationConnectorsConnection_LogConfigRef {
        IntegrationConnectorsConnection_LogConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnection_LogConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nEnabled represents whether logging is enabled or not for a connection."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `level` after provisioning.\nLog configuration level. Possible values: [\"LOG_LEVEL_UNSPECIFIED\", \"ERROR\", \"INFO\", \"DEBUG\"]"]
    pub fn level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.level", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnection_NodeConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_node_count: std::option::Option<terrars::PrimField<f64>>,
}

impl IntegrationConnectorsConnection_NodeConfig {
    #[doc = "Set the field `max_node_count`.\nMinimum number of nodes in the runtime nodes."]
    pub fn set_max_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_node_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_node_count`.\nMinimum number of nodes in the runtime nodes."]
    pub fn set_min_node_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_node_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IntegrationConnectorsConnection_NodeConfig {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnection_NodeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnection_NodeConfig {}

impl BuildIntegrationConnectorsConnection_NodeConfig {
    pub fn build(self) -> IntegrationConnectorsConnection_NodeConfig {
        IntegrationConnectorsConnection_NodeConfig {
            max_node_count: core::default::Default::default(),
            min_node_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationConnectorsConnection_NodeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnection_NodeConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IntegrationConnectorsConnection_NodeConfigRef {
        IntegrationConnectorsConnection_NodeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnection_NodeConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_node_count` after provisioning.\nMinimum number of nodes in the runtime nodes."]
    pub fn max_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_node_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_node_count` after provisioning.\nMinimum number of nodes in the runtime nodes."]
    pub fn min_node_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_node_count", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnection_SslConfig_ClientCertificate {
    secret_version: terrars::PrimField<std::string::String>,
}

impl IntegrationConnectorsConnection_SslConfig_ClientCertificate { }

impl terrars::ToListMappable for IntegrationConnectorsConnection_SslConfig_ClientCertificate {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnection_SslConfig_ClientCertificate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnection_SslConfig_ClientCertificate {
    #[doc = "Secret version of Secret Value for Config variable."]
    pub secret_version: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnection_SslConfig_ClientCertificate {
    pub fn build(self) -> IntegrationConnectorsConnection_SslConfig_ClientCertificate {
        IntegrationConnectorsConnection_SslConfig_ClientCertificate { secret_version: self.secret_version }
    }
}

#[allow(dead_code)]
pub struct IntegrationConnectorsConnection_SslConfig_ClientCertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnection_SslConfig_ClientCertificateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnection_SslConfig_ClientCertificateRef {
        IntegrationConnectorsConnection_SslConfig_ClientCertificateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnection_SslConfig_ClientCertificateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `secret_version` after provisioning.\nSecret version of Secret Value for Config variable."]
    pub fn secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_version", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnection_SslConfig_ClientPrivateKey {
    secret_version: terrars::PrimField<std::string::String>,
}

impl IntegrationConnectorsConnection_SslConfig_ClientPrivateKey { }

impl terrars::ToListMappable for IntegrationConnectorsConnection_SslConfig_ClientPrivateKey {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnection_SslConfig_ClientPrivateKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnection_SslConfig_ClientPrivateKey {
    #[doc = "Secret version of Secret Value for Config variable."]
    pub secret_version: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnection_SslConfig_ClientPrivateKey {
    pub fn build(self) -> IntegrationConnectorsConnection_SslConfig_ClientPrivateKey {
        IntegrationConnectorsConnection_SslConfig_ClientPrivateKey { secret_version: self.secret_version }
    }
}

#[allow(dead_code)]
pub struct IntegrationConnectorsConnection_SslConfig_ClientPrivateKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnection_SslConfig_ClientPrivateKeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnection_SslConfig_ClientPrivateKeyRef {
        IntegrationConnectorsConnection_SslConfig_ClientPrivateKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnection_SslConfig_ClientPrivateKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `secret_version` after provisioning.\nSecret version of Secret Value for Config variable."]
    pub fn secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_version", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnection_SslConfig_ClientPrivateKeyPass {
    secret_version: terrars::PrimField<std::string::String>,
}

impl IntegrationConnectorsConnection_SslConfig_ClientPrivateKeyPass { }

impl terrars::ToListMappable for IntegrationConnectorsConnection_SslConfig_ClientPrivateKeyPass {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnection_SslConfig_ClientPrivateKeyPass>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnection_SslConfig_ClientPrivateKeyPass {
    #[doc = "Secret version of Secret Value for Config variable."]
    pub secret_version: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnection_SslConfig_ClientPrivateKeyPass {
    pub fn build(self) -> IntegrationConnectorsConnection_SslConfig_ClientPrivateKeyPass {
        IntegrationConnectorsConnection_SslConfig_ClientPrivateKeyPass { secret_version: self.secret_version }
    }
}

#[allow(dead_code)]
pub struct IntegrationConnectorsConnection_SslConfig_ClientPrivateKeyPassRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnection_SslConfig_ClientPrivateKeyPassRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnection_SslConfig_ClientPrivateKeyPassRef {
        IntegrationConnectorsConnection_SslConfig_ClientPrivateKeyPassRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnection_SslConfig_ClientPrivateKeyPassRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `secret_version` after provisioning.\nSecret version of Secret Value for Config variable."]
    pub fn secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_version", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnection_SslConfig_PrivateServerCertificate {
    secret_version: terrars::PrimField<std::string::String>,
}

impl IntegrationConnectorsConnection_SslConfig_PrivateServerCertificate { }

impl terrars::ToListMappable for IntegrationConnectorsConnection_SslConfig_PrivateServerCertificate {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnection_SslConfig_PrivateServerCertificate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnection_SslConfig_PrivateServerCertificate {
    #[doc = "Secret version of Secret Value for Config variable."]
    pub secret_version: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnection_SslConfig_PrivateServerCertificate {
    pub fn build(self) -> IntegrationConnectorsConnection_SslConfig_PrivateServerCertificate {
        IntegrationConnectorsConnection_SslConfig_PrivateServerCertificate { secret_version: self.secret_version }
    }
}

#[allow(dead_code)]
pub struct IntegrationConnectorsConnection_SslConfig_PrivateServerCertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnection_SslConfig_PrivateServerCertificateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnection_SslConfig_PrivateServerCertificateRef {
        IntegrationConnectorsConnection_SslConfig_PrivateServerCertificateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnection_SslConfig_PrivateServerCertificateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `secret_version` after provisioning.\nSecret version of Secret Value for Config variable."]
    pub fn secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_version", self.base))
    }
}

#[derive(Serialize, Default)]
struct IntegrationConnectorsConnection_SslConfigDynamic {
    additional_variable: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnection_SslConfig_AdditionalVariable>>,
    client_certificate: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnection_SslConfig_ClientCertificate>>,
    client_private_key: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnection_SslConfig_ClientPrivateKey>>,
    client_private_key_pass: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnection_SslConfig_ClientPrivateKeyPass>>,
    private_server_certificate: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnection_SslConfig_PrivateServerCertificate>>,
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnection_SslConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_cert_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_cert_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust_model: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_ssl: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_variable: std::option::Option<std::vec::Vec<IntegrationConnectorsConnection_SslConfig_AdditionalVariable>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_certificate: std::option::Option<std::vec::Vec<IntegrationConnectorsConnection_SslConfig_ClientCertificate>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_private_key: std::option::Option<std::vec::Vec<IntegrationConnectorsConnection_SslConfig_ClientPrivateKey>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_private_key_pass: std
    ::option
    ::Option<std::vec::Vec<IntegrationConnectorsConnection_SslConfig_ClientPrivateKeyPass>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_server_certificate: std
    ::option
    ::Option<std::vec::Vec<IntegrationConnectorsConnection_SslConfig_PrivateServerCertificate>>,
    dynamic: IntegrationConnectorsConnection_SslConfigDynamic,
}

impl IntegrationConnectorsConnection_SslConfig {
    #[doc = "Set the field `client_cert_type`.\nType of Client Cert (PEM/JKS/.. etc.) Possible values: [\"PEM\"]"]
    pub fn set_client_cert_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_cert_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `server_cert_type`.\nType of Server Cert (PEM/JKS/.. etc.) Possible values: [\"PEM\"]"]
    pub fn set_server_cert_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.server_cert_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `trust_model`.\nEnum for Trust Model Possible values: [\"PUBLIC\", \"PRIVATE\", \"INSECURE\"]"]
    pub fn set_trust_model(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.trust_model = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_ssl`.\nBool for enabling SSL"]
    pub fn set_use_ssl(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_ssl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `additional_variable`.\n"]
    pub fn set_additional_variable(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<IntegrationConnectorsConnection_SslConfig_AdditionalVariable>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.additional_variable = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.additional_variable = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `client_certificate`.\n"]
    pub fn set_client_certificate(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<IntegrationConnectorsConnection_SslConfig_ClientCertificate>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.client_certificate = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.client_certificate = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `client_private_key`.\n"]
    pub fn set_client_private_key(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<IntegrationConnectorsConnection_SslConfig_ClientPrivateKey>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.client_private_key = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.client_private_key = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `client_private_key_pass`.\n"]
    pub fn set_client_private_key_pass(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<IntegrationConnectorsConnection_SslConfig_ClientPrivateKeyPass>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.client_private_key_pass = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.client_private_key_pass = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `private_server_certificate`.\n"]
    pub fn set_private_server_certificate(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<IntegrationConnectorsConnection_SslConfig_PrivateServerCertificate>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.private_server_certificate = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.private_server_certificate = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IntegrationConnectorsConnection_SslConfig {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnection_SslConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnection_SslConfig {
    #[doc = "Enum for controlling the SSL Type (TLS/MTLS) Possible values: [\"TLS\", \"MTLS\"]"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnection_SslConfig {
    pub fn build(self) -> IntegrationConnectorsConnection_SslConfig {
        IntegrationConnectorsConnection_SslConfig {
            client_cert_type: core::default::Default::default(),
            server_cert_type: core::default::Default::default(),
            trust_model: core::default::Default::default(),
            type_: self.type_,
            use_ssl: core::default::Default::default(),
            additional_variable: core::default::Default::default(),
            client_certificate: core::default::Default::default(),
            client_private_key: core::default::Default::default(),
            client_private_key_pass: core::default::Default::default(),
            private_server_certificate: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationConnectorsConnection_SslConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnection_SslConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IntegrationConnectorsConnection_SslConfigRef {
        IntegrationConnectorsConnection_SslConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnection_SslConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `client_cert_type` after provisioning.\nType of Client Cert (PEM/JKS/.. etc.) Possible values: [\"PEM\"]"]
    pub fn client_cert_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_cert_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `server_cert_type` after provisioning.\nType of Server Cert (PEM/JKS/.. etc.) Possible values: [\"PEM\"]"]
    pub fn server_cert_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server_cert_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `trust_model` after provisioning.\nEnum for Trust Model Possible values: [\"PUBLIC\", \"PRIVATE\", \"INSECURE\"]"]
    pub fn trust_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_model", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nEnum for controlling the SSL Type (TLS/MTLS) Possible values: [\"TLS\", \"MTLS\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `use_ssl` after provisioning.\nBool for enabling SSL"]
    pub fn use_ssl(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_ssl", self.base))
    }

    #[doc = "Get a reference to the value of field `additional_variable` after provisioning.\n"]
    pub fn additional_variable(
        &self,
    ) -> terrars::ListRef<IntegrationConnectorsConnection_SslConfig_AdditionalVariableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_variable", self.base))
    }

    #[doc = "Get a reference to the value of field `client_certificate` after provisioning.\n"]
    pub fn client_certificate(&self) -> terrars::ListRef<IntegrationConnectorsConnection_SslConfig_ClientCertificateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `client_private_key` after provisioning.\n"]
    pub fn client_private_key(&self) -> terrars::ListRef<IntegrationConnectorsConnection_SslConfig_ClientPrivateKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_private_key", self.base))
    }

    #[doc = "Get a reference to the value of field `client_private_key_pass` after provisioning.\n"]
    pub fn client_private_key_pass(
        &self,
    ) -> terrars::ListRef<IntegrationConnectorsConnection_SslConfig_ClientPrivateKeyPassRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_private_key_pass", self.base))
    }

    #[doc = "Get a reference to the value of field `private_server_certificate` after provisioning.\n"]
    pub fn private_server_certificate(
        &self,
    ) -> terrars::ListRef<IntegrationConnectorsConnection_SslConfig_PrivateServerCertificateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_server_certificate", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnection_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IntegrationConnectorsConnection_Timeouts {
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

impl terrars::ToListMappable for IntegrationConnectorsConnection_Timeouts {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnection_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnection_Timeouts {}

impl BuildIntegrationConnectorsConnection_Timeouts {
    pub fn build(self) -> IntegrationConnectorsConnection_Timeouts {
        IntegrationConnectorsConnection_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationConnectorsConnection_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnection_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IntegrationConnectorsConnection_TimeoutsRef {
        IntegrationConnectorsConnection_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnection_TimeoutsRef {
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
