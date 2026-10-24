use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct AppEngineFlexibleAppVersionData {
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
    beta_settings: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_expiration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_service_on_destroy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env_variables: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inbound_services: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_class: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nobuild_files_regex: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    noop_on_destroy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    runtime: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    runtime_api_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    runtime_channel: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    runtime_main_executable_path: std::option::Option<terrars::PrimField<std::string::String>>,
    service: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    serving_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_config: std::option::Option<std::vec::Vec<AppEngineFlexibleAppVersionApiConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automatic_scaling: std::option::Option<std::vec::Vec<AppEngineFlexibleAppVersionAutomaticScalingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment: std::option::Option<std::vec::Vec<AppEngineFlexibleAppVersionDeploymentEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoints_api_service: std::option::Option<std::vec::Vec<AppEngineFlexibleAppVersionEndpointsApiServiceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entrypoint: std::option::Option<std::vec::Vec<AppEngineFlexibleAppVersionEntrypointEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    handlers: std::option::Option<std::vec::Vec<AppEngineFlexibleAppVersionHandlersEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    liveness_check: std::option::Option<std::vec::Vec<AppEngineFlexibleAppVersionLivenessCheckEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    manual_scaling: std::option::Option<std::vec::Vec<AppEngineFlexibleAppVersionManualScalingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<std::vec::Vec<AppEngineFlexibleAppVersionNetworkEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    readiness_check: std::option::Option<std::vec::Vec<AppEngineFlexibleAppVersionReadinessCheckEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<std::vec::Vec<AppEngineFlexibleAppVersionResourcesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<AppEngineFlexibleAppVersionTimeoutsEl>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_access_connector: std::option::Option<std::vec::Vec<AppEngineFlexibleAppVersionVpcAccessConnectorEl>>,
    dynamic: AppEngineFlexibleAppVersionDynamic,
}

struct AppEngineFlexibleAppVersion_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AppEngineFlexibleAppVersionData>,
}

#[derive(Clone)]
pub struct AppEngineFlexibleAppVersion(std::rc::Rc<AppEngineFlexibleAppVersion_>);

