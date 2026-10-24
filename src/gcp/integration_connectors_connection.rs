use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct IntegrationConnectorsConnectionData {
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
    auth_config: std::option::Option<std::vec::Vec<IntegrationConnectorsConnectionAuthConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config_variable: std::option::Option<std::vec::Vec<IntegrationConnectorsConnectionConfigVariableEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_config: std::option::Option<std::vec::Vec<IntegrationConnectorsConnectionDestinationConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eventing_config: std::option::Option<std::vec::Vec<IntegrationConnectorsConnectionEventingConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lock_config: std::option::Option<std::vec::Vec<IntegrationConnectorsConnectionLockConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_config: std::option::Option<std::vec::Vec<IntegrationConnectorsConnectionLogConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_config: std::option::Option<std::vec::Vec<IntegrationConnectorsConnectionNodeConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_config: std::option::Option<std::vec::Vec<IntegrationConnectorsConnectionSslConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<IntegrationConnectorsConnectionTimeoutsEl>,
    dynamic: IntegrationConnectorsConnectionDynamic,
}

struct IntegrationConnectorsConnection_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<IntegrationConnectorsConnectionData>,
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

    #[doc = "Set the field `description`.\nAn arbitrary description for the Conection."]
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
        v: impl std::convert::Into<terrars::BlockAssignable<IntegrationConnectorsConnectionAuthConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<IntegrationConnectorsConnectionConfigVariableEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<IntegrationConnectorsConnectionDestinationConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<IntegrationConnectorsConnectionEventingConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<IntegrationConnectorsConnectionLockConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<IntegrationConnectorsConnectionLogConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<IntegrationConnectorsConnectionNodeConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<IntegrationConnectorsConnectionSslConfigEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<IntegrationConnectorsConnectionTimeoutsEl>) -> Self {
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
        "Get a reference to the value of field `connector_version_infra_config` after provisioning.\nThis cofiguration provides infra configs like rate limit threshold which need to be configurable for every connector version."]
    pub fn connector_version_infra_config(
        &self,
    ) -> terrars::ListRef<IntegrationConnectorsConnectionConnectorVersionInfraConfigElRef> {
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
        "Get a reference to the value of field `description` after provisioning.\nAn arbitrary description for the Conection."]
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
    pub fn eventing_runtime_data(&self) -> terrars::ListRef<IntegrationConnectorsConnectionEventingRuntimeDataElRef> {
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
    pub fn status(&self) -> terrars::ListRef<IntegrationConnectorsConnectionStatusElRef> {
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
    pub fn auth_config(&self) -> terrars::ListRef<IntegrationConnectorsConnectionAuthConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auth_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config_variable` after provisioning.\n"]
    pub fn config_variable(&self) -> terrars::ListRef<IntegrationConnectorsConnectionConfigVariableElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config_variable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_config` after provisioning.\n"]
    pub fn destination_config(&self) -> terrars::ListRef<IntegrationConnectorsConnectionDestinationConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `eventing_config` after provisioning.\n"]
    pub fn eventing_config(&self) -> terrars::ListRef<IntegrationConnectorsConnectionEventingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.eventing_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lock_config` after provisioning.\n"]
    pub fn lock_config(&self) -> terrars::ListRef<IntegrationConnectorsConnectionLockConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lock_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_config` after provisioning.\n"]
    pub fn log_config(&self) -> terrars::ListRef<IntegrationConnectorsConnectionLogConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> terrars::ListRef<IntegrationConnectorsConnectionNodeConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ssl_config` after provisioning.\n"]
    pub fn ssl_config(&self) -> terrars::ListRef<IntegrationConnectorsConnectionSslConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssl_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IntegrationConnectorsConnectionTimeoutsElRef {
        IntegrationConnectorsConnectionTimeoutsElRef::new(
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
            data: std::cell::RefCell::new(IntegrationConnectorsConnectionData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                connector_version: self.connector_version,
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
        "Get a reference to the value of field `connector_version_infra_config` after provisioning.\nThis cofiguration provides infra configs like rate limit threshold which need to be configurable for every connector version."]
    pub fn connector_version_infra_config(
        &self,
    ) -> terrars::ListRef<IntegrationConnectorsConnectionConnectorVersionInfraConfigElRef> {
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
        "Get a reference to the value of field `description` after provisioning.\nAn arbitrary description for the Conection."]
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
    pub fn eventing_runtime_data(&self) -> terrars::ListRef<IntegrationConnectorsConnectionEventingRuntimeDataElRef> {
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
    pub fn status(&self) -> terrars::ListRef<IntegrationConnectorsConnectionStatusElRef> {
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
    pub fn auth_config(&self) -> terrars::ListRef<IntegrationConnectorsConnectionAuthConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auth_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config_variable` after provisioning.\n"]
    pub fn config_variable(&self) -> terrars::ListRef<IntegrationConnectorsConnectionConfigVariableElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config_variable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_config` after provisioning.\n"]
    pub fn destination_config(&self) -> terrars::ListRef<IntegrationConnectorsConnectionDestinationConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `eventing_config` after provisioning.\n"]
    pub fn eventing_config(&self) -> terrars::ListRef<IntegrationConnectorsConnectionEventingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.eventing_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lock_config` after provisioning.\n"]
    pub fn lock_config(&self) -> terrars::ListRef<IntegrationConnectorsConnectionLockConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lock_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_config` after provisioning.\n"]
    pub fn log_config(&self) -> terrars::ListRef<IntegrationConnectorsConnectionLogConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> terrars::ListRef<IntegrationConnectorsConnectionNodeConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ssl_config` after provisioning.\n"]
    pub fn ssl_config(&self) -> terrars::ListRef<IntegrationConnectorsConnectionSslConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssl_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IntegrationConnectorsConnectionTimeoutsElRef {
        IntegrationConnectorsConnectionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnectionConnectorVersionInfraConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ratelimit_threshold: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IntegrationConnectorsConnectionConnectorVersionInfraConfigEl {
    #[doc = "Set the field `ratelimit_threshold`.\n"]
    pub fn set_ratelimit_threshold(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ratelimit_threshold = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IntegrationConnectorsConnectionConnectorVersionInfraConfigEl {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnectionConnectorVersionInfraConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionConnectorVersionInfraConfigEl {}

impl BuildIntegrationConnectorsConnectionConnectorVersionInfraConfigEl {
    pub fn build(self) -> IntegrationConnectorsConnectionConnectorVersionInfraConfigEl {
        IntegrationConnectorsConnectionConnectorVersionInfraConfigEl {
            ratelimit_threshold: core::default::Default::default(),
        }
    }
}

pub struct IntegrationConnectorsConnectionConnectorVersionInfraConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionConnectorVersionInfraConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionConnectorVersionInfraConfigElRef {
        IntegrationConnectorsConnectionConnectorVersionInfraConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionConnectorVersionInfraConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ratelimit_threshold` after provisioning.\n"]
    pub fn ratelimit_threshold(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ratelimit_threshold", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnectionEventingRuntimeDataElStatusEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IntegrationConnectorsConnectionEventingRuntimeDataElStatusEl {
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

impl terrars::ToListMappable for IntegrationConnectorsConnectionEventingRuntimeDataElStatusEl {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnectionEventingRuntimeDataElStatusEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionEventingRuntimeDataElStatusEl {}

impl BuildIntegrationConnectorsConnectionEventingRuntimeDataElStatusEl {
    pub fn build(self) -> IntegrationConnectorsConnectionEventingRuntimeDataElStatusEl {
        IntegrationConnectorsConnectionEventingRuntimeDataElStatusEl {
            description: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

pub struct IntegrationConnectorsConnectionEventingRuntimeDataElStatusElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionEventingRuntimeDataElStatusElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionEventingRuntimeDataElStatusElRef {
        IntegrationConnectorsConnectionEventingRuntimeDataElStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionEventingRuntimeDataElStatusElRef {
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
pub struct IntegrationConnectorsConnectionEventingRuntimeDataEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    events_listener_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::ListField<IntegrationConnectorsConnectionEventingRuntimeDataElStatusEl>>,
}

impl IntegrationConnectorsConnectionEventingRuntimeDataEl {
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
        v: impl std::convert::Into<terrars::ListField<IntegrationConnectorsConnectionEventingRuntimeDataElStatusEl>>,
    ) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IntegrationConnectorsConnectionEventingRuntimeDataEl {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnectionEventingRuntimeDataEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionEventingRuntimeDataEl {}

impl BuildIntegrationConnectorsConnectionEventingRuntimeDataEl {
    pub fn build(self) -> IntegrationConnectorsConnectionEventingRuntimeDataEl {
        IntegrationConnectorsConnectionEventingRuntimeDataEl {
            events_listener_endpoint: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct IntegrationConnectorsConnectionEventingRuntimeDataElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionEventingRuntimeDataElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionEventingRuntimeDataElRef {
        IntegrationConnectorsConnectionEventingRuntimeDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionEventingRuntimeDataElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `events_listener_endpoint` after provisioning.\n"]
    pub fn events_listener_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.events_listener_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::ListRef<IntegrationConnectorsConnectionEventingRuntimeDataElStatusElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnectionStatusEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IntegrationConnectorsConnectionStatusEl {
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

impl terrars::ToListMappable for IntegrationConnectorsConnectionStatusEl {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnectionStatusEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionStatusEl {}

impl BuildIntegrationConnectorsConnectionStatusEl {
    pub fn build(self) -> IntegrationConnectorsConnectionStatusEl {
        IntegrationConnectorsConnectionStatusEl {
            description: core::default::Default::default(),
            state: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct IntegrationConnectorsConnectionStatusElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionStatusElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IntegrationConnectorsConnectionStatusElRef {
        IntegrationConnectorsConnectionStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionStatusElRef {
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
pub struct IntegrationConnectorsConnectionAuthConfigElAdditionalVariableElEncryptionKeyValueEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl IntegrationConnectorsConnectionAuthConfigElAdditionalVariableElEncryptionKeyValueEl {
    #[doc =
        "Set the field `kms_key_name`.\nThe [KMS key name] with which the content of the Operation is encrypted. The expected\nformat: projects/*/locations/*/keyRings/*/cryptoKeys/*.\nWill be empty string if google managed."]
    pub fn set_kms_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IntegrationConnectorsConnectionAuthConfigElAdditionalVariableElEncryptionKeyValueEl {
    type O =
        terrars
        ::BlockAssignable<IntegrationConnectorsConnectionAuthConfigElAdditionalVariableElEncryptionKeyValueEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionAuthConfigElAdditionalVariableElEncryptionKeyValueEl {
    #[doc = "Type of Encription Key Possible values: [\"GOOGLE_MANAGED\", \"CUSTOMER_MANAGED\"]"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnectionAuthConfigElAdditionalVariableElEncryptionKeyValueEl {
    pub fn build(self) -> IntegrationConnectorsConnectionAuthConfigElAdditionalVariableElEncryptionKeyValueEl {
        IntegrationConnectorsConnectionAuthConfigElAdditionalVariableElEncryptionKeyValueEl {
            kms_key_name: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct IntegrationConnectorsConnectionAuthConfigElAdditionalVariableElEncryptionKeyValueElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionAuthConfigElAdditionalVariableElEncryptionKeyValueElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionAuthConfigElAdditionalVariableElEncryptionKeyValueElRef {
        IntegrationConnectorsConnectionAuthConfigElAdditionalVariableElEncryptionKeyValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionAuthConfigElAdditionalVariableElEncryptionKeyValueElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nThe [KMS key name] with which the content of the Operation is encrypted. The expected\nformat: projects/*/locations/*/keyRings/*/cryptoKeys/*.\nWill be empty string if google managed."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nType of Encription Key Possible values: [\"GOOGLE_MANAGED\", \"CUSTOMER_MANAGED\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnectionAuthConfigElAdditionalVariableElSecretValueEl {
    secret_version: terrars::PrimField<std::string::String>,
}

impl IntegrationConnectorsConnectionAuthConfigElAdditionalVariableElSecretValueEl { }

impl terrars::ToListMappable for IntegrationConnectorsConnectionAuthConfigElAdditionalVariableElSecretValueEl {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnectionAuthConfigElAdditionalVariableElSecretValueEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionAuthConfigElAdditionalVariableElSecretValueEl {
    #[doc = "Secret version of Secret Value for Config variable."]
    pub secret_version: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnectionAuthConfigElAdditionalVariableElSecretValueEl {
    pub fn build(self) -> IntegrationConnectorsConnectionAuthConfigElAdditionalVariableElSecretValueEl {
        IntegrationConnectorsConnectionAuthConfigElAdditionalVariableElSecretValueEl {
            secret_version: self.secret_version,
        }
    }
}

pub struct IntegrationConnectorsConnectionAuthConfigElAdditionalVariableElSecretValueElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionAuthConfigElAdditionalVariableElSecretValueElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionAuthConfigElAdditionalVariableElSecretValueElRef {
        IntegrationConnectorsConnectionAuthConfigElAdditionalVariableElSecretValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionAuthConfigElAdditionalVariableElSecretValueElRef {
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
struct IntegrationConnectorsConnectionAuthConfigElAdditionalVariableElDynamic {
    encryption_key_value: std
    ::option
    ::Option<
        terrars::DynamicBlock<IntegrationConnectorsConnectionAuthConfigElAdditionalVariableElEncryptionKeyValueEl>,
    >,
    secret_value: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnectionAuthConfigElAdditionalVariableElSecretValueEl>>,
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnectionAuthConfigElAdditionalVariableEl {
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
    ::Option<std::vec::Vec<IntegrationConnectorsConnectionAuthConfigElAdditionalVariableElEncryptionKeyValueEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_value: std
    ::option
    ::Option<std::vec::Vec<IntegrationConnectorsConnectionAuthConfigElAdditionalVariableElSecretValueEl>>,
    dynamic: IntegrationConnectorsConnectionAuthConfigElAdditionalVariableElDynamic,
}

impl IntegrationConnectorsConnectionAuthConfigElAdditionalVariableEl {
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
                            IntegrationConnectorsConnectionAuthConfigElAdditionalVariableElEncryptionKeyValueEl,
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
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            IntegrationConnectorsConnectionAuthConfigElAdditionalVariableElSecretValueEl,
                        >,
                    >,
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

impl terrars::ToListMappable for IntegrationConnectorsConnectionAuthConfigElAdditionalVariableEl {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnectionAuthConfigElAdditionalVariableEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionAuthConfigElAdditionalVariableEl {
    #[doc = "Key for the configVariable"]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnectionAuthConfigElAdditionalVariableEl {
    pub fn build(self) -> IntegrationConnectorsConnectionAuthConfigElAdditionalVariableEl {
        IntegrationConnectorsConnectionAuthConfigElAdditionalVariableEl {
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

pub struct IntegrationConnectorsConnectionAuthConfigElAdditionalVariableElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionAuthConfigElAdditionalVariableElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionAuthConfigElAdditionalVariableElRef {
        IntegrationConnectorsConnectionAuthConfigElAdditionalVariableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionAuthConfigElAdditionalVariableElRef {
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
    ) -> terrars::ListRef<IntegrationConnectorsConnectionAuthConfigElAdditionalVariableElEncryptionKeyValueElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_key_value", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_value` after provisioning.\n"]
    pub fn secret_value(
        &self,
    ) -> terrars::ListRef<IntegrationConnectorsConnectionAuthConfigElAdditionalVariableElSecretValueElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_value", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnectionAuthConfigElOauth2AuthCodeFlowElClientSecretEl {
    secret_version: terrars::PrimField<std::string::String>,
}

impl IntegrationConnectorsConnectionAuthConfigElOauth2AuthCodeFlowElClientSecretEl { }

impl terrars::ToListMappable for IntegrationConnectorsConnectionAuthConfigElOauth2AuthCodeFlowElClientSecretEl {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnectionAuthConfigElOauth2AuthCodeFlowElClientSecretEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionAuthConfigElOauth2AuthCodeFlowElClientSecretEl {
    #[doc = "The resource name of the secret version in the format,\nformat as: projects/*/secrets/*/versions/*."]
    pub secret_version: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnectionAuthConfigElOauth2AuthCodeFlowElClientSecretEl {
    pub fn build(self) -> IntegrationConnectorsConnectionAuthConfigElOauth2AuthCodeFlowElClientSecretEl {
        IntegrationConnectorsConnectionAuthConfigElOauth2AuthCodeFlowElClientSecretEl {
            secret_version: self.secret_version,
        }
    }
}

pub struct IntegrationConnectorsConnectionAuthConfigElOauth2AuthCodeFlowElClientSecretElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionAuthConfigElOauth2AuthCodeFlowElClientSecretElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionAuthConfigElOauth2AuthCodeFlowElClientSecretElRef {
        IntegrationConnectorsConnectionAuthConfigElOauth2AuthCodeFlowElClientSecretElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionAuthConfigElOauth2AuthCodeFlowElClientSecretElRef {
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
struct IntegrationConnectorsConnectionAuthConfigElOauth2AuthCodeFlowElDynamic {
    client_secret: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnectionAuthConfigElOauth2AuthCodeFlowElClientSecretEl>>,
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnectionAuthConfigElOauth2AuthCodeFlowEl {
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
    ::Option<std::vec::Vec<IntegrationConnectorsConnectionAuthConfigElOauth2AuthCodeFlowElClientSecretEl>>,
    dynamic: IntegrationConnectorsConnectionAuthConfigElOauth2AuthCodeFlowElDynamic,
}

impl IntegrationConnectorsConnectionAuthConfigElOauth2AuthCodeFlowEl {
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
                        ::BlockAssignable<
                            IntegrationConnectorsConnectionAuthConfigElOauth2AuthCodeFlowElClientSecretEl,
                        >,
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

impl terrars::ToListMappable for IntegrationConnectorsConnectionAuthConfigElOauth2AuthCodeFlowEl {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnectionAuthConfigElOauth2AuthCodeFlowEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionAuthConfigElOauth2AuthCodeFlowEl {}

impl BuildIntegrationConnectorsConnectionAuthConfigElOauth2AuthCodeFlowEl {
    pub fn build(self) -> IntegrationConnectorsConnectionAuthConfigElOauth2AuthCodeFlowEl {
        IntegrationConnectorsConnectionAuthConfigElOauth2AuthCodeFlowEl {
            auth_uri: core::default::Default::default(),
            client_id: core::default::Default::default(),
            enable_pkce: core::default::Default::default(),
            scopes: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct IntegrationConnectorsConnectionAuthConfigElOauth2AuthCodeFlowElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionAuthConfigElOauth2AuthCodeFlowElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionAuthConfigElOauth2AuthCodeFlowElRef {
        IntegrationConnectorsConnectionAuthConfigElOauth2AuthCodeFlowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionAuthConfigElOauth2AuthCodeFlowElRef {
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
    ) -> terrars::ListRef<IntegrationConnectorsConnectionAuthConfigElOauth2AuthCodeFlowElClientSecretElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnectionAuthConfigElOauth2ClientCredentialsElClientSecretEl {
    secret_version: terrars::PrimField<std::string::String>,
}

impl IntegrationConnectorsConnectionAuthConfigElOauth2ClientCredentialsElClientSecretEl { }

impl terrars::ToListMappable for IntegrationConnectorsConnectionAuthConfigElOauth2ClientCredentialsElClientSecretEl {
    type O =
        terrars::BlockAssignable<IntegrationConnectorsConnectionAuthConfigElOauth2ClientCredentialsElClientSecretEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionAuthConfigElOauth2ClientCredentialsElClientSecretEl {
    #[doc = "The resource name of the secret version in the format,\nformat as: projects/*/secrets/*/versions/*."]
    pub secret_version: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnectionAuthConfigElOauth2ClientCredentialsElClientSecretEl {
    pub fn build(self) -> IntegrationConnectorsConnectionAuthConfigElOauth2ClientCredentialsElClientSecretEl {
        IntegrationConnectorsConnectionAuthConfigElOauth2ClientCredentialsElClientSecretEl {
            secret_version: self.secret_version,
        }
    }
}

pub struct IntegrationConnectorsConnectionAuthConfigElOauth2ClientCredentialsElClientSecretElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionAuthConfigElOauth2ClientCredentialsElClientSecretElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionAuthConfigElOauth2ClientCredentialsElClientSecretElRef {
        IntegrationConnectorsConnectionAuthConfigElOauth2ClientCredentialsElClientSecretElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionAuthConfigElOauth2ClientCredentialsElClientSecretElRef {
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
struct IntegrationConnectorsConnectionAuthConfigElOauth2ClientCredentialsElDynamic {
    client_secret: std
    ::option
    ::Option<
        terrars::DynamicBlock<IntegrationConnectorsConnectionAuthConfigElOauth2ClientCredentialsElClientSecretEl>,
    >,
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnectionAuthConfigElOauth2ClientCredentialsEl {
    client_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std
    ::option
    ::Option<std::vec::Vec<IntegrationConnectorsConnectionAuthConfigElOauth2ClientCredentialsElClientSecretEl>>,
    dynamic: IntegrationConnectorsConnectionAuthConfigElOauth2ClientCredentialsElDynamic,
}

impl IntegrationConnectorsConnectionAuthConfigElOauth2ClientCredentialsEl {
    #[doc = "Set the field `client_secret`.\n"]
    pub fn set_client_secret(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            IntegrationConnectorsConnectionAuthConfigElOauth2ClientCredentialsElClientSecretEl,
                        >,
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

impl terrars::ToListMappable for IntegrationConnectorsConnectionAuthConfigElOauth2ClientCredentialsEl {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnectionAuthConfigElOauth2ClientCredentialsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionAuthConfigElOauth2ClientCredentialsEl {
    #[doc = "Secret version of Password for Authentication."]
    pub client_id: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnectionAuthConfigElOauth2ClientCredentialsEl {
    pub fn build(self) -> IntegrationConnectorsConnectionAuthConfigElOauth2ClientCredentialsEl {
        IntegrationConnectorsConnectionAuthConfigElOauth2ClientCredentialsEl {
            client_id: self.client_id,
            client_secret: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct IntegrationConnectorsConnectionAuthConfigElOauth2ClientCredentialsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionAuthConfigElOauth2ClientCredentialsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionAuthConfigElOauth2ClientCredentialsElRef {
        IntegrationConnectorsConnectionAuthConfigElOauth2ClientCredentialsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionAuthConfigElOauth2ClientCredentialsElRef {
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
    ) -> terrars::ListRef<IntegrationConnectorsConnectionAuthConfigElOauth2ClientCredentialsElClientSecretElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElClientKeyEl {
    secret_version: terrars::PrimField<std::string::String>,
}

impl IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElClientKeyEl { }

impl terrars::ToListMappable for IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElClientKeyEl {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElClientKeyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElClientKeyEl {
    #[doc = "The resource name of the secret version in the format,\nformat as: projects/*/secrets/*/versions/*."]
    pub secret_version: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElClientKeyEl {
    pub fn build(self) -> IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElClientKeyEl {
        IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElClientKeyEl {
            secret_version: self.secret_version,
        }
    }
}

pub struct IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElClientKeyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElClientKeyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElClientKeyElRef {
        IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElClientKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElClientKeyElRef {
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
pub struct IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElJwtClaimsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audience: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    issuer: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElJwtClaimsEl {
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

impl terrars::ToListMappable for IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElJwtClaimsEl {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElJwtClaimsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElJwtClaimsEl {}

impl BuildIntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElJwtClaimsEl {
    pub fn build(self) -> IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElJwtClaimsEl {
        IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElJwtClaimsEl {
            audience: core::default::Default::default(),
            issuer: core::default::Default::default(),
            subject: core::default::Default::default(),
        }
    }
}

pub struct IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElJwtClaimsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElJwtClaimsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElJwtClaimsElRef {
        IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElJwtClaimsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElJwtClaimsElRef {
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
struct IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElDynamic {
    client_key: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElClientKeyEl>>,
    jwt_claims: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElJwtClaimsEl>>,
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_key: std
    ::option
    ::Option<std::vec::Vec<IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElClientKeyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jwt_claims: std
    ::option
    ::Option<std::vec::Vec<IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElJwtClaimsEl>>,
    dynamic: IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElDynamic,
}

impl IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerEl {
    #[doc = "Set the field `client_key`.\n"]
    pub fn set_client_key(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElClientKeyEl>,
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
                        terrars
                        ::BlockAssignable<IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElJwtClaimsEl>,
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

impl terrars::ToListMappable for IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerEl {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerEl {}

impl BuildIntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerEl {
    pub fn build(self) -> IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerEl {
        IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerEl {
            client_key: core::default::Default::default(),
            jwt_claims: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElRef {
        IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_key` after provisioning.\n"]
    pub fn client_key(
        &self,
    ) -> terrars::ListRef<IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElClientKeyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_key", self.base))
    }

    #[doc = "Get a reference to the value of field `jwt_claims` after provisioning.\n"]
    pub fn jwt_claims(
        &self,
    ) -> terrars::ListRef<IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElJwtClaimsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jwt_claims", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnectionAuthConfigElSshPublicKeyElSshClientCertEl {
    secret_version: terrars::PrimField<std::string::String>,
}

impl IntegrationConnectorsConnectionAuthConfigElSshPublicKeyElSshClientCertEl { }

impl terrars::ToListMappable for IntegrationConnectorsConnectionAuthConfigElSshPublicKeyElSshClientCertEl {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnectionAuthConfigElSshPublicKeyElSshClientCertEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionAuthConfigElSshPublicKeyElSshClientCertEl {
    #[doc = "The resource name of the secret version in the format,\nformat as: projects/*/secrets/*/versions/*."]
    pub secret_version: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnectionAuthConfigElSshPublicKeyElSshClientCertEl {
    pub fn build(self) -> IntegrationConnectorsConnectionAuthConfigElSshPublicKeyElSshClientCertEl {
        IntegrationConnectorsConnectionAuthConfigElSshPublicKeyElSshClientCertEl {
            secret_version: self.secret_version,
        }
    }
}

pub struct IntegrationConnectorsConnectionAuthConfigElSshPublicKeyElSshClientCertElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionAuthConfigElSshPublicKeyElSshClientCertElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionAuthConfigElSshPublicKeyElSshClientCertElRef {
        IntegrationConnectorsConnectionAuthConfigElSshPublicKeyElSshClientCertElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionAuthConfigElSshPublicKeyElSshClientCertElRef {
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
pub struct IntegrationConnectorsConnectionAuthConfigElSshPublicKeyElSshClientCertPassEl {
    secret_version: terrars::PrimField<std::string::String>,
}

impl IntegrationConnectorsConnectionAuthConfigElSshPublicKeyElSshClientCertPassEl { }

impl terrars::ToListMappable for IntegrationConnectorsConnectionAuthConfigElSshPublicKeyElSshClientCertPassEl {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnectionAuthConfigElSshPublicKeyElSshClientCertPassEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionAuthConfigElSshPublicKeyElSshClientCertPassEl {
    #[doc = "The resource name of the secret version in the format,\nformat as: projects/*/secrets/*/versions/*."]
    pub secret_version: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnectionAuthConfigElSshPublicKeyElSshClientCertPassEl {
    pub fn build(self) -> IntegrationConnectorsConnectionAuthConfigElSshPublicKeyElSshClientCertPassEl {
        IntegrationConnectorsConnectionAuthConfigElSshPublicKeyElSshClientCertPassEl {
            secret_version: self.secret_version,
        }
    }
}

pub struct IntegrationConnectorsConnectionAuthConfigElSshPublicKeyElSshClientCertPassElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionAuthConfigElSshPublicKeyElSshClientCertPassElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionAuthConfigElSshPublicKeyElSshClientCertPassElRef {
        IntegrationConnectorsConnectionAuthConfigElSshPublicKeyElSshClientCertPassElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionAuthConfigElSshPublicKeyElSshClientCertPassElRef {
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
struct IntegrationConnectorsConnectionAuthConfigElSshPublicKeyElDynamic {
    ssh_client_cert: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnectionAuthConfigElSshPublicKeyElSshClientCertEl>>,
    ssh_client_cert_pass: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnectionAuthConfigElSshPublicKeyElSshClientCertPassEl>>,
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnectionAuthConfigElSshPublicKeyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cert_type: std::option::Option<terrars::PrimField<std::string::String>>,
    username: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssh_client_cert: std
    ::option
    ::Option<std::vec::Vec<IntegrationConnectorsConnectionAuthConfigElSshPublicKeyElSshClientCertEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssh_client_cert_pass: std
    ::option
    ::Option<std::vec::Vec<IntegrationConnectorsConnectionAuthConfigElSshPublicKeyElSshClientCertPassEl>>,
    dynamic: IntegrationConnectorsConnectionAuthConfigElSshPublicKeyElDynamic,
}

impl IntegrationConnectorsConnectionAuthConfigElSshPublicKeyEl {
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
                        ::BlockAssignable<IntegrationConnectorsConnectionAuthConfigElSshPublicKeyElSshClientCertEl>,
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
                        ::BlockAssignable<
                            IntegrationConnectorsConnectionAuthConfigElSshPublicKeyElSshClientCertPassEl,
                        >,
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

impl terrars::ToListMappable for IntegrationConnectorsConnectionAuthConfigElSshPublicKeyEl {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnectionAuthConfigElSshPublicKeyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionAuthConfigElSshPublicKeyEl {
    #[doc = "The user account used to authenticate."]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnectionAuthConfigElSshPublicKeyEl {
    pub fn build(self) -> IntegrationConnectorsConnectionAuthConfigElSshPublicKeyEl {
        IntegrationConnectorsConnectionAuthConfigElSshPublicKeyEl {
            cert_type: core::default::Default::default(),
            username: self.username,
            ssh_client_cert: core::default::Default::default(),
            ssh_client_cert_pass: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct IntegrationConnectorsConnectionAuthConfigElSshPublicKeyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionAuthConfigElSshPublicKeyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionAuthConfigElSshPublicKeyElRef {
        IntegrationConnectorsConnectionAuthConfigElSshPublicKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionAuthConfigElSshPublicKeyElRef {
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
    ) -> terrars::ListRef<IntegrationConnectorsConnectionAuthConfigElSshPublicKeyElSshClientCertElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssh_client_cert", self.base))
    }

    #[doc = "Get a reference to the value of field `ssh_client_cert_pass` after provisioning.\n"]
    pub fn ssh_client_cert_pass(
        &self,
    ) -> terrars::ListRef<IntegrationConnectorsConnectionAuthConfigElSshPublicKeyElSshClientCertPassElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssh_client_cert_pass", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnectionAuthConfigElUserPasswordElPasswordEl {
    secret_version: terrars::PrimField<std::string::String>,
}

impl IntegrationConnectorsConnectionAuthConfigElUserPasswordElPasswordEl { }

impl terrars::ToListMappable for IntegrationConnectorsConnectionAuthConfigElUserPasswordElPasswordEl {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnectionAuthConfigElUserPasswordElPasswordEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionAuthConfigElUserPasswordElPasswordEl {
    #[doc = "The resource name of the secret version in the format,\nformat as: projects/*/secrets/*/versions/*."]
    pub secret_version: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnectionAuthConfigElUserPasswordElPasswordEl {
    pub fn build(self) -> IntegrationConnectorsConnectionAuthConfigElUserPasswordElPasswordEl {
        IntegrationConnectorsConnectionAuthConfigElUserPasswordElPasswordEl { secret_version: self.secret_version }
    }
}

pub struct IntegrationConnectorsConnectionAuthConfigElUserPasswordElPasswordElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionAuthConfigElUserPasswordElPasswordElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionAuthConfigElUserPasswordElPasswordElRef {
        IntegrationConnectorsConnectionAuthConfigElUserPasswordElPasswordElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionAuthConfigElUserPasswordElPasswordElRef {
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
struct IntegrationConnectorsConnectionAuthConfigElUserPasswordElDynamic {
    password: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnectionAuthConfigElUserPasswordElPasswordEl>>,
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnectionAuthConfigElUserPasswordEl {
    username: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<std::vec::Vec<IntegrationConnectorsConnectionAuthConfigElUserPasswordElPasswordEl>>,
    dynamic: IntegrationConnectorsConnectionAuthConfigElUserPasswordElDynamic,
}

impl IntegrationConnectorsConnectionAuthConfigElUserPasswordEl {
    #[doc = "Set the field `password`.\n"]
    pub fn set_password(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<IntegrationConnectorsConnectionAuthConfigElUserPasswordElPasswordEl>,
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

impl terrars::ToListMappable for IntegrationConnectorsConnectionAuthConfigElUserPasswordEl {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnectionAuthConfigElUserPasswordEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionAuthConfigElUserPasswordEl {
    #[doc = "Username for Authentication."]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnectionAuthConfigElUserPasswordEl {
    pub fn build(self) -> IntegrationConnectorsConnectionAuthConfigElUserPasswordEl {
        IntegrationConnectorsConnectionAuthConfigElUserPasswordEl {
            username: self.username,
            password: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct IntegrationConnectorsConnectionAuthConfigElUserPasswordElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionAuthConfigElUserPasswordElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionAuthConfigElUserPasswordElRef {
        IntegrationConnectorsConnectionAuthConfigElUserPasswordElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionAuthConfigElUserPasswordElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\nUsername for Authentication."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> terrars::ListRef<IntegrationConnectorsConnectionAuthConfigElUserPasswordElPasswordElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.password", self.base))
    }
}

#[derive(Serialize, Default)]
struct IntegrationConnectorsConnectionAuthConfigElDynamic {
    additional_variable: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnectionAuthConfigElAdditionalVariableEl>>,
    oauth2_auth_code_flow: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnectionAuthConfigElOauth2AuthCodeFlowEl>>,
    oauth2_client_credentials: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnectionAuthConfigElOauth2ClientCredentialsEl>>,
    oauth2_jwt_bearer: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerEl>>,
    ssh_public_key: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnectionAuthConfigElSshPublicKeyEl>>,
    user_password: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnectionAuthConfigElUserPasswordEl>>,
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnectionAuthConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_key: std::option::Option<terrars::PrimField<std::string::String>>,
    auth_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_variable: std
    ::option
    ::Option<std::vec::Vec<IntegrationConnectorsConnectionAuthConfigElAdditionalVariableEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth2_auth_code_flow: std
    ::option
    ::Option<std::vec::Vec<IntegrationConnectorsConnectionAuthConfigElOauth2AuthCodeFlowEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth2_client_credentials: std
    ::option
    ::Option<std::vec::Vec<IntegrationConnectorsConnectionAuthConfigElOauth2ClientCredentialsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth2_jwt_bearer: std::option::Option<std::vec::Vec<IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssh_public_key: std::option::Option<std::vec::Vec<IntegrationConnectorsConnectionAuthConfigElSshPublicKeyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_password: std::option::Option<std::vec::Vec<IntegrationConnectorsConnectionAuthConfigElUserPasswordEl>>,
    dynamic: IntegrationConnectorsConnectionAuthConfigElDynamic,
}

impl IntegrationConnectorsConnectionAuthConfigEl {
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
                    ::Into<terrars::BlockAssignable<IntegrationConnectorsConnectionAuthConfigElAdditionalVariableEl>>,
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
                    ::Into<terrars::BlockAssignable<IntegrationConnectorsConnectionAuthConfigElOauth2AuthCodeFlowEl>>,
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
                        terrars::BlockAssignable<IntegrationConnectorsConnectionAuthConfigElOauth2ClientCredentialsEl>,
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
                    ::Into<terrars::BlockAssignable<IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerEl>>,
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
                    ::Into<terrars::BlockAssignable<IntegrationConnectorsConnectionAuthConfigElSshPublicKeyEl>>,
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
                    ::Into<terrars::BlockAssignable<IntegrationConnectorsConnectionAuthConfigElUserPasswordEl>>,
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

impl terrars::ToListMappable for IntegrationConnectorsConnectionAuthConfigEl {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnectionAuthConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionAuthConfigEl {
    #[doc =
        "authType of the Connection Possible values: [\"USER_PASSWORD\", \"OAUTH2_JWT_BEARER\", \"OAUTH2_CLIENT_CREDENTIALS\", \"SSH_PUBLIC_KEY\", \"OAUTH2_AUTH_CODE_FLOW\"]"]
    pub auth_type: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnectionAuthConfigEl {
    pub fn build(self) -> IntegrationConnectorsConnectionAuthConfigEl {
        IntegrationConnectorsConnectionAuthConfigEl {
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

pub struct IntegrationConnectorsConnectionAuthConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionAuthConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionAuthConfigElRef {
        IntegrationConnectorsConnectionAuthConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionAuthConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `auth_key` after provisioning.\nThe type of authentication configured."]
    pub fn auth_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `auth_type` after provisioning.\nauthType of the Connection Possible values: [\"USER_PASSWORD\", \"OAUTH2_JWT_BEARER\", \"OAUTH2_CLIENT_CREDENTIALS\", \"SSH_PUBLIC_KEY\", \"OAUTH2_AUTH_CODE_FLOW\"]"]
    pub fn auth_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_type", self.base))
    }

    #[doc = "Get a reference to the value of field `additional_variable` after provisioning.\n"]
    pub fn additional_variable(
        &self,
    ) -> terrars::ListRef<IntegrationConnectorsConnectionAuthConfigElAdditionalVariableElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_variable", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth2_auth_code_flow` after provisioning.\n"]
    pub fn oauth2_auth_code_flow(
        &self,
    ) -> terrars::ListRef<IntegrationConnectorsConnectionAuthConfigElOauth2AuthCodeFlowElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth2_auth_code_flow", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth2_client_credentials` after provisioning.\n"]
    pub fn oauth2_client_credentials(
        &self,
    ) -> terrars::ListRef<IntegrationConnectorsConnectionAuthConfigElOauth2ClientCredentialsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth2_client_credentials", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth2_jwt_bearer` after provisioning.\n"]
    pub fn oauth2_jwt_bearer(&self) -> terrars::ListRef<IntegrationConnectorsConnectionAuthConfigElOauth2JwtBearerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth2_jwt_bearer", self.base))
    }

    #[doc = "Get a reference to the value of field `ssh_public_key` after provisioning.\n"]
    pub fn ssh_public_key(&self) -> terrars::ListRef<IntegrationConnectorsConnectionAuthConfigElSshPublicKeyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssh_public_key", self.base))
    }

    #[doc = "Get a reference to the value of field `user_password` after provisioning.\n"]
    pub fn user_password(&self) -> terrars::ListRef<IntegrationConnectorsConnectionAuthConfigElUserPasswordElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_password", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnectionConfigVariableElEncryptionKeyValueEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl IntegrationConnectorsConnectionConfigVariableElEncryptionKeyValueEl {
    #[doc =
        "Set the field `kms_key_name`.\nThe [KMS key name] with which the content of the Operation is encrypted. The expected\nformat: projects/*/locations/*/keyRings/*/cryptoKeys/*.\nWill be empty string if google managed."]
    pub fn set_kms_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IntegrationConnectorsConnectionConfigVariableElEncryptionKeyValueEl {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnectionConfigVariableElEncryptionKeyValueEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionConfigVariableElEncryptionKeyValueEl {
    #[doc = "Type of Encription Key Possible values: [\"GOOGLE_MANAGED\", \"CUSTOMER_MANAGED\"]"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnectionConfigVariableElEncryptionKeyValueEl {
    pub fn build(self) -> IntegrationConnectorsConnectionConfigVariableElEncryptionKeyValueEl {
        IntegrationConnectorsConnectionConfigVariableElEncryptionKeyValueEl {
            kms_key_name: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct IntegrationConnectorsConnectionConfigVariableElEncryptionKeyValueElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionConfigVariableElEncryptionKeyValueElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionConfigVariableElEncryptionKeyValueElRef {
        IntegrationConnectorsConnectionConfigVariableElEncryptionKeyValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionConfigVariableElEncryptionKeyValueElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nThe [KMS key name] with which the content of the Operation is encrypted. The expected\nformat: projects/*/locations/*/keyRings/*/cryptoKeys/*.\nWill be empty string if google managed."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nType of Encription Key Possible values: [\"GOOGLE_MANAGED\", \"CUSTOMER_MANAGED\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnectionConfigVariableElSecretValueEl {
    secret_version: terrars::PrimField<std::string::String>,
}

impl IntegrationConnectorsConnectionConfigVariableElSecretValueEl { }

impl terrars::ToListMappable for IntegrationConnectorsConnectionConfigVariableElSecretValueEl {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnectionConfigVariableElSecretValueEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionConfigVariableElSecretValueEl {
    #[doc = "Secret version of Secret Value for Config variable."]
    pub secret_version: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnectionConfigVariableElSecretValueEl {
    pub fn build(self) -> IntegrationConnectorsConnectionConfigVariableElSecretValueEl {
        IntegrationConnectorsConnectionConfigVariableElSecretValueEl { secret_version: self.secret_version }
    }
}

pub struct IntegrationConnectorsConnectionConfigVariableElSecretValueElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionConfigVariableElSecretValueElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionConfigVariableElSecretValueElRef {
        IntegrationConnectorsConnectionConfigVariableElSecretValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionConfigVariableElSecretValueElRef {
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
struct IntegrationConnectorsConnectionConfigVariableElDynamic {
    encryption_key_value: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnectionConfigVariableElEncryptionKeyValueEl>>,
    secret_value: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnectionConfigVariableElSecretValueEl>>,
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnectionConfigVariableEl {
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
    ::Option<std::vec::Vec<IntegrationConnectorsConnectionConfigVariableElEncryptionKeyValueEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_value: std::option::Option<std::vec::Vec<IntegrationConnectorsConnectionConfigVariableElSecretValueEl>>,
    dynamic: IntegrationConnectorsConnectionConfigVariableElDynamic,
}

impl IntegrationConnectorsConnectionConfigVariableEl {
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
                        terrars::BlockAssignable<IntegrationConnectorsConnectionConfigVariableElEncryptionKeyValueEl>,
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
                    ::Into<terrars::BlockAssignable<IntegrationConnectorsConnectionConfigVariableElSecretValueEl>>,
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

impl terrars::ToListMappable for IntegrationConnectorsConnectionConfigVariableEl {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnectionConfigVariableEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionConfigVariableEl {
    #[doc = "Key for the configVariable"]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnectionConfigVariableEl {
    pub fn build(self) -> IntegrationConnectorsConnectionConfigVariableEl {
        IntegrationConnectorsConnectionConfigVariableEl {
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

pub struct IntegrationConnectorsConnectionConfigVariableElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionConfigVariableElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionConfigVariableElRef {
        IntegrationConnectorsConnectionConfigVariableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionConfigVariableElRef {
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
    ) -> terrars::ListRef<IntegrationConnectorsConnectionConfigVariableElEncryptionKeyValueElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_key_value", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_value` after provisioning.\n"]
    pub fn secret_value(&self) -> terrars::ListRef<IntegrationConnectorsConnectionConfigVariableElSecretValueElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_value", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnectionDestinationConfigElDestinationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_attachment: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IntegrationConnectorsConnectionDestinationConfigElDestinationEl {
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

impl terrars::ToListMappable for IntegrationConnectorsConnectionDestinationConfigElDestinationEl {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnectionDestinationConfigElDestinationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionDestinationConfigElDestinationEl {}

impl BuildIntegrationConnectorsConnectionDestinationConfigElDestinationEl {
    pub fn build(self) -> IntegrationConnectorsConnectionDestinationConfigElDestinationEl {
        IntegrationConnectorsConnectionDestinationConfigElDestinationEl {
            host: core::default::Default::default(),
            port: core::default::Default::default(),
            service_attachment: core::default::Default::default(),
        }
    }
}

pub struct IntegrationConnectorsConnectionDestinationConfigElDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionDestinationConfigElDestinationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionDestinationConfigElDestinationElRef {
        IntegrationConnectorsConnectionDestinationConfigElDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionDestinationConfigElDestinationElRef {
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
struct IntegrationConnectorsConnectionDestinationConfigElDynamic {
    destination: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnectionDestinationConfigElDestinationEl>>,
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnectionDestinationConfigEl {
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std::option::Option<std::vec::Vec<IntegrationConnectorsConnectionDestinationConfigElDestinationEl>>,
    dynamic: IntegrationConnectorsConnectionDestinationConfigElDynamic,
}

impl IntegrationConnectorsConnectionDestinationConfigEl {
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<IntegrationConnectorsConnectionDestinationConfigElDestinationEl>>,
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

impl terrars::ToListMappable for IntegrationConnectorsConnectionDestinationConfigEl {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnectionDestinationConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionDestinationConfigEl {
    #[doc = "The key is the destination identifier that is supported by the Connector."]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnectionDestinationConfigEl {
    pub fn build(self) -> IntegrationConnectorsConnectionDestinationConfigEl {
        IntegrationConnectorsConnectionDestinationConfigEl {
            key: self.key,
            destination: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct IntegrationConnectorsConnectionDestinationConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionDestinationConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionDestinationConfigElRef {
        IntegrationConnectorsConnectionDestinationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionDestinationConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `key` after provisioning.\nThe key is the destination identifier that is supported by the Connector."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> terrars::ListRef<IntegrationConnectorsConnectionDestinationConfigElDestinationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnectionEventingConfigElAdditionalVariableElEncryptionKeyValueEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IntegrationConnectorsConnectionEventingConfigElAdditionalVariableElEncryptionKeyValueEl {
    #[doc =
        "Set the field `kms_key_name`.\nThe [KMS key name] with which the content of the Operation is encrypted. The expected\nformat: projects/*/locations/*/keyRings/*/cryptoKeys/*.\nWill be empty string if google managed."]
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

impl terrars
::ToListMappable for IntegrationConnectorsConnectionEventingConfigElAdditionalVariableElEncryptionKeyValueEl {
    type O =
        terrars
        ::BlockAssignable<IntegrationConnectorsConnectionEventingConfigElAdditionalVariableElEncryptionKeyValueEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionEventingConfigElAdditionalVariableElEncryptionKeyValueEl {}

impl BuildIntegrationConnectorsConnectionEventingConfigElAdditionalVariableElEncryptionKeyValueEl {
    pub fn build(self) -> IntegrationConnectorsConnectionEventingConfigElAdditionalVariableElEncryptionKeyValueEl {
        IntegrationConnectorsConnectionEventingConfigElAdditionalVariableElEncryptionKeyValueEl {
            kms_key_name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct IntegrationConnectorsConnectionEventingConfigElAdditionalVariableElEncryptionKeyValueElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionEventingConfigElAdditionalVariableElEncryptionKeyValueElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionEventingConfigElAdditionalVariableElEncryptionKeyValueElRef {
        IntegrationConnectorsConnectionEventingConfigElAdditionalVariableElEncryptionKeyValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionEventingConfigElAdditionalVariableElEncryptionKeyValueElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nThe [KMS key name] with which the content of the Operation is encrypted. The expected\nformat: projects/*/locations/*/keyRings/*/cryptoKeys/*.\nWill be empty string if google managed."]
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
pub struct IntegrationConnectorsConnectionEventingConfigElAdditionalVariableElSecretValueEl {
    secret_version: terrars::PrimField<std::string::String>,
}

impl IntegrationConnectorsConnectionEventingConfigElAdditionalVariableElSecretValueEl { }

impl terrars::ToListMappable for IntegrationConnectorsConnectionEventingConfigElAdditionalVariableElSecretValueEl {
    type O =
        terrars::BlockAssignable<IntegrationConnectorsConnectionEventingConfigElAdditionalVariableElSecretValueEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionEventingConfigElAdditionalVariableElSecretValueEl {
    #[doc = "Secret version of Secret Value for Config variable."]
    pub secret_version: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnectionEventingConfigElAdditionalVariableElSecretValueEl {
    pub fn build(self) -> IntegrationConnectorsConnectionEventingConfigElAdditionalVariableElSecretValueEl {
        IntegrationConnectorsConnectionEventingConfigElAdditionalVariableElSecretValueEl {
            secret_version: self.secret_version,
        }
    }
}

pub struct IntegrationConnectorsConnectionEventingConfigElAdditionalVariableElSecretValueElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionEventingConfigElAdditionalVariableElSecretValueElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionEventingConfigElAdditionalVariableElSecretValueElRef {
        IntegrationConnectorsConnectionEventingConfigElAdditionalVariableElSecretValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionEventingConfigElAdditionalVariableElSecretValueElRef {
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
struct IntegrationConnectorsConnectionEventingConfigElAdditionalVariableElDynamic {
    encryption_key_value: std
    ::option
    ::Option<
        terrars::DynamicBlock<IntegrationConnectorsConnectionEventingConfigElAdditionalVariableElEncryptionKeyValueEl>,
    >,
    secret_value: std
    ::option
    ::Option<
        terrars::DynamicBlock<IntegrationConnectorsConnectionEventingConfigElAdditionalVariableElSecretValueEl>,
    >,
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnectionEventingConfigElAdditionalVariableEl {
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
    ::Option<std::vec::Vec<IntegrationConnectorsConnectionEventingConfigElAdditionalVariableElEncryptionKeyValueEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_value: std
    ::option
    ::Option<std::vec::Vec<IntegrationConnectorsConnectionEventingConfigElAdditionalVariableElSecretValueEl>>,
    dynamic: IntegrationConnectorsConnectionEventingConfigElAdditionalVariableElDynamic,
}

impl IntegrationConnectorsConnectionEventingConfigElAdditionalVariableEl {
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
                            IntegrationConnectorsConnectionEventingConfigElAdditionalVariableElEncryptionKeyValueEl,
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
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            IntegrationConnectorsConnectionEventingConfigElAdditionalVariableElSecretValueEl,
                        >,
                    >,
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

impl terrars::ToListMappable for IntegrationConnectorsConnectionEventingConfigElAdditionalVariableEl {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnectionEventingConfigElAdditionalVariableEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionEventingConfigElAdditionalVariableEl {
    #[doc = "Key for the configVariable"]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnectionEventingConfigElAdditionalVariableEl {
    pub fn build(self) -> IntegrationConnectorsConnectionEventingConfigElAdditionalVariableEl {
        IntegrationConnectorsConnectionEventingConfigElAdditionalVariableEl {
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

pub struct IntegrationConnectorsConnectionEventingConfigElAdditionalVariableElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionEventingConfigElAdditionalVariableElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionEventingConfigElAdditionalVariableElRef {
        IntegrationConnectorsConnectionEventingConfigElAdditionalVariableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionEventingConfigElAdditionalVariableElRef {
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
    ) -> terrars::ListRef<IntegrationConnectorsConnectionEventingConfigElAdditionalVariableElEncryptionKeyValueElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_key_value", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_value` after provisioning.\n"]
    pub fn secret_value(
        &self,
    ) -> terrars::ListRef<IntegrationConnectorsConnectionEventingConfigElAdditionalVariableElSecretValueElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_value", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElEncryptionKeyValueEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElEncryptionKeyValueEl {
    #[doc =
        "Set the field `kms_key_name`.\nThe [KMS key name] with which the content of the Operation is encrypted. The expected\nformat: projects/*/locations/*/keyRings/*/cryptoKeys/*.\nWill be empty string if google managed."]
    pub fn set_kms_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nType of Encription Key Possible values: [\"GOOGLE_MANAGED\", \"CUSTOMER_MANAGED\"]"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElEncryptionKeyValueEl {
    type O =
        terrars
        ::BlockAssignable<
            IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElEncryptionKeyValueEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElEncryptionKeyValueEl {}

impl BuildIntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElEncryptionKeyValueEl {
    pub fn build(
        self,
    ) -> IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElEncryptionKeyValueEl {
        IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElEncryptionKeyValueEl {
            kms_key_name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElEncryptionKeyValueElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElEncryptionKeyValueElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElEncryptionKeyValueElRef {
        IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElEncryptionKeyValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElEncryptionKeyValueElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nThe [KMS key name] with which the content of the Operation is encrypted. The expected\nformat: projects/*/locations/*/keyRings/*/cryptoKeys/*.\nWill be empty string if google managed."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nType of Encription Key Possible values: [\"GOOGLE_MANAGED\", \"CUSTOMER_MANAGED\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElSecretValueEl {
    secret_version: terrars::PrimField<std::string::String>,
}

impl IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElSecretValueEl { }

impl terrars
::ToListMappable for IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElSecretValueEl {
    type O =
        terrars
        ::BlockAssignable<
            IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElSecretValueEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElSecretValueEl {
    #[doc = "Secret version of Secret Value for Config variable."]
    pub secret_version: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElSecretValueEl {
    pub fn build(
        self,
    ) -> IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElSecretValueEl {
        IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElSecretValueEl {
            secret_version: self.secret_version,
        }
    }
}

pub struct IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElSecretValueElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElSecretValueElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElSecretValueElRef {
        IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElSecretValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElSecretValueElRef {
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
struct IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElDynamic {
    encryption_key_value: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElEncryptionKeyValueEl,
        >,
    >,
    secret_value: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElSecretValueEl>,
    >,
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableEl {
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
    ::Option<
        std
        ::vec
        ::Vec<IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElEncryptionKeyValueEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_value: std
    ::option
    ::Option<
        std::vec::Vec<IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElSecretValueEl>,
    >,
    dynamic: IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElDynamic,
}

impl IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableEl {
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
                            IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElEncryptionKeyValueEl,
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
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElSecretValueEl,
                        >,
                    >,
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

impl terrars::ToListMappable for IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableEl {
    type O =
        terrars::BlockAssignable<IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableEl {
    #[doc = "Key for the configVariable"]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableEl {
    pub fn build(self) -> IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableEl {
        IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableEl {
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

pub struct IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElRef {
        IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElRef {
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
    ) -> terrars
    ::ListRef<IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElEncryptionKeyValueElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_key_value", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_value` after provisioning.\n"]
    pub fn secret_value(
        &self,
    ) -> terrars
    ::ListRef<IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElSecretValueElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_value", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnectionEventingConfigElAuthConfigElUserPasswordElPasswordEl {
    secret_version: terrars::PrimField<std::string::String>,
}

impl IntegrationConnectorsConnectionEventingConfigElAuthConfigElUserPasswordElPasswordEl { }

impl terrars::ToListMappable for IntegrationConnectorsConnectionEventingConfigElAuthConfigElUserPasswordElPasswordEl {
    type O =
        terrars
        ::BlockAssignable<IntegrationConnectorsConnectionEventingConfigElAuthConfigElUserPasswordElPasswordEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionEventingConfigElAuthConfigElUserPasswordElPasswordEl {
    #[doc = "The resource name of the secret version in the format,\nformat as: projects/*/secrets/*/versions/*."]
    pub secret_version: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnectionEventingConfigElAuthConfigElUserPasswordElPasswordEl {
    pub fn build(self) -> IntegrationConnectorsConnectionEventingConfigElAuthConfigElUserPasswordElPasswordEl {
        IntegrationConnectorsConnectionEventingConfigElAuthConfigElUserPasswordElPasswordEl {
            secret_version: self.secret_version,
        }
    }
}

pub struct IntegrationConnectorsConnectionEventingConfigElAuthConfigElUserPasswordElPasswordElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionEventingConfigElAuthConfigElUserPasswordElPasswordElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionEventingConfigElAuthConfigElUserPasswordElPasswordElRef {
        IntegrationConnectorsConnectionEventingConfigElAuthConfigElUserPasswordElPasswordElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionEventingConfigElAuthConfigElUserPasswordElPasswordElRef {
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
struct IntegrationConnectorsConnectionEventingConfigElAuthConfigElUserPasswordElDynamic {
    password: std
    ::option
    ::Option<
        terrars::DynamicBlock<IntegrationConnectorsConnectionEventingConfigElAuthConfigElUserPasswordElPasswordEl>,
    >,
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnectionEventingConfigElAuthConfigElUserPasswordEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std
    ::option
    ::Option<std::vec::Vec<IntegrationConnectorsConnectionEventingConfigElAuthConfigElUserPasswordElPasswordEl>>,
    dynamic: IntegrationConnectorsConnectionEventingConfigElAuthConfigElUserPasswordElDynamic,
}

impl IntegrationConnectorsConnectionEventingConfigElAuthConfigElUserPasswordEl {
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
                        terrars
                        ::BlockAssignable<
                            IntegrationConnectorsConnectionEventingConfigElAuthConfigElUserPasswordElPasswordEl,
                        >,
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

impl terrars::ToListMappable for IntegrationConnectorsConnectionEventingConfigElAuthConfigElUserPasswordEl {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnectionEventingConfigElAuthConfigElUserPasswordEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionEventingConfigElAuthConfigElUserPasswordEl {}

impl BuildIntegrationConnectorsConnectionEventingConfigElAuthConfigElUserPasswordEl {
    pub fn build(self) -> IntegrationConnectorsConnectionEventingConfigElAuthConfigElUserPasswordEl {
        IntegrationConnectorsConnectionEventingConfigElAuthConfigElUserPasswordEl {
            username: core::default::Default::default(),
            password: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct IntegrationConnectorsConnectionEventingConfigElAuthConfigElUserPasswordElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionEventingConfigElAuthConfigElUserPasswordElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionEventingConfigElAuthConfigElUserPasswordElRef {
        IntegrationConnectorsConnectionEventingConfigElAuthConfigElUserPasswordElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionEventingConfigElAuthConfigElUserPasswordElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\nUsername for Authentication."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(
        &self,
    ) -> terrars::ListRef<IntegrationConnectorsConnectionEventingConfigElAuthConfigElUserPasswordElPasswordElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.password", self.base))
    }
}

#[derive(Serialize, Default)]
struct IntegrationConnectorsConnectionEventingConfigElAuthConfigElDynamic {
    additional_variable: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableEl>>,
    user_password: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnectionEventingConfigElAuthConfigElUserPasswordEl>>,
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnectionEventingConfigElAuthConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_key: std::option::Option<terrars::PrimField<std::string::String>>,
    auth_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_variable: std
    ::option
    ::Option<std::vec::Vec<IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_password: std
    ::option
    ::Option<std::vec::Vec<IntegrationConnectorsConnectionEventingConfigElAuthConfigElUserPasswordEl>>,
    dynamic: IntegrationConnectorsConnectionEventingConfigElAuthConfigElDynamic,
}

impl IntegrationConnectorsConnectionEventingConfigElAuthConfigEl {
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
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableEl,
                        >,
                    >,
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
                        ::BlockAssignable<IntegrationConnectorsConnectionEventingConfigElAuthConfigElUserPasswordEl>,
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

impl terrars::ToListMappable for IntegrationConnectorsConnectionEventingConfigElAuthConfigEl {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnectionEventingConfigElAuthConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionEventingConfigElAuthConfigEl {
    #[doc = "authType of the Connection Possible values: [\"USER_PASSWORD\"]"]
    pub auth_type: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnectionEventingConfigElAuthConfigEl {
    pub fn build(self) -> IntegrationConnectorsConnectionEventingConfigElAuthConfigEl {
        IntegrationConnectorsConnectionEventingConfigElAuthConfigEl {
            auth_key: core::default::Default::default(),
            auth_type: self.auth_type,
            additional_variable: core::default::Default::default(),
            user_password: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct IntegrationConnectorsConnectionEventingConfigElAuthConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionEventingConfigElAuthConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionEventingConfigElAuthConfigElRef {
        IntegrationConnectorsConnectionEventingConfigElAuthConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionEventingConfigElAuthConfigElRef {
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
    ) -> terrars::ListRef<IntegrationConnectorsConnectionEventingConfigElAuthConfigElAdditionalVariableElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_variable", self.base))
    }

    #[doc = "Get a reference to the value of field `user_password` after provisioning.\n"]
    pub fn user_password(
        &self,
    ) -> terrars::ListRef<IntegrationConnectorsConnectionEventingConfigElAuthConfigElUserPasswordElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_password", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnectionEventingConfigElRegistrationDestinationConfigElDestinationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_attachment: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IntegrationConnectorsConnectionEventingConfigElRegistrationDestinationConfigElDestinationEl {
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
::ToListMappable for IntegrationConnectorsConnectionEventingConfigElRegistrationDestinationConfigElDestinationEl {
    type O =
        terrars
        ::BlockAssignable<
            IntegrationConnectorsConnectionEventingConfigElRegistrationDestinationConfigElDestinationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionEventingConfigElRegistrationDestinationConfigElDestinationEl {}

impl BuildIntegrationConnectorsConnectionEventingConfigElRegistrationDestinationConfigElDestinationEl {
    pub fn build(
        self,
    ) -> IntegrationConnectorsConnectionEventingConfigElRegistrationDestinationConfigElDestinationEl {
        IntegrationConnectorsConnectionEventingConfigElRegistrationDestinationConfigElDestinationEl {
            host: core::default::Default::default(),
            port: core::default::Default::default(),
            service_attachment: core::default::Default::default(),
        }
    }
}

pub struct IntegrationConnectorsConnectionEventingConfigElRegistrationDestinationConfigElDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionEventingConfigElRegistrationDestinationConfigElDestinationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionEventingConfigElRegistrationDestinationConfigElDestinationElRef {
        IntegrationConnectorsConnectionEventingConfigElRegistrationDestinationConfigElDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionEventingConfigElRegistrationDestinationConfigElDestinationElRef {
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
struct IntegrationConnectorsConnectionEventingConfigElRegistrationDestinationConfigElDynamic {
    destination: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<IntegrationConnectorsConnectionEventingConfigElRegistrationDestinationConfigElDestinationEl>,
    >,
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnectionEventingConfigElRegistrationDestinationConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std
    ::option
    ::Option<
        std::vec::Vec<IntegrationConnectorsConnectionEventingConfigElRegistrationDestinationConfigElDestinationEl>,
    >,
    dynamic: IntegrationConnectorsConnectionEventingConfigElRegistrationDestinationConfigElDynamic,
}

impl IntegrationConnectorsConnectionEventingConfigElRegistrationDestinationConfigEl {
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
                            IntegrationConnectorsConnectionEventingConfigElRegistrationDestinationConfigElDestinationEl,
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

impl terrars::ToListMappable for IntegrationConnectorsConnectionEventingConfigElRegistrationDestinationConfigEl {
    type O =
        terrars::BlockAssignable<IntegrationConnectorsConnectionEventingConfigElRegistrationDestinationConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionEventingConfigElRegistrationDestinationConfigEl {}

impl BuildIntegrationConnectorsConnectionEventingConfigElRegistrationDestinationConfigEl {
    pub fn build(self) -> IntegrationConnectorsConnectionEventingConfigElRegistrationDestinationConfigEl {
        IntegrationConnectorsConnectionEventingConfigElRegistrationDestinationConfigEl {
            key: core::default::Default::default(),
            destination: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct IntegrationConnectorsConnectionEventingConfigElRegistrationDestinationConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionEventingConfigElRegistrationDestinationConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionEventingConfigElRegistrationDestinationConfigElRef {
        IntegrationConnectorsConnectionEventingConfigElRegistrationDestinationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionEventingConfigElRegistrationDestinationConfigElRef {
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
    ::ListRef<IntegrationConnectorsConnectionEventingConfigElRegistrationDestinationConfigElDestinationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }
}

#[derive(Serialize, Default)]
struct IntegrationConnectorsConnectionEventingConfigElDynamic {
    additional_variable: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnectionEventingConfigElAdditionalVariableEl>>,
    auth_config: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnectionEventingConfigElAuthConfigEl>>,
    registration_destination_config: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnectionEventingConfigElRegistrationDestinationConfigEl>>,
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnectionEventingConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enrichment_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_variable: std
    ::option
    ::Option<std::vec::Vec<IntegrationConnectorsConnectionEventingConfigElAdditionalVariableEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_config: std::option::Option<std::vec::Vec<IntegrationConnectorsConnectionEventingConfigElAuthConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registration_destination_config: std
    ::option
    ::Option<std::vec::Vec<IntegrationConnectorsConnectionEventingConfigElRegistrationDestinationConfigEl>>,
    dynamic: IntegrationConnectorsConnectionEventingConfigElDynamic,
}

impl IntegrationConnectorsConnectionEventingConfigEl {
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
                    ::Into<
                        terrars::BlockAssignable<IntegrationConnectorsConnectionEventingConfigElAdditionalVariableEl>,
                    >,
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
                    ::Into<terrars::BlockAssignable<IntegrationConnectorsConnectionEventingConfigElAuthConfigEl>>,
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
                            IntegrationConnectorsConnectionEventingConfigElRegistrationDestinationConfigEl,
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

impl terrars::ToListMappable for IntegrationConnectorsConnectionEventingConfigEl {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnectionEventingConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionEventingConfigEl {}

impl BuildIntegrationConnectorsConnectionEventingConfigEl {
    pub fn build(self) -> IntegrationConnectorsConnectionEventingConfigEl {
        IntegrationConnectorsConnectionEventingConfigEl {
            enrichment_enabled: core::default::Default::default(),
            additional_variable: core::default::Default::default(),
            auth_config: core::default::Default::default(),
            registration_destination_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct IntegrationConnectorsConnectionEventingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionEventingConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionEventingConfigElRef {
        IntegrationConnectorsConnectionEventingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionEventingConfigElRef {
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
    ) -> terrars::ListRef<IntegrationConnectorsConnectionEventingConfigElAdditionalVariableElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_variable", self.base))
    }

    #[doc = "Get a reference to the value of field `auth_config` after provisioning.\n"]
    pub fn auth_config(&self) -> terrars::ListRef<IntegrationConnectorsConnectionEventingConfigElAuthConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auth_config", self.base))
    }

    #[doc = "Get a reference to the value of field `registration_destination_config` after provisioning.\n"]
    pub fn registration_destination_config(
        &self,
    ) -> terrars::ListRef<IntegrationConnectorsConnectionEventingConfigElRegistrationDestinationConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.registration_destination_config", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnectionLockConfigEl {
    locked: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reason: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IntegrationConnectorsConnectionLockConfigEl {
    #[doc = "Set the field `reason`.\nDescribes why a connection is locked."]
    pub fn set_reason(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.reason = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IntegrationConnectorsConnectionLockConfigEl {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnectionLockConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionLockConfigEl {
    #[doc = "Indicates whether or not the connection is locked."]
    pub locked: terrars::PrimField<bool>,
}

impl BuildIntegrationConnectorsConnectionLockConfigEl {
    pub fn build(self) -> IntegrationConnectorsConnectionLockConfigEl {
        IntegrationConnectorsConnectionLockConfigEl {
            locked: self.locked,
            reason: core::default::Default::default(),
        }
    }
}

pub struct IntegrationConnectorsConnectionLockConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionLockConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionLockConfigElRef {
        IntegrationConnectorsConnectionLockConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionLockConfigElRef {
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
pub struct IntegrationConnectorsConnectionLogConfigEl {
    enabled: terrars::PrimField<bool>,
}

impl IntegrationConnectorsConnectionLogConfigEl { }

impl terrars::ToListMappable for IntegrationConnectorsConnectionLogConfigEl {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnectionLogConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionLogConfigEl {
    #[doc = "Enabled represents whether logging is enabled or not for a connection."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildIntegrationConnectorsConnectionLogConfigEl {
    pub fn build(self) -> IntegrationConnectorsConnectionLogConfigEl {
        IntegrationConnectorsConnectionLogConfigEl { enabled: self.enabled }
    }
}

pub struct IntegrationConnectorsConnectionLogConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionLogConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IntegrationConnectorsConnectionLogConfigElRef {
        IntegrationConnectorsConnectionLogConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionLogConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nEnabled represents whether logging is enabled or not for a connection."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnectionNodeConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_node_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_node_count: std::option::Option<terrars::PrimField<f64>>,
}

impl IntegrationConnectorsConnectionNodeConfigEl {
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

impl terrars::ToListMappable for IntegrationConnectorsConnectionNodeConfigEl {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnectionNodeConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionNodeConfigEl {}

impl BuildIntegrationConnectorsConnectionNodeConfigEl {
    pub fn build(self) -> IntegrationConnectorsConnectionNodeConfigEl {
        IntegrationConnectorsConnectionNodeConfigEl {
            max_node_count: core::default::Default::default(),
            min_node_count: core::default::Default::default(),
        }
    }
}

pub struct IntegrationConnectorsConnectionNodeConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionNodeConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionNodeConfigElRef {
        IntegrationConnectorsConnectionNodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionNodeConfigElRef {
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
pub struct IntegrationConnectorsConnectionSslConfigElAdditionalVariableElEncryptionKeyValueEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IntegrationConnectorsConnectionSslConfigElAdditionalVariableElEncryptionKeyValueEl {
    #[doc =
        "Set the field `kms_key_name`.\nThe [KMS key name] with which the content of the Operation is encrypted. The expected\nformat: projects/*/locations/*/keyRings/*/cryptoKeys/*.\nWill be empty string if google managed."]
    pub fn set_kms_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nType of Encription Key Possible values: [\"GOOGLE_MANAGED\", \"CUSTOMER_MANAGED\"]"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IntegrationConnectorsConnectionSslConfigElAdditionalVariableElEncryptionKeyValueEl {
    type O =
        terrars::BlockAssignable<IntegrationConnectorsConnectionSslConfigElAdditionalVariableElEncryptionKeyValueEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionSslConfigElAdditionalVariableElEncryptionKeyValueEl {}

impl BuildIntegrationConnectorsConnectionSslConfigElAdditionalVariableElEncryptionKeyValueEl {
    pub fn build(self) -> IntegrationConnectorsConnectionSslConfigElAdditionalVariableElEncryptionKeyValueEl {
        IntegrationConnectorsConnectionSslConfigElAdditionalVariableElEncryptionKeyValueEl {
            kms_key_name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct IntegrationConnectorsConnectionSslConfigElAdditionalVariableElEncryptionKeyValueElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionSslConfigElAdditionalVariableElEncryptionKeyValueElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionSslConfigElAdditionalVariableElEncryptionKeyValueElRef {
        IntegrationConnectorsConnectionSslConfigElAdditionalVariableElEncryptionKeyValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionSslConfigElAdditionalVariableElEncryptionKeyValueElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nThe [KMS key name] with which the content of the Operation is encrypted. The expected\nformat: projects/*/locations/*/keyRings/*/cryptoKeys/*.\nWill be empty string if google managed."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nType of Encription Key Possible values: [\"GOOGLE_MANAGED\", \"CUSTOMER_MANAGED\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnectionSslConfigElAdditionalVariableElSecretValueEl {
    secret_version: terrars::PrimField<std::string::String>,
}

impl IntegrationConnectorsConnectionSslConfigElAdditionalVariableElSecretValueEl { }

impl terrars::ToListMappable for IntegrationConnectorsConnectionSslConfigElAdditionalVariableElSecretValueEl {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnectionSslConfigElAdditionalVariableElSecretValueEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionSslConfigElAdditionalVariableElSecretValueEl {
    #[doc = "Secret version of Secret Value for Config variable."]
    pub secret_version: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnectionSslConfigElAdditionalVariableElSecretValueEl {
    pub fn build(self) -> IntegrationConnectorsConnectionSslConfigElAdditionalVariableElSecretValueEl {
        IntegrationConnectorsConnectionSslConfigElAdditionalVariableElSecretValueEl {
            secret_version: self.secret_version,
        }
    }
}

pub struct IntegrationConnectorsConnectionSslConfigElAdditionalVariableElSecretValueElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionSslConfigElAdditionalVariableElSecretValueElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionSslConfigElAdditionalVariableElSecretValueElRef {
        IntegrationConnectorsConnectionSslConfigElAdditionalVariableElSecretValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionSslConfigElAdditionalVariableElSecretValueElRef {
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
struct IntegrationConnectorsConnectionSslConfigElAdditionalVariableElDynamic {
    encryption_key_value: std
    ::option
    ::Option<
        terrars::DynamicBlock<IntegrationConnectorsConnectionSslConfigElAdditionalVariableElEncryptionKeyValueEl>,
    >,
    secret_value: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnectionSslConfigElAdditionalVariableElSecretValueEl>>,
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnectionSslConfigElAdditionalVariableEl {
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
    ::Option<std::vec::Vec<IntegrationConnectorsConnectionSslConfigElAdditionalVariableElEncryptionKeyValueEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_value: std
    ::option
    ::Option<std::vec::Vec<IntegrationConnectorsConnectionSslConfigElAdditionalVariableElSecretValueEl>>,
    dynamic: IntegrationConnectorsConnectionSslConfigElAdditionalVariableElDynamic,
}

impl IntegrationConnectorsConnectionSslConfigElAdditionalVariableEl {
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
                            IntegrationConnectorsConnectionSslConfigElAdditionalVariableElEncryptionKeyValueEl,
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
                    ::Into<
                        terrars
                        ::BlockAssignable<IntegrationConnectorsConnectionSslConfigElAdditionalVariableElSecretValueEl>,
                    >,
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

impl terrars::ToListMappable for IntegrationConnectorsConnectionSslConfigElAdditionalVariableEl {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnectionSslConfigElAdditionalVariableEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionSslConfigElAdditionalVariableEl {
    #[doc = "Key for the configVariable"]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnectionSslConfigElAdditionalVariableEl {
    pub fn build(self) -> IntegrationConnectorsConnectionSslConfigElAdditionalVariableEl {
        IntegrationConnectorsConnectionSslConfigElAdditionalVariableEl {
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

pub struct IntegrationConnectorsConnectionSslConfigElAdditionalVariableElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionSslConfigElAdditionalVariableElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionSslConfigElAdditionalVariableElRef {
        IntegrationConnectorsConnectionSslConfigElAdditionalVariableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionSslConfigElAdditionalVariableElRef {
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
    ) -> terrars::ListRef<IntegrationConnectorsConnectionSslConfigElAdditionalVariableElEncryptionKeyValueElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_key_value", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_value` after provisioning.\n"]
    pub fn secret_value(
        &self,
    ) -> terrars::ListRef<IntegrationConnectorsConnectionSslConfigElAdditionalVariableElSecretValueElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_value", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnectionSslConfigElClientCertificateEl {
    secret_version: terrars::PrimField<std::string::String>,
}

impl IntegrationConnectorsConnectionSslConfigElClientCertificateEl { }

impl terrars::ToListMappable for IntegrationConnectorsConnectionSslConfigElClientCertificateEl {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnectionSslConfigElClientCertificateEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionSslConfigElClientCertificateEl {
    #[doc = "Secret version of Secret Value for Config variable."]
    pub secret_version: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnectionSslConfigElClientCertificateEl {
    pub fn build(self) -> IntegrationConnectorsConnectionSslConfigElClientCertificateEl {
        IntegrationConnectorsConnectionSslConfigElClientCertificateEl { secret_version: self.secret_version }
    }
}

pub struct IntegrationConnectorsConnectionSslConfigElClientCertificateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionSslConfigElClientCertificateElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionSslConfigElClientCertificateElRef {
        IntegrationConnectorsConnectionSslConfigElClientCertificateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionSslConfigElClientCertificateElRef {
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
pub struct IntegrationConnectorsConnectionSslConfigElClientPrivateKeyEl {
    secret_version: terrars::PrimField<std::string::String>,
}

impl IntegrationConnectorsConnectionSslConfigElClientPrivateKeyEl { }

impl terrars::ToListMappable for IntegrationConnectorsConnectionSslConfigElClientPrivateKeyEl {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnectionSslConfigElClientPrivateKeyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionSslConfigElClientPrivateKeyEl {
    #[doc = "Secret version of Secret Value for Config variable."]
    pub secret_version: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnectionSslConfigElClientPrivateKeyEl {
    pub fn build(self) -> IntegrationConnectorsConnectionSslConfigElClientPrivateKeyEl {
        IntegrationConnectorsConnectionSslConfigElClientPrivateKeyEl { secret_version: self.secret_version }
    }
}

pub struct IntegrationConnectorsConnectionSslConfigElClientPrivateKeyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionSslConfigElClientPrivateKeyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionSslConfigElClientPrivateKeyElRef {
        IntegrationConnectorsConnectionSslConfigElClientPrivateKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionSslConfigElClientPrivateKeyElRef {
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
pub struct IntegrationConnectorsConnectionSslConfigElClientPrivateKeyPassEl {
    secret_version: terrars::PrimField<std::string::String>,
}

impl IntegrationConnectorsConnectionSslConfigElClientPrivateKeyPassEl { }

impl terrars::ToListMappable for IntegrationConnectorsConnectionSslConfigElClientPrivateKeyPassEl {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnectionSslConfigElClientPrivateKeyPassEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionSslConfigElClientPrivateKeyPassEl {
    #[doc = "Secret version of Secret Value for Config variable."]
    pub secret_version: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnectionSslConfigElClientPrivateKeyPassEl {
    pub fn build(self) -> IntegrationConnectorsConnectionSslConfigElClientPrivateKeyPassEl {
        IntegrationConnectorsConnectionSslConfigElClientPrivateKeyPassEl { secret_version: self.secret_version }
    }
}

pub struct IntegrationConnectorsConnectionSslConfigElClientPrivateKeyPassElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionSslConfigElClientPrivateKeyPassElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionSslConfigElClientPrivateKeyPassElRef {
        IntegrationConnectorsConnectionSslConfigElClientPrivateKeyPassElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionSslConfigElClientPrivateKeyPassElRef {
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
pub struct IntegrationConnectorsConnectionSslConfigElPrivateServerCertificateEl {
    secret_version: terrars::PrimField<std::string::String>,
}

impl IntegrationConnectorsConnectionSslConfigElPrivateServerCertificateEl { }

impl terrars::ToListMappable for IntegrationConnectorsConnectionSslConfigElPrivateServerCertificateEl {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnectionSslConfigElPrivateServerCertificateEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionSslConfigElPrivateServerCertificateEl {
    #[doc = "Secret version of Secret Value for Config variable."]
    pub secret_version: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnectionSslConfigElPrivateServerCertificateEl {
    pub fn build(self) -> IntegrationConnectorsConnectionSslConfigElPrivateServerCertificateEl {
        IntegrationConnectorsConnectionSslConfigElPrivateServerCertificateEl { secret_version: self.secret_version }
    }
}

pub struct IntegrationConnectorsConnectionSslConfigElPrivateServerCertificateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionSslConfigElPrivateServerCertificateElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IntegrationConnectorsConnectionSslConfigElPrivateServerCertificateElRef {
        IntegrationConnectorsConnectionSslConfigElPrivateServerCertificateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionSslConfigElPrivateServerCertificateElRef {
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
struct IntegrationConnectorsConnectionSslConfigElDynamic {
    additional_variable: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnectionSslConfigElAdditionalVariableEl>>,
    client_certificate: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnectionSslConfigElClientCertificateEl>>,
    client_private_key: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnectionSslConfigElClientPrivateKeyEl>>,
    client_private_key_pass: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnectionSslConfigElClientPrivateKeyPassEl>>,
    private_server_certificate: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnectionSslConfigElPrivateServerCertificateEl>>,
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnectionSslConfigEl {
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
    additional_variable: std
    ::option
    ::Option<std::vec::Vec<IntegrationConnectorsConnectionSslConfigElAdditionalVariableEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_certificate: std::option::Option<std::vec::Vec<IntegrationConnectorsConnectionSslConfigElClientCertificateEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_private_key: std::option::Option<std::vec::Vec<IntegrationConnectorsConnectionSslConfigElClientPrivateKeyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_private_key_pass: std
    ::option
    ::Option<std::vec::Vec<IntegrationConnectorsConnectionSslConfigElClientPrivateKeyPassEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_server_certificate: std
    ::option
    ::Option<std::vec::Vec<IntegrationConnectorsConnectionSslConfigElPrivateServerCertificateEl>>,
    dynamic: IntegrationConnectorsConnectionSslConfigElDynamic,
}

impl IntegrationConnectorsConnectionSslConfigEl {
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
                    ::Into<terrars::BlockAssignable<IntegrationConnectorsConnectionSslConfigElAdditionalVariableEl>>,
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
                    ::Into<terrars::BlockAssignable<IntegrationConnectorsConnectionSslConfigElClientCertificateEl>>,
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
                    ::Into<terrars::BlockAssignable<IntegrationConnectorsConnectionSslConfigElClientPrivateKeyEl>>,
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
                    ::Into<
                        terrars::BlockAssignable<IntegrationConnectorsConnectionSslConfigElClientPrivateKeyPassEl>,
                    >,
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
                        terrars::BlockAssignable<IntegrationConnectorsConnectionSslConfigElPrivateServerCertificateEl>,
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

impl terrars::ToListMappable for IntegrationConnectorsConnectionSslConfigEl {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnectionSslConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionSslConfigEl {
    #[doc = "Enum for controlling the SSL Type (TLS/MTLS) Possible values: [\"TLS\", \"MTLS\"]"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationConnectorsConnectionSslConfigEl {
    pub fn build(self) -> IntegrationConnectorsConnectionSslConfigEl {
        IntegrationConnectorsConnectionSslConfigEl {
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

pub struct IntegrationConnectorsConnectionSslConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionSslConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IntegrationConnectorsConnectionSslConfigElRef {
        IntegrationConnectorsConnectionSslConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionSslConfigElRef {
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
    ) -> terrars::ListRef<IntegrationConnectorsConnectionSslConfigElAdditionalVariableElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_variable", self.base))
    }

    #[doc = "Get a reference to the value of field `client_certificate` after provisioning.\n"]
    pub fn client_certificate(
        &self,
    ) -> terrars::ListRef<IntegrationConnectorsConnectionSslConfigElClientCertificateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_certificate", self.base))
    }

    #[doc = "Get a reference to the value of field `client_private_key` after provisioning.\n"]
    pub fn client_private_key(
        &self,
    ) -> terrars::ListRef<IntegrationConnectorsConnectionSslConfigElClientPrivateKeyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_private_key", self.base))
    }

    #[doc = "Get a reference to the value of field `client_private_key_pass` after provisioning.\n"]
    pub fn client_private_key_pass(
        &self,
    ) -> terrars::ListRef<IntegrationConnectorsConnectionSslConfigElClientPrivateKeyPassElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_private_key_pass", self.base))
    }

    #[doc = "Get a reference to the value of field `private_server_certificate` after provisioning.\n"]
    pub fn private_server_certificate(
        &self,
    ) -> terrars::ListRef<IntegrationConnectorsConnectionSslConfigElPrivateServerCertificateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_server_certificate", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationConnectorsConnectionTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IntegrationConnectorsConnectionTimeoutsEl {
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

impl terrars::ToListMappable for IntegrationConnectorsConnectionTimeoutsEl {
    type O = terrars::BlockAssignable<IntegrationConnectorsConnectionTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationConnectorsConnectionTimeoutsEl {}

impl BuildIntegrationConnectorsConnectionTimeoutsEl {
    pub fn build(self) -> IntegrationConnectorsConnectionTimeoutsEl {
        IntegrationConnectorsConnectionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct IntegrationConnectorsConnectionTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationConnectorsConnectionTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IntegrationConnectorsConnectionTimeoutsElRef {
        IntegrationConnectorsConnectionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationConnectorsConnectionTimeoutsElRef {
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
struct IntegrationConnectorsConnectionDynamic {
    auth_config: std::option::Option<terrars::DynamicBlock<IntegrationConnectorsConnectionAuthConfigEl>>,
    config_variable: std::option::Option<terrars::DynamicBlock<IntegrationConnectorsConnectionConfigVariableEl>>,
    destination_config: std
    ::option
    ::Option<terrars::DynamicBlock<IntegrationConnectorsConnectionDestinationConfigEl>>,
    eventing_config: std::option::Option<terrars::DynamicBlock<IntegrationConnectorsConnectionEventingConfigEl>>,
    lock_config: std::option::Option<terrars::DynamicBlock<IntegrationConnectorsConnectionLockConfigEl>>,
    log_config: std::option::Option<terrars::DynamicBlock<IntegrationConnectorsConnectionLogConfigEl>>,
    node_config: std::option::Option<terrars::DynamicBlock<IntegrationConnectorsConnectionNodeConfigEl>>,
    ssl_config: std::option::Option<terrars::DynamicBlock<IntegrationConnectorsConnectionSslConfigEl>>,
}
