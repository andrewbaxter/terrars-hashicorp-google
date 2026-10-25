#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct CesAppData_ {
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
    app_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    global_instruction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guardrails: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    locked: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pinned: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_agent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool_execution_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_processing_config: std::option::Option<std::vec::Vec<CesApp_AudioProcessingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_certificate_settings: std::option::Option<std::vec::Vec<CesApp_ClientCertificateSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_store_settings: std::option::Option<std::vec::Vec<CesApp_DataStoreSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_channel_profile: std::option::Option<std::vec::Vec<CesApp_DefaultChannelProfile>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_handling_settings: std::option::Option<std::vec::Vec<CesApp_ErrorHandlingSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluation_metrics_thresholds: std::option::Option<std::vec::Vec<CesApp_EvaluationMetricsThresholds>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    language_settings: std::option::Option<std::vec::Vec<CesApp_LanguageSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_settings: std::option::Option<std::vec::Vec<CesApp_LoggingSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_settings: std::option::Option<std::vec::Vec<CesApp_ModelSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_zone_settings: std::option::Option<std::vec::Vec<CesApp_TimeZoneSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<CesApp_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    variable_declarations: std::option::Option<std::vec::Vec<CesApp_VariableDeclarations>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_sc_settings: std::option::Option<std::vec::Vec<CesApp_VpcScSettings>>,
    dynamic: CesAppDynamic,
}

struct CesApp_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CesAppData_>,
}

#[derive(Clone)]
pub struct CesApp(std::rc::Rc<CesApp_>);

impl CesApp {
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