impl AppEngineFlexibleAppVersion {
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
        "Set the field `beta_settings`.\nMetadata settings that are supplied to this version to enable beta runtime features."]
    pub fn set_beta_settings(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().beta_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `default_expiration`.\nDuration that static files should be cached by web proxies and browsers.\nOnly applicable if the corresponding StaticFilesHandler does not specify its own expiration time."]
    pub fn set_default_expiration(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().default_expiration = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete_service_on_destroy`.\nIf set to 'true', the service will be deleted if it is the last version."]
    pub fn set_delete_service_on_destroy(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_service_on_destroy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `env_variables`.\nEnvironment variables available to the application.  As these are not returned in the API request, Terraform will not detect any changes made outside of the Terraform config."]
    pub fn set_env_variables(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().env_variables = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `inbound_services`.\nA list of the types of messages that this application is able to receive. Possible values: [\"INBOUND_SERVICE_MAIL\", \"INBOUND_SERVICE_MAIL_BOUNCE\", \"INBOUND_SERVICE_XMPP_ERROR\", \"INBOUND_SERVICE_XMPP_MESSAGE\", \"INBOUND_SERVICE_XMPP_SUBSCRIBE\", \"INBOUND_SERVICE_XMPP_PRESENCE\", \"INBOUND_SERVICE_CHANNEL_PRESENCE\", \"INBOUND_SERVICE_WARMUP\"]"]
    pub fn set_inbound_services(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().inbound_services = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `instance_class`.\nInstance class that is used to run this version. Valid values are\nAutomaticScaling: F1, F2, F4, F4_1G\nManualScaling: B1, B2, B4, B8, B4_1G\nDefaults to F1 for AutomaticScaling and B1 for ManualScaling."]
    pub fn set_instance_class(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().instance_class = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `nobuild_files_regex`.\nFiles that match this pattern will not be built into this version. Only applicable for Go runtimes."]
    pub fn set_nobuild_files_regex(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().nobuild_files_regex = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `noop_on_destroy`.\nIf set to 'true', the application version will not be deleted."]
    pub fn set_noop_on_destroy(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().noop_on_destroy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `runtime_api_version`.\nThe version of the API in the given runtime environment.\nPlease see the app.yaml reference for valid values at 'https://cloud.google.com/appengine/docs/standard/<language>/config/appref'\\\nSubstitute '<language>' with 'python', 'java', 'php', 'ruby', 'go' or 'nodejs'."]
    pub fn set_runtime_api_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().runtime_api_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `runtime_channel`.\nThe channel of the runtime to use. Only available for some runtimes."]
    pub fn set_runtime_channel(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().runtime_channel = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `runtime_main_executable_path`.\nThe path or name of the app's main executable."]
    pub fn set_runtime_main_executable_path(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().runtime_main_executable_path = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_account`.\nThe identity that the deployed version will run as. Admin API will use the App Engine Appspot service account as\ndefault if this field is neither provided in app.yaml file nor through CLI flag."]
    pub fn set_service_account(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `serving_status`.\nCurrent serving status of this version. Only the versions with a SERVING status create instances and can be billed. Default value: \"SERVING\" Possible values: [\"SERVING\", \"STOPPED\"]"]
    pub fn set_serving_status(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().serving_status = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `version_id`.\nRelative name of the version within the service. For example, 'v1'. Version names can contain only lowercase letters, numbers, or hyphens.\nReserved names,\"default\", \"latest\", and any name with the prefix \"ah-\"."]
    pub fn set_version_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().version_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `api_config`.\n"]
    pub fn set_api_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppEngineFlexibleAppVersionApiConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().api_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.api_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `automatic_scaling`.\n"]
    pub fn set_automatic_scaling(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppEngineFlexibleAppVersionAutomaticScalingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().automatic_scaling = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.automatic_scaling = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `deployment`.\n"]
    pub fn set_deployment(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppEngineFlexibleAppVersionDeploymentEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().deployment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.deployment = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `endpoints_api_service`.\n"]
    pub fn set_endpoints_api_service(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppEngineFlexibleAppVersionEndpointsApiServiceEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().endpoints_api_service = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.endpoints_api_service = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `entrypoint`.\n"]
    pub fn set_entrypoint(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppEngineFlexibleAppVersionEntrypointEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().entrypoint = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.entrypoint = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `handlers`.\n"]
    pub fn set_handlers(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppEngineFlexibleAppVersionHandlersEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().handlers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.handlers = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `liveness_check`.\n"]
    pub fn set_liveness_check(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppEngineFlexibleAppVersionLivenessCheckEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().liveness_check = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.liveness_check = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `manual_scaling`.\n"]
    pub fn set_manual_scaling(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppEngineFlexibleAppVersionManualScalingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().manual_scaling = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.manual_scaling = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network`.\n"]
    pub fn set_network(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppEngineFlexibleAppVersionNetworkEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `readiness_check`.\n"]
    pub fn set_readiness_check(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppEngineFlexibleAppVersionReadinessCheckEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().readiness_check = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.readiness_check = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppEngineFlexibleAppVersionResourcesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().resources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.resources = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<AppEngineFlexibleAppVersionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_access_connector`.\n"]
    pub fn set_vpc_access_connector(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppEngineFlexibleAppVersionVpcAccessConnectorEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_access_connector = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_access_connector = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `beta_settings` after provisioning.\nMetadata settings that are supplied to this version to enable beta runtime features."]
    pub fn beta_settings(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.beta_settings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_expiration` after provisioning.\nDuration that static files should be cached by web proxies and browsers.\nOnly applicable if the corresponding StaticFilesHandler does not specify its own expiration time."]
    pub fn default_expiration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_expiration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_service_on_destroy` after provisioning.\nIf set to 'true', the service will be deleted if it is the last version."]
    pub fn delete_service_on_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_service_on_destroy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `env_variables` after provisioning.\nEnvironment variables available to the application.  As these are not returned in the API request, Terraform will not detect any changes made outside of the Terraform config."]
    pub fn env_variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.env_variables", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `inbound_services` after provisioning.\nA list of the types of messages that this application is able to receive. Possible values: [\"INBOUND_SERVICE_MAIL\", \"INBOUND_SERVICE_MAIL_BOUNCE\", \"INBOUND_SERVICE_XMPP_ERROR\", \"INBOUND_SERVICE_XMPP_MESSAGE\", \"INBOUND_SERVICE_XMPP_SUBSCRIBE\", \"INBOUND_SERVICE_XMPP_PRESENCE\", \"INBOUND_SERVICE_CHANNEL_PRESENCE\", \"INBOUND_SERVICE_WARMUP\"]"]
    pub fn inbound_services(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.inbound_services", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_class` after provisioning.\nInstance class that is used to run this version. Valid values are\nAutomaticScaling: F1, F2, F4, F4_1G\nManualScaling: B1, B2, B4, B8, B4_1G\nDefaults to F1 for AutomaticScaling and B1 for ManualScaling."]
    pub fn instance_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_class", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nFull path to the Version resource in the API. Example, \"v1\"."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `nobuild_files_regex` after provisioning.\nFiles that match this pattern will not be built into this version. Only applicable for Go runtimes."]
    pub fn nobuild_files_regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nobuild_files_regex", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `noop_on_destroy` after provisioning.\nIf set to 'true', the application version will not be deleted."]
    pub fn noop_on_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.noop_on_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `runtime` after provisioning.\nDesired runtime. Example python27."]
    pub fn runtime(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.runtime", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `runtime_api_version` after provisioning.\nThe version of the API in the given runtime environment.\nPlease see the app.yaml reference for valid values at 'https://cloud.google.com/appengine/docs/standard/<language>/config/appref'\\\nSubstitute '<language>' with 'python', 'java', 'php', 'ruby', 'go' or 'nodejs'."]
    pub fn runtime_api_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.runtime_api_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `runtime_channel` after provisioning.\nThe channel of the runtime to use. Only available for some runtimes."]
    pub fn runtime_channel(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.runtime_channel", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `runtime_main_executable_path` after provisioning.\nThe path or name of the app's main executable."]
    pub fn runtime_main_executable_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.runtime_main_executable_path", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nAppEngine service resource. Can contain numbers, letters, and hyphens."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nThe identity that the deployed version will run as. Admin API will use the App Engine Appspot service account as\ndefault if this field is neither provided in app.yaml file nor through CLI flag."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `serving_status` after provisioning.\nCurrent serving status of this version. Only the versions with a SERVING status create instances and can be billed. Default value: \"SERVING\" Possible values: [\"SERVING\", \"STOPPED\"]"]
    pub fn serving_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serving_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version_id` after provisioning.\nRelative name of the version within the service. For example, 'v1'. Version names can contain only lowercase letters, numbers, or hyphens.\nReserved names,\"default\", \"latest\", and any name with the prefix \"ah-\"."]
    pub fn version_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `api_config` after provisioning.\n"]
    pub fn api_config(&self) -> terrars::ListRef<AppEngineFlexibleAppVersionApiConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `automatic_scaling` after provisioning.\n"]
    pub fn automatic_scaling(&self) -> terrars::ListRef<AppEngineFlexibleAppVersionAutomaticScalingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.automatic_scaling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment` after provisioning.\n"]
    pub fn deployment(&self) -> terrars::ListRef<AppEngineFlexibleAppVersionDeploymentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoints_api_service` after provisioning.\n"]
    pub fn endpoints_api_service(&self) -> terrars::ListRef<AppEngineFlexibleAppVersionEndpointsApiServiceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoints_api_service", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entrypoint` after provisioning.\n"]
    pub fn entrypoint(&self) -> terrars::ListRef<AppEngineFlexibleAppVersionEntrypointElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entrypoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `handlers` after provisioning.\n"]
    pub fn handlers(&self) -> terrars::ListRef<AppEngineFlexibleAppVersionHandlersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.handlers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `liveness_check` after provisioning.\n"]
    pub fn liveness_check(&self) -> terrars::ListRef<AppEngineFlexibleAppVersionLivenessCheckElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.liveness_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `manual_scaling` after provisioning.\n"]
    pub fn manual_scaling(&self) -> terrars::ListRef<AppEngineFlexibleAppVersionManualScalingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.manual_scaling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> terrars::ListRef<AppEngineFlexibleAppVersionNetworkElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `readiness_check` after provisioning.\n"]
    pub fn readiness_check(&self) -> terrars::ListRef<AppEngineFlexibleAppVersionReadinessCheckElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.readiness_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::ListRef<AppEngineFlexibleAppVersionResourcesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AppEngineFlexibleAppVersionTimeoutsElRef {
        AppEngineFlexibleAppVersionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_access_connector` after provisioning.\n"]
    pub fn vpc_access_connector(&self) -> terrars::ListRef<AppEngineFlexibleAppVersionVpcAccessConnectorElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_access_connector", self.extract_ref()))
    }
}

impl terrars::Referable for AppEngineFlexibleAppVersion {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AppEngineFlexibleAppVersion { }

impl terrars::ToListMappable for AppEngineFlexibleAppVersion {
    type O = terrars::ListRef<AppEngineFlexibleAppVersionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AppEngineFlexibleAppVersion_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_app_engine_flexible_app_version".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppEngineFlexibleAppVersion {
    pub tf_id: std::string::String,
    #[doc = "Desired runtime. Example python27."]
    pub runtime: terrars::PrimField<std::string::String>,
    #[doc = "AppEngine service resource. Can contain numbers, letters, and hyphens."]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildAppEngineFlexibleAppVersion {
    pub fn build(self, stack: &mut terrars::Stack) -> AppEngineFlexibleAppVersion {
        let out = AppEngineFlexibleAppVersion(std::rc::Rc::new(AppEngineFlexibleAppVersion_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AppEngineFlexibleAppVersionData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                beta_settings: core::default::Default::default(),
                default_expiration: core::default::Default::default(),
                delete_service_on_destroy: core::default::Default::default(),
                env_variables: core::default::Default::default(),
                id: core::default::Default::default(),
                inbound_services: core::default::Default::default(),
                instance_class: core::default::Default::default(),
                nobuild_files_regex: core::default::Default::default(),
                noop_on_destroy: core::default::Default::default(),
                project: core::default::Default::default(),
                runtime: self.runtime,
                runtime_api_version: core::default::Default::default(),
                runtime_channel: core::default::Default::default(),
                runtime_main_executable_path: core::default::Default::default(),
                service: self.service,
                service_account: core::default::Default::default(),
                serving_status: core::default::Default::default(),
                version_id: core::default::Default::default(),
                api_config: core::default::Default::default(),
                automatic_scaling: core::default::Default::default(),
                deployment: core::default::Default::default(),
                endpoints_api_service: core::default::Default::default(),
                entrypoint: core::default::Default::default(),
                handlers: core::default::Default::default(),
                liveness_check: core::default::Default::default(),
                manual_scaling: core::default::Default::default(),
                network: core::default::Default::default(),
                readiness_check: core::default::Default::default(),
                resources: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vpc_access_connector: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppEngineFlexibleAppVersionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineFlexibleAppVersionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AppEngineFlexibleAppVersionRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `beta_settings` after provisioning.\nMetadata settings that are supplied to this version to enable beta runtime features."]
    pub fn beta_settings(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.beta_settings", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_expiration` after provisioning.\nDuration that static files should be cached by web proxies and browsers.\nOnly applicable if the corresponding StaticFilesHandler does not specify its own expiration time."]
    pub fn default_expiration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_expiration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_service_on_destroy` after provisioning.\nIf set to 'true', the service will be deleted if it is the last version."]
    pub fn delete_service_on_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_service_on_destroy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `env_variables` after provisioning.\nEnvironment variables available to the application.  As these are not returned in the API request, Terraform will not detect any changes made outside of the Terraform config."]
    pub fn env_variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.env_variables", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `inbound_services` after provisioning.\nA list of the types of messages that this application is able to receive. Possible values: [\"INBOUND_SERVICE_MAIL\", \"INBOUND_SERVICE_MAIL_BOUNCE\", \"INBOUND_SERVICE_XMPP_ERROR\", \"INBOUND_SERVICE_XMPP_MESSAGE\", \"INBOUND_SERVICE_XMPP_SUBSCRIBE\", \"INBOUND_SERVICE_XMPP_PRESENCE\", \"INBOUND_SERVICE_CHANNEL_PRESENCE\", \"INBOUND_SERVICE_WARMUP\"]"]
    pub fn inbound_services(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.inbound_services", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_class` after provisioning.\nInstance class that is used to run this version. Valid values are\nAutomaticScaling: F1, F2, F4, F4_1G\nManualScaling: B1, B2, B4, B8, B4_1G\nDefaults to F1 for AutomaticScaling and B1 for ManualScaling."]
    pub fn instance_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_class", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nFull path to the Version resource in the API. Example, \"v1\"."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `nobuild_files_regex` after provisioning.\nFiles that match this pattern will not be built into this version. Only applicable for Go runtimes."]
    pub fn nobuild_files_regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nobuild_files_regex", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `noop_on_destroy` after provisioning.\nIf set to 'true', the application version will not be deleted."]
    pub fn noop_on_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.noop_on_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `runtime` after provisioning.\nDesired runtime. Example python27."]
    pub fn runtime(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.runtime", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `runtime_api_version` after provisioning.\nThe version of the API in the given runtime environment.\nPlease see the app.yaml reference for valid values at 'https://cloud.google.com/appengine/docs/standard/<language>/config/appref'\\\nSubstitute '<language>' with 'python', 'java', 'php', 'ruby', 'go' or 'nodejs'."]
    pub fn runtime_api_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.runtime_api_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `runtime_channel` after provisioning.\nThe channel of the runtime to use. Only available for some runtimes."]
    pub fn runtime_channel(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.runtime_channel", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `runtime_main_executable_path` after provisioning.\nThe path or name of the app's main executable."]
    pub fn runtime_main_executable_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.runtime_main_executable_path", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nAppEngine service resource. Can contain numbers, letters, and hyphens."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nThe identity that the deployed version will run as. Admin API will use the App Engine Appspot service account as\ndefault if this field is neither provided in app.yaml file nor through CLI flag."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `serving_status` after provisioning.\nCurrent serving status of this version. Only the versions with a SERVING status create instances and can be billed. Default value: \"SERVING\" Possible values: [\"SERVING\", \"STOPPED\"]"]
    pub fn serving_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serving_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version_id` after provisioning.\nRelative name of the version within the service. For example, 'v1'. Version names can contain only lowercase letters, numbers, or hyphens.\nReserved names,\"default\", \"latest\", and any name with the prefix \"ah-\"."]
    pub fn version_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `api_config` after provisioning.\n"]
    pub fn api_config(&self) -> terrars::ListRef<AppEngineFlexibleAppVersionApiConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `automatic_scaling` after provisioning.\n"]
    pub fn automatic_scaling(&self) -> terrars::ListRef<AppEngineFlexibleAppVersionAutomaticScalingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.automatic_scaling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment` after provisioning.\n"]
    pub fn deployment(&self) -> terrars::ListRef<AppEngineFlexibleAppVersionDeploymentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoints_api_service` after provisioning.\n"]
    pub fn endpoints_api_service(&self) -> terrars::ListRef<AppEngineFlexibleAppVersionEndpointsApiServiceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoints_api_service", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entrypoint` after provisioning.\n"]
    pub fn entrypoint(&self) -> terrars::ListRef<AppEngineFlexibleAppVersionEntrypointElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entrypoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `handlers` after provisioning.\n"]
    pub fn handlers(&self) -> terrars::ListRef<AppEngineFlexibleAppVersionHandlersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.handlers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `liveness_check` after provisioning.\n"]
    pub fn liveness_check(&self) -> terrars::ListRef<AppEngineFlexibleAppVersionLivenessCheckElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.liveness_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `manual_scaling` after provisioning.\n"]
    pub fn manual_scaling(&self) -> terrars::ListRef<AppEngineFlexibleAppVersionManualScalingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.manual_scaling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> terrars::ListRef<AppEngineFlexibleAppVersionNetworkElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `readiness_check` after provisioning.\n"]
    pub fn readiness_check(&self) -> terrars::ListRef<AppEngineFlexibleAppVersionReadinessCheckElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.readiness_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::ListRef<AppEngineFlexibleAppVersionResourcesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AppEngineFlexibleAppVersionTimeoutsElRef {
        AppEngineFlexibleAppVersionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_access_connector` after provisioning.\n"]
    pub fn vpc_access_connector(&self) -> terrars::ListRef<AppEngineFlexibleAppVersionVpcAccessConnectorElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_access_connector", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AppEngineFlexibleAppVersionApiConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_fail_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    login: std::option::Option<terrars::PrimField<std::string::String>>,
    script: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppEngineFlexibleAppVersionApiConfigEl {
    #[doc =
        "Set the field `auth_fail_action`.\nAction to take when users access resources that require authentication. Default value: \"AUTH_FAIL_ACTION_REDIRECT\" Possible values: [\"AUTH_FAIL_ACTION_REDIRECT\", \"AUTH_FAIL_ACTION_UNAUTHORIZED\"]"]
    pub fn set_auth_fail_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth_fail_action = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `login`.\nLevel of login required to access this resource. Default value: \"LOGIN_OPTIONAL\" Possible values: [\"LOGIN_OPTIONAL\", \"LOGIN_ADMIN\", \"LOGIN_REQUIRED\"]"]
    pub fn set_login(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.login = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `security_level`.\nSecurity (HTTPS) enforcement for this URL. Possible values: [\"SECURE_DEFAULT\", \"SECURE_NEVER\", \"SECURE_OPTIONAL\", \"SECURE_ALWAYS\"]"]
    pub fn set_security_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.security_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `url`.\nURL to serve the endpoint at."]
    pub fn set_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppEngineFlexibleAppVersionApiConfigEl {
    type O = terrars::BlockAssignable<AppEngineFlexibleAppVersionApiConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineFlexibleAppVersionApiConfigEl {
    #[doc = "Path to the script from the application root directory."]
    pub script: terrars::PrimField<std::string::String>,
}

impl BuildAppEngineFlexibleAppVersionApiConfigEl {
    pub fn build(self) -> AppEngineFlexibleAppVersionApiConfigEl {
        AppEngineFlexibleAppVersionApiConfigEl {
            auth_fail_action: core::default::Default::default(),
            login: core::default::Default::default(),
            script: self.script,
            security_level: core::default::Default::default(),
            url: core::default::Default::default(),
        }
    }
}

pub struct AppEngineFlexibleAppVersionApiConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineFlexibleAppVersionApiConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppEngineFlexibleAppVersionApiConfigElRef {
        AppEngineFlexibleAppVersionApiConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineFlexibleAppVersionApiConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `auth_fail_action` after provisioning.\nAction to take when users access resources that require authentication. Default value: \"AUTH_FAIL_ACTION_REDIRECT\" Possible values: [\"AUTH_FAIL_ACTION_REDIRECT\", \"AUTH_FAIL_ACTION_UNAUTHORIZED\"]"]
    pub fn auth_fail_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_fail_action", self.base))
    }

    #[doc =
        "Get a reference to the value of field `login` after provisioning.\nLevel of login required to access this resource. Default value: \"LOGIN_OPTIONAL\" Possible values: [\"LOGIN_OPTIONAL\", \"LOGIN_ADMIN\", \"LOGIN_REQUIRED\"]"]
    pub fn login(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.login", self.base))
    }

    #[doc =
        "Get a reference to the value of field `script` after provisioning.\nPath to the script from the application root directory."]
    pub fn script(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.script", self.base))
    }

    #[doc =
        "Get a reference to the value of field `security_level` after provisioning.\nSecurity (HTTPS) enforcement for this URL. Possible values: [\"SECURE_DEFAULT\", \"SECURE_NEVER\", \"SECURE_OPTIONAL\", \"SECURE_ALWAYS\"]"]
    pub fn security_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_level", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\nURL to serve the endpoint at."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize)]
pub struct AppEngineFlexibleAppVersionAutomaticScalingElCpuUtilizationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aggregation_window_length: std::option::Option<terrars::PrimField<std::string::String>>,
    target_utilization: terrars::PrimField<f64>,
}

impl AppEngineFlexibleAppVersionAutomaticScalingElCpuUtilizationEl {
    #[doc = "Set the field `aggregation_window_length`.\nPeriod of time over which CPU utilization is calculated."]
    pub fn set_aggregation_window_length(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.aggregation_window_length = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppEngineFlexibleAppVersionAutomaticScalingElCpuUtilizationEl {
    type O = terrars::BlockAssignable<AppEngineFlexibleAppVersionAutomaticScalingElCpuUtilizationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineFlexibleAppVersionAutomaticScalingElCpuUtilizationEl {
    #[doc = "Target CPU utilization ratio to maintain when scaling. Must be between 0 and 1."]
    pub target_utilization: terrars::PrimField<f64>,
}

impl BuildAppEngineFlexibleAppVersionAutomaticScalingElCpuUtilizationEl {
    pub fn build(self) -> AppEngineFlexibleAppVersionAutomaticScalingElCpuUtilizationEl {
        AppEngineFlexibleAppVersionAutomaticScalingElCpuUtilizationEl {
            aggregation_window_length: core::default::Default::default(),
            target_utilization: self.target_utilization,
        }
    }
}

pub struct AppEngineFlexibleAppVersionAutomaticScalingElCpuUtilizationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineFlexibleAppVersionAutomaticScalingElCpuUtilizationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppEngineFlexibleAppVersionAutomaticScalingElCpuUtilizationElRef {
        AppEngineFlexibleAppVersionAutomaticScalingElCpuUtilizationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineFlexibleAppVersionAutomaticScalingElCpuUtilizationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `aggregation_window_length` after provisioning.\nPeriod of time over which CPU utilization is calculated."]
    pub fn aggregation_window_length(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aggregation_window_length", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_utilization` after provisioning.\nTarget CPU utilization ratio to maintain when scaling. Must be between 0 and 1."]
    pub fn target_utilization(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_utilization", self.base))
    }
}

#[derive(Serialize)]
pub struct AppEngineFlexibleAppVersionAutomaticScalingElDiskUtilizationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_read_bytes_per_second: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_read_ops_per_second: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_write_bytes_per_second: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_write_ops_per_second: std::option::Option<terrars::PrimField<f64>>,
}

impl AppEngineFlexibleAppVersionAutomaticScalingElDiskUtilizationEl {
    #[doc = "Set the field `target_read_bytes_per_second`.\nTarget bytes read per second."]
    pub fn set_target_read_bytes_per_second(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_read_bytes_per_second = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_read_ops_per_second`.\nTarget ops read per seconds."]
    pub fn set_target_read_ops_per_second(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_read_ops_per_second = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_write_bytes_per_second`.\nTarget bytes written per second."]
    pub fn set_target_write_bytes_per_second(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_write_bytes_per_second = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_write_ops_per_second`.\nTarget ops written per second."]
    pub fn set_target_write_ops_per_second(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_write_ops_per_second = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppEngineFlexibleAppVersionAutomaticScalingElDiskUtilizationEl {
    type O = terrars::BlockAssignable<AppEngineFlexibleAppVersionAutomaticScalingElDiskUtilizationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineFlexibleAppVersionAutomaticScalingElDiskUtilizationEl {}

impl BuildAppEngineFlexibleAppVersionAutomaticScalingElDiskUtilizationEl {
    pub fn build(self) -> AppEngineFlexibleAppVersionAutomaticScalingElDiskUtilizationEl {
        AppEngineFlexibleAppVersionAutomaticScalingElDiskUtilizationEl {
            target_read_bytes_per_second: core::default::Default::default(),
            target_read_ops_per_second: core::default::Default::default(),
            target_write_bytes_per_second: core::default::Default::default(),
            target_write_ops_per_second: core::default::Default::default(),
        }
    }
}

pub struct AppEngineFlexibleAppVersionAutomaticScalingElDiskUtilizationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineFlexibleAppVersionAutomaticScalingElDiskUtilizationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppEngineFlexibleAppVersionAutomaticScalingElDiskUtilizationElRef {
        AppEngineFlexibleAppVersionAutomaticScalingElDiskUtilizationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineFlexibleAppVersionAutomaticScalingElDiskUtilizationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `target_read_bytes_per_second` after provisioning.\nTarget bytes read per second."]
    pub fn target_read_bytes_per_second(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_read_bytes_per_second", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_read_ops_per_second` after provisioning.\nTarget ops read per seconds."]
    pub fn target_read_ops_per_second(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_read_ops_per_second", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_write_bytes_per_second` after provisioning.\nTarget bytes written per second."]
    pub fn target_write_bytes_per_second(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_write_bytes_per_second", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_write_ops_per_second` after provisioning.\nTarget ops written per second."]
    pub fn target_write_ops_per_second(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_write_ops_per_second", self.base))
    }
}

#[derive(Serialize)]
pub struct AppEngineFlexibleAppVersionAutomaticScalingElNetworkUtilizationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_received_bytes_per_second: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_received_packets_per_second: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_sent_bytes_per_second: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_sent_packets_per_second: std::option::Option<terrars::PrimField<f64>>,
}

impl AppEngineFlexibleAppVersionAutomaticScalingElNetworkUtilizationEl {
    #[doc = "Set the field `target_received_bytes_per_second`.\nTarget bytes received per second."]
    pub fn set_target_received_bytes_per_second(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_received_bytes_per_second = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_received_packets_per_second`.\nTarget packets received per second."]
    pub fn set_target_received_packets_per_second(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.target_received_packets_per_second = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_sent_bytes_per_second`.\nTarget bytes sent per second."]
    pub fn set_target_sent_bytes_per_second(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_sent_bytes_per_second = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_sent_packets_per_second`.\nTarget packets sent per second."]
    pub fn set_target_sent_packets_per_second(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_sent_packets_per_second = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppEngineFlexibleAppVersionAutomaticScalingElNetworkUtilizationEl {
    type O = terrars::BlockAssignable<AppEngineFlexibleAppVersionAutomaticScalingElNetworkUtilizationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineFlexibleAppVersionAutomaticScalingElNetworkUtilizationEl {}

impl BuildAppEngineFlexibleAppVersionAutomaticScalingElNetworkUtilizationEl {
    pub fn build(self) -> AppEngineFlexibleAppVersionAutomaticScalingElNetworkUtilizationEl {
        AppEngineFlexibleAppVersionAutomaticScalingElNetworkUtilizationEl {
            target_received_bytes_per_second: core::default::Default::default(),
            target_received_packets_per_second: core::default::Default::default(),
            target_sent_bytes_per_second: core::default::Default::default(),
            target_sent_packets_per_second: core::default::Default::default(),
        }
    }
}

pub struct AppEngineFlexibleAppVersionAutomaticScalingElNetworkUtilizationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineFlexibleAppVersionAutomaticScalingElNetworkUtilizationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppEngineFlexibleAppVersionAutomaticScalingElNetworkUtilizationElRef {
        AppEngineFlexibleAppVersionAutomaticScalingElNetworkUtilizationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineFlexibleAppVersionAutomaticScalingElNetworkUtilizationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `target_received_bytes_per_second` after provisioning.\nTarget bytes received per second."]
    pub fn target_received_bytes_per_second(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_received_bytes_per_second", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_received_packets_per_second` after provisioning.\nTarget packets received per second."]
    pub fn target_received_packets_per_second(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_received_packets_per_second", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_sent_bytes_per_second` after provisioning.\nTarget bytes sent per second."]
    pub fn target_sent_bytes_per_second(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_sent_bytes_per_second", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_sent_packets_per_second` after provisioning.\nTarget packets sent per second."]
    pub fn target_sent_packets_per_second(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_sent_packets_per_second", self.base))
    }
}

#[derive(Serialize)]
pub struct AppEngineFlexibleAppVersionAutomaticScalingElRequestUtilizationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_concurrent_requests: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_request_count_per_second: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppEngineFlexibleAppVersionAutomaticScalingElRequestUtilizationEl {
    #[doc = "Set the field `target_concurrent_requests`.\nTarget number of concurrent requests."]
    pub fn set_target_concurrent_requests(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_concurrent_requests = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_request_count_per_second`.\nTarget requests per second."]
    pub fn set_target_request_count_per_second(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.target_request_count_per_second = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppEngineFlexibleAppVersionAutomaticScalingElRequestUtilizationEl {
    type O = terrars::BlockAssignable<AppEngineFlexibleAppVersionAutomaticScalingElRequestUtilizationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineFlexibleAppVersionAutomaticScalingElRequestUtilizationEl {}

impl BuildAppEngineFlexibleAppVersionAutomaticScalingElRequestUtilizationEl {
    pub fn build(self) -> AppEngineFlexibleAppVersionAutomaticScalingElRequestUtilizationEl {
        AppEngineFlexibleAppVersionAutomaticScalingElRequestUtilizationEl {
            target_concurrent_requests: core::default::Default::default(),
            target_request_count_per_second: core::default::Default::default(),
        }
    }
}

pub struct AppEngineFlexibleAppVersionAutomaticScalingElRequestUtilizationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineFlexibleAppVersionAutomaticScalingElRequestUtilizationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppEngineFlexibleAppVersionAutomaticScalingElRequestUtilizationElRef {
        AppEngineFlexibleAppVersionAutomaticScalingElRequestUtilizationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineFlexibleAppVersionAutomaticScalingElRequestUtilizationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `target_concurrent_requests` after provisioning.\nTarget number of concurrent requests."]
    pub fn target_concurrent_requests(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_concurrent_requests", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_request_count_per_second` after provisioning.\nTarget requests per second."]
    pub fn target_request_count_per_second(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_request_count_per_second", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppEngineFlexibleAppVersionAutomaticScalingElDynamic {
    cpu_utilization: std
    ::option
    ::Option<terrars::DynamicBlock<AppEngineFlexibleAppVersionAutomaticScalingElCpuUtilizationEl>>,
    disk_utilization: std
    ::option
    ::Option<terrars::DynamicBlock<AppEngineFlexibleAppVersionAutomaticScalingElDiskUtilizationEl>>,
    network_utilization: std
    ::option
    ::Option<terrars::DynamicBlock<AppEngineFlexibleAppVersionAutomaticScalingElNetworkUtilizationEl>>,
    request_utilization: std
    ::option
    ::Option<terrars::DynamicBlock<AppEngineFlexibleAppVersionAutomaticScalingElRequestUtilizationEl>>,
}

#[derive(Serialize)]
pub struct AppEngineFlexibleAppVersionAutomaticScalingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cool_down_period: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_concurrent_requests: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_idle_instances: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_pending_latency: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_total_instances: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_idle_instances: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_pending_latency: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_total_instances: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_utilization: std::option::Option<std::vec::Vec<AppEngineFlexibleAppVersionAutomaticScalingElCpuUtilizationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_utilization: std::option::Option<std::vec::Vec<AppEngineFlexibleAppVersionAutomaticScalingElDiskUtilizationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_utilization: std
    ::option
    ::Option<std::vec::Vec<AppEngineFlexibleAppVersionAutomaticScalingElNetworkUtilizationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_utilization: std
    ::option
    ::Option<std::vec::Vec<AppEngineFlexibleAppVersionAutomaticScalingElRequestUtilizationEl>>,
    dynamic: AppEngineFlexibleAppVersionAutomaticScalingElDynamic,
}

impl AppEngineFlexibleAppVersionAutomaticScalingEl {
    #[doc =
        "Set the field `cool_down_period`.\nThe time period that the Autoscaler should wait before it starts collecting information from a new instance.\nThis prevents the autoscaler from collecting information when the instance is initializing,\nduring which the collected usage would not be reliable. Default: 120s"]
    pub fn set_cool_down_period(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cool_down_period = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_concurrent_requests`.\nNumber of concurrent requests an automatic scaling instance can accept before the scheduler spawns a new instance.\n\nDefaults to a runtime-specific value."]
    pub fn set_max_concurrent_requests(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_concurrent_requests = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_idle_instances`.\nMaximum number of idle instances that should be maintained for this version."]
    pub fn set_max_idle_instances(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_idle_instances = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_pending_latency`.\nMaximum amount of time that a request should wait in the pending queue before starting a new instance to handle it."]
    pub fn set_max_pending_latency(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.max_pending_latency = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_total_instances`.\nMaximum number of instances that should be started to handle requests for this version. Default: 20"]
    pub fn set_max_total_instances(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_total_instances = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_idle_instances`.\nMinimum number of idle instances that should be maintained for this version. Only applicable for the default version of a service."]
    pub fn set_min_idle_instances(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_idle_instances = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_pending_latency`.\nMinimum amount of time a request should wait in the pending queue before starting a new instance to handle it."]
    pub fn set_min_pending_latency(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.min_pending_latency = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_total_instances`.\nMinimum number of running instances that should be maintained for this version. Default: 2"]
    pub fn set_min_total_instances(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_total_instances = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu_utilization`.\n"]
    pub fn set_cpu_utilization(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppEngineFlexibleAppVersionAutomaticScalingElCpuUtilizationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cpu_utilization = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cpu_utilization = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `disk_utilization`.\n"]
    pub fn set_disk_utilization(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppEngineFlexibleAppVersionAutomaticScalingElDiskUtilizationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.disk_utilization = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.disk_utilization = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_utilization`.\n"]
    pub fn set_network_utilization(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AppEngineFlexibleAppVersionAutomaticScalingElNetworkUtilizationEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_utilization = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_utilization = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `request_utilization`.\n"]
    pub fn set_request_utilization(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AppEngineFlexibleAppVersionAutomaticScalingElRequestUtilizationEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.request_utilization = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.request_utilization = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppEngineFlexibleAppVersionAutomaticScalingEl {
    type O = terrars::BlockAssignable<AppEngineFlexibleAppVersionAutomaticScalingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineFlexibleAppVersionAutomaticScalingEl {}

impl BuildAppEngineFlexibleAppVersionAutomaticScalingEl {
    pub fn build(self) -> AppEngineFlexibleAppVersionAutomaticScalingEl {
        AppEngineFlexibleAppVersionAutomaticScalingEl {
            cool_down_period: core::default::Default::default(),
            max_concurrent_requests: core::default::Default::default(),
            max_idle_instances: core::default::Default::default(),
            max_pending_latency: core::default::Default::default(),
            max_total_instances: core::default::Default::default(),
            min_idle_instances: core::default::Default::default(),
            min_pending_latency: core::default::Default::default(),
            min_total_instances: core::default::Default::default(),
            cpu_utilization: core::default::Default::default(),
            disk_utilization: core::default::Default::default(),
            network_utilization: core::default::Default::default(),
            request_utilization: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppEngineFlexibleAppVersionAutomaticScalingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineFlexibleAppVersionAutomaticScalingElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppEngineFlexibleAppVersionAutomaticScalingElRef {
        AppEngineFlexibleAppVersionAutomaticScalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineFlexibleAppVersionAutomaticScalingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cool_down_period` after provisioning.\nThe time period that the Autoscaler should wait before it starts collecting information from a new instance.\nThis prevents the autoscaler from collecting information when the instance is initializing,\nduring which the collected usage would not be reliable. Default: 120s"]
    pub fn cool_down_period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cool_down_period", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_concurrent_requests` after provisioning.\nNumber of concurrent requests an automatic scaling instance can accept before the scheduler spawns a new instance.\n\nDefaults to a runtime-specific value."]
    pub fn max_concurrent_requests(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_concurrent_requests", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_idle_instances` after provisioning.\nMaximum number of idle instances that should be maintained for this version."]
    pub fn max_idle_instances(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_idle_instances", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_pending_latency` after provisioning.\nMaximum amount of time that a request should wait in the pending queue before starting a new instance to handle it."]
    pub fn max_pending_latency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_pending_latency", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_total_instances` after provisioning.\nMaximum number of instances that should be started to handle requests for this version. Default: 20"]
    pub fn max_total_instances(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_total_instances", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_idle_instances` after provisioning.\nMinimum number of idle instances that should be maintained for this version. Only applicable for the default version of a service."]
    pub fn min_idle_instances(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_idle_instances", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_pending_latency` after provisioning.\nMinimum amount of time a request should wait in the pending queue before starting a new instance to handle it."]
    pub fn min_pending_latency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_pending_latency", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_total_instances` after provisioning.\nMinimum number of running instances that should be maintained for this version. Default: 2"]
    pub fn min_total_instances(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_total_instances", self.base))
    }

    #[doc = "Get a reference to the value of field `cpu_utilization` after provisioning.\n"]
    pub fn cpu_utilization(&self) -> terrars::ListRef<AppEngineFlexibleAppVersionAutomaticScalingElCpuUtilizationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cpu_utilization", self.base))
    }

    #[doc = "Get a reference to the value of field `disk_utilization` after provisioning.\n"]
    pub fn disk_utilization(
        &self,
    ) -> terrars::ListRef<AppEngineFlexibleAppVersionAutomaticScalingElDiskUtilizationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disk_utilization", self.base))
    }

    #[doc = "Get a reference to the value of field `network_utilization` after provisioning.\n"]
    pub fn network_utilization(
        &self,
    ) -> terrars::ListRef<AppEngineFlexibleAppVersionAutomaticScalingElNetworkUtilizationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_utilization", self.base))
    }

    #[doc = "Get a reference to the value of field `request_utilization` after provisioning.\n"]
    pub fn request_utilization(
        &self,
    ) -> terrars::ListRef<AppEngineFlexibleAppVersionAutomaticScalingElRequestUtilizationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_utilization", self.base))
    }
}

#[derive(Serialize)]
pub struct AppEngineFlexibleAppVersionDeploymentElCloudBuildOptionsEl {
    app_yaml_path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_build_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppEngineFlexibleAppVersionDeploymentElCloudBuildOptionsEl {
    #[doc =
        "Set the field `cloud_build_timeout`.\nThe Cloud Build timeout used as part of any dependent builds performed by version creation. Defaults to 10 minutes.\n\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn set_cloud_build_timeout(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cloud_build_timeout = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppEngineFlexibleAppVersionDeploymentElCloudBuildOptionsEl {
    type O = terrars::BlockAssignable<AppEngineFlexibleAppVersionDeploymentElCloudBuildOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineFlexibleAppVersionDeploymentElCloudBuildOptionsEl {
    #[doc = "Path to the yaml file used in deployment, used to determine runtime configuration details."]
    pub app_yaml_path: terrars::PrimField<std::string::String>,
}

impl BuildAppEngineFlexibleAppVersionDeploymentElCloudBuildOptionsEl {
    pub fn build(self) -> AppEngineFlexibleAppVersionDeploymentElCloudBuildOptionsEl {
        AppEngineFlexibleAppVersionDeploymentElCloudBuildOptionsEl {
            app_yaml_path: self.app_yaml_path,
            cloud_build_timeout: core::default::Default::default(),
        }
    }
}

pub struct AppEngineFlexibleAppVersionDeploymentElCloudBuildOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineFlexibleAppVersionDeploymentElCloudBuildOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppEngineFlexibleAppVersionDeploymentElCloudBuildOptionsElRef {
        AppEngineFlexibleAppVersionDeploymentElCloudBuildOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineFlexibleAppVersionDeploymentElCloudBuildOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `app_yaml_path` after provisioning.\nPath to the yaml file used in deployment, used to determine runtime configuration details."]
    pub fn app_yaml_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_yaml_path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cloud_build_timeout` after provisioning.\nThe Cloud Build timeout used as part of any dependent builds performed by version creation. Defaults to 10 minutes.\n\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn cloud_build_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_build_timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct AppEngineFlexibleAppVersionDeploymentElContainerEl {
    image: terrars::PrimField<std::string::String>,
}

impl AppEngineFlexibleAppVersionDeploymentElContainerEl { }

impl terrars::ToListMappable for AppEngineFlexibleAppVersionDeploymentElContainerEl {
    type O = terrars::BlockAssignable<AppEngineFlexibleAppVersionDeploymentElContainerEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineFlexibleAppVersionDeploymentElContainerEl {
    #[doc =
        "URI to the hosted container image in Google Container Registry. The URI must be fully qualified and include a tag or digest.\nExamples: \"gcr.io/my-project/image:tag\" or \"gcr.io/my-project/image@digest\""]
    pub image: terrars::PrimField<std::string::String>,
}

impl BuildAppEngineFlexibleAppVersionDeploymentElContainerEl {
    pub fn build(self) -> AppEngineFlexibleAppVersionDeploymentElContainerEl {
        AppEngineFlexibleAppVersionDeploymentElContainerEl { image: self.image }
    }
}

pub struct AppEngineFlexibleAppVersionDeploymentElContainerElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineFlexibleAppVersionDeploymentElContainerElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppEngineFlexibleAppVersionDeploymentElContainerElRef {
        AppEngineFlexibleAppVersionDeploymentElContainerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineFlexibleAppVersionDeploymentElContainerElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `image` after provisioning.\nURI to the hosted container image in Google Container Registry. The URI must be fully qualified and include a tag or digest.\nExamples: \"gcr.io/my-project/image:tag\" or \"gcr.io/my-project/image@digest\""]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }
}

#[derive(Serialize)]
pub struct AppEngineFlexibleAppVersionDeploymentElFilesEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sha1_sum: std::option::Option<terrars::PrimField<std::string::String>>,
    source_url: terrars::PrimField<std::string::String>,
}

impl AppEngineFlexibleAppVersionDeploymentElFilesEl {
    #[doc = "Set the field `sha1_sum`.\nSHA1 checksum of the file"]
    pub fn set_sha1_sum(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sha1_sum = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppEngineFlexibleAppVersionDeploymentElFilesEl {
    type O = terrars::BlockAssignable<AppEngineFlexibleAppVersionDeploymentElFilesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineFlexibleAppVersionDeploymentElFilesEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "Source URL"]
    pub source_url: terrars::PrimField<std::string::String>,
}

impl BuildAppEngineFlexibleAppVersionDeploymentElFilesEl {
    pub fn build(self) -> AppEngineFlexibleAppVersionDeploymentElFilesEl {
        AppEngineFlexibleAppVersionDeploymentElFilesEl {
            name: self.name,
            sha1_sum: core::default::Default::default(),
            source_url: self.source_url,
        }
    }
}

pub struct AppEngineFlexibleAppVersionDeploymentElFilesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineFlexibleAppVersionDeploymentElFilesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppEngineFlexibleAppVersionDeploymentElFilesElRef {
        AppEngineFlexibleAppVersionDeploymentElFilesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineFlexibleAppVersionDeploymentElFilesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `sha1_sum` after provisioning.\nSHA1 checksum of the file"]
    pub fn sha1_sum(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sha1_sum", self.base))
    }

    #[doc = "Get a reference to the value of field `source_url` after provisioning.\nSource URL"]
    pub fn source_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_url", self.base))
    }
}

#[derive(Serialize)]
pub struct AppEngineFlexibleAppVersionDeploymentElZipEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    files_count: std::option::Option<terrars::PrimField<f64>>,
    source_url: terrars::PrimField<std::string::String>,
}

impl AppEngineFlexibleAppVersionDeploymentElZipEl {
    #[doc = "Set the field `files_count`.\nfiles count"]
    pub fn set_files_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.files_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppEngineFlexibleAppVersionDeploymentElZipEl {
    type O = terrars::BlockAssignable<AppEngineFlexibleAppVersionDeploymentElZipEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineFlexibleAppVersionDeploymentElZipEl {
    #[doc = "Source URL"]
    pub source_url: terrars::PrimField<std::string::String>,
}

impl BuildAppEngineFlexibleAppVersionDeploymentElZipEl {
    pub fn build(self) -> AppEngineFlexibleAppVersionDeploymentElZipEl {
        AppEngineFlexibleAppVersionDeploymentElZipEl {
            files_count: core::default::Default::default(),
            source_url: self.source_url,
        }
    }
}

pub struct AppEngineFlexibleAppVersionDeploymentElZipElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineFlexibleAppVersionDeploymentElZipElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppEngineFlexibleAppVersionDeploymentElZipElRef {
        AppEngineFlexibleAppVersionDeploymentElZipElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineFlexibleAppVersionDeploymentElZipElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `files_count` after provisioning.\nfiles count"]
    pub fn files_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.files_count", self.base))
    }

    #[doc = "Get a reference to the value of field `source_url` after provisioning.\nSource URL"]
    pub fn source_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_url", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppEngineFlexibleAppVersionDeploymentElDynamic {
    cloud_build_options: std
    ::option
    ::Option<terrars::DynamicBlock<AppEngineFlexibleAppVersionDeploymentElCloudBuildOptionsEl>>,
    container: std::option::Option<terrars::DynamicBlock<AppEngineFlexibleAppVersionDeploymentElContainerEl>>,
    files: std::option::Option<terrars::DynamicBlock<AppEngineFlexibleAppVersionDeploymentElFilesEl>>,
    zip: std::option::Option<terrars::DynamicBlock<AppEngineFlexibleAppVersionDeploymentElZipEl>>,
}

#[derive(Serialize)]
pub struct AppEngineFlexibleAppVersionDeploymentEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_build_options: std::option::Option<std::vec::Vec<AppEngineFlexibleAppVersionDeploymentElCloudBuildOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container: std::option::Option<std::vec::Vec<AppEngineFlexibleAppVersionDeploymentElContainerEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    files: std::option::Option<std::vec::Vec<AppEngineFlexibleAppVersionDeploymentElFilesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zip: std::option::Option<std::vec::Vec<AppEngineFlexibleAppVersionDeploymentElZipEl>>,
    dynamic: AppEngineFlexibleAppVersionDeploymentElDynamic,
}

impl AppEngineFlexibleAppVersionDeploymentEl {
    #[doc = "Set the field `cloud_build_options`.\n"]
    pub fn set_cloud_build_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppEngineFlexibleAppVersionDeploymentElCloudBuildOptionsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloud_build_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_build_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `container`.\n"]
    pub fn set_container(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppEngineFlexibleAppVersionDeploymentElContainerEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.container = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.container = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `files`.\n"]
    pub fn set_files(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppEngineFlexibleAppVersionDeploymentElFilesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.files = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.files = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `zip`.\n"]
    pub fn set_zip(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppEngineFlexibleAppVersionDeploymentElZipEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.zip = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.zip = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppEngineFlexibleAppVersionDeploymentEl {
    type O = terrars::BlockAssignable<AppEngineFlexibleAppVersionDeploymentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineFlexibleAppVersionDeploymentEl {}

impl BuildAppEngineFlexibleAppVersionDeploymentEl {
    pub fn build(self) -> AppEngineFlexibleAppVersionDeploymentEl {
        AppEngineFlexibleAppVersionDeploymentEl {
            cloud_build_options: core::default::Default::default(),
            container: core::default::Default::default(),
            files: core::default::Default::default(),
            zip: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppEngineFlexibleAppVersionDeploymentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineFlexibleAppVersionDeploymentElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppEngineFlexibleAppVersionDeploymentElRef {
        AppEngineFlexibleAppVersionDeploymentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineFlexibleAppVersionDeploymentElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_build_options` after provisioning.\n"]
    pub fn cloud_build_options(&self) -> terrars::ListRef<AppEngineFlexibleAppVersionDeploymentElCloudBuildOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_build_options", self.base))
    }

    #[doc = "Get a reference to the value of field `container` after provisioning.\n"]
    pub fn container(&self) -> terrars::ListRef<AppEngineFlexibleAppVersionDeploymentElContainerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container", self.base))
    }

    #[doc = "Get a reference to the value of field `zip` after provisioning.\n"]
    pub fn zip(&self) -> terrars::ListRef<AppEngineFlexibleAppVersionDeploymentElZipElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zip", self.base))
    }
}

#[derive(Serialize)]
pub struct AppEngineFlexibleAppVersionEndpointsApiServiceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_trace_sampling: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rollout_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppEngineFlexibleAppVersionEndpointsApiServiceEl {
    #[doc =
        "Set the field `config_id`.\nEndpoints service configuration ID as specified by the Service Management API. For example \"2016-09-19r1\".\n\nBy default, the rollout strategy for Endpoints is \"FIXED\". This means that Endpoints starts up with a particular configuration ID.\nWhen a new configuration is rolled out, Endpoints must be given the new configuration ID. The configId field is used to give the configuration ID\nand is required in this case.\n\nEndpoints also has a rollout strategy called \"MANAGED\". When using this, Endpoints fetches the latest configuration and does not need\nthe configuration ID. In this case, configId must be omitted."]
    pub fn set_config_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.config_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disable_trace_sampling`.\nEnable or disable trace sampling. By default, this is set to false for enabled."]
    pub fn set_disable_trace_sampling(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_trace_sampling = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `rollout_strategy`.\nEndpoints rollout strategy. If FIXED, configId must be specified. If MANAGED, configId must be omitted. Default value: \"FIXED\" Possible values: [\"FIXED\", \"MANAGED\"]"]
    pub fn set_rollout_strategy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rollout_strategy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppEngineFlexibleAppVersionEndpointsApiServiceEl {
    type O = terrars::BlockAssignable<AppEngineFlexibleAppVersionEndpointsApiServiceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineFlexibleAppVersionEndpointsApiServiceEl {
    #[doc =
        "Endpoints service name which is the name of the \"service\" resource in the Service Management API.\nFor example \"myapi.endpoints.myproject.cloud.goog\""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAppEngineFlexibleAppVersionEndpointsApiServiceEl {
    pub fn build(self) -> AppEngineFlexibleAppVersionEndpointsApiServiceEl {
        AppEngineFlexibleAppVersionEndpointsApiServiceEl {
            config_id: core::default::Default::default(),
            disable_trace_sampling: core::default::Default::default(),
            name: self.name,
            rollout_strategy: core::default::Default::default(),
        }
    }
}

pub struct AppEngineFlexibleAppVersionEndpointsApiServiceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineFlexibleAppVersionEndpointsApiServiceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppEngineFlexibleAppVersionEndpointsApiServiceElRef {
        AppEngineFlexibleAppVersionEndpointsApiServiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineFlexibleAppVersionEndpointsApiServiceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `config_id` after provisioning.\nEndpoints service configuration ID as specified by the Service Management API. For example \"2016-09-19r1\".\n\nBy default, the rollout strategy for Endpoints is \"FIXED\". This means that Endpoints starts up with a particular configuration ID.\nWhen a new configuration is rolled out, Endpoints must be given the new configuration ID. The configId field is used to give the configuration ID\nand is required in this case.\n\nEndpoints also has a rollout strategy called \"MANAGED\". When using this, Endpoints fetches the latest configuration and does not need\nthe configuration ID. In this case, configId must be omitted."]
    pub fn config_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.config_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disable_trace_sampling` after provisioning.\nEnable or disable trace sampling. By default, this is set to false for enabled."]
    pub fn disable_trace_sampling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_trace_sampling", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nEndpoints service name which is the name of the \"service\" resource in the Service Management API.\nFor example \"myapi.endpoints.myproject.cloud.goog\""]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rollout_strategy` after provisioning.\nEndpoints rollout strategy. If FIXED, configId must be specified. If MANAGED, configId must be omitted. Default value: \"FIXED\" Possible values: [\"FIXED\", \"MANAGED\"]"]
    pub fn rollout_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rollout_strategy", self.base))
    }
}

#[derive(Serialize)]
pub struct AppEngineFlexibleAppVersionEntrypointEl {
    shell: terrars::PrimField<std::string::String>,
}

impl AppEngineFlexibleAppVersionEntrypointEl { }

impl terrars::ToListMappable for AppEngineFlexibleAppVersionEntrypointEl {
    type O = terrars::BlockAssignable<AppEngineFlexibleAppVersionEntrypointEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineFlexibleAppVersionEntrypointEl {
    #[doc = "The format should be a shell command that can be fed to bash -c."]
    pub shell: terrars::PrimField<std::string::String>,
}

impl BuildAppEngineFlexibleAppVersionEntrypointEl {
    pub fn build(self) -> AppEngineFlexibleAppVersionEntrypointEl {
        AppEngineFlexibleAppVersionEntrypointEl { shell: self.shell }
    }
}

pub struct AppEngineFlexibleAppVersionEntrypointElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineFlexibleAppVersionEntrypointElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppEngineFlexibleAppVersionEntrypointElRef {
        AppEngineFlexibleAppVersionEntrypointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineFlexibleAppVersionEntrypointElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `shell` after provisioning.\nThe format should be a shell command that can be fed to bash -c."]
    pub fn shell(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shell", self.base))
    }
}

#[derive(Serialize)]
pub struct AppEngineFlexibleAppVersionHandlersElScriptEl {
    script_path: terrars::PrimField<std::string::String>,
}

impl AppEngineFlexibleAppVersionHandlersElScriptEl { }

impl terrars::ToListMappable for AppEngineFlexibleAppVersionHandlersElScriptEl {
    type O = terrars::BlockAssignable<AppEngineFlexibleAppVersionHandlersElScriptEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineFlexibleAppVersionHandlersElScriptEl {
    #[doc = "Path to the script from the application root directory."]
    pub script_path: terrars::PrimField<std::string::String>,
}

impl BuildAppEngineFlexibleAppVersionHandlersElScriptEl {
    pub fn build(self) -> AppEngineFlexibleAppVersionHandlersElScriptEl {
        AppEngineFlexibleAppVersionHandlersElScriptEl { script_path: self.script_path }
    }
}

pub struct AppEngineFlexibleAppVersionHandlersElScriptElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineFlexibleAppVersionHandlersElScriptElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppEngineFlexibleAppVersionHandlersElScriptElRef {
        AppEngineFlexibleAppVersionHandlersElScriptElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineFlexibleAppVersionHandlersElScriptElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `script_path` after provisioning.\nPath to the script from the application root directory."]
    pub fn script_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.script_path", self.base))
    }
}

#[derive(Serialize)]
pub struct AppEngineFlexibleAppVersionHandlersElStaticFilesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    application_readable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expiration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_headers: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mime_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_matching_file: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upload_path_regex: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppEngineFlexibleAppVersionHandlersElStaticFilesEl {
    #[doc =
        "Set the field `application_readable`.\nWhether files should also be uploaded as code data. By default, files declared in static file handlers are\nuploaded as static data and are only served to end users; they cannot be read by the application. If enabled,\nuploads are charged against both your code and static data storage resource quotas."]
    pub fn set_application_readable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.application_readable = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `expiration`.\nTime a static file served by this handler should be cached by web proxies and browsers.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example \"3.5s\".\nDefault is '0s'"]
    pub fn set_expiration(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expiration = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `http_headers`.\nHTTP headers to use for all responses from these URLs.\nAn object containing a list of \"key:value\" value pairs.\"."]
    pub fn set_http_headers(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.http_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `mime_type`.\nMIME type used to serve all files served by this handler.\nDefaults to file-specific MIME types, which are derived from each file's filename extension."]
    pub fn set_mime_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mime_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `path`.\nPath to the static files matched by the URL pattern, from the application root directory.\nThe path can refer to text matched in groupings in the URL pattern."]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `require_matching_file`.\nWhether this handler should match the request if the file referenced by the handler does not exist."]
    pub fn set_require_matching_file(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.require_matching_file = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `upload_path_regex`.\nRegular expression that matches the file paths for all files that should be referenced by this handler."]
    pub fn set_upload_path_regex(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.upload_path_regex = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppEngineFlexibleAppVersionHandlersElStaticFilesEl {
    type O = terrars::BlockAssignable<AppEngineFlexibleAppVersionHandlersElStaticFilesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineFlexibleAppVersionHandlersElStaticFilesEl {}

impl BuildAppEngineFlexibleAppVersionHandlersElStaticFilesEl {
    pub fn build(self) -> AppEngineFlexibleAppVersionHandlersElStaticFilesEl {
        AppEngineFlexibleAppVersionHandlersElStaticFilesEl {
            application_readable: core::default::Default::default(),
            expiration: core::default::Default::default(),
            http_headers: core::default::Default::default(),
            mime_type: core::default::Default::default(),
            path: core::default::Default::default(),
            require_matching_file: core::default::Default::default(),
            upload_path_regex: core::default::Default::default(),
        }
    }
}

pub struct AppEngineFlexibleAppVersionHandlersElStaticFilesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineFlexibleAppVersionHandlersElStaticFilesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppEngineFlexibleAppVersionHandlersElStaticFilesElRef {
        AppEngineFlexibleAppVersionHandlersElStaticFilesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineFlexibleAppVersionHandlersElStaticFilesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `application_readable` after provisioning.\nWhether files should also be uploaded as code data. By default, files declared in static file handlers are\nuploaded as static data and are only served to end users; they cannot be read by the application. If enabled,\nuploads are charged against both your code and static data storage resource quotas."]
    pub fn application_readable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_readable", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expiration` after provisioning.\nTime a static file served by this handler should be cached by web proxies and browsers.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example \"3.5s\".\nDefault is '0s'"]
    pub fn expiration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expiration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `http_headers` after provisioning.\nHTTP headers to use for all responses from these URLs.\nAn object containing a list of \"key:value\" value pairs.\"."]
    pub fn http_headers(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.http_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mime_type` after provisioning.\nMIME type used to serve all files served by this handler.\nDefaults to file-specific MIME types, which are derived from each file's filename extension."]
    pub fn mime_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mime_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nPath to the static files matched by the URL pattern, from the application root directory.\nThe path can refer to text matched in groupings in the URL pattern."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `require_matching_file` after provisioning.\nWhether this handler should match the request if the file referenced by the handler does not exist."]
    pub fn require_matching_file(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_matching_file", self.base))
    }

    #[doc =
        "Get a reference to the value of field `upload_path_regex` after provisioning.\nRegular expression that matches the file paths for all files that should be referenced by this handler."]
    pub fn upload_path_regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.upload_path_regex", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppEngineFlexibleAppVersionHandlersElDynamic {
    script: std::option::Option<terrars::DynamicBlock<AppEngineFlexibleAppVersionHandlersElScriptEl>>,
    static_files: std::option::Option<terrars::DynamicBlock<AppEngineFlexibleAppVersionHandlersElStaticFilesEl>>,
}

#[derive(Serialize)]
pub struct AppEngineFlexibleAppVersionHandlersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_fail_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    login: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect_http_response_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url_regex: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    script: std::option::Option<std::vec::Vec<AppEngineFlexibleAppVersionHandlersElScriptEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    static_files: std::option::Option<std::vec::Vec<AppEngineFlexibleAppVersionHandlersElStaticFilesEl>>,
    dynamic: AppEngineFlexibleAppVersionHandlersElDynamic,
}

impl AppEngineFlexibleAppVersionHandlersEl {
    #[doc =
        "Set the field `auth_fail_action`.\nActions to take when the user is not logged in. Possible values: [\"AUTH_FAIL_ACTION_REDIRECT\", \"AUTH_FAIL_ACTION_UNAUTHORIZED\"]"]
    pub fn set_auth_fail_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth_fail_action = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `login`.\nMethods to restrict access to a URL based on login status. Possible values: [\"LOGIN_OPTIONAL\", \"LOGIN_ADMIN\", \"LOGIN_REQUIRED\"]"]
    pub fn set_login(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.login = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `redirect_http_response_code`.\n30x code to use when performing redirects for the secure field. Possible values: [\"REDIRECT_HTTP_RESPONSE_CODE_301\", \"REDIRECT_HTTP_RESPONSE_CODE_302\", \"REDIRECT_HTTP_RESPONSE_CODE_303\", \"REDIRECT_HTTP_RESPONSE_CODE_307\"]"]
    pub fn set_redirect_http_response_code(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.redirect_http_response_code = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `security_level`.\nSecurity (HTTPS) enforcement for this URL. Possible values: [\"SECURE_DEFAULT\", \"SECURE_NEVER\", \"SECURE_OPTIONAL\", \"SECURE_ALWAYS\"]"]
    pub fn set_security_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.security_level = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `url_regex`.\nURL prefix. Uses regular expression syntax, which means regexp special characters must be escaped, but should not contain groupings.\nAll URLs that begin with this prefix are handled by this handler, using the portion of the URL after the prefix as part of the file path."]
    pub fn set_url_regex(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url_regex = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `script`.\n"]
    pub fn set_script(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppEngineFlexibleAppVersionHandlersElScriptEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.script = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.script = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `static_files`.\n"]
    pub fn set_static_files(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppEngineFlexibleAppVersionHandlersElStaticFilesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.static_files = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.static_files = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppEngineFlexibleAppVersionHandlersEl {
    type O = terrars::BlockAssignable<AppEngineFlexibleAppVersionHandlersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineFlexibleAppVersionHandlersEl {}

impl BuildAppEngineFlexibleAppVersionHandlersEl {
    pub fn build(self) -> AppEngineFlexibleAppVersionHandlersEl {
        AppEngineFlexibleAppVersionHandlersEl {
            auth_fail_action: core::default::Default::default(),
            login: core::default::Default::default(),
            redirect_http_response_code: core::default::Default::default(),
            security_level: core::default::Default::default(),
            url_regex: core::default::Default::default(),
            script: core::default::Default::default(),
            static_files: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppEngineFlexibleAppVersionHandlersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineFlexibleAppVersionHandlersElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppEngineFlexibleAppVersionHandlersElRef {
        AppEngineFlexibleAppVersionHandlersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineFlexibleAppVersionHandlersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `auth_fail_action` after provisioning.\nActions to take when the user is not logged in. Possible values: [\"AUTH_FAIL_ACTION_REDIRECT\", \"AUTH_FAIL_ACTION_UNAUTHORIZED\"]"]
    pub fn auth_fail_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_fail_action", self.base))
    }

    #[doc =
        "Get a reference to the value of field `login` after provisioning.\nMethods to restrict access to a URL based on login status. Possible values: [\"LOGIN_OPTIONAL\", \"LOGIN_ADMIN\", \"LOGIN_REQUIRED\"]"]
    pub fn login(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.login", self.base))
    }

    #[doc =
        "Get a reference to the value of field `redirect_http_response_code` after provisioning.\n30x code to use when performing redirects for the secure field. Possible values: [\"REDIRECT_HTTP_RESPONSE_CODE_301\", \"REDIRECT_HTTP_RESPONSE_CODE_302\", \"REDIRECT_HTTP_RESPONSE_CODE_303\", \"REDIRECT_HTTP_RESPONSE_CODE_307\"]"]
    pub fn redirect_http_response_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redirect_http_response_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `security_level` after provisioning.\nSecurity (HTTPS) enforcement for this URL. Possible values: [\"SECURE_DEFAULT\", \"SECURE_NEVER\", \"SECURE_OPTIONAL\", \"SECURE_ALWAYS\"]"]
    pub fn security_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_level", self.base))
    }

    #[doc =
        "Get a reference to the value of field `url_regex` after provisioning.\nURL prefix. Uses regular expression syntax, which means regexp special characters must be escaped, but should not contain groupings.\nAll URLs that begin with this prefix are handled by this handler, using the portion of the URL after the prefix as part of the file path."]
    pub fn url_regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url_regex", self.base))
    }

    #[doc = "Get a reference to the value of field `script` after provisioning.\n"]
    pub fn script(&self) -> terrars::ListRef<AppEngineFlexibleAppVersionHandlersElScriptElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.script", self.base))
    }

    #[doc = "Get a reference to the value of field `static_files` after provisioning.\n"]
    pub fn static_files(&self) -> terrars::ListRef<AppEngineFlexibleAppVersionHandlersElStaticFilesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.static_files", self.base))
    }
}

#[derive(Serialize)]
pub struct AppEngineFlexibleAppVersionLivenessCheckEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    check_interval: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_delay: std::option::Option<terrars::PrimField<std::string::String>>,
    path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    success_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppEngineFlexibleAppVersionLivenessCheckEl {
    #[doc = "Set the field `check_interval`.\nInterval between health checks."]
    pub fn set_check_interval(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.check_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `failure_threshold`.\nNumber of consecutive failed checks required before considering the VM unhealthy. Default: 4."]
    pub fn set_failure_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.failure_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `host`.\nHost header to send when performing a HTTP Readiness check. Example: \"myapp.appspot.com\""]
    pub fn set_host(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `initial_delay`.\nThe initial delay before starting to execute the checks. Default: \"300s\""]
    pub fn set_initial_delay(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.initial_delay = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `success_threshold`.\nNumber of consecutive successful checks required before considering the VM healthy. Default: 2."]
    pub fn set_success_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.success_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout`.\nTime before the check is considered failed. Default: \"4s\""]
    pub fn set_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.timeout = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppEngineFlexibleAppVersionLivenessCheckEl {
    type O = terrars::BlockAssignable<AppEngineFlexibleAppVersionLivenessCheckEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineFlexibleAppVersionLivenessCheckEl {
    #[doc = "The request path."]
    pub path: terrars::PrimField<std::string::String>,
}

impl BuildAppEngineFlexibleAppVersionLivenessCheckEl {
    pub fn build(self) -> AppEngineFlexibleAppVersionLivenessCheckEl {
        AppEngineFlexibleAppVersionLivenessCheckEl {
            check_interval: core::default::Default::default(),
            failure_threshold: core::default::Default::default(),
            host: core::default::Default::default(),
            initial_delay: core::default::Default::default(),
            path: self.path,
            success_threshold: core::default::Default::default(),
            timeout: core::default::Default::default(),
        }
    }
}

pub struct AppEngineFlexibleAppVersionLivenessCheckElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineFlexibleAppVersionLivenessCheckElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppEngineFlexibleAppVersionLivenessCheckElRef {
        AppEngineFlexibleAppVersionLivenessCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineFlexibleAppVersionLivenessCheckElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `check_interval` after provisioning.\nInterval between health checks."]
    pub fn check_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.check_interval", self.base))
    }

    #[doc =
        "Get a reference to the value of field `failure_threshold` after provisioning.\nNumber of consecutive failed checks required before considering the VM unhealthy. Default: 4."]
    pub fn failure_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_threshold", self.base))
    }

    #[doc =
        "Get a reference to the value of field `host` after provisioning.\nHost header to send when performing a HTTP Readiness check. Example: \"myapp.appspot.com\""]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc =
        "Get a reference to the value of field `initial_delay` after provisioning.\nThe initial delay before starting to execute the checks. Default: \"300s\""]
    pub fn initial_delay(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_delay", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\nThe request path."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `success_threshold` after provisioning.\nNumber of consecutive successful checks required before considering the VM healthy. Default: 2."]
    pub fn success_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.success_threshold", self.base))
    }

    #[doc =
        "Get a reference to the value of field `timeout` after provisioning.\nTime before the check is considered failed. Default: \"4s\""]
    pub fn timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct AppEngineFlexibleAppVersionManualScalingEl {
    instances: terrars::PrimField<f64>,
}

