#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DiscoveryEngineDataConnectorData_ {
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
    auto_run_disabled: std::option::Option<terrars::PrimField<bool>>,
    collection_display_name: terrars::PrimField<std::string::String>,
    collection_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connector_modes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    data_source: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_source_version: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    incremental_refresh_interval: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    incremental_sync_disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json_params: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    params: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    refresh_interval: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    static_ip_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sync_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_config: std::option::Option<std::vec::Vec<DiscoveryEngineDataConnector_ActionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bap_config: std::option::Option<std::vec::Vec<DiscoveryEngineDataConnector_BapConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_configs: std::option::Option<std::vec::Vec<DiscoveryEngineDataConnector_DestinationConfigs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entities: std::option::Option<std::vec::Vec<DiscoveryEngineDataConnector_Entities>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<std::vec::Vec<DiscoveryEngineDataConnector_Metadata>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DiscoveryEngineDataConnector_Timeouts>,
    dynamic: DiscoveryEngineDataConnectorDynamic,
}

struct DiscoveryEngineDataConnector_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DiscoveryEngineDataConnectorData_>,
}

#[derive(Clone)]
pub struct DiscoveryEngineDataConnector(std::rc::Rc<DiscoveryEngineDataConnector_>);

impl DiscoveryEngineDataConnector {
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