    #[doc = "Set the field `description`.\nHuman-readable description of the app."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `global_instruction`.\nInstructions for all the agents in the app.\nYou can use this instruction to set up a stable identity or personality\nacross all the agents."]
    pub fn set_global_instruction(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().global_instruction = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `guardrails`.\nList of guardrails for the app.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/guardrails/{guardrail}'"]
    pub fn set_guardrails(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().guardrails = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `locked`.\nIndicates whether the app is locked for changes. If the app is locked,\nmodifications to the app resources will be rejected."]
    pub fn set_locked(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().locked = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `metadata`.\nMetadata about the app. This field can be used to store additional\ninformation relevant to the app's details or intended usages."]
    pub fn set_metadata(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pinned`.\nWhether the app is pinned in the app list."]
    pub fn set_pinned(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().pinned = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `root_agent`.\nThe root agent is the entry point of the app.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/agents/{agent}'"]
    pub fn set_root_agent(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().root_agent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tool_execution_mode`.\nThe tool execution mode for the app.\nSee the [API reference](https://docs.cloud.google.com/customer-engagement-ai/conversational-agents/ps/reference/rpc/google.cloud.ces.v1#google.cloud.ces.v1.App.ToolExecutionMode) for more details."]
    pub fn set_tool_execution_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tool_execution_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `audio_processing_config`.\n"]
    pub fn set_audio_processing_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesApp_AudioProcessingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().audio_processing_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.audio_processing_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `client_certificate_settings`.\n"]
    pub fn set_client_certificate_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesApp_ClientCertificateSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().client_certificate_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.client_certificate_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `data_store_settings`.\n"]
    pub fn set_data_store_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesApp_DataStoreSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_store_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_store_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_channel_profile`.\n"]
    pub fn set_default_channel_profile(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesApp_DefaultChannelProfile>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_channel_profile = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_channel_profile = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `error_handling_settings`.\n"]
    pub fn set_error_handling_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesApp_ErrorHandlingSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().error_handling_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.error_handling_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `evaluation_metrics_thresholds`.\n"]
    pub fn set_evaluation_metrics_thresholds(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesApp_EvaluationMetricsThresholds>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().evaluation_metrics_thresholds = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.evaluation_metrics_thresholds = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `language_settings`.\n"]
    pub fn set_language_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesApp_LanguageSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().language_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.language_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `logging_settings`.\n"]
    pub fn set_logging_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesApp_LoggingSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `model_settings`.\n"]
    pub fn set_model_settings(self, v: impl std::convert::Into<terrars::BlockAssignable<CesApp_ModelSettings>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().model_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.model_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `time_zone_settings`.\n"]
    pub fn set_time_zone_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesApp_TimeZoneSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().time_zone_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.time_zone_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<CesApp_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `variable_declarations`.\n"]
    pub fn set_variable_declarations(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesApp_VariableDeclarations>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().variable_declarations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.variable_declarations = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vpc_sc_settings`.\n"]
    pub fn set_vpc_sc_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesApp_VpcScSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_sc_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_sc_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `app_id` after provisioning.\nThe ID to use for the app, which will become the final component of\nthe app's resource name. If not provided, a unique ID will be\nautomatically assigned for the app."]
    pub fn app_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the app was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deployment_count` after provisioning.\nNumber of deployments in the app."]
    pub fn deployment_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nHuman-readable description of the app."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the app."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nEtag used to ensure the object hasn't changed during a read-modify-write\noperation. If the etag is empty, the update will overwrite any concurrent\nchanges."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `global_instruction` after provisioning.\nInstructions for all the agents in the app.\nYou can use this instruction to set up a stable identity or personality\nacross all the agents."]
    pub fn global_instruction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.global_instruction", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `guardrails` after provisioning.\nList of guardrails for the app.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/guardrails/{guardrail}'"]
    pub fn guardrails(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.guardrails", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `locked` after provisioning.\nIndicates whether the app is locked for changes. If the app is locked,\nmodifications to the app resources will be rejected."]
    pub fn locked(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.locked", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `metadata` after provisioning.\nMetadata about the app. This field can be used to store additional\ninformation relevant to the app's details or intended usages."]
    pub fn metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.metadata", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The unique identifier of the app.\nFormat: 'projects/{project}/locations/{location}/apps/{app}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pinned` after provisioning.\nWhether the app is pinned in the app list."]
    pub fn pinned(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pinned", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `root_agent` after provisioning.\nThe root agent is the entry point of the app.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/agents/{agent}'"]
    pub fn root_agent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_agent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tool_execution_mode` after provisioning.\nThe tool execution mode for the app.\nSee the [API reference](https://docs.cloud.google.com/customer-engagement-ai/conversational-agents/ps/reference/rpc/google.cloud.ces.v1#google.cloud.ces.v1.App.ToolExecutionMode) for more details."]
    pub fn tool_execution_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tool_execution_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when the app was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `audio_processing_config` after provisioning.\n"]
    pub fn audio_processing_config(&self) -> terrars::ListRef<CesApp_AudioProcessingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio_processing_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_certificate_settings` after provisioning.\n"]
    pub fn client_certificate_settings(&self) -> terrars::ListRef<CesApp_ClientCertificateSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_certificate_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_store_settings` after provisioning.\n"]
    pub fn data_store_settings(&self) -> terrars::ListRef<CesApp_DataStoreSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_store_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_channel_profile` after provisioning.\n"]
    pub fn default_channel_profile(&self) -> terrars::ListRef<CesApp_DefaultChannelProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_channel_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `error_handling_settings` after provisioning.\n"]
    pub fn error_handling_settings(&self) -> terrars::ListRef<CesApp_ErrorHandlingSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error_handling_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `evaluation_metrics_thresholds` after provisioning.\n"]
    pub fn evaluation_metrics_thresholds(&self) -> terrars::ListRef<CesApp_EvaluationMetricsThresholdsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.evaluation_metrics_thresholds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `language_settings` after provisioning.\n"]
    pub fn language_settings(&self) -> terrars::ListRef<CesApp_LanguageSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.language_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_settings` after provisioning.\n"]
    pub fn logging_settings(&self) -> terrars::ListRef<CesApp_LoggingSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_settings` after provisioning.\n"]
    pub fn model_settings(&self) -> terrars::ListRef<CesApp_ModelSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `time_zone_settings` after provisioning.\n"]
    pub fn time_zone_settings(&self) -> terrars::ListRef<CesApp_TimeZoneSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time_zone_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CesApp_TimeoutsRef {
        CesApp_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `variable_declarations` after provisioning.\n"]
    pub fn variable_declarations(&self) -> terrars::ListRef<CesApp_VariableDeclarationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.variable_declarations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_sc_settings` after provisioning.\n"]
    pub fn vpc_sc_settings(&self) -> terrars::ListRef<CesApp_VpcScSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_sc_settings", self.extract_ref()))
    }
}

impl terrars::Referable for CesApp {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CesApp { }

impl terrars::ToListMappable for CesApp {
    type O = terrars::ListRef<CesAppRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CesApp_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_ces_app".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCesApp {
    pub tf_id: std::string::String,
    #[doc =
        "The ID to use for the app, which will become the final component of\nthe app's resource name. If not provided, a unique ID will be\nautomatically assigned for the app."]
    pub app_id: terrars::PrimField<std::string::String>,
    #[doc = "Display name of the app."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildCesApp {
    pub fn build(self, stack: &mut terrars::Stack) -> CesApp {
        let out = CesApp(std::rc::Rc::new(CesApp_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CesAppData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                app_id: self.app_id,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                global_instruction: core::default::Default::default(),
                guardrails: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                locked: core::default::Default::default(),
                metadata: core::default::Default::default(),
                pinned: core::default::Default::default(),
                project: core::default::Default::default(),
                root_agent: core::default::Default::default(),
                tool_execution_mode: core::default::Default::default(),
                audio_processing_config: core::default::Default::default(),
                client_certificate_settings: core::default::Default::default(),
                data_store_settings: core::default::Default::default(),
                default_channel_profile: core::default::Default::default(),
                error_handling_settings: core::default::Default::default(),
                evaluation_metrics_thresholds: core::default::Default::default(),
                language_settings: core::default::Default::default(),
                logging_settings: core::default::Default::default(),
                model_settings: core::default::Default::default(),
                time_zone_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                variable_declarations: core::default::Default::default(),
                vpc_sc_settings: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CesAppRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CesAppRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `app_id` after provisioning.\nThe ID to use for the app, which will become the final component of\nthe app's resource name. If not provided, a unique ID will be\nautomatically assigned for the app."]
    pub fn app_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the app was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deployment_count` after provisioning.\nNumber of deployments in the app."]
    pub fn deployment_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nHuman-readable description of the app."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the app."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nEtag used to ensure the object hasn't changed during a read-modify-write\noperation. If the etag is empty, the update will overwrite any concurrent\nchanges."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `global_instruction` after provisioning.\nInstructions for all the agents in the app.\nYou can use this instruction to set up a stable identity or personality\nacross all the agents."]
    pub fn global_instruction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.global_instruction", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `guardrails` after provisioning.\nList of guardrails for the app.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/guardrails/{guardrail}'"]
    pub fn guardrails(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.guardrails", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `locked` after provisioning.\nIndicates whether the app is locked for changes. If the app is locked,\nmodifications to the app resources will be rejected."]
    pub fn locked(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.locked", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `metadata` after provisioning.\nMetadata about the app. This field can be used to store additional\ninformation relevant to the app's details or intended usages."]
    pub fn metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.metadata", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The unique identifier of the app.\nFormat: 'projects/{project}/locations/{location}/apps/{app}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `pinned` after provisioning.\nWhether the app is pinned in the app list."]
    pub fn pinned(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pinned", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `root_agent` after provisioning.\nThe root agent is the entry point of the app.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/agents/{agent}'"]
    pub fn root_agent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_agent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tool_execution_mode` after provisioning.\nThe tool execution mode for the app.\nSee the [API reference](https://docs.cloud.google.com/customer-engagement-ai/conversational-agents/ps/reference/rpc/google.cloud.ces.v1#google.cloud.ces.v1.App.ToolExecutionMode) for more details."]
    pub fn tool_execution_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tool_execution_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when the app was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `audio_processing_config` after provisioning.\n"]
    pub fn audio_processing_config(&self) -> terrars::ListRef<CesApp_AudioProcessingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio_processing_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_certificate_settings` after provisioning.\n"]
    pub fn client_certificate_settings(&self) -> terrars::ListRef<CesApp_ClientCertificateSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_certificate_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_store_settings` after provisioning.\n"]
    pub fn data_store_settings(&self) -> terrars::ListRef<CesApp_DataStoreSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_store_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_channel_profile` after provisioning.\n"]
    pub fn default_channel_profile(&self) -> terrars::ListRef<CesApp_DefaultChannelProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_channel_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `error_handling_settings` after provisioning.\n"]
    pub fn error_handling_settings(&self) -> terrars::ListRef<CesApp_ErrorHandlingSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error_handling_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `evaluation_metrics_thresholds` after provisioning.\n"]
    pub fn evaluation_metrics_thresholds(&self) -> terrars::ListRef<CesApp_EvaluationMetricsThresholdsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.evaluation_metrics_thresholds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `language_settings` after provisioning.\n"]
    pub fn language_settings(&self) -> terrars::ListRef<CesApp_LanguageSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.language_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_settings` after provisioning.\n"]
    pub fn logging_settings(&self) -> terrars::ListRef<CesApp_LoggingSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model_settings` after provisioning.\n"]
    pub fn model_settings(&self) -> terrars::ListRef<CesApp_ModelSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `time_zone_settings` after provisioning.\n"]
    pub fn time_zone_settings(&self) -> terrars::ListRef<CesApp_TimeZoneSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time_zone_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CesApp_TimeoutsRef {
        CesApp_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `variable_declarations` after provisioning.\n"]
    pub fn variable_declarations(&self) -> terrars::ListRef<CesApp_VariableDeclarationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.variable_declarations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_sc_settings` after provisioning.\n"]
    pub fn vpc_sc_settings(&self) -> terrars::ListRef<CesApp_VpcScSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_sc_settings", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CesAppDynamic {
    audio_processing_config: std::option::Option<terrars::DynamicBlock<CesApp_AudioProcessingConfig>>,
    client_certificate_settings: std::option::Option<terrars::DynamicBlock<CesApp_ClientCertificateSettings>>,
    data_store_settings: std::option::Option<terrars::DynamicBlock<CesApp_DataStoreSettings>>,
    default_channel_profile: std::option::Option<terrars::DynamicBlock<CesApp_DefaultChannelProfile>>,
    error_handling_settings: std::option::Option<terrars::DynamicBlock<CesApp_ErrorHandlingSettings>>,
    evaluation_metrics_thresholds: std::option::Option<terrars::DynamicBlock<CesApp_EvaluationMetricsThresholds>>,
    language_settings: std::option::Option<terrars::DynamicBlock<CesApp_LanguageSettings>>,
    logging_settings: std::option::Option<terrars::DynamicBlock<CesApp_LoggingSettings>>,
    model_settings: std::option::Option<terrars::DynamicBlock<CesApp_ModelSettings>>,
    time_zone_settings: std::option::Option<terrars::DynamicBlock<CesApp_TimeZoneSettings>>,
    variable_declarations: std::option::Option<terrars::DynamicBlock<CesApp_VariableDeclarations>>,
    vpc_sc_settings: std::option::Option<terrars::DynamicBlock<CesApp_VpcScSettings>>,
}

#[derive(Serialize)]
pub struct CesApp_AudioProcessingConfig_AmbientSoundConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prebuilt_ambient_sound: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_gain_db: std::option::Option<terrars::PrimField<f64>>,
}

impl CesApp_AudioProcessingConfig_AmbientSoundConfig {
    #[doc =
        "Set the field `gcs_uri`.\nAmbient noise as a mono-channel, 16kHz WAV file stored in [Cloud\nStorage](https://cloud.google.com/storage).\nNote: Please make sure the CES service agent\n'service-@gcp-sa-ces.iam.gserviceaccount.com' has\n'storage.objects.get' permission to the Cloud Storage object."]
    pub fn set_gcs_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gcs_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `prebuilt_ambient_sound`.\nName of the prebuilt ambient sound.\nValid values are: - \"coffee_shop\" - \"keyboard\" - \"keypad\" - \"hum\"\n-\"office_1\" - \"office_2\" - \"office_3\"\n-\"room_1\" - \"room_2\" - \"room_3\"\n-\"room_4\" - \"room_5\" - \"air_conditioner\""]
    pub fn set_prebuilt_ambient_sound(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.prebuilt_ambient_sound = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `volume_gain_db`.\nVolume gain (in dB) of the normal native volume supported by\nambient noise, in the range [-96.0, 16.0]. If unset, or set to a value of\n0.0 (dB), will play at normal native signal amplitude. A value of -6.0 (dB)\nwill play at approximately half the amplitude of the normal native signal\namplitude. A value of +6.0 (dB) will play at approximately twice the\namplitude of the normal native signal amplitude. We strongly recommend not\nto exceed +10 (dB) as there's usually no effective increase in loudness for\nany value greater than that."]
    pub fn set_volume_gain_db(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.volume_gain_db = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesApp_AudioProcessingConfig_AmbientSoundConfig {
    type O = terrars::BlockAssignable<CesApp_AudioProcessingConfig_AmbientSoundConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesApp_AudioProcessingConfig_AmbientSoundConfig {}

impl BuildCesApp_AudioProcessingConfig_AmbientSoundConfig {
    pub fn build(self) -> CesApp_AudioProcessingConfig_AmbientSoundConfig {
        CesApp_AudioProcessingConfig_AmbientSoundConfig {
            gcs_uri: core::default::Default::default(),
            prebuilt_ambient_sound: core::default::Default::default(),
            volume_gain_db: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesApp_AudioProcessingConfig_AmbientSoundConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesApp_AudioProcessingConfig_AmbientSoundConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesApp_AudioProcessingConfig_AmbientSoundConfigRef {
        CesApp_AudioProcessingConfig_AmbientSoundConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesApp_AudioProcessingConfig_AmbientSoundConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `gcs_uri` after provisioning.\nAmbient noise as a mono-channel, 16kHz WAV file stored in [Cloud\nStorage](https://cloud.google.com/storage).\nNote: Please make sure the CES service agent\n'service-@gcp-sa-ces.iam.gserviceaccount.com' has\n'storage.objects.get' permission to the Cloud Storage object."]
    pub fn gcs_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcs_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prebuilt_ambient_sound` after provisioning.\nName of the prebuilt ambient sound.\nValid values are: - \"coffee_shop\" - \"keyboard\" - \"keypad\" - \"hum\"\n-\"office_1\" - \"office_2\" - \"office_3\"\n-\"room_1\" - \"room_2\" - \"room_3\"\n-\"room_4\" - \"room_5\" - \"air_conditioner\""]
    pub fn prebuilt_ambient_sound(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prebuilt_ambient_sound", self.base))
    }

    #[doc =
        "Get a reference to the value of field `volume_gain_db` after provisioning.\nVolume gain (in dB) of the normal native volume supported by\nambient noise, in the range [-96.0, 16.0]. If unset, or set to a value of\n0.0 (dB), will play at normal native signal amplitude. A value of -6.0 (dB)\nwill play at approximately half the amplitude of the normal native signal\namplitude. A value of +6.0 (dB) will play at approximately twice the\namplitude of the normal native signal amplitude. We strongly recommend not\nto exceed +10 (dB) as there's usually no effective increase in loudness for\nany value greater than that."]
    pub fn volume_gain_db(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_gain_db", self.base))
    }
}

#[derive(Serialize)]
pub struct CesApp_AudioProcessingConfig_BargeInConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    barge_in_awareness: std::option::Option<terrars::PrimField<bool>>,
}

impl CesApp_AudioProcessingConfig_BargeInConfig {
    #[doc =
        "Set the field `barge_in_awareness`.\nIf enabled, the agent will adapt its next response based on the assumption\nthat the user hasn't heard the full preceding agent message.\nThis should not be used in scenarios where agent responses are displayed\nvisually."]
    pub fn set_barge_in_awareness(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.barge_in_awareness = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesApp_AudioProcessingConfig_BargeInConfig {
    type O = terrars::BlockAssignable<CesApp_AudioProcessingConfig_BargeInConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesApp_AudioProcessingConfig_BargeInConfig {}

impl BuildCesApp_AudioProcessingConfig_BargeInConfig {
    pub fn build(self) -> CesApp_AudioProcessingConfig_BargeInConfig {
        CesApp_AudioProcessingConfig_BargeInConfig { barge_in_awareness: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CesApp_AudioProcessingConfig_BargeInConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesApp_AudioProcessingConfig_BargeInConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesApp_AudioProcessingConfig_BargeInConfigRef {
        CesApp_AudioProcessingConfig_BargeInConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesApp_AudioProcessingConfig_BargeInConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `barge_in_awareness` after provisioning.\nIf enabled, the agent will adapt its next response based on the assumption\nthat the user hasn't heard the full preceding agent message.\nThis should not be used in scenarios where agent responses are displayed\nvisually."]
    pub fn barge_in_awareness(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.barge_in_awareness", self.base))
    }
}

#[derive(Serialize)]
pub struct CesApp_AudioProcessingConfig_SynthesizeSpeechConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instruction: std::option::Option<terrars::PrimField<std::string::String>>,
    language_code: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    speaking_rate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    voice: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesApp_AudioProcessingConfig_SynthesizeSpeechConfigs {
    #[doc = "Set the field `instruction`.\nThe instruction used to synthesize speech when using a generative model."]
    pub fn set_instruction(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instruction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model`.\nThe model used to synthesize audio."]
    pub fn set_model(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `speaking_rate`.\nThe speaking rate/speed in the range [0.25, 2.0]. 1.0 is the normal native\nspeed supported by the specific voice. 2.0 is twice as fast, and 0.5 is\nhalf as fast. Values outside of the range [0.25, 2.0] will return an error."]
    pub fn set_speaking_rate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.speaking_rate = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `voice`.\nThe name of the voice. If not set, the service will choose a\nvoice based on the other parameters such as language_code.\nFor the list of available voices, please refer to Supported voices and\nlanguages from Cloud Text-to-Speech."]
    pub fn set_voice(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.voice = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesApp_AudioProcessingConfig_SynthesizeSpeechConfigs {
    type O = terrars::BlockAssignable<CesApp_AudioProcessingConfig_SynthesizeSpeechConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesApp_AudioProcessingConfig_SynthesizeSpeechConfigs {
    #[doc = ""]
    pub language_code: terrars::PrimField<std::string::String>,
}

impl BuildCesApp_AudioProcessingConfig_SynthesizeSpeechConfigs {
    pub fn build(self) -> CesApp_AudioProcessingConfig_SynthesizeSpeechConfigs {
        CesApp_AudioProcessingConfig_SynthesizeSpeechConfigs {
            instruction: core::default::Default::default(),
            language_code: self.language_code,
            model: core::default::Default::default(),
            speaking_rate: core::default::Default::default(),
            voice: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesApp_AudioProcessingConfig_SynthesizeSpeechConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesApp_AudioProcessingConfig_SynthesizeSpeechConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesApp_AudioProcessingConfig_SynthesizeSpeechConfigsRef {
        CesApp_AudioProcessingConfig_SynthesizeSpeechConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesApp_AudioProcessingConfig_SynthesizeSpeechConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `instruction` after provisioning.\nThe instruction used to synthesize speech when using a generative model."]
    pub fn instruction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instruction", self.base))
    }

    #[doc = "Get a reference to the value of field `language_code` after provisioning.\n"]
    pub fn language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_code", self.base))
    }

    #[doc = "Get a reference to the value of field `model` after provisioning.\nThe model used to synthesize audio."]
    pub fn model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model", self.base))
    }

    #[doc =
        "Get a reference to the value of field `speaking_rate` after provisioning.\nThe speaking rate/speed in the range [0.25, 2.0]. 1.0 is the normal native\nspeed supported by the specific voice. 2.0 is twice as fast, and 0.5 is\nhalf as fast. Values outside of the range [0.25, 2.0] will return an error."]
    pub fn speaking_rate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.speaking_rate", self.base))
    }

    #[doc =
        "Get a reference to the value of field `voice` after provisioning.\nThe name of the voice. If not set, the service will choose a\nvoice based on the other parameters such as language_code.\nFor the list of available voices, please refer to Supported voices and\nlanguages from Cloud Text-to-Speech."]
    pub fn voice(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.voice", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesApp_AudioProcessingConfigDynamic {
    ambient_sound_config: std::option::Option<terrars::DynamicBlock<CesApp_AudioProcessingConfig_AmbientSoundConfig>>,
    barge_in_config: std::option::Option<terrars::DynamicBlock<CesApp_AudioProcessingConfig_BargeInConfig>>,
    synthesize_speech_configs: std
    ::option
    ::Option<terrars::DynamicBlock<CesApp_AudioProcessingConfig_SynthesizeSpeechConfigs>>,
}

#[derive(Serialize)]
pub struct CesApp_AudioProcessingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inactivity_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ambient_sound_config: std::option::Option<std::vec::Vec<CesApp_AudioProcessingConfig_AmbientSoundConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    barge_in_config: std::option::Option<std::vec::Vec<CesApp_AudioProcessingConfig_BargeInConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    synthesize_speech_configs: std::option::Option<std::vec::Vec<CesApp_AudioProcessingConfig_SynthesizeSpeechConfigs>>,
    dynamic: CesApp_AudioProcessingConfigDynamic,
}

impl CesApp_AudioProcessingConfig {
    #[doc =
        "Set the field `inactivity_timeout`.\nThe duration of user inactivity (no speech or interaction) before the agent\nprompts the user for reengagement. If not set, the agent will not prompt\nthe user for reengagement."]
    pub fn set_inactivity_timeout(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.inactivity_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ambient_sound_config`.\n"]
    pub fn set_ambient_sound_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesApp_AudioProcessingConfig_AmbientSoundConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ambient_sound_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ambient_sound_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `barge_in_config`.\n"]
    pub fn set_barge_in_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesApp_AudioProcessingConfig_BargeInConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.barge_in_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.barge_in_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `synthesize_speech_configs`.\n"]
    pub fn set_synthesize_speech_configs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesApp_AudioProcessingConfig_SynthesizeSpeechConfigs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.synthesize_speech_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.synthesize_speech_configs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesApp_AudioProcessingConfig {
    type O = terrars::BlockAssignable<CesApp_AudioProcessingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesApp_AudioProcessingConfig {}

impl BuildCesApp_AudioProcessingConfig {
    pub fn build(self) -> CesApp_AudioProcessingConfig {
        CesApp_AudioProcessingConfig {
            inactivity_timeout: core::default::Default::default(),
            ambient_sound_config: core::default::Default::default(),
            barge_in_config: core::default::Default::default(),
            synthesize_speech_configs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesApp_AudioProcessingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesApp_AudioProcessingConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesApp_AudioProcessingConfigRef {
        CesApp_AudioProcessingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesApp_AudioProcessingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `inactivity_timeout` after provisioning.\nThe duration of user inactivity (no speech or interaction) before the agent\nprompts the user for reengagement. If not set, the agent will not prompt\nthe user for reengagement."]
    pub fn inactivity_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inactivity_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `ambient_sound_config` after provisioning.\n"]
    pub fn ambient_sound_config(&self) -> terrars::ListRef<CesApp_AudioProcessingConfig_AmbientSoundConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ambient_sound_config", self.base))
    }

    #[doc = "Get a reference to the value of field `barge_in_config` after provisioning.\n"]
    pub fn barge_in_config(&self) -> terrars::ListRef<CesApp_AudioProcessingConfig_BargeInConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.barge_in_config", self.base))
    }
}

#[derive(Serialize)]
pub struct CesApp_ClientCertificateSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    passphrase: std::option::Option<terrars::PrimField<std::string::String>>,
    private_key: terrars::PrimField<std::string::String>,
    tls_certificate: terrars::PrimField<std::string::String>,
}

impl CesApp_ClientCertificateSettings {
    #[doc =
        "Set the field `passphrase`.\nThe passphrase to decrypt the private key.\nShould be left unset if the private key is not encrypted."]
    pub fn set_passphrase(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.passphrase = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesApp_ClientCertificateSettings {
    type O = terrars::BlockAssignable<CesApp_ClientCertificateSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesApp_ClientCertificateSettings {
    #[doc =
        "The name of the SecretManager secret version resource\nstoring the private key encoded in PEM format.\nFormat: projects/{project}/secrets/{secret}/versions/{version}"]
    pub private_key: terrars::PrimField<std::string::String>,
    #[doc =
        "The TLS certificate encoded in PEM format.\nThis string must include the begin header and end footer lines."]
    pub tls_certificate: terrars::PrimField<std::string::String>,
}

impl BuildCesApp_ClientCertificateSettings {
    pub fn build(self) -> CesApp_ClientCertificateSettings {
        CesApp_ClientCertificateSettings {
            passphrase: core::default::Default::default(),
            private_key: self.private_key,
            tls_certificate: self.tls_certificate,
        }
    }
}

#[allow(dead_code)]
pub struct CesApp_ClientCertificateSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesApp_ClientCertificateSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesApp_ClientCertificateSettingsRef {
        CesApp_ClientCertificateSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesApp_ClientCertificateSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `passphrase` after provisioning.\nThe passphrase to decrypt the private key.\nShould be left unset if the private key is not encrypted."]
    pub fn passphrase(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.passphrase", self.base))
    }

    #[doc =
        "Get a reference to the value of field `private_key` after provisioning.\nThe name of the SecretManager secret version resource\nstoring the private key encoded in PEM format.\nFormat: projects/{project}/secrets/{secret}/versions/{version}"]
    pub fn private_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tls_certificate` after provisioning.\nThe TLS certificate encoded in PEM format.\nThis string must include the begin header and end footer lines."]
    pub fn tls_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls_certificate", self.base))
    }
}

#[derive(Serialize)]
pub struct CesApp_DataStoreSettings_Engines {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesApp_DataStoreSettings_Engines {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesApp_DataStoreSettings_Engines {
    type O = terrars::BlockAssignable<CesApp_DataStoreSettings_Engines>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesApp_DataStoreSettings_Engines {}

impl BuildCesApp_DataStoreSettings_Engines {
    pub fn build(self) -> CesApp_DataStoreSettings_Engines {
        CesApp_DataStoreSettings_Engines {
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesApp_DataStoreSettings_EnginesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesApp_DataStoreSettings_EnginesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesApp_DataStoreSettings_EnginesRef {
        CesApp_DataStoreSettings_EnginesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesApp_DataStoreSettings_EnginesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct CesApp_DataStoreSettings {}

impl CesApp_DataStoreSettings { }

impl terrars::ToListMappable for CesApp_DataStoreSettings {
    type O = terrars::BlockAssignable<CesApp_DataStoreSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesApp_DataStoreSettings {}

impl BuildCesApp_DataStoreSettings {
    pub fn build(self) -> CesApp_DataStoreSettings {
        CesApp_DataStoreSettings {}
    }
}

#[allow(dead_code)]
pub struct CesApp_DataStoreSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesApp_DataStoreSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesApp_DataStoreSettingsRef {
        CesApp_DataStoreSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesApp_DataStoreSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `engines` after provisioning.\nThe engines for the app."]
    pub fn engines(&self) -> terrars::ListRef<CesApp_DataStoreSettings_EnginesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.engines", self.base))
    }
}

#[derive(Serialize)]
pub struct CesApp_DefaultChannelProfile_PersonaProperty {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    persona: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesApp_DefaultChannelProfile_PersonaProperty {
    #[doc = "Set the field `persona`.\nThe persona of the channel.\nPossible values:\nUNKNOWN\nCONCISE\nCHATTY"]
    pub fn set_persona(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.persona = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesApp_DefaultChannelProfile_PersonaProperty {
    type O = terrars::BlockAssignable<CesApp_DefaultChannelProfile_PersonaProperty>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesApp_DefaultChannelProfile_PersonaProperty {}

impl BuildCesApp_DefaultChannelProfile_PersonaProperty {
    pub fn build(self) -> CesApp_DefaultChannelProfile_PersonaProperty {
        CesApp_DefaultChannelProfile_PersonaProperty { persona: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CesApp_DefaultChannelProfile_PersonaPropertyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesApp_DefaultChannelProfile_PersonaPropertyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesApp_DefaultChannelProfile_PersonaPropertyRef {
        CesApp_DefaultChannelProfile_PersonaPropertyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesApp_DefaultChannelProfile_PersonaPropertyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `persona` after provisioning.\nThe persona of the channel.\nPossible values:\nUNKNOWN\nCONCISE\nCHATTY"]
    pub fn persona(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.persona", self.base))
    }
}

#[derive(Serialize)]
pub struct CesApp_DefaultChannelProfile_WebWidgetConfig_SecuritySettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_origins: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_origin_check: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_public_access: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_recaptcha: std::option::Option<terrars::PrimField<bool>>,
}

impl CesApp_DefaultChannelProfile_WebWidgetConfig_SecuritySettings {
    #[doc =
        "Set the field `allowed_origins`.\nThe origins that are allowed to host the web widget. An origin is\ndefined by RFC 6454. If empty, all origins are allowed.\nA maximum of 100 origins is allowed.\nExample: \"https://example.com\""]
    pub fn set_allowed_origins(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_origins = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_origin_check`.\nIndicates whether origin check for the web widget is enabled.\nIf 'true', the web widget will check the origin of the website that\nloads the web widget and only allow it to be loaded in the same origin\nor any of the allowed origins."]
    pub fn set_enable_origin_check(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_origin_check = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_public_access`.\nIndicates whether public access to the web widget is enabled.\nIf 'true', the web widget will be publicly accessible.\nIf 'false', the web widget must be integrated with your own\nauthentication and authorization system to return valid credentials for\naccessing the CES agent."]
    pub fn set_enable_public_access(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_public_access = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_recaptcha`.\nIndicates whether reCAPTCHA verification for the web widget is enabled."]
    pub fn set_enable_recaptcha(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_recaptcha = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesApp_DefaultChannelProfile_WebWidgetConfig_SecuritySettings {
    type O = terrars::BlockAssignable<CesApp_DefaultChannelProfile_WebWidgetConfig_SecuritySettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesApp_DefaultChannelProfile_WebWidgetConfig_SecuritySettings {}

impl BuildCesApp_DefaultChannelProfile_WebWidgetConfig_SecuritySettings {
    pub fn build(self) -> CesApp_DefaultChannelProfile_WebWidgetConfig_SecuritySettings {
        CesApp_DefaultChannelProfile_WebWidgetConfig_SecuritySettings {
            allowed_origins: core::default::Default::default(),
            enable_origin_check: core::default::Default::default(),
            enable_public_access: core::default::Default::default(),
            enable_recaptcha: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesApp_DefaultChannelProfile_WebWidgetConfig_SecuritySettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesApp_DefaultChannelProfile_WebWidgetConfig_SecuritySettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesApp_DefaultChannelProfile_WebWidgetConfig_SecuritySettingsRef {
        CesApp_DefaultChannelProfile_WebWidgetConfig_SecuritySettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesApp_DefaultChannelProfile_WebWidgetConfig_SecuritySettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_origins` after provisioning.\nThe origins that are allowed to host the web widget. An origin is\ndefined by RFC 6454. If empty, all origins are allowed.\nA maximum of 100 origins is allowed.\nExample: \"https://example.com\""]
    pub fn allowed_origins(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_origins", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_origin_check` after provisioning.\nIndicates whether origin check for the web widget is enabled.\nIf 'true', the web widget will check the origin of the website that\nloads the web widget and only allow it to be loaded in the same origin\nor any of the allowed origins."]
    pub fn enable_origin_check(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_origin_check", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_public_access` after provisioning.\nIndicates whether public access to the web widget is enabled.\nIf 'true', the web widget will be publicly accessible.\nIf 'false', the web widget must be integrated with your own\nauthentication and authorization system to return valid credentials for\naccessing the CES agent."]
    pub fn enable_public_access(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_public_access", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_recaptcha` after provisioning.\nIndicates whether reCAPTCHA verification for the web widget is enabled."]
    pub fn enable_recaptcha(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_recaptcha", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesApp_DefaultChannelProfile_WebWidgetConfigDynamic {
    security_settings: std
    ::option
    ::Option<terrars::DynamicBlock<CesApp_DefaultChannelProfile_WebWidgetConfig_SecuritySettings>>,
}

#[derive(Serialize)]
pub struct CesApp_DefaultChannelProfile_WebWidgetConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    modality: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    theme: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_widget_title: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_settings: std::option::Option<std::vec::Vec<CesApp_DefaultChannelProfile_WebWidgetConfig_SecuritySettings>>,
    dynamic: CesApp_DefaultChannelProfile_WebWidgetConfigDynamic,
}

impl CesApp_DefaultChannelProfile_WebWidgetConfig {
    #[doc =
        "Set the field `modality`.\nThe modality of the web widget.\nPossible values:\nUNKNOWN_MODALITY\nCHAT_AND_VOICE\nVOICE_ONLY\nCHAT_ONLY"]
    pub fn set_modality(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.modality = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `theme`.\nThe theme of the web widget.\nPossible values:\nUNKNOWN_THEME\nLIGHT\nDARK"]
    pub fn set_theme(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.theme = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `web_widget_title`.\nThe title of the web widget."]
    pub fn set_web_widget_title(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.web_widget_title = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_settings`.\n"]
    pub fn set_security_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CesApp_DefaultChannelProfile_WebWidgetConfig_SecuritySettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.security_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.security_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesApp_DefaultChannelProfile_WebWidgetConfig {
    type O = terrars::BlockAssignable<CesApp_DefaultChannelProfile_WebWidgetConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesApp_DefaultChannelProfile_WebWidgetConfig {}

impl BuildCesApp_DefaultChannelProfile_WebWidgetConfig {
    pub fn build(self) -> CesApp_DefaultChannelProfile_WebWidgetConfig {
        CesApp_DefaultChannelProfile_WebWidgetConfig {
            modality: core::default::Default::default(),
            theme: core::default::Default::default(),
            web_widget_title: core::default::Default::default(),
            security_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesApp_DefaultChannelProfile_WebWidgetConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesApp_DefaultChannelProfile_WebWidgetConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesApp_DefaultChannelProfile_WebWidgetConfigRef {
        CesApp_DefaultChannelProfile_WebWidgetConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesApp_DefaultChannelProfile_WebWidgetConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `modality` after provisioning.\nThe modality of the web widget.\nPossible values:\nUNKNOWN_MODALITY\nCHAT_AND_VOICE\nVOICE_ONLY\nCHAT_ONLY"]
    pub fn modality(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modality", self.base))
    }

    #[doc =
        "Get a reference to the value of field `theme` after provisioning.\nThe theme of the web widget.\nPossible values:\nUNKNOWN_THEME\nLIGHT\nDARK"]
    pub fn theme(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.theme", self.base))
    }

    #[doc =
        "Get a reference to the value of field `web_widget_title` after provisioning.\nThe title of the web widget."]
    pub fn web_widget_title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.web_widget_title", self.base))
    }

    #[doc = "Get a reference to the value of field `security_settings` after provisioning.\n"]
    pub fn security_settings(
        &self,
    ) -> terrars::ListRef<CesApp_DefaultChannelProfile_WebWidgetConfig_SecuritySettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct CesApp_DefaultChannelProfile_WhatsappConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phone_number: std::option::Option<terrars::PrimField<std::string::String>>,
    phone_number_id: terrars::PrimField<std::string::String>,
    waba_id: terrars::PrimField<std::string::String>,
}

impl CesApp_DefaultChannelProfile_WhatsappConfig {
    #[doc = "Set the field `phone_number`.\nThe phone number in E.164 format."]
    pub fn set_phone_number(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.phone_number = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesApp_DefaultChannelProfile_WhatsappConfig {
    type O = terrars::BlockAssignable<CesApp_DefaultChannelProfile_WhatsappConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesApp_DefaultChannelProfile_WhatsappConfig {
    #[doc = "The Meta phone number ID."]
    pub phone_number_id: terrars::PrimField<std::string::String>,
    #[doc = "The WhatsApp Business Account ID."]
    pub waba_id: terrars::PrimField<std::string::String>,
}

impl BuildCesApp_DefaultChannelProfile_WhatsappConfig {
    pub fn build(self) -> CesApp_DefaultChannelProfile_WhatsappConfig {
        CesApp_DefaultChannelProfile_WhatsappConfig {
            phone_number: core::default::Default::default(),
            phone_number_id: self.phone_number_id,
            waba_id: self.waba_id,
        }
    }
}

#[allow(dead_code)]
pub struct CesApp_DefaultChannelProfile_WhatsappConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesApp_DefaultChannelProfile_WhatsappConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesApp_DefaultChannelProfile_WhatsappConfigRef {
        CesApp_DefaultChannelProfile_WhatsappConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesApp_DefaultChannelProfile_WhatsappConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of the Meta business page or profile."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe fetched Meta business page name."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `phone_number` after provisioning.\nThe phone number in E.164 format."]
    pub fn phone_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }

    #[doc = "Get a reference to the value of field `phone_number_id` after provisioning.\nThe Meta phone number ID."]
    pub fn phone_number_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phone_number_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `thumbnail_url` after provisioning.\nThe fetched Meta business profile thumbnail URL."]
    pub fn thumbnail_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.thumbnail_url", self.base))
    }

    #[doc = "Get a reference to the value of field `waba_id` after provisioning.\nThe WhatsApp Business Account ID."]
    pub fn waba_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.waba_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesApp_DefaultChannelProfileDynamic {
    persona_property: std::option::Option<terrars::DynamicBlock<CesApp_DefaultChannelProfile_PersonaProperty>>,
    web_widget_config: std::option::Option<terrars::DynamicBlock<CesApp_DefaultChannelProfile_WebWidgetConfig>>,
    whatsapp_config: std::option::Option<terrars::DynamicBlock<CesApp_DefaultChannelProfile_WhatsappConfig>>,
}

#[derive(Serialize)]
pub struct CesApp_DefaultChannelProfile {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    channel_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_barge_in_control: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_dtmf: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    profile_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    persona_property: std::option::Option<std::vec::Vec<CesApp_DefaultChannelProfile_PersonaProperty>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_widget_config: std::option::Option<std::vec::Vec<CesApp_DefaultChannelProfile_WebWidgetConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    whatsapp_config: std::option::Option<std::vec::Vec<CesApp_DefaultChannelProfile_WhatsappConfig>>,
    dynamic: CesApp_DefaultChannelProfileDynamic,
}

impl CesApp_DefaultChannelProfile {
    #[doc =
        "Set the field `channel_type`.\nThe type of the channel profile.\nPossible values:\nUNKNOWN\nWEB_UI\nAPI\nTWILIO\nGOOGLE_TELEPHONY_PLATFORM\nCONTACT_CENTER_AS_A_SERVICE"]
    pub fn set_channel_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.channel_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disable_barge_in_control`.\nWhether to disable user barge-in in the conversation.\n- true: User interruptions are disabled while the agent is speaking.\n- false: The agent retains automatic control over when the user can interrupt."]
    pub fn set_disable_barge_in_control(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_barge_in_control = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_dtmf`.\nWhether to disable DTMF (dual-tone multi-frequency)."]
    pub fn set_disable_dtmf(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_dtmf = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `profile_id`.\nThe unique identifier of the channel profile."]
    pub fn set_profile_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.profile_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `persona_property`.\n"]
    pub fn set_persona_property(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesApp_DefaultChannelProfile_PersonaProperty>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.persona_property = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.persona_property = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `web_widget_config`.\n"]
    pub fn set_web_widget_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesApp_DefaultChannelProfile_WebWidgetConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.web_widget_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.web_widget_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `whatsapp_config`.\n"]
    pub fn set_whatsapp_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesApp_DefaultChannelProfile_WhatsappConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.whatsapp_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.whatsapp_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesApp_DefaultChannelProfile {
    type O = terrars::BlockAssignable<CesApp_DefaultChannelProfile>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesApp_DefaultChannelProfile {}

impl BuildCesApp_DefaultChannelProfile {
    pub fn build(self) -> CesApp_DefaultChannelProfile {
        CesApp_DefaultChannelProfile {
            channel_type: core::default::Default::default(),
            disable_barge_in_control: core::default::Default::default(),
            disable_dtmf: core::default::Default::default(),
            profile_id: core::default::Default::default(),
            persona_property: core::default::Default::default(),
            web_widget_config: core::default::Default::default(),
            whatsapp_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesApp_DefaultChannelProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesApp_DefaultChannelProfileRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesApp_DefaultChannelProfileRef {
        CesApp_DefaultChannelProfileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesApp_DefaultChannelProfileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `channel_type` after provisioning.\nThe type of the channel profile.\nPossible values:\nUNKNOWN\nWEB_UI\nAPI\nTWILIO\nGOOGLE_TELEPHONY_PLATFORM\nCONTACT_CENTER_AS_A_SERVICE"]
    pub fn channel_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disable_barge_in_control` after provisioning.\nWhether to disable user barge-in in the conversation.\n- true: User interruptions are disabled while the agent is speaking.\n- false: The agent retains automatic control over when the user can interrupt."]
    pub fn disable_barge_in_control(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_barge_in_control", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disable_dtmf` after provisioning.\nWhether to disable DTMF (dual-tone multi-frequency)."]
    pub fn disable_dtmf(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_dtmf", self.base))
    }

    #[doc =
        "Get a reference to the value of field `profile_id` after provisioning.\nThe unique identifier of the channel profile."]
    pub fn profile_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.profile_id", self.base))
    }

    #[doc = "Get a reference to the value of field `persona_property` after provisioning.\n"]
    pub fn persona_property(&self) -> terrars::ListRef<CesApp_DefaultChannelProfile_PersonaPropertyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.persona_property", self.base))
    }

    #[doc = "Get a reference to the value of field `web_widget_config` after provisioning.\n"]
    pub fn web_widget_config(&self) -> terrars::ListRef<CesApp_DefaultChannelProfile_WebWidgetConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.web_widget_config", self.base))
    }

    #[doc = "Get a reference to the value of field `whatsapp_config` after provisioning.\n"]
    pub fn whatsapp_config(&self) -> terrars::ListRef<CesApp_DefaultChannelProfile_WhatsappConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.whatsapp_config", self.base))
    }
}

#[derive(Serialize)]
pub struct CesApp_ErrorHandlingSettings_EndSessionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    escalate_session: std::option::Option<terrars::PrimField<bool>>,
}

impl CesApp_ErrorHandlingSettings_EndSessionConfig {
    #[doc =
        "Set the field `escalate_session`.\nWhether to escalate the session in EndSession. If session is escalated,\nmetadata in EndSession will contain session_escalated = true."]
    pub fn set_escalate_session(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.escalate_session = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesApp_ErrorHandlingSettings_EndSessionConfig {
    type O = terrars::BlockAssignable<CesApp_ErrorHandlingSettings_EndSessionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesApp_ErrorHandlingSettings_EndSessionConfig {}

impl BuildCesApp_ErrorHandlingSettings_EndSessionConfig {
    pub fn build(self) -> CesApp_ErrorHandlingSettings_EndSessionConfig {
        CesApp_ErrorHandlingSettings_EndSessionConfig { escalate_session: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CesApp_ErrorHandlingSettings_EndSessionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesApp_ErrorHandlingSettings_EndSessionConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesApp_ErrorHandlingSettings_EndSessionConfigRef {
        CesApp_ErrorHandlingSettings_EndSessionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesApp_ErrorHandlingSettings_EndSessionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `escalate_session` after provisioning.\nWhether to escalate the session in EndSession. If session is escalated,\nmetadata in EndSession will contain session_escalated = true."]
    pub fn escalate_session(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.escalate_session", self.base))
    }
}

#[derive(Serialize)]
pub struct CesApp_ErrorHandlingSettings_FallbackResponseConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_fallback_messages: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_fallback_attempts: std::option::Option<terrars::PrimField<f64>>,
}

impl CesApp_ErrorHandlingSettings_FallbackResponseConfig {
    #[doc =
        "Set the field `custom_fallback_messages`.\nThe fallback messages in case of system errors (e.g. LLM errors),\nmapped by supported language code\n(https://docs.cloud.google.com/customer-engagement-ai/conversational-agents/ps/reference/language)."]
    pub fn set_custom_fallback_messages(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.custom_fallback_messages = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_fallback_attempts`.\nThe maximum number of fallback attempts to make before the agent\nemitting EndSession Signal."]
    pub fn set_max_fallback_attempts(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_fallback_attempts = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesApp_ErrorHandlingSettings_FallbackResponseConfig {
    type O = terrars::BlockAssignable<CesApp_ErrorHandlingSettings_FallbackResponseConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesApp_ErrorHandlingSettings_FallbackResponseConfig {}

impl BuildCesApp_ErrorHandlingSettings_FallbackResponseConfig {
    pub fn build(self) -> CesApp_ErrorHandlingSettings_FallbackResponseConfig {
        CesApp_ErrorHandlingSettings_FallbackResponseConfig {
            custom_fallback_messages: core::default::Default::default(),
            max_fallback_attempts: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesApp_ErrorHandlingSettings_FallbackResponseConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesApp_ErrorHandlingSettings_FallbackResponseConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesApp_ErrorHandlingSettings_FallbackResponseConfigRef {
        CesApp_ErrorHandlingSettings_FallbackResponseConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesApp_ErrorHandlingSettings_FallbackResponseConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `custom_fallback_messages` after provisioning.\nThe fallback messages in case of system errors (e.g. LLM errors),\nmapped by supported language code\n(https://docs.cloud.google.com/customer-engagement-ai/conversational-agents/ps/reference/language)."]
    pub fn custom_fallback_messages(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.custom_fallback_messages", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_fallback_attempts` after provisioning.\nThe maximum number of fallback attempts to make before the agent\nemitting EndSession Signal."]
    pub fn max_fallback_attempts(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_fallback_attempts", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesApp_ErrorHandlingSettingsDynamic {
    end_session_config: std::option::Option<terrars::DynamicBlock<CesApp_ErrorHandlingSettings_EndSessionConfig>>,
    fallback_response_config: std
    ::option
    ::Option<terrars::DynamicBlock<CesApp_ErrorHandlingSettings_FallbackResponseConfig>>,
}

#[derive(Serialize)]
pub struct CesApp_ErrorHandlingSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_handling_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_session_config: std::option::Option<std::vec::Vec<CesApp_ErrorHandlingSettings_EndSessionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fallback_response_config: std::option::Option<std::vec::Vec<CesApp_ErrorHandlingSettings_FallbackResponseConfig>>,
    dynamic: CesApp_ErrorHandlingSettingsDynamic,
}

impl CesApp_ErrorHandlingSettings {
    #[doc =
        "Set the field `error_handling_strategy`.\nThe strategy to use for error handling.\nPossible values:\nNONE\nFALLBACK_RESPONSE\nEND_SESSION"]
    pub fn set_error_handling_strategy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.error_handling_strategy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `end_session_config`.\n"]
    pub fn set_end_session_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesApp_ErrorHandlingSettings_EndSessionConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.end_session_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.end_session_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `fallback_response_config`.\n"]
    pub fn set_fallback_response_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesApp_ErrorHandlingSettings_FallbackResponseConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fallback_response_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fallback_response_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesApp_ErrorHandlingSettings {
    type O = terrars::BlockAssignable<CesApp_ErrorHandlingSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesApp_ErrorHandlingSettings {}

impl BuildCesApp_ErrorHandlingSettings {
    pub fn build(self) -> CesApp_ErrorHandlingSettings {
        CesApp_ErrorHandlingSettings {
            error_handling_strategy: core::default::Default::default(),
            end_session_config: core::default::Default::default(),
            fallback_response_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesApp_ErrorHandlingSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesApp_ErrorHandlingSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesApp_ErrorHandlingSettingsRef {
        CesApp_ErrorHandlingSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesApp_ErrorHandlingSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `error_handling_strategy` after provisioning.\nThe strategy to use for error handling.\nPossible values:\nNONE\nFALLBACK_RESPONSE\nEND_SESSION"]
    pub fn error_handling_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_handling_strategy", self.base))
    }

    #[doc = "Get a reference to the value of field `end_session_config` after provisioning.\n"]
    pub fn end_session_config(&self) -> terrars::ListRef<CesApp_ErrorHandlingSettings_EndSessionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.end_session_config", self.base))
    }

    #[doc = "Get a reference to the value of field `fallback_response_config` after provisioning.\n"]
    pub fn fallback_response_config(&self) -> terrars::ListRef<CesApp_ErrorHandlingSettings_FallbackResponseConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fallback_response_config", self.base))
    }
}

#[derive(Serialize)]
pub struct CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ExpectationLevelMetricsThresholds {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool_invocation_parameter_correctness_threshold: std::option::Option<terrars::PrimField<f64>>,
}

impl CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ExpectationLevelMetricsThresholds {
    #[doc =
        "Set the field `tool_invocation_parameter_correctness_threshold`.\nThe success threshold for individual tool invocation parameter\ncorrectness. Must be a float between 0 and 1. Default is 1.0."]
    pub fn set_tool_invocation_parameter_correctness_threshold(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.tool_invocation_parameter_correctness_threshold = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ExpectationLevelMetricsThresholds {
    type O =
        terrars
        ::BlockAssignable<
            CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ExpectationLevelMetricsThresholds,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ExpectationLevelMetricsThresholds {}

impl BuildCesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ExpectationLevelMetricsThresholds {
    pub fn build(
        self,
    ) -> CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ExpectationLevelMetricsThresholds {
        CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ExpectationLevelMetricsThresholds {
            tool_invocation_parameter_correctness_threshold: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ExpectationLevelMetricsThresholdsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ExpectationLevelMetricsThresholdsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ExpectationLevelMetricsThresholdsRef {
        CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ExpectationLevelMetricsThresholdsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ExpectationLevelMetricsThresholdsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `tool_invocation_parameter_correctness_threshold` after provisioning.\nThe success threshold for individual tool invocation parameter\ncorrectness. Must be a float between 0 and 1. Default is 1.0."]
    pub fn tool_invocation_parameter_correctness_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tool_invocation_parameter_correctness_threshold", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ToolMatchingSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extra_tool_call_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ToolMatchingSettings {
    #[doc =
        "Set the field `extra_tool_call_behavior`.\nDefines the behavior when an extra tool call is encountered. An extra\ntool call is a tool call that is present in the execution but does\nnot match any tool call in the golden expectation. Possible values: [\"FAIL\", \"ALLOW\"]"]
    pub fn set_extra_tool_call_behavior(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.extra_tool_call_behavior = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ToolMatchingSettings {
    type O =
        terrars
        ::BlockAssignable<CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ToolMatchingSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ToolMatchingSettings {}

impl BuildCesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ToolMatchingSettings {
    pub fn build(self) -> CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ToolMatchingSettings {
        CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ToolMatchingSettings {
            extra_tool_call_behavior: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ToolMatchingSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ToolMatchingSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ToolMatchingSettingsRef {
        CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ToolMatchingSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ToolMatchingSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `extra_tool_call_behavior` after provisioning.\nDefines the behavior when an extra tool call is encountered. An extra\ntool call is a tool call that is present in the execution but does\nnot match any tool call in the golden expectation. Possible values: [\"FAIL\", \"ALLOW\"]"]
    pub fn extra_tool_call_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.extra_tool_call_behavior", self.base))
    }
}

#[derive(Serialize)]
pub struct CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_TurnLevelMetricsThresholds {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    overall_tool_invocation_correctness_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    semantic_similarity_channel: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    semantic_similarity_success_threshold: std::option::Option<terrars::PrimField<f64>>,
}

impl CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_TurnLevelMetricsThresholds {
    #[doc =
        "Set the field `overall_tool_invocation_correctness_threshold`.\nThe success threshold for overall tool invocation correctness. Must be\na float between 0 and 1. Default is 1.0."]
    pub fn set_overall_tool_invocation_correctness_threshold(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.overall_tool_invocation_correctness_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `semantic_similarity_channel`.\nThe semantic similarity channel to use for evaluation.\nPossible values:\nSEMANTIC_SIMILARITY_CHANNEL_UNSPECIFIED\nTEXT\nAUDIO"]
    pub fn set_semantic_similarity_channel(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.semantic_similarity_channel = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `semantic_similarity_success_threshold`.\nThe success threshold for semantic similarity. Must be an integer\nbetween 0 and 4. Default is >= 3."]
    pub fn set_semantic_similarity_success_threshold(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.semantic_similarity_success_threshold = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_TurnLevelMetricsThresholds {
    type O =
        terrars
        ::BlockAssignable<
            CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_TurnLevelMetricsThresholds,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_TurnLevelMetricsThresholds {}

impl BuildCesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_TurnLevelMetricsThresholds {
    pub fn build(
        self,
    ) -> CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_TurnLevelMetricsThresholds {
        CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_TurnLevelMetricsThresholds {
            overall_tool_invocation_correctness_threshold: core::default::Default::default(),
            semantic_similarity_channel: core::default::Default::default(),
            semantic_similarity_success_threshold: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_TurnLevelMetricsThresholdsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_TurnLevelMetricsThresholdsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_TurnLevelMetricsThresholdsRef {
        CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_TurnLevelMetricsThresholdsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_TurnLevelMetricsThresholdsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `overall_tool_invocation_correctness_threshold` after provisioning.\nThe success threshold for overall tool invocation correctness. Must be\na float between 0 and 1. Default is 1.0."]
    pub fn overall_tool_invocation_correctness_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.overall_tool_invocation_correctness_threshold", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `semantic_similarity_channel` after provisioning.\nThe semantic similarity channel to use for evaluation.\nPossible values:\nSEMANTIC_SIMILARITY_CHANNEL_UNSPECIFIED\nTEXT\nAUDIO"]
    pub fn semantic_similarity_channel(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.semantic_similarity_channel", self.base))
    }

    #[doc =
        "Get a reference to the value of field `semantic_similarity_success_threshold` after provisioning.\nThe success threshold for semantic similarity. Must be an integer\nbetween 0 and 4. Default is >= 3."]
    pub fn semantic_similarity_success_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.semantic_similarity_success_threshold", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholdsDynamic {
    expectation_level_metrics_thresholds: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ExpectationLevelMetricsThresholds,
        >,
    >,
    tool_matching_settings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ToolMatchingSettings>,
    >,
    turn_level_metrics_thresholds: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_TurnLevelMetricsThresholds,
        >,
    >,
}

#[derive(Serialize)]
pub struct CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expectation_level_metrics_thresholds: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ExpectationLevelMetricsThresholds>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool_matching_settings: std
    ::option
    ::Option<
        std::vec::Vec<CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ToolMatchingSettings>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    turn_level_metrics_thresholds: std
    ::option
    ::Option<
        std::vec::Vec<CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_TurnLevelMetricsThresholds>,
    >,
    dynamic: CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholdsDynamic,
}

impl CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds {
    #[doc = "Set the field `expectation_level_metrics_thresholds`.\n"]
    pub fn set_expectation_level_metrics_thresholds(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ExpectationLevelMetricsThresholds,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.expectation_level_metrics_thresholds = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.expectation_level_metrics_thresholds = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tool_matching_settings`.\n"]
    pub fn set_tool_matching_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ToolMatchingSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tool_matching_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tool_matching_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `turn_level_metrics_thresholds`.\n"]
    pub fn set_turn_level_metrics_thresholds(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_TurnLevelMetricsThresholds,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.turn_level_metrics_thresholds = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.turn_level_metrics_thresholds = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds {
    type O = terrars::BlockAssignable<CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds {}

impl BuildCesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds {
    pub fn build(self) -> CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds {
        CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds {
            expectation_level_metrics_thresholds: core::default::Default::default(),
            tool_matching_settings: core::default::Default::default(),
            turn_level_metrics_thresholds: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholdsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholdsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholdsRef {
        CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholdsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholdsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expectation_level_metrics_thresholds` after provisioning.\n"]
    pub fn expectation_level_metrics_thresholds(
        &self,
    ) -> terrars
    ::ListRef<
        CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ExpectationLevelMetricsThresholdsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expectation_level_metrics_thresholds", self.base))
    }

    #[doc = "Get a reference to the value of field `tool_matching_settings` after provisioning.\n"]
    pub fn tool_matching_settings(
        &self,
    ) -> terrars
    ::ListRef<CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ToolMatchingSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tool_matching_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `turn_level_metrics_thresholds` after provisioning.\n"]
    pub fn turn_level_metrics_thresholds(
        &self,
    ) -> terrars
    ::ListRef<CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_TurnLevelMetricsThresholdsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.turn_level_metrics_thresholds", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesApp_EvaluationMetricsThresholdsDynamic {
    golden_evaluation_metrics_thresholds: std
    ::option
    ::Option<terrars::DynamicBlock<CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds>>,
}

#[derive(Serialize)]
pub struct CesApp_EvaluationMetricsThresholds {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    golden_hallucination_metric_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scenario_hallucination_metric_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    golden_evaluation_metrics_thresholds: std
    ::option
    ::Option<std::vec::Vec<CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds>>,
    dynamic: CesApp_EvaluationMetricsThresholdsDynamic,
}

impl CesApp_EvaluationMetricsThresholds {
    #[doc =
        "Set the field `golden_hallucination_metric_behavior`.\nThe hallucination metric behavior for golden evaluations. Possible values: [\"DISABLED\", \"ENABLED\"]"]
    pub fn set_golden_hallucination_metric_behavior(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.golden_hallucination_metric_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `scenario_hallucination_metric_behavior`.\nThe hallucination metric behavior for scenario evaluations. Possible values: [\"DISABLED\", \"ENABLED\"]"]
    pub fn set_scenario_hallucination_metric_behavior(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.scenario_hallucination_metric_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `golden_evaluation_metrics_thresholds`.\n"]
    pub fn set_golden_evaluation_metrics_thresholds(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.golden_evaluation_metrics_thresholds = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.golden_evaluation_metrics_thresholds = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesApp_EvaluationMetricsThresholds {
    type O = terrars::BlockAssignable<CesApp_EvaluationMetricsThresholds>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesApp_EvaluationMetricsThresholds {}

impl BuildCesApp_EvaluationMetricsThresholds {
    pub fn build(self) -> CesApp_EvaluationMetricsThresholds {
        CesApp_EvaluationMetricsThresholds {
            golden_hallucination_metric_behavior: core::default::Default::default(),
            scenario_hallucination_metric_behavior: core::default::Default::default(),
            golden_evaluation_metrics_thresholds: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesApp_EvaluationMetricsThresholdsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesApp_EvaluationMetricsThresholdsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesApp_EvaluationMetricsThresholdsRef {
        CesApp_EvaluationMetricsThresholdsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesApp_EvaluationMetricsThresholdsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `golden_hallucination_metric_behavior` after provisioning.\nThe hallucination metric behavior for golden evaluations. Possible values: [\"DISABLED\", \"ENABLED\"]"]
    pub fn golden_hallucination_metric_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.golden_hallucination_metric_behavior", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scenario_hallucination_metric_behavior` after provisioning.\nThe hallucination metric behavior for scenario evaluations. Possible values: [\"DISABLED\", \"ENABLED\"]"]
    pub fn scenario_hallucination_metric_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.scenario_hallucination_metric_behavior", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `golden_evaluation_metrics_thresholds` after provisioning.\n"]
    pub fn golden_evaluation_metrics_thresholds(
        &self,
    ) -> terrars::ListRef<CesApp_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholdsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.golden_evaluation_metrics_thresholds", self.base))
    }
}

#[derive(Serialize)]
pub struct CesApp_LanguageSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_language_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_multilingual_support: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fallback_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    supported_language_codes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl CesApp_LanguageSettings {
    #[doc = "Set the field `default_language_code`.\nThe default language code of the app."]
    pub fn set_default_language_code(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.default_language_code = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_multilingual_support`.\nEnables multilingual support. If true, agents in the app will use pre-built\ninstructions to improve handling of multilingual input."]
    pub fn set_enable_multilingual_support(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_multilingual_support = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `fallback_action`.\nThe action to perform when an agent receives input in an unsupported\nlanguage.\nThis can be a predefined action or a custom tool call.\nValid values are:\n- A tool's full resource name, which triggers a specific tool execution.\n- A predefined system action, such as \"escalate\" or \"exit\", which triggers\nan EndSession signal with corresponding metadata\nto terminate the conversation."]
    pub fn set_fallback_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fallback_action = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `supported_language_codes`.\nList of languages codes supported by the app, in addition to the\n'default_language_code'."]
    pub fn set_supported_language_codes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.supported_language_codes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesApp_LanguageSettings {
    type O = terrars::BlockAssignable<CesApp_LanguageSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesApp_LanguageSettings {}

impl BuildCesApp_LanguageSettings {
    pub fn build(self) -> CesApp_LanguageSettings {
        CesApp_LanguageSettings {
            default_language_code: core::default::Default::default(),
            enable_multilingual_support: core::default::Default::default(),
            fallback_action: core::default::Default::default(),
            supported_language_codes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesApp_LanguageSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesApp_LanguageSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesApp_LanguageSettingsRef {
        CesApp_LanguageSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesApp_LanguageSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `default_language_code` after provisioning.\nThe default language code of the app."]
    pub fn default_language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_language_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_multilingual_support` after provisioning.\nEnables multilingual support. If true, agents in the app will use pre-built\ninstructions to improve handling of multilingual input."]
    pub fn enable_multilingual_support(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_multilingual_support", self.base))
    }

    #[doc =
        "Get a reference to the value of field `fallback_action` after provisioning.\nThe action to perform when an agent receives input in an unsupported\nlanguage.\nThis can be a predefined action or a custom tool call.\nValid values are:\n- A tool's full resource name, which triggers a specific tool execution.\n- A predefined system action, such as \"escalate\" or \"exit\", which triggers\nan EndSession signal with corresponding metadata\nto terminate the conversation."]
    pub fn fallback_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fallback_action", self.base))
    }

    #[doc =
        "Get a reference to the value of field `supported_language_codes` after provisioning.\nList of languages codes supported by the app, in addition to the\n'default_language_code'."]
    pub fn supported_language_codes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_language_codes", self.base))
    }
}

#[derive(Serialize)]
pub struct CesApp_LoggingSettings_AudioRecordingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs_bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs_path_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesApp_LoggingSettings_AudioRecordingConfig {
    #[doc =
        "Set the field `gcs_bucket`.\nThe [Cloud Storage](https://cloud.google.com/storage) bucket to store the\nsession audio recordings. The URI must start with \"gs://\".\nNote: If the Cloud Storage bucket is in a different project from the app,\nyou should grant 'storage.objects.create' permission to the CES service\nagent 'service-@gcp-sa-ces.iam.gserviceaccount.com'."]
    pub fn set_gcs_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gcs_bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `gcs_path_prefix`.\nThe Cloud Storage path prefix for audio recordings.\nThis prefix can include the following placeholders, which will be\ndynamically substituted at serving time:\n- $project:   project ID\n- $location:  app location\n- $app:       app ID\n- $date:      session date in YYYY-MM-DD format\n- $session:   session ID\nIf the path prefix is not specified, the default prefix\n'$project/$location/$app/$date/$session/' will be used."]
    pub fn set_gcs_path_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gcs_path_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesApp_LoggingSettings_AudioRecordingConfig {
    type O = terrars::BlockAssignable<CesApp_LoggingSettings_AudioRecordingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesApp_LoggingSettings_AudioRecordingConfig {}

impl BuildCesApp_LoggingSettings_AudioRecordingConfig {
    pub fn build(self) -> CesApp_LoggingSettings_AudioRecordingConfig {
        CesApp_LoggingSettings_AudioRecordingConfig {
            gcs_bucket: core::default::Default::default(),
            gcs_path_prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesApp_LoggingSettings_AudioRecordingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesApp_LoggingSettings_AudioRecordingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesApp_LoggingSettings_AudioRecordingConfigRef {
        CesApp_LoggingSettings_AudioRecordingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesApp_LoggingSettings_AudioRecordingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `gcs_bucket` after provisioning.\nThe [Cloud Storage](https://cloud.google.com/storage) bucket to store the\nsession audio recordings. The URI must start with \"gs://\".\nNote: If the Cloud Storage bucket is in a different project from the app,\nyou should grant 'storage.objects.create' permission to the CES service\nagent 'service-@gcp-sa-ces.iam.gserviceaccount.com'."]
    pub fn gcs_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcs_bucket", self.base))
    }

    #[doc =
        "Get a reference to the value of field `gcs_path_prefix` after provisioning.\nThe Cloud Storage path prefix for audio recordings.\nThis prefix can include the following placeholders, which will be\ndynamically substituted at serving time:\n- $project:   project ID\n- $location:  app location\n- $app:       app ID\n- $date:      session date in YYYY-MM-DD format\n- $session:   session ID\nIf the path prefix is not specified, the default prefix\n'$project/$location/$app/$date/$session/' will be used."]
    pub fn gcs_path_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcs_path_prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct CesApp_LoggingSettings_BigqueryExportSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataset: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesApp_LoggingSettings_BigqueryExportSettings {
    #[doc = "Set the field `dataset`.\nThe BigQuery dataset to export the data to."]
    pub fn set_dataset(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dataset = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\nIndicates whether the BigQuery export is enabled."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project`.\nThe project ID of the BigQuery dataset to export the data to.\nNote: If the BigQuery dataset is in a different project from the app, you should grant\nroles/bigquery.admin role to the CES service agent service-<PROJECT-\nNUMBER>@gcp-sa-ces.iam.gserviceaccount.com."]
    pub fn set_project(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesApp_LoggingSettings_BigqueryExportSettings {
    type O = terrars::BlockAssignable<CesApp_LoggingSettings_BigqueryExportSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesApp_LoggingSettings_BigqueryExportSettings {}

impl BuildCesApp_LoggingSettings_BigqueryExportSettings {
    pub fn build(self) -> CesApp_LoggingSettings_BigqueryExportSettings {
        CesApp_LoggingSettings_BigqueryExportSettings {
            dataset: core::default::Default::default(),
            enabled: core::default::Default::default(),
            project: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesApp_LoggingSettings_BigqueryExportSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesApp_LoggingSettings_BigqueryExportSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesApp_LoggingSettings_BigqueryExportSettingsRef {
        CesApp_LoggingSettings_BigqueryExportSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesApp_LoggingSettings_BigqueryExportSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dataset` after provisioning.\nThe BigQuery dataset to export the data to."]
    pub fn dataset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nIndicates whether the BigQuery export is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe project ID of the BigQuery dataset to export the data to.\nNote: If the BigQuery dataset is in a different project from the app, you should grant\nroles/bigquery.admin role to the CES service agent service-<PROJECT-\nNUMBER>@gcp-sa-ces.iam.gserviceaccount.com."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
}

#[derive(Serialize)]
pub struct CesApp_LoggingSettings_CloudLoggingSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_cloud_logging: std::option::Option<terrars::PrimField<bool>>,
}

impl CesApp_LoggingSettings_CloudLoggingSettings {
    #[doc = "Set the field `enable_cloud_logging`.\nWhether to enable Cloud Logging for the sessions."]
    pub fn set_enable_cloud_logging(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_cloud_logging = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesApp_LoggingSettings_CloudLoggingSettings {
    type O = terrars::BlockAssignable<CesApp_LoggingSettings_CloudLoggingSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesApp_LoggingSettings_CloudLoggingSettings {}

impl BuildCesApp_LoggingSettings_CloudLoggingSettings {
    pub fn build(self) -> CesApp_LoggingSettings_CloudLoggingSettings {
        CesApp_LoggingSettings_CloudLoggingSettings { enable_cloud_logging: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CesApp_LoggingSettings_CloudLoggingSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesApp_LoggingSettings_CloudLoggingSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesApp_LoggingSettings_CloudLoggingSettingsRef {
        CesApp_LoggingSettings_CloudLoggingSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesApp_LoggingSettings_CloudLoggingSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_cloud_logging` after provisioning.\nWhether to enable Cloud Logging for the sessions."]
    pub fn enable_cloud_logging(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_cloud_logging", self.base))
    }
}

#[derive(Serialize)]
pub struct CesApp_LoggingSettings_ConversationLoggingSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_conversation_logging: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_window: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesApp_LoggingSettings_ConversationLoggingSettings {
    #[doc = "Set the field `disable_conversation_logging`.\nWhether to disable conversation logging for the sessions."]
    pub fn set_disable_conversation_logging(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_conversation_logging = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `retention_window`.\nControls the retention window for the conversation.\nIf not set, the conversation will be retained for 365 days."]
    pub fn set_retention_window(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.retention_window = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesApp_LoggingSettings_ConversationLoggingSettings {
    type O = terrars::BlockAssignable<CesApp_LoggingSettings_ConversationLoggingSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesApp_LoggingSettings_ConversationLoggingSettings {}

impl BuildCesApp_LoggingSettings_ConversationLoggingSettings {
    pub fn build(self) -> CesApp_LoggingSettings_ConversationLoggingSettings {
        CesApp_LoggingSettings_ConversationLoggingSettings {
            disable_conversation_logging: core::default::Default::default(),
            retention_window: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesApp_LoggingSettings_ConversationLoggingSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesApp_LoggingSettings_ConversationLoggingSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesApp_LoggingSettings_ConversationLoggingSettingsRef {
        CesApp_LoggingSettings_ConversationLoggingSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesApp_LoggingSettings_ConversationLoggingSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disable_conversation_logging` after provisioning.\nWhether to disable conversation logging for the sessions."]
    pub fn disable_conversation_logging(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_conversation_logging", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retention_window` after provisioning.\nControls the retention window for the conversation.\nIf not set, the conversation will be retained for 365 days."]
    pub fn retention_window(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_window", self.base))
    }
}

#[derive(Serialize)]
pub struct CesApp_LoggingSettings_MetricAnalysisSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    llm_metrics_opted_out: std::option::Option<terrars::PrimField<bool>>,
}

impl CesApp_LoggingSettings_MetricAnalysisSettings {
    #[doc =
        "Set the field `llm_metrics_opted_out`.\nWhether to collect conversation data for llm analysis metrics. If true,\nconversation data will not be collected for llm analysis metrics;\notherwise, conversation data will be collected."]
    pub fn set_llm_metrics_opted_out(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.llm_metrics_opted_out = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesApp_LoggingSettings_MetricAnalysisSettings {
    type O = terrars::BlockAssignable<CesApp_LoggingSettings_MetricAnalysisSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesApp_LoggingSettings_MetricAnalysisSettings {}

impl BuildCesApp_LoggingSettings_MetricAnalysisSettings {
    pub fn build(self) -> CesApp_LoggingSettings_MetricAnalysisSettings {
        CesApp_LoggingSettings_MetricAnalysisSettings { llm_metrics_opted_out: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CesApp_LoggingSettings_MetricAnalysisSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesApp_LoggingSettings_MetricAnalysisSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesApp_LoggingSettings_MetricAnalysisSettingsRef {
        CesApp_LoggingSettings_MetricAnalysisSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesApp_LoggingSettings_MetricAnalysisSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `llm_metrics_opted_out` after provisioning.\nWhether to collect conversation data for llm analysis metrics. If true,\nconversation data will not be collected for llm analysis metrics;\notherwise, conversation data will be collected."]
    pub fn llm_metrics_opted_out(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.llm_metrics_opted_out", self.base))
    }
}

#[derive(Serialize)]
pub struct CesApp_LoggingSettings_RedactionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deidentify_template: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_redaction: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inspect_template: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesApp_LoggingSettings_RedactionConfig {
    #[doc =
        "Set the field `deidentify_template`.\n[DLP](https://cloud.google.com/dlp/docs) deidentify template name to\ninstruct on how to de-identify content.\nFormat:\n'projects/{project}/locations/{location}/deidentifyTemplates/{deidentify_template}'"]
    pub fn set_deidentify_template(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.deidentify_template = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_redaction`.\nIf true, redaction will be applied in various logging scenarios, including\nconversation history, Cloud Logging and audio recording."]
    pub fn set_enable_redaction(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_redaction = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `inspect_template`.\n[DLP](https://cloud.google.com/dlp/docs) inspect template name to configure\ndetection of sensitive data types.\nFormat:\n'projects/{project}/locations/{location}/inspectTemplates/{inspect_template}'"]
    pub fn set_inspect_template(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.inspect_template = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesApp_LoggingSettings_RedactionConfig {
    type O = terrars::BlockAssignable<CesApp_LoggingSettings_RedactionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesApp_LoggingSettings_RedactionConfig {}

impl BuildCesApp_LoggingSettings_RedactionConfig {
    pub fn build(self) -> CesApp_LoggingSettings_RedactionConfig {
        CesApp_LoggingSettings_RedactionConfig {
            deidentify_template: core::default::Default::default(),
            enable_redaction: core::default::Default::default(),
            inspect_template: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesApp_LoggingSettings_RedactionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesApp_LoggingSettings_RedactionConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesApp_LoggingSettings_RedactionConfigRef {
        CesApp_LoggingSettings_RedactionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesApp_LoggingSettings_RedactionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `deidentify_template` after provisioning.\n[DLP](https://cloud.google.com/dlp/docs) deidentify template name to\ninstruct on how to de-identify content.\nFormat:\n'projects/{project}/locations/{location}/deidentifyTemplates/{deidentify_template}'"]
    pub fn deidentify_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deidentify_template", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_redaction` after provisioning.\nIf true, redaction will be applied in various logging scenarios, including\nconversation history, Cloud Logging and audio recording."]
    pub fn enable_redaction(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_redaction", self.base))
    }

    #[doc =
        "Get a reference to the value of field `inspect_template` after provisioning.\n[DLP](https://cloud.google.com/dlp/docs) inspect template name to configure\ndetection of sensitive data types.\nFormat:\n'projects/{project}/locations/{location}/inspectTemplates/{inspect_template}'"]
    pub fn inspect_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inspect_template", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesApp_LoggingSettingsDynamic {
    audio_recording_config: std::option::Option<terrars::DynamicBlock<CesApp_LoggingSettings_AudioRecordingConfig>>,
    bigquery_export_settings: std
    ::option
    ::Option<terrars::DynamicBlock<CesApp_LoggingSettings_BigqueryExportSettings>>,
    cloud_logging_settings: std::option::Option<terrars::DynamicBlock<CesApp_LoggingSettings_CloudLoggingSettings>>,
    conversation_logging_settings: std
    ::option
    ::Option<terrars::DynamicBlock<CesApp_LoggingSettings_ConversationLoggingSettings>>,
    metric_analysis_settings: std
    ::option
    ::Option<terrars::DynamicBlock<CesApp_LoggingSettings_MetricAnalysisSettings>>,
    redaction_config: std::option::Option<terrars::DynamicBlock<CesApp_LoggingSettings_RedactionConfig>>,
}

#[derive(Serialize)]
pub struct CesApp_LoggingSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_recording_config: std::option::Option<std::vec::Vec<CesApp_LoggingSettings_AudioRecordingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bigquery_export_settings: std::option::Option<std::vec::Vec<CesApp_LoggingSettings_BigqueryExportSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_logging_settings: std::option::Option<std::vec::Vec<CesApp_LoggingSettings_CloudLoggingSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conversation_logging_settings: std::option::Option<std::vec::Vec<CesApp_LoggingSettings_ConversationLoggingSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_analysis_settings: std::option::Option<std::vec::Vec<CesApp_LoggingSettings_MetricAnalysisSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redaction_config: std::option::Option<std::vec::Vec<CesApp_LoggingSettings_RedactionConfig>>,
    dynamic: CesApp_LoggingSettingsDynamic,
}

impl CesApp_LoggingSettings {
    #[doc = "Set the field `audio_recording_config`.\n"]
    pub fn set_audio_recording_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesApp_LoggingSettings_AudioRecordingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.audio_recording_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.audio_recording_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `bigquery_export_settings`.\n"]
    pub fn set_bigquery_export_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesApp_LoggingSettings_BigqueryExportSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bigquery_export_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bigquery_export_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cloud_logging_settings`.\n"]
    pub fn set_cloud_logging_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesApp_LoggingSettings_CloudLoggingSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloud_logging_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_logging_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `conversation_logging_settings`.\n"]
    pub fn set_conversation_logging_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesApp_LoggingSettings_ConversationLoggingSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conversation_logging_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conversation_logging_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `metric_analysis_settings`.\n"]
    pub fn set_metric_analysis_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesApp_LoggingSettings_MetricAnalysisSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metric_analysis_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metric_analysis_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `redaction_config`.\n"]
    pub fn set_redaction_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesApp_LoggingSettings_RedactionConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.redaction_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.redaction_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesApp_LoggingSettings {
    type O = terrars::BlockAssignable<CesApp_LoggingSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesApp_LoggingSettings {}

impl BuildCesApp_LoggingSettings {
    pub fn build(self) -> CesApp_LoggingSettings {
        CesApp_LoggingSettings {
            audio_recording_config: core::default::Default::default(),
            bigquery_export_settings: core::default::Default::default(),
            cloud_logging_settings: core::default::Default::default(),
            conversation_logging_settings: core::default::Default::default(),
            metric_analysis_settings: core::default::Default::default(),
            redaction_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesApp_LoggingSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesApp_LoggingSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesApp_LoggingSettingsRef {
        CesApp_LoggingSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesApp_LoggingSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audio_recording_config` after provisioning.\n"]
    pub fn audio_recording_config(&self) -> terrars::ListRef<CesApp_LoggingSettings_AudioRecordingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio_recording_config", self.base))
    }

    #[doc = "Get a reference to the value of field `bigquery_export_settings` after provisioning.\n"]
    pub fn bigquery_export_settings(&self) -> terrars::ListRef<CesApp_LoggingSettings_BigqueryExportSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigquery_export_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_logging_settings` after provisioning.\n"]
    pub fn cloud_logging_settings(&self) -> terrars::ListRef<CesApp_LoggingSettings_CloudLoggingSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_logging_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `conversation_logging_settings` after provisioning.\n"]
    pub fn conversation_logging_settings(
        &self,
    ) -> terrars::ListRef<CesApp_LoggingSettings_ConversationLoggingSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conversation_logging_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `metric_analysis_settings` after provisioning.\n"]
    pub fn metric_analysis_settings(&self) -> terrars::ListRef<CesApp_LoggingSettings_MetricAnalysisSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_analysis_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `redaction_config` after provisioning.\n"]
    pub fn redaction_config(&self) -> terrars::ListRef<CesApp_LoggingSettings_RedactionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redaction_config", self.base))
    }
}

#[derive(Serialize)]
pub struct CesApp_ModelSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    temperature: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    thinking_level: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesApp_ModelSettings {
    #[doc =
        "Set the field `model`.\nThe LLM model that the agent should use.\nIf not set, the agent will inherit the model from its parent agent."]
    pub fn set_model(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `temperature`.\nIf set, this temperature will be used for the LLM model. Temperature\ncontrols the randomness of the model's responses. Lower temperatures\nproduce responses that are more predictable. Higher temperatures produce\nresponses that are more creative."]
    pub fn set_temperature(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.temperature = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `thinking_level`.\nThe thinking level of the model. Possible values: [\"DEFAULT\", \"LOW\", \"MEDIUM\", \"HIGH\"]"]
    pub fn set_thinking_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.thinking_level = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesApp_ModelSettings {
    type O = terrars::BlockAssignable<CesApp_ModelSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesApp_ModelSettings {}

impl BuildCesApp_ModelSettings {
    pub fn build(self) -> CesApp_ModelSettings {
        CesApp_ModelSettings {
            model: core::default::Default::default(),
            temperature: core::default::Default::default(),
            thinking_level: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesApp_ModelSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesApp_ModelSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesApp_ModelSettingsRef {
        CesApp_ModelSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesApp_ModelSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `model` after provisioning.\nThe LLM model that the agent should use.\nIf not set, the agent will inherit the model from its parent agent."]
    pub fn model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model", self.base))
    }

    #[doc =
        "Get a reference to the value of field `temperature` after provisioning.\nIf set, this temperature will be used for the LLM model. Temperature\ncontrols the randomness of the model's responses. Lower temperatures\nproduce responses that are more predictable. Higher temperatures produce\nresponses that are more creative."]
    pub fn temperature(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.temperature", self.base))
    }

    #[doc =
        "Get a reference to the value of field `thinking_level` after provisioning.\nThe thinking level of the model. Possible values: [\"DEFAULT\", \"LOW\", \"MEDIUM\", \"HIGH\"]"]
    pub fn thinking_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.thinking_level", self.base))
    }
}

#[derive(Serialize)]
pub struct CesApp_TimeZoneSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesApp_TimeZoneSettings {
    #[doc =
        "Set the field `time_zone`.\nThe time zone of the app from the time zone database, e.g., America/Los_Angeles, Europe/Paris."]
    pub fn set_time_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.time_zone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesApp_TimeZoneSettings {
    type O = terrars::BlockAssignable<CesApp_TimeZoneSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesApp_TimeZoneSettings {}

impl BuildCesApp_TimeZoneSettings {
    pub fn build(self) -> CesApp_TimeZoneSettings {
        CesApp_TimeZoneSettings { time_zone: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CesApp_TimeZoneSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesApp_TimeZoneSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesApp_TimeZoneSettingsRef {
        CesApp_TimeZoneSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesApp_TimeZoneSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `time_zone` after provisioning.\nThe time zone of the app from the time zone database, e.g., America/Los_Angeles, Europe/Paris."]
    pub fn time_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }
}

#[derive(Serialize)]
pub struct CesApp_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesApp_Timeouts {
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

impl terrars::ToListMappable for CesApp_Timeouts {
    type O = terrars::BlockAssignable<CesApp_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesApp_Timeouts {}

impl BuildCesApp_Timeouts {
    pub fn build(self) -> CesApp_Timeouts {
        CesApp_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesApp_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesApp_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesApp_TimeoutsRef {
        CesApp_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesApp_TimeoutsRef {
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

#[derive(Serialize)]
pub struct CesApp_VariableDeclarations_Schema {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_properties: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    any_of: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    defs: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "enum", skip_serializing_if = "std::option::Option::is_none")]
    enum_: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nullable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_items: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "ref", skip_serializing_if = "std::option::Option::is_none")]
    ref_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    required: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    title: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unique_items: std::option::Option<terrars::PrimField<bool>>,
}

impl CesApp_VariableDeclarations_Schema {
    #[doc =
        "Set the field `additional_properties`.\nOptional. Defines the schema for additional properties allowed in an object.\nThe value must be a valid JSON string representing the Schema object.\n(Note: OpenAPI also allows a boolean, this definition expects a Schema JSON)."]
    pub fn set_additional_properties(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.additional_properties = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `any_of`.\nOptional. The instance value should be valid against at least one of the schemas in this list."]
    pub fn set_any_of(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.any_of = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `default`.\nOptional. Default value of the data. Represents a dynamically typed value\nwhich can be either null, a number, a string, a boolean, a struct,\nor a list of values. The provided default value must be encoded as a JSON string.\nUse 'jsonencode' in Terraform HCL to encode the default value."]
    pub fn set_default(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.default = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `defs`.\nA map of definitions for use by ref. Only allowed at the root of the schema."]
    pub fn set_defs(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.defs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nThe description of the data."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enum_`.\nPossible values of the element of primitive type with enum format.\nExamples:\n1. We can define direction as :\n{type:STRING, format:enum, enum:[\"EAST\", NORTH\", \"SOUTH\", \"WEST\"]}\n2. We can define apartment number as :\n{type:INTEGER, format:enum, enum:[\"101\", \"201\", \"301\"]}"]
    pub fn set_enum(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.enum_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `items`.\nSchema of the elements of Type.ARRAY."]
    pub fn set_items(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nullable`.\nIndicates if the value may be null."]
    pub fn set_nullable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.nullable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix_items`.\nOptional. Schemas of initial elements of Type.ARRAY."]
    pub fn set_prefix_items(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `properties`.\nProperties of Type.OBJECT."]
    pub fn set_properties(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ref_`.\nAllows indirect references between schema nodes. The value should be a\nvalid reference to a child of the root 'defs'.\nFor example, the following schema defines a reference to a schema node\nnamed \"Pet\":\ntype: object\nproperties:\n  pet:\n    ref: #/defs/Pet\ndefs:\n  Pet:\n    type: object\n    properties:\n      name:\n        type: string\nThe value of the \"pet\" property is a reference to the schema node\nnamed \"Pet\".\nSee details in\nhttps://json-schema.org/understanding-json-schema/structuring."]
    pub fn set_ref(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ref_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `required`.\nRequired properties of Type.OBJECT."]
    pub fn set_required(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.required = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `title`.\nThe title of the schema."]
    pub fn set_title(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.title = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `unique_items`.\nIndicate the items in the array must be unique. Only applies to TYPE.ARRAY."]
    pub fn set_unique_items(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.unique_items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesApp_VariableDeclarations_Schema {
    type O = terrars::BlockAssignable<CesApp_VariableDeclarations_Schema>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesApp_VariableDeclarations_Schema {
    #[doc = "The type of the data.\nPossible values:\nSTRING\nINTEGER\nNUMBER\nBOOLEAN\nOBJECT\nARRAY"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildCesApp_VariableDeclarations_Schema {
    pub fn build(self) -> CesApp_VariableDeclarations_Schema {
        CesApp_VariableDeclarations_Schema {
            additional_properties: core::default::Default::default(),
            any_of: core::default::Default::default(),
            default: core::default::Default::default(),
            defs: core::default::Default::default(),
            description: core::default::Default::default(),
            enum_: core::default::Default::default(),
            items: core::default::Default::default(),
            nullable: core::default::Default::default(),
            prefix_items: core::default::Default::default(),
            properties: core::default::Default::default(),
            ref_: core::default::Default::default(),
            required: core::default::Default::default(),
            title: core::default::Default::default(),
            type_: self.type_,
            unique_items: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesApp_VariableDeclarations_SchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesApp_VariableDeclarations_SchemaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesApp_VariableDeclarations_SchemaRef {
        CesApp_VariableDeclarations_SchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesApp_VariableDeclarations_SchemaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `additional_properties` after provisioning.\nOptional. Defines the schema for additional properties allowed in an object.\nThe value must be a valid JSON string representing the Schema object.\n(Note: OpenAPI also allows a boolean, this definition expects a Schema JSON)."]
    pub fn additional_properties(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.additional_properties", self.base))
    }

    #[doc =
        "Get a reference to the value of field `any_of` after provisioning.\nOptional. The instance value should be valid against at least one of the schemas in this list."]
    pub fn any_of(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.any_of", self.base))
    }

    #[doc =
        "Get a reference to the value of field `default` after provisioning.\nOptional. Default value of the data. Represents a dynamically typed value\nwhich can be either null, a number, a string, a boolean, a struct,\nor a list of values. The provided default value must be encoded as a JSON string.\nUse 'jsonencode' in Terraform HCL to encode the default value."]
    pub fn default(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default", self.base))
    }

    #[doc =
        "Get a reference to the value of field `defs` after provisioning.\nA map of definitions for use by ref. Only allowed at the root of the schema."]
    pub fn defs(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.defs", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the data."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enum_` after provisioning.\nPossible values of the element of primitive type with enum format.\nExamples:\n1. We can define direction as :\n{type:STRING, format:enum, enum:[\"EAST\", NORTH\", \"SOUTH\", \"WEST\"]}\n2. We can define apartment number as :\n{type:INTEGER, format:enum, enum:[\"101\", \"201\", \"301\"]}"]
    pub fn enum_(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enum", self.base))
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\nSchema of the elements of Type.ARRAY."]
    pub fn items(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.items", self.base))
    }

    #[doc = "Get a reference to the value of field `nullable` after provisioning.\nIndicates if the value may be null."]
    pub fn nullable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nullable", self.base))
    }

    #[doc =
        "Get a reference to the value of field `prefix_items` after provisioning.\nOptional. Schemas of initial elements of Type.ARRAY."]
    pub fn prefix_items(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix_items", self.base))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\nProperties of Type.OBJECT."]
    pub fn properties(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.properties", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ref_` after provisioning.\nAllows indirect references between schema nodes. The value should be a\nvalid reference to a child of the root 'defs'.\nFor example, the following schema defines a reference to a schema node\nnamed \"Pet\":\ntype: object\nproperties:\n  pet:\n    ref: #/defs/Pet\ndefs:\n  Pet:\n    type: object\n    properties:\n      name:\n        type: string\nThe value of the \"pet\" property is a reference to the schema node\nnamed \"Pet\".\nSee details in\nhttps://json-schema.org/understanding-json-schema/structuring."]
    pub fn ref_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ref", self.base))
    }

    #[doc = "Get a reference to the value of field `required` after provisioning.\nRequired properties of Type.OBJECT."]
    pub fn required(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.required", self.base))
    }

    #[doc = "Get a reference to the value of field `title` after provisioning.\nThe title of the schema."]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of the data.\nPossible values:\nSTRING\nINTEGER\nNUMBER\nBOOLEAN\nOBJECT\nARRAY"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `unique_items` after provisioning.\nIndicate the items in the array must be unique. Only applies to TYPE.ARRAY."]
    pub fn unique_items(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unique_items", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesApp_VariableDeclarationsDynamic {
    schema: std::option::Option<terrars::DynamicBlock<CesApp_VariableDeclarations_Schema>>,
}

#[derive(Serialize)]
pub struct CesApp_VariableDeclarations {
    description: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema: std::option::Option<std::vec::Vec<CesApp_VariableDeclarations_Schema>>,
    dynamic: CesApp_VariableDeclarationsDynamic,
}

impl CesApp_VariableDeclarations {
    #[doc = "Set the field `schema`.\n"]
    pub fn set_schema(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesApp_VariableDeclarations_Schema>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.schema = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.schema = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesApp_VariableDeclarations {
    type O = terrars::BlockAssignable<CesApp_VariableDeclarations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesApp_VariableDeclarations {
    #[doc = "The description of the variable."]
    pub description: terrars::PrimField<std::string::String>,
    #[doc =
        "The name of the variable. The name must start with a letter or underscore\nand contain only letters, numbers, or underscores."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCesApp_VariableDeclarations {
    pub fn build(self) -> CesApp_VariableDeclarations {
        CesApp_VariableDeclarations {
            description: self.description,
            name: self.name,
            schema: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesApp_VariableDeclarationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesApp_VariableDeclarationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesApp_VariableDeclarationsRef {
        CesApp_VariableDeclarationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesApp_VariableDeclarationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the variable."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the variable. The name must start with a letter or underscore\nand contain only letters, numbers, or underscores."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> terrars::ListRef<CesApp_VariableDeclarations_SchemaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema", self.base))
    }
}

#[derive(Serialize)]
pub struct CesApp_VpcScSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_origins: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl CesApp_VpcScSettings {
    #[doc =
        "Set the field `allowed_origins`.\nThe allowed HTTP(s) origins that OpenAPI tools in the App are\nable to directly call when VPC Service Controls are enabled. These strings\nmust match the origin exactly, including the port if specified. For\nexample, \"https://example.com\" or \"https://example.com:443\". This list does\nnot yet apply to Python tools that may make direct HTTP calls."]
    pub fn set_allowed_origins(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_origins = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesApp_VpcScSettings {
    type O = terrars::BlockAssignable<CesApp_VpcScSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesApp_VpcScSettings {}

impl BuildCesApp_VpcScSettings {
    pub fn build(self) -> CesApp_VpcScSettings {
        CesApp_VpcScSettings { allowed_origins: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CesApp_VpcScSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesApp_VpcScSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesApp_VpcScSettingsRef {
        CesApp_VpcScSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesApp_VpcScSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_origins` after provisioning.\nThe allowed HTTP(s) origins that OpenAPI tools in the App are\nable to directly call when VPC Service Controls are enabled. These strings\nmust match the origin exactly, including the port if specified. For\nexample, \"https://example.com\" or \"https://example.com:443\". This list does\nnot yet apply to Python tools that may make direct HTTP calls."]
    pub fn allowed_origins(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_origins", self.base))
    }
}