impl AppEngineFlexibleAppVersionManualScalingEl { }

impl terrars::ToListMappable for AppEngineFlexibleAppVersionManualScalingEl {
    type O = terrars::BlockAssignable<AppEngineFlexibleAppVersionManualScalingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineFlexibleAppVersionManualScalingEl {
    #[doc =
        "Number of instances to assign to the service at the start.\n\n**Note:** When managing the number of instances at runtime through the App Engine Admin API or the (now deprecated) Python 2\nModules API set_num_instances() you must use 'lifecycle.ignore_changes = [\"manual_scaling\"[0].instances]' to prevent drift detection."]
    pub instances: terrars::PrimField<f64>,
}

impl BuildAppEngineFlexibleAppVersionManualScalingEl {
    pub fn build(self) -> AppEngineFlexibleAppVersionManualScalingEl {
        AppEngineFlexibleAppVersionManualScalingEl { instances: self.instances }
    }
}

pub struct AppEngineFlexibleAppVersionManualScalingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineFlexibleAppVersionManualScalingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppEngineFlexibleAppVersionManualScalingElRef {
        AppEngineFlexibleAppVersionManualScalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineFlexibleAppVersionManualScalingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `instances` after provisioning.\nNumber of instances to assign to the service at the start.\n\n**Note:** When managing the number of instances at runtime through the App Engine Admin API or the (now deprecated) Python 2\nModules API set_num_instances() you must use 'lifecycle.ignore_changes = [\"manual_scaling\"[0].instances]' to prevent drift detection."]
    pub fn instances(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instances", self.base))
    }
}