    #[doc = "Set the field `auto_run_disabled`.\nIndicates whether full syncs are paused for this connector"]
    pub fn set_auto_run_disabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_run_disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `connector_modes`.\nThe modes enabled for this connector. The possible value can be:\n'DATA_INGESTION', 'ACTIONS', 'FEDERATED'\n'EUA', 'FEDERATED_AND_EUA'."]
    pub fn set_connector_modes(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().connector_modes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_source_version`.\nThe version of the data source. For example, '3' for Jira v3."]
    pub fn set_data_source_version(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().data_source_version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `incremental_refresh_interval`.\nThe refresh interval specifically for incremental data syncs. If unset,\nincremental syncs will use the default from env, set to 3hrs.\nThe minimum is 30 minutes and maximum is 7 days. Applicable to only 3P\nconnectors. When the refresh interval is\nset to the same value as the incremental refresh interval, incremental\nsync will be disabled."]
    pub fn set_incremental_refresh_interval(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().incremental_refresh_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `incremental_sync_disabled`.\nIndicates whether incremental syncs are paused for this connector."]
    pub fn set_incremental_sync_disabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().incremental_sync_disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `json_params`.\nParams needed to access the source in the format of json string."]
    pub fn set_json_params(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().json_params = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kms_key_name`.\nThe KMS key to be used to protect the DataStores managed by this connector.\nMust be set for requests that need to comply with CMEK Org Policy\nprotections.\nIf this field is set and processed successfully, the DataStores created by\nthis connector will be protected by the KMS key."]
    pub fn set_kms_key_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `params`.\nParams needed to access the source in the format of String-to-String (Key, Value) pairs."]
    pub fn set_params(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().params = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `static_ip_enabled`.\nWhether customer has enabled static IP addresses for this connector."]
    pub fn set_static_ip_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().static_ip_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `sync_mode`.\nThe data synchronization mode supported by the data connector. The possible value can be:\n'PERIODIC', 'STREAMING'."]
    pub fn set_sync_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().sync_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag`.\nUser-facing, version-independent label for this connector."]
    pub fn set_tag(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `action_config`.\n"]
    pub fn set_action_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DiscoveryEngineDataConnector_ActionConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().action_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.action_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `bap_config`.\n"]
    pub fn set_bap_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DiscoveryEngineDataConnector_BapConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().bap_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.bap_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `destination_configs`.\n"]
    pub fn set_destination_configs(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DiscoveryEngineDataConnector_DestinationConfigs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination_configs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `entities`.\n"]
    pub fn set_entities(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DiscoveryEngineDataConnector_Entities>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().entities = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.entities = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DiscoveryEngineDataConnector_Metadata>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().metadata = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.metadata = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DiscoveryEngineDataConnector_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `action_state` after provisioning.\nState of the action connector. This reflects whether the action connector\nis initializing, active or has encountered errors. The possible value can be:\n'STATE_UNSPECIFIED', 'CREATING', 'ACTIVE', 'FAILED', 'RUNNING', 'WARNING',\n'INITIALIZATION_FAILED', 'UPDATING'."]
    pub fn action_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auto_run_disabled` after provisioning.\nIndicates whether full syncs are paused for this connector"]
    pub fn auto_run_disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_run_disabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `blocking_reasons` after provisioning.\nUser actions that must be completed before the connector can start syncing data.\nThe possible values can be: 'ALLOWLIST_STATIC_IP', 'ALLOWLIST_IN_SERVICE_ATTACHMENT'."]
    pub fn blocking_reasons(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.blocking_reasons", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `collection_display_name` after provisioning.\nThe display name of the Collection.\nShould be human readable, used to display collections in the Console\nDashboard. UTF-8 encoded string with limit of 1024 characters."]
    pub fn collection_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collection_display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `collection_id` after provisioning.\nThe ID to use for the Collection, which will become the final component\nof the Collection's resource name. A new Collection is created as\npart of the DataConnector setup. DataConnector is a singleton\nresource under Collection, managing all DataStores of the Collection.\nThis field must conform to [RFC-1034](https://tools.ietf.org/html/rfc1034)\nstandard with a length limit of 63 characters. Otherwise, an\nINVALID_ARGUMENT error is returned."]
    pub fn collection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collection_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `connector_modes` after provisioning.\nThe modes enabled for this connector. The possible value can be:\n'DATA_INGESTION', 'ACTIONS', 'FEDERATED'\n'EUA', 'FEDERATED_AND_EUA'."]
    pub fn connector_modes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connector_modes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `connector_type` after provisioning.\nThe type of connector. Each source can only map to one type.\nFor example, salesforce, confluence and jira have THIRD_PARTY connector\ntype. It is not mutable once set by system. The possible value can be:\n'CONNECTOR_TYPE_UNSPECIFIED', 'THIRD_PARTY', 'GCP_FHIR', 'BIG_QUERY',\n'GCS', 'GOOGLE_MAIL', 'GOOGLE_CALENDAR', 'GOOGLE_DRIVE',\n'NATIVE_CLOUD_IDENTITY', 'THIRD_PARTY_FEDERATED', 'THIRD_PARTY_EUA', 'GCNV'."]
    pub fn connector_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connector_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the DataConnector was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_source` after provisioning.\nThe identifier for the data source.\nThis is a partial list of supported connectors. Please refer to the\n[documentation](https://docs.cloud.google.com/gemini/enterprise/docs/connectors/introduction-to-connectors-and-data-stores)\nfor the full list of connectors.\n\nSupported first-party connectors include:\n\n*   'bigquery'\n*   'gcp_fhir'\n*   'google_mail'\n*   'google_drive'\n*   'google_calendar'\n*   'google_chat'\n\nSupported third-party connectors include:\nGenerally available (GA) connectors:\n\n*   'onedrive'\n*   'outlook'\n*   'confluence'\n*   'jira'\n*   'servicenow'\n*   'sharepoint'\n\nPreview connectors:\n\n*   'asana'\n*   'azure_active_directory'\n*   'box'\n*   'canva'\n*   'confluence_server'\n*   'custom_connector'\n*   'docusign'\n*   'dropbox'\n*   'dynamics365'\n*   'github'\n*   'gitlab'\n*   'hubspot'\n*   'jira_server'\n*   'linear'\n*   'native_cloud_identity'\n*   'notion'\n*   'okta'\n*   'pagerduty'\n*   'peoplesoft'\n*   'salesforce'\n*   'shopify'\n*   'slack'\n*   'snowflake'\n*   'teams'\n*   'trello'\n*   'workday'\n*   'zendesk'"]
    pub fn data_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_source_version` after provisioning.\nThe version of the data source. For example, '3' for Jira v3."]
    pub fn data_source_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `errors` after provisioning.\nThe errors from initialization or from the latest connector run."]
    pub fn errors(&self) -> terrars::ListRef<DiscoveryEngineDataConnector_ErrorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.errors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `incremental_refresh_interval` after provisioning.\nThe refresh interval specifically for incremental data syncs. If unset,\nincremental syncs will use the default from env, set to 3hrs.\nThe minimum is 30 minutes and maximum is 7 days. Applicable to only 3P\nconnectors. When the refresh interval is\nset to the same value as the incremental refresh interval, incremental\nsync will be disabled."]
    pub fn incremental_refresh_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.incremental_refresh_interval", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `incremental_sync_disabled` after provisioning.\nIndicates whether incremental syncs are paused for this connector."]
    pub fn incremental_sync_disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.incremental_sync_disabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `json_params` after provisioning.\nParams needed to access the source in the format of json string."]
    pub fn json_params(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.json_params", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nThe KMS key to be used to protect the DataStores managed by this connector.\nMust be set for requests that need to comply with CMEK Org Policy\nprotections.\nIf this field is set and processed successfully, the DataStores created by\nthis connector will be protected by the KMS key."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_sync_time` after provisioning.\nFor periodic connectors only, the last time a data sync was completed."]
    pub fn last_sync_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_sync_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `latest_pause_time` after provisioning.\nThe most recent timestamp when this [DataConnector][] was paused,\naffecting all functionalities such as data synchronization.\nPausing a connector has the following effects:\n  - All functionalities, including data synchronization, are halted.\n  - Any ongoing data synchronization job will be canceled.\n  - No future data synchronization runs will be scheduled nor can be\ntriggered."]
    pub fn latest_pause_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latest_pause_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe full resource name of the Data Connector.\nFormat: 'projects/*/locations/*/collections/*/dataConnector'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `params` after provisioning.\nParams needed to access the source in the format of String-to-String (Key, Value) pairs."]
    pub fn params(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.params", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `private_connectivity_project_id` after provisioning.\nThe tenant project ID associated with private connectivity connectors.\nThis project must be allowlisted by in order for the connector to function."]
    pub fn private_connectivity_project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_connectivity_project_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `realtime_state` after provisioning.\nThe real-time sync state. The possible values can be:\n'STATE_UNSPECIFIED', 'CREATING', 'ACTIVE', 'FAILED', 'RUNNING', 'WARNING',\n'INITIALIZATION_FAILED', 'UPDATING'."]
    pub fn realtime_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.realtime_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `refresh_interval` after provisioning.\nThe refresh interval for data sync. If duration is set to 0, the data will\nbe synced in real time. The streaming feature is not supported yet. The\nminimum is 30 minutes and maximum is 7 days. When the refresh interval is\nset to the same value as the incremental refresh interval, incremental\nsync will be disabled."]
    pub fn refresh_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refresh_interval", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe state of connector. The possible value can be:\n'STATE_UNSPECIFIED', 'CREATING', 'ACTIVE', 'FAILED', 'RUNNING', 'WARNING',\n'INITIALIZATION_FAILED', 'UPDATING'."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `static_ip_addresses` after provisioning.\nThe static IP addresses used by this connector."]
    pub fn static_ip_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.static_ip_addresses", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `static_ip_enabled` after provisioning.\nWhether customer has enabled static IP addresses for this connector."]
    pub fn static_ip_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.static_ip_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sync_mode` after provisioning.\nThe data synchronization mode supported by the data connector. The possible value can be:\n'PERIODIC', 'STREAMING'."]
    pub fn sync_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sync_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tag` after provisioning.\nUser-facing, version-independent label for this connector."]
    pub fn tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when the DataConnector was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `action_config` after provisioning.\n"]
    pub fn action_config(&self) -> terrars::ListRef<DiscoveryEngineDataConnector_ActionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bap_config` after provisioning.\n"]
    pub fn bap_config(&self) -> terrars::ListRef<DiscoveryEngineDataConnector_BapConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bap_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_configs` after provisioning.\n"]
    pub fn destination_configs(&self) -> terrars::ListRef<DiscoveryEngineDataConnector_DestinationConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_configs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entities` after provisioning.\n"]
    pub fn entities(&self) -> terrars::ListRef<DiscoveryEngineDataConnector_EntitiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entities", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> terrars::ListRef<DiscoveryEngineDataConnector_MetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineDataConnector_TimeoutsRef {
        DiscoveryEngineDataConnector_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DiscoveryEngineDataConnector {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DiscoveryEngineDataConnector { }

impl terrars::ToListMappable for DiscoveryEngineDataConnector {
    type O = terrars::ListRef<DiscoveryEngineDataConnectorRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DiscoveryEngineDataConnector_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_discovery_engine_data_connector".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDiscoveryEngineDataConnector {
    pub tf_id: std::string::String,
    #[doc =
        "The display name of the Collection.\nShould be human readable, used to display collections in the Console\nDashboard. UTF-8 encoded string with limit of 1024 characters."]
    pub collection_display_name: terrars::PrimField<std::string::String>,
    #[doc =
        "The ID to use for the Collection, which will become the final component\nof the Collection's resource name. A new Collection is created as\npart of the DataConnector setup. DataConnector is a singleton\nresource under Collection, managing all DataStores of the Collection.\nThis field must conform to [RFC-1034](https://tools.ietf.org/html/rfc1034)\nstandard with a length limit of 63 characters. Otherwise, an\nINVALID_ARGUMENT error is returned."]
    pub collection_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The identifier for the data source.\nThis is a partial list of supported connectors. Please refer to the\n[documentation](https://docs.cloud.google.com/gemini/enterprise/docs/connectors/introduction-to-connectors-and-data-stores)\nfor the full list of connectors.\n\nSupported first-party connectors include:\n\n*   'bigquery'\n*   'gcp_fhir'\n*   'google_mail'\n*   'google_drive'\n*   'google_calendar'\n*   'google_chat'\n\nSupported third-party connectors include:\nGenerally available (GA) connectors:\n\n*   'onedrive'\n*   'outlook'\n*   'confluence'\n*   'jira'\n*   'servicenow'\n*   'sharepoint'\n\nPreview connectors:\n\n*   'asana'\n*   'azure_active_directory'\n*   'box'\n*   'canva'\n*   'confluence_server'\n*   'custom_connector'\n*   'docusign'\n*   'dropbox'\n*   'dynamics365'\n*   'github'\n*   'gitlab'\n*   'hubspot'\n*   'jira_server'\n*   'linear'\n*   'native_cloud_identity'\n*   'notion'\n*   'okta'\n*   'pagerduty'\n*   'peoplesoft'\n*   'salesforce'\n*   'shopify'\n*   'slack'\n*   'snowflake'\n*   'teams'\n*   'trello'\n*   'workday'\n*   'zendesk'"]
    pub data_source: terrars::PrimField<std::string::String>,
    #[doc =
        "The geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "The refresh interval for data sync. If duration is set to 0, the data will\nbe synced in real time. The streaming feature is not supported yet. The\nminimum is 30 minutes and maximum is 7 days. When the refresh interval is\nset to the same value as the incremental refresh interval, incremental\nsync will be disabled."]
    pub refresh_interval: terrars::PrimField<std::string::String>,
}

impl BuildDiscoveryEngineDataConnector {
    pub fn build(self, stack: &mut terrars::Stack) -> DiscoveryEngineDataConnector {
        let out = DiscoveryEngineDataConnector(std::rc::Rc::new(DiscoveryEngineDataConnector_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DiscoveryEngineDataConnectorData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                auto_run_disabled: core::default::Default::default(),
                collection_display_name: self.collection_display_name,
                collection_id: self.collection_id,
                connector_modes: core::default::Default::default(),
                data_source: self.data_source,
                data_source_version: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                incremental_refresh_interval: core::default::Default::default(),
                incremental_sync_disabled: core::default::Default::default(),
                json_params: core::default::Default::default(),
                kms_key_name: core::default::Default::default(),
                location: self.location,
                params: core::default::Default::default(),
                project: core::default::Default::default(),
                refresh_interval: self.refresh_interval,
                static_ip_enabled: core::default::Default::default(),
                sync_mode: core::default::Default::default(),
                tag: core::default::Default::default(),
                action_config: core::default::Default::default(),
                bap_config: core::default::Default::default(),
                destination_configs: core::default::Default::default(),
                entities: core::default::Default::default(),
                metadata: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DiscoveryEngineDataConnectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineDataConnectorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DiscoveryEngineDataConnectorRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action_state` after provisioning.\nState of the action connector. This reflects whether the action connector\nis initializing, active or has encountered errors. The possible value can be:\n'STATE_UNSPECIFIED', 'CREATING', 'ACTIVE', 'FAILED', 'RUNNING', 'WARNING',\n'INITIALIZATION_FAILED', 'UPDATING'."]
    pub fn action_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auto_run_disabled` after provisioning.\nIndicates whether full syncs are paused for this connector"]
    pub fn auto_run_disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_run_disabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `blocking_reasons` after provisioning.\nUser actions that must be completed before the connector can start syncing data.\nThe possible values can be: 'ALLOWLIST_STATIC_IP', 'ALLOWLIST_IN_SERVICE_ATTACHMENT'."]
    pub fn blocking_reasons(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.blocking_reasons", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `collection_display_name` after provisioning.\nThe display name of the Collection.\nShould be human readable, used to display collections in the Console\nDashboard. UTF-8 encoded string with limit of 1024 characters."]
    pub fn collection_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collection_display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `collection_id` after provisioning.\nThe ID to use for the Collection, which will become the final component\nof the Collection's resource name. A new Collection is created as\npart of the DataConnector setup. DataConnector is a singleton\nresource under Collection, managing all DataStores of the Collection.\nThis field must conform to [RFC-1034](https://tools.ietf.org/html/rfc1034)\nstandard with a length limit of 63 characters. Otherwise, an\nINVALID_ARGUMENT error is returned."]
    pub fn collection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collection_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `connector_modes` after provisioning.\nThe modes enabled for this connector. The possible value can be:\n'DATA_INGESTION', 'ACTIONS', 'FEDERATED'\n'EUA', 'FEDERATED_AND_EUA'."]
    pub fn connector_modes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connector_modes", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `connector_type` after provisioning.\nThe type of connector. Each source can only map to one type.\nFor example, salesforce, confluence and jira have THIRD_PARTY connector\ntype. It is not mutable once set by system. The possible value can be:\n'CONNECTOR_TYPE_UNSPECIFIED', 'THIRD_PARTY', 'GCP_FHIR', 'BIG_QUERY',\n'GCS', 'GOOGLE_MAIL', 'GOOGLE_CALENDAR', 'GOOGLE_DRIVE',\n'NATIVE_CLOUD_IDENTITY', 'THIRD_PARTY_FEDERATED', 'THIRD_PARTY_EUA', 'GCNV'."]
    pub fn connector_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connector_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the DataConnector was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_source` after provisioning.\nThe identifier for the data source.\nThis is a partial list of supported connectors. Please refer to the\n[documentation](https://docs.cloud.google.com/gemini/enterprise/docs/connectors/introduction-to-connectors-and-data-stores)\nfor the full list of connectors.\n\nSupported first-party connectors include:\n\n*   'bigquery'\n*   'gcp_fhir'\n*   'google_mail'\n*   'google_drive'\n*   'google_calendar'\n*   'google_chat'\n\nSupported third-party connectors include:\nGenerally available (GA) connectors:\n\n*   'onedrive'\n*   'outlook'\n*   'confluence'\n*   'jira'\n*   'servicenow'\n*   'sharepoint'\n\nPreview connectors:\n\n*   'asana'\n*   'azure_active_directory'\n*   'box'\n*   'canva'\n*   'confluence_server'\n*   'custom_connector'\n*   'docusign'\n*   'dropbox'\n*   'dynamics365'\n*   'github'\n*   'gitlab'\n*   'hubspot'\n*   'jira_server'\n*   'linear'\n*   'native_cloud_identity'\n*   'notion'\n*   'okta'\n*   'pagerduty'\n*   'peoplesoft'\n*   'salesforce'\n*   'shopify'\n*   'slack'\n*   'snowflake'\n*   'teams'\n*   'trello'\n*   'workday'\n*   'zendesk'"]
    pub fn data_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_source_version` after provisioning.\nThe version of the data source. For example, '3' for Jira v3."]
    pub fn data_source_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `errors` after provisioning.\nThe errors from initialization or from the latest connector run."]
    pub fn errors(&self) -> terrars::ListRef<DiscoveryEngineDataConnector_ErrorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.errors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `incremental_refresh_interval` after provisioning.\nThe refresh interval specifically for incremental data syncs. If unset,\nincremental syncs will use the default from env, set to 3hrs.\nThe minimum is 30 minutes and maximum is 7 days. Applicable to only 3P\nconnectors. When the refresh interval is\nset to the same value as the incremental refresh interval, incremental\nsync will be disabled."]
    pub fn incremental_refresh_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.incremental_refresh_interval", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `incremental_sync_disabled` after provisioning.\nIndicates whether incremental syncs are paused for this connector."]
    pub fn incremental_sync_disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.incremental_sync_disabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `json_params` after provisioning.\nParams needed to access the source in the format of json string."]
    pub fn json_params(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.json_params", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nThe KMS key to be used to protect the DataStores managed by this connector.\nMust be set for requests that need to comply with CMEK Org Policy\nprotections.\nIf this field is set and processed successfully, the DataStores created by\nthis connector will be protected by the KMS key."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_sync_time` after provisioning.\nFor periodic connectors only, the last time a data sync was completed."]
    pub fn last_sync_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_sync_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `latest_pause_time` after provisioning.\nThe most recent timestamp when this [DataConnector][] was paused,\naffecting all functionalities such as data synchronization.\nPausing a connector has the following effects:\n  - All functionalities, including data synchronization, are halted.\n  - Any ongoing data synchronization job will be canceled.\n  - No future data synchronization runs will be scheduled nor can be\ntriggered."]
    pub fn latest_pause_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latest_pause_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe full resource name of the Data Connector.\nFormat: 'projects/*/locations/*/collections/*/dataConnector'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `params` after provisioning.\nParams needed to access the source in the format of String-to-String (Key, Value) pairs."]
    pub fn params(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.params", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `private_connectivity_project_id` after provisioning.\nThe tenant project ID associated with private connectivity connectors.\nThis project must be allowlisted by in order for the connector to function."]
    pub fn private_connectivity_project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_connectivity_project_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `realtime_state` after provisioning.\nThe real-time sync state. The possible values can be:\n'STATE_UNSPECIFIED', 'CREATING', 'ACTIVE', 'FAILED', 'RUNNING', 'WARNING',\n'INITIALIZATION_FAILED', 'UPDATING'."]
    pub fn realtime_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.realtime_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `refresh_interval` after provisioning.\nThe refresh interval for data sync. If duration is set to 0, the data will\nbe synced in real time. The streaming feature is not supported yet. The\nminimum is 30 minutes and maximum is 7 days. When the refresh interval is\nset to the same value as the incremental refresh interval, incremental\nsync will be disabled."]
    pub fn refresh_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refresh_interval", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe state of connector. The possible value can be:\n'STATE_UNSPECIFIED', 'CREATING', 'ACTIVE', 'FAILED', 'RUNNING', 'WARNING',\n'INITIALIZATION_FAILED', 'UPDATING'."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `static_ip_addresses` after provisioning.\nThe static IP addresses used by this connector."]
    pub fn static_ip_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.static_ip_addresses", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `static_ip_enabled` after provisioning.\nWhether customer has enabled static IP addresses for this connector."]
    pub fn static_ip_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.static_ip_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `sync_mode` after provisioning.\nThe data synchronization mode supported by the data connector. The possible value can be:\n'PERIODIC', 'STREAMING'."]
    pub fn sync_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sync_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tag` after provisioning.\nUser-facing, version-independent label for this connector."]
    pub fn tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when the DataConnector was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `action_config` after provisioning.\n"]
    pub fn action_config(&self) -> terrars::ListRef<DiscoveryEngineDataConnector_ActionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bap_config` after provisioning.\n"]
    pub fn bap_config(&self) -> terrars::ListRef<DiscoveryEngineDataConnector_BapConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bap_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_configs` after provisioning.\n"]
    pub fn destination_configs(&self) -> terrars::ListRef<DiscoveryEngineDataConnector_DestinationConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_configs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entities` after provisioning.\n"]
    pub fn entities(&self) -> terrars::ListRef<DiscoveryEngineDataConnector_EntitiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entities", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> terrars::ListRef<DiscoveryEngineDataConnector_MetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineDataConnector_TimeoutsRef {
        DiscoveryEngineDataConnector_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct DiscoveryEngineDataConnectorDynamic {
    action_config: std::option::Option<terrars::DynamicBlock<DiscoveryEngineDataConnector_ActionConfig>>,
    bap_config: std::option::Option<terrars::DynamicBlock<DiscoveryEngineDataConnector_BapConfig>>,
    destination_configs: std::option::Option<terrars::DynamicBlock<DiscoveryEngineDataConnector_DestinationConfigs>>,
    entities: std::option::Option<terrars::DynamicBlock<DiscoveryEngineDataConnector_Entities>>,
    metadata: std::option::Option<terrars::DynamicBlock<DiscoveryEngineDataConnector_Metadata>>,
}

#[derive(Serialize)]
pub struct DiscoveryEngineDataConnector_Errors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DiscoveryEngineDataConnector_Errors {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineDataConnector_Errors {
    type O = terrars::BlockAssignable<DiscoveryEngineDataConnector_Errors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineDataConnector_Errors {}

impl BuildDiscoveryEngineDataConnector_Errors {
    pub fn build(self) -> DiscoveryEngineDataConnector_Errors {
        DiscoveryEngineDataConnector_Errors {
            code: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineDataConnector_ErrorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineDataConnector_ErrorsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DiscoveryEngineDataConnector_ErrorsRef {
        DiscoveryEngineDataConnector_ErrorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineDataConnector_ErrorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineDataConnector_ActionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_params: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_bap_connection: std::option::Option<terrars::PrimField<bool>>,
}

impl DiscoveryEngineDataConnector_ActionConfig {
    #[doc =
        "Set the field `action_params`.\nParams needed to configure the actions in the format of\nString-to-String (Key, Value) pairs. Contains connection\ncredentials and configuration for the action connector."]
    pub fn set_action_params(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.action_params = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `create_bap_connection`.\nWhether to create a BAP (Business Application Platform) connection\nfor this action connector."]
    pub fn set_create_bap_connection(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.create_bap_connection = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineDataConnector_ActionConfig {
    type O = terrars::BlockAssignable<DiscoveryEngineDataConnector_ActionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineDataConnector_ActionConfig {}

impl BuildDiscoveryEngineDataConnector_ActionConfig {
    pub fn build(self) -> DiscoveryEngineDataConnector_ActionConfig {
        DiscoveryEngineDataConnector_ActionConfig {
            action_params: core::default::Default::default(),
            create_bap_connection: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineDataConnector_ActionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineDataConnector_ActionConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DiscoveryEngineDataConnector_ActionConfigRef {
        DiscoveryEngineDataConnector_ActionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineDataConnector_ActionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action_params` after provisioning.\nParams needed to configure the actions in the format of\nString-to-String (Key, Value) pairs. Contains connection\ncredentials and configuration for the action connector."]
    pub fn action_params(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.action_params", self.base))
    }

    #[doc =
        "Get a reference to the value of field `create_bap_connection` after provisioning.\nWhether to create a BAP (Business Application Platform) connection\nfor this action connector."]
    pub fn create_bap_connection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_bap_connection", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_action_configured` after provisioning.\nWhether the action connector is fully configured. Set by the system\nafter the action configuration is validated."]
    pub fn is_action_configured(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_action_configured", self.base))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineDataConnector_BapConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled_actions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    supported_connector_modes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DiscoveryEngineDataConnector_BapConfig {
    #[doc =
        "Set the field `enabled_actions`.\nThe list of enabled actions for this connector. Supported\nvalues include: 'create_issue', 'update_issue',\n'change_issue_status', 'create_comment', 'update_comment',\n'upload_attachment'."]
    pub fn set_enabled_actions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.enabled_actions = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `supported_connector_modes`.\nThe connector modes supported by the BAP configuration.\nThe possible values include: 'ACTIONS'."]
    pub fn set_supported_connector_modes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.supported_connector_modes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineDataConnector_BapConfig {
    type O = terrars::BlockAssignable<DiscoveryEngineDataConnector_BapConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineDataConnector_BapConfig {}

impl BuildDiscoveryEngineDataConnector_BapConfig {
    pub fn build(self) -> DiscoveryEngineDataConnector_BapConfig {
        DiscoveryEngineDataConnector_BapConfig {
            enabled_actions: core::default::Default::default(),
            supported_connector_modes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineDataConnector_BapConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineDataConnector_BapConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DiscoveryEngineDataConnector_BapConfigRef {
        DiscoveryEngineDataConnector_BapConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineDataConnector_BapConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled_actions` after provisioning.\nThe list of enabled actions for this connector. Supported\nvalues include: 'create_issue', 'update_issue',\n'change_issue_status', 'create_comment', 'update_comment',\n'upload_attachment'."]
    pub fn enabled_actions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enabled_actions", self.base))
    }

    #[doc =
        "Get a reference to the value of field `supported_connector_modes` after provisioning.\nThe connector modes supported by the BAP configuration.\nThe possible values include: 'ACTIONS'."]
    pub fn supported_connector_modes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_connector_modes", self.base))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineDataConnector_DestinationConfigs_Destinations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
}

impl DiscoveryEngineDataConnector_DestinationConfigs_Destinations {
    #[doc = "Set the field `host`.\nThe host of the destination, for example\n'https://example.atlassian.net'."]
    pub fn set_host(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\nTarget port number accepted by the destination."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineDataConnector_DestinationConfigs_Destinations {
    type O = terrars::BlockAssignable<DiscoveryEngineDataConnector_DestinationConfigs_Destinations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineDataConnector_DestinationConfigs_Destinations {}

impl BuildDiscoveryEngineDataConnector_DestinationConfigs_Destinations {
    pub fn build(self) -> DiscoveryEngineDataConnector_DestinationConfigs_Destinations {
        DiscoveryEngineDataConnector_DestinationConfigs_Destinations {
            host: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineDataConnector_DestinationConfigs_DestinationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineDataConnector_DestinationConfigs_DestinationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineDataConnector_DestinationConfigs_DestinationsRef {
        DiscoveryEngineDataConnector_DestinationConfigs_DestinationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineDataConnector_DestinationConfigs_DestinationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `host` after provisioning.\nThe host of the destination, for example\n'https://example.atlassian.net'."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nTarget port number accepted by the destination."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize, Default)]
struct DiscoveryEngineDataConnector_DestinationConfigsDynamic {
    destinations: std
    ::option
    ::Option<terrars::DynamicBlock<DiscoveryEngineDataConnector_DestinationConfigs_Destinations>>,
}

#[derive(Serialize)]
pub struct DiscoveryEngineDataConnector_DestinationConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    params: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destinations: std::option::Option<std::vec::Vec<DiscoveryEngineDataConnector_DestinationConfigs_Destinations>>,
    dynamic: DiscoveryEngineDataConnector_DestinationConfigsDynamic,
}

impl DiscoveryEngineDataConnector_DestinationConfigs {
    #[doc = "Set the field `key`.\nThe key of the destination configuration, for example 'url'."]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `params`.\nAdditional parameters for this destination config in structured json format."]
    pub fn set_params(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.params = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destinations`.\n"]
    pub fn set_destinations(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DiscoveryEngineDataConnector_DestinationConfigs_Destinations>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destinations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destinations = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineDataConnector_DestinationConfigs {
    type O = terrars::BlockAssignable<DiscoveryEngineDataConnector_DestinationConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineDataConnector_DestinationConfigs {}

impl BuildDiscoveryEngineDataConnector_DestinationConfigs {
    pub fn build(self) -> DiscoveryEngineDataConnector_DestinationConfigs {
        DiscoveryEngineDataConnector_DestinationConfigs {
            key: core::default::Default::default(),
            params: core::default::Default::default(),
            destinations: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineDataConnector_DestinationConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineDataConnector_DestinationConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineDataConnector_DestinationConfigsRef {
        DiscoveryEngineDataConnector_DestinationConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineDataConnector_DestinationConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `key` after provisioning.\nThe key of the destination configuration, for example 'url'."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `params` after provisioning.\nAdditional parameters for this destination config in structured json format."]
    pub fn params(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.params", self.base))
    }

    #[doc = "Get a reference to the value of field `destinations` after provisioning.\n"]
    pub fn destinations(&self) -> terrars::ListRef<DiscoveryEngineDataConnector_DestinationConfigs_DestinationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destinations", self.base))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineDataConnector_Entities {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entity_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_property_mappings: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    params: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DiscoveryEngineDataConnector_Entities {
    #[doc =
        "Set the field `entity_name`.\nThe name of the entity. Supported values by data source:\n* Salesforce: 'Lead', 'Opportunity', 'Contact', 'Account', 'Case', 'Contract', 'Campaign'\n* Jira: project, issue, attachment, comment, worklog\n* Confluence: 'Content', 'Space'"]
    pub fn set_entity_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.entity_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `key_property_mappings`.\nAttributes for indexing.\nKey: Field name.\nValue: The key property to map a field to, such as 'title', and\n'description'. Supported key properties:\n* 'title': The title for data record. This would be displayed on search\n  results.\n* 'description': The description for data record. This would be displayed\n  on search results."]
    pub fn set_key_property_mappings(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.key_property_mappings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `params`.\nThe parameters for the entity to facilitate data ingestion."]
    pub fn set_params(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.params = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineDataConnector_Entities {
    type O = terrars::BlockAssignable<DiscoveryEngineDataConnector_Entities>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineDataConnector_Entities {}

impl BuildDiscoveryEngineDataConnector_Entities {
    pub fn build(self) -> DiscoveryEngineDataConnector_Entities {
        DiscoveryEngineDataConnector_Entities {
            entity_name: core::default::Default::default(),
            key_property_mappings: core::default::Default::default(),
            params: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineDataConnector_EntitiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineDataConnector_EntitiesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DiscoveryEngineDataConnector_EntitiesRef {
        DiscoveryEngineDataConnector_EntitiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineDataConnector_EntitiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `data_store` after provisioning.\nThe full resource name of the associated data store for the source\nentity.\nFormat: 'projects/*/locations/*/collections/*/dataStores/*'.\nWhen the connector is initialized by the DataConnectorService.SetUpDataConnector\nmethod, a DataStore is automatically created for each source entity."]
    pub fn data_store(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_store", self.base))
    }

    #[doc =
        "Get a reference to the value of field `entity_name` after provisioning.\nThe name of the entity. Supported values by data source:\n* Salesforce: 'Lead', 'Opportunity', 'Contact', 'Account', 'Case', 'Contract', 'Campaign'\n* Jira: project, issue, attachment, comment, worklog\n* Confluence: 'Content', 'Space'"]
    pub fn entity_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entity_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key_property_mappings` after provisioning.\nAttributes for indexing.\nKey: Field name.\nValue: The key property to map a field to, such as 'title', and\n'description'. Supported key properties:\n* 'title': The title for data record. This would be displayed on search\n  results.\n* 'description': The description for data record. This would be displayed\n  on search results."]
    pub fn key_property_mappings(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.key_property_mappings", self.base))
    }

    #[doc =
        "Get a reference to the value of field `params` after provisioning.\nThe parameters for the entity to facilitate data ingestion."]
    pub fn params(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.params", self.base))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineDataConnector_Metadata {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    author: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    note: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    short_description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    title: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DiscoveryEngineDataConnector_Metadata {
    #[doc =
        "Set the field `author`.\nThe party that authored the connector, e.g. \"Google\" or a third-party provider name."]
    pub fn set_author(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.author = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nHuman-readable description of the connector."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `note`.\nFree-form, multi-line note about the connector's capabilities."]
    pub fn set_note(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.note = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `short_description`.\nShort, subtitle-length description of the connector."]
    pub fn set_short_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.short_description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `title`.\nDisplay title of the connector."]
    pub fn set_title(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.title = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineDataConnector_Metadata {
    type O = terrars::BlockAssignable<DiscoveryEngineDataConnector_Metadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineDataConnector_Metadata {}

impl BuildDiscoveryEngineDataConnector_Metadata {
    pub fn build(self) -> DiscoveryEngineDataConnector_Metadata {
        DiscoveryEngineDataConnector_Metadata {
            author: core::default::Default::default(),
            description: core::default::Default::default(),
            note: core::default::Default::default(),
            short_description: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineDataConnector_MetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineDataConnector_MetadataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DiscoveryEngineDataConnector_MetadataRef {
        DiscoveryEngineDataConnector_MetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineDataConnector_MetadataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `author` after provisioning.\nThe party that authored the connector, e.g. \"Google\" or a third-party provider name."]
    pub fn author(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.author", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nHuman-readable description of the connector."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `note` after provisioning.\nFree-form, multi-line note about the connector's capabilities."]
    pub fn note(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.note", self.base))
    }

    #[doc =
        "Get a reference to the value of field `short_description` after provisioning.\nShort, subtitle-length description of the connector."]
    pub fn short_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.short_description", self.base))
    }

    #[doc = "Get a reference to the value of field `title` after provisioning.\nDisplay title of the connector."]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineDataConnector_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DiscoveryEngineDataConnector_Timeouts {
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

impl terrars::ToListMappable for DiscoveryEngineDataConnector_Timeouts {
    type O = terrars::BlockAssignable<DiscoveryEngineDataConnector_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineDataConnector_Timeouts {}

impl BuildDiscoveryEngineDataConnector_Timeouts {
    pub fn build(self) -> DiscoveryEngineDataConnector_Timeouts {
        DiscoveryEngineDataConnector_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineDataConnector_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineDataConnector_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DiscoveryEngineDataConnector_TimeoutsRef {
        DiscoveryEngineDataConnector_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineDataConnector_TimeoutsRef {
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