#[derive(Serialize)]
pub struct AppEngineFlexibleAppVersionNetworkEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forwarded_ports: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_tag: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_affinity: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppEngineFlexibleAppVersionNetworkEl {
    #[doc =
        "Set the field `forwarded_ports`.\nList of ports, or port pairs, to forward from the virtual machine to the application container."]
    pub fn set_forwarded_ports(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.forwarded_ports = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_tag`.\nTag to apply to the instance during creation."]
    pub fn set_instance_tag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_tag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `session_affinity`.\nEnable session affinity."]
    pub fn set_session_affinity(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.session_affinity = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `subnetwork`.\nGoogle Cloud Platform sub-network where the virtual machines are created. Specify the short name, not the resource path.\n\nIf the network that the instance is being created in is a Legacy network, then the IP address is allocated from the IPv4Range.\nIf the network that the instance is being created in is an auto Subnet Mode Network, then only network name should be specified (not the subnetworkName) and the IP address is created from the IPCidrRange of the subnetwork that exists in that zone for that network.\nIf the network that the instance is being created in is a custom Subnet Mode Network, then the subnetworkName must be specified and the IP address is created from the IPCidrRange of the subnetwork.\nIf specified, the subnetwork must exist in the same region as the App Engine flexible environment application."]
    pub fn set_subnetwork(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnetwork = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppEngineFlexibleAppVersionNetworkEl {
    type O = terrars::BlockAssignable<AppEngineFlexibleAppVersionNetworkEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineFlexibleAppVersionNetworkEl {
    #[doc =
        "Google Compute Engine network where the virtual machines are created. Specify the short name, not the resource path."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAppEngineFlexibleAppVersionNetworkEl {
    pub fn build(self) -> AppEngineFlexibleAppVersionNetworkEl {
        AppEngineFlexibleAppVersionNetworkEl {
            forwarded_ports: core::default::Default::default(),
            instance_tag: core::default::Default::default(),
            name: self.name,
            session_affinity: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
        }
    }
}

pub struct AppEngineFlexibleAppVersionNetworkElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineFlexibleAppVersionNetworkElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppEngineFlexibleAppVersionNetworkElRef {
        AppEngineFlexibleAppVersionNetworkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineFlexibleAppVersionNetworkElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `forwarded_ports` after provisioning.\nList of ports, or port pairs, to forward from the virtual machine to the application container."]
    pub fn forwarded_ports(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forwarded_ports", self.base))
    }

    #[doc =
        "Get a reference to the value of field `instance_tag` after provisioning.\nTag to apply to the instance during creation."]
    pub fn instance_tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_tag", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nGoogle Compute Engine network where the virtual machines are created. Specify the short name, not the resource path."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `session_affinity` after provisioning.\nEnable session affinity."]
    pub fn session_affinity(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_affinity", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subnetwork` after provisioning.\nGoogle Cloud Platform sub-network where the virtual machines are created. Specify the short name, not the resource path.\n\nIf the network that the instance is being created in is a Legacy network, then the IP address is allocated from the IPv4Range.\nIf the network that the instance is being created in is an auto Subnet Mode Network, then only network name should be specified (not the subnetworkName) and the IP address is created from the IPCidrRange of the subnetwork that exists in that zone for that network.\nIf the network that the instance is being created in is a custom Subnet Mode Network, then the subnetworkName must be specified and the IP address is created from the IPCidrRange of the subnetwork.\nIf specified, the subnetwork must exist in the same region as the App Engine flexible environment application."]
    pub fn subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
}

#[derive(Serialize)]
pub struct AppEngineFlexibleAppVersionReadinessCheckEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    app_start_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    check_interval: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
    path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    success_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppEngineFlexibleAppVersionReadinessCheckEl {
    #[doc =
        "Set the field `app_start_timeout`.\nA maximum time limit on application initialization, measured from moment the application successfully\nreplies to a healthcheck until it is ready to serve traffic. Default: \"300s\""]
    pub fn set_app_start_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.app_start_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `check_interval`.\nInterval between health checks.  Default: \"5s\"."]
    pub fn set_check_interval(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.check_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `failure_threshold`.\nNumber of consecutive failed checks required before removing traffic. Default: 2."]
    pub fn set_failure_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.failure_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `host`.\nHost header to send when performing a HTTP Readiness check. Example: \"myapp.appspot.com\""]
    pub fn set_host(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `success_threshold`.\nNumber of consecutive successful checks required before receiving traffic. Default: 2."]
    pub fn set_success_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.success_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout`.\nTime before the check is considered failed. Default: \"4s\""]
    pub fn set_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.timeout = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppEngineFlexibleAppVersionReadinessCheckEl {
    type O = terrars::BlockAssignable<AppEngineFlexibleAppVersionReadinessCheckEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineFlexibleAppVersionReadinessCheckEl {
    #[doc = "The request path."]
    pub path: terrars::PrimField<std::string::String>,
}

impl BuildAppEngineFlexibleAppVersionReadinessCheckEl {
    pub fn build(self) -> AppEngineFlexibleAppVersionReadinessCheckEl {
        AppEngineFlexibleAppVersionReadinessCheckEl {
            app_start_timeout: core::default::Default::default(),
            check_interval: core::default::Default::default(),
            failure_threshold: core::default::Default::default(),
            host: core::default::Default::default(),
            path: self.path,
            success_threshold: core::default::Default::default(),
            timeout: core::default::Default::default(),
        }
    }
}

pub struct AppEngineFlexibleAppVersionReadinessCheckElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineFlexibleAppVersionReadinessCheckElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppEngineFlexibleAppVersionReadinessCheckElRef {
        AppEngineFlexibleAppVersionReadinessCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineFlexibleAppVersionReadinessCheckElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `app_start_timeout` after provisioning.\nA maximum time limit on application initialization, measured from moment the application successfully\nreplies to a healthcheck until it is ready to serve traffic. Default: \"300s\""]
    pub fn app_start_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_start_timeout", self.base))
    }

    #[doc =
        "Get a reference to the value of field `check_interval` after provisioning.\nInterval between health checks.  Default: \"5s\"."]
    pub fn check_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.check_interval", self.base))
    }

    #[doc =
        "Get a reference to the value of field `failure_threshold` after provisioning.\nNumber of consecutive failed checks required before removing traffic. Default: 2."]
    pub fn failure_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_threshold", self.base))
    }

    #[doc =
        "Get a reference to the value of field `host` after provisioning.\nHost header to send when performing a HTTP Readiness check. Example: \"myapp.appspot.com\""]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\nThe request path."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `success_threshold` after provisioning.\nNumber of consecutive successful checks required before receiving traffic. Default: 2."]
    pub fn success_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.success_threshold", self.base))
    }

    #[doc =
        "Get a reference to the value of field `timeout` after provisioning.\nTime before the check is considered failed. Default: \"4s\""]
    pub fn timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct AppEngineFlexibleAppVersionResourcesElVolumesEl {
    name: terrars::PrimField<std::string::String>,
    size_gb: terrars::PrimField<f64>,
    volume_type: terrars::PrimField<std::string::String>,
}

impl AppEngineFlexibleAppVersionResourcesElVolumesEl { }

impl terrars::ToListMappable for AppEngineFlexibleAppVersionResourcesElVolumesEl {
    type O = terrars::BlockAssignable<AppEngineFlexibleAppVersionResourcesElVolumesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineFlexibleAppVersionResourcesElVolumesEl {
    #[doc = "Unique name for the volume."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "Volume size in gigabytes."]
    pub size_gb: terrars::PrimField<f64>,
    #[doc = "Underlying volume type, e.g. 'tmpfs'."]
    pub volume_type: terrars::PrimField<std::string::String>,
}

impl BuildAppEngineFlexibleAppVersionResourcesElVolumesEl {
    pub fn build(self) -> AppEngineFlexibleAppVersionResourcesElVolumesEl {
        AppEngineFlexibleAppVersionResourcesElVolumesEl {
            name: self.name,
            size_gb: self.size_gb,
            volume_type: self.volume_type,
        }
    }
}

pub struct AppEngineFlexibleAppVersionResourcesElVolumesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineFlexibleAppVersionResourcesElVolumesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppEngineFlexibleAppVersionResourcesElVolumesElRef {
        AppEngineFlexibleAppVersionResourcesElVolumesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineFlexibleAppVersionResourcesElVolumesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nUnique name for the volume."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `size_gb` after provisioning.\nVolume size in gigabytes."]
    pub fn size_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `volume_type` after provisioning.\nUnderlying volume type, e.g. 'tmpfs'."]
    pub fn volume_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppEngineFlexibleAppVersionResourcesElDynamic {
    volumes: std::option::Option<terrars::DynamicBlock<AppEngineFlexibleAppVersionResourcesElVolumesEl>>,
}

#[derive(Serialize)]
pub struct AppEngineFlexibleAppVersionResourcesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_gb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volumes: std::option::Option<std::vec::Vec<AppEngineFlexibleAppVersionResourcesElVolumesEl>>,
    dynamic: AppEngineFlexibleAppVersionResourcesElDynamic,
}

impl AppEngineFlexibleAppVersionResourcesEl {
    #[doc = "Set the field `cpu`.\nNumber of CPU cores needed."]
    pub fn set_cpu(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk_gb`.\nDisk size (GB) needed."]
    pub fn set_disk_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.disk_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_gb`.\nMemory (GB) needed."]
    pub fn set_memory_gb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volumes`.\n"]
    pub fn set_volumes(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppEngineFlexibleAppVersionResourcesElVolumesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.volumes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.volumes = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppEngineFlexibleAppVersionResourcesEl {
    type O = terrars::BlockAssignable<AppEngineFlexibleAppVersionResourcesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineFlexibleAppVersionResourcesEl {}

impl BuildAppEngineFlexibleAppVersionResourcesEl {
    pub fn build(self) -> AppEngineFlexibleAppVersionResourcesEl {
        AppEngineFlexibleAppVersionResourcesEl {
            cpu: core::default::Default::default(),
            disk_gb: core::default::Default::default(),
            memory_gb: core::default::Default::default(),
            volumes: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppEngineFlexibleAppVersionResourcesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineFlexibleAppVersionResourcesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppEngineFlexibleAppVersionResourcesElRef {
        AppEngineFlexibleAppVersionResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineFlexibleAppVersionResourcesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\nNumber of CPU cores needed."]
    pub fn cpu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc = "Get a reference to the value of field `disk_gb` after provisioning.\nDisk size (GB) needed."]
    pub fn disk_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_gb` after provisioning.\nMemory (GB) needed."]
    pub fn memory_gb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `volumes` after provisioning.\n"]
    pub fn volumes(&self) -> terrars::ListRef<AppEngineFlexibleAppVersionResourcesElVolumesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.volumes", self.base))
    }
}

#[derive(Serialize)]
pub struct AppEngineFlexibleAppVersionTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppEngineFlexibleAppVersionTimeoutsEl {
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

impl terrars::ToListMappable for AppEngineFlexibleAppVersionTimeoutsEl {
    type O = terrars::BlockAssignable<AppEngineFlexibleAppVersionTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineFlexibleAppVersionTimeoutsEl {}

impl BuildAppEngineFlexibleAppVersionTimeoutsEl {
    pub fn build(self) -> AppEngineFlexibleAppVersionTimeoutsEl {
        AppEngineFlexibleAppVersionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct AppEngineFlexibleAppVersionTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineFlexibleAppVersionTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppEngineFlexibleAppVersionTimeoutsElRef {
        AppEngineFlexibleAppVersionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineFlexibleAppVersionTimeoutsElRef {
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
pub struct AppEngineFlexibleAppVersionVpcAccessConnectorEl {
    name: terrars::PrimField<std::string::String>,
}

impl AppEngineFlexibleAppVersionVpcAccessConnectorEl { }

impl terrars::ToListMappable for AppEngineFlexibleAppVersionVpcAccessConnectorEl {
    type O = terrars::BlockAssignable<AppEngineFlexibleAppVersionVpcAccessConnectorEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineFlexibleAppVersionVpcAccessConnectorEl {
    #[doc = "Full Serverless VPC Access Connector name e.g. /projects/my-project/locations/us-central1/connectors/c1."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAppEngineFlexibleAppVersionVpcAccessConnectorEl {
    pub fn build(self) -> AppEngineFlexibleAppVersionVpcAccessConnectorEl {
        AppEngineFlexibleAppVersionVpcAccessConnectorEl { name: self.name }
    }
}

pub struct AppEngineFlexibleAppVersionVpcAccessConnectorElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineFlexibleAppVersionVpcAccessConnectorElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppEngineFlexibleAppVersionVpcAccessConnectorElRef {
        AppEngineFlexibleAppVersionVpcAccessConnectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineFlexibleAppVersionVpcAccessConnectorElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nFull Serverless VPC Access Connector name e.g. /projects/my-project/locations/us-central1/connectors/c1."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppEngineFlexibleAppVersionDynamic {
    api_config: std::option::Option<terrars::DynamicBlock<AppEngineFlexibleAppVersionApiConfigEl>>,
    automatic_scaling: std::option::Option<terrars::DynamicBlock<AppEngineFlexibleAppVersionAutomaticScalingEl>>,
    deployment: std::option::Option<terrars::DynamicBlock<AppEngineFlexibleAppVersionDeploymentEl>>,
    endpoints_api_service: std
    ::option
    ::Option<terrars::DynamicBlock<AppEngineFlexibleAppVersionEndpointsApiServiceEl>>,
    entrypoint: std::option::Option<terrars::DynamicBlock<AppEngineFlexibleAppVersionEntrypointEl>>,
    handlers: std::option::Option<terrars::DynamicBlock<AppEngineFlexibleAppVersionHandlersEl>>,
    liveness_check: std::option::Option<terrars::DynamicBlock<AppEngineFlexibleAppVersionLivenessCheckEl>>,
    manual_scaling: std::option::Option<terrars::DynamicBlock<AppEngineFlexibleAppVersionManualScalingEl>>,
    network: std::option::Option<terrars::DynamicBlock<AppEngineFlexibleAppVersionNetworkEl>>,
    readiness_check: std::option::Option<terrars::DynamicBlock<AppEngineFlexibleAppVersionReadinessCheckEl>>,
    resources: std::option::Option<terrars::DynamicBlock<AppEngineFlexibleAppVersionResourcesEl>>,
    vpc_access_connector: std
    ::option
    ::Option<terrars::DynamicBlock<AppEngineFlexibleAppVersionVpcAccessConnectorEl>>,
}
