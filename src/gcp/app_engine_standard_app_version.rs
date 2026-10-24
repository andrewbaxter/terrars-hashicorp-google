use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct AppEngineStandardAppVersionData {
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
    app_engine_apis: std::option::Option<terrars::PrimField<bool>>,
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
    noop_on_destroy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    runtime: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    runtime_api_version: std::option::Option<terrars::PrimField<std::string::String>>,
    service: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threadsafe: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automatic_scaling: std::option::Option<std::vec::Vec<AppEngineStandardAppVersionAutomaticScalingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    basic_scaling: std::option::Option<std::vec::Vec<AppEngineStandardAppVersionBasicScalingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment: std::option::Option<std::vec::Vec<AppEngineStandardAppVersionDeploymentEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entrypoint: std::option::Option<std::vec::Vec<AppEngineStandardAppVersionEntrypointEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    handlers: std::option::Option<std::vec::Vec<AppEngineStandardAppVersionHandlersEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    libraries: std::option::Option<std::vec::Vec<AppEngineStandardAppVersionLibrariesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    manual_scaling: std::option::Option<std::vec::Vec<AppEngineStandardAppVersionManualScalingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<AppEngineStandardAppVersionTimeoutsEl>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_access_connector: std::option::Option<std::vec::Vec<AppEngineStandardAppVersionVpcAccessConnectorEl>>,
    dynamic: AppEngineStandardAppVersionDynamic,
}

struct AppEngineStandardAppVersion_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AppEngineStandardAppVersionData>,
}

#[derive(Clone)]
pub struct AppEngineStandardAppVersion(std::rc::Rc<AppEngineStandardAppVersion_>);

impl AppEngineStandardAppVersion {
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
        "Set the field `app_engine_apis`.\nAllows App Engine second generation runtimes to access the legacy bundled services."]
    pub fn set_app_engine_apis(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().app_engine_apis = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete_service_on_destroy`.\nIf set to 'true', the service will be deleted if it is the last version."]
    pub fn set_delete_service_on_destroy(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_service_on_destroy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `env_variables`.\nEnvironment variables available to the application."]
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
        "Set the field `instance_class`.\nInstance class that is used to run this version. Valid values are\nAutomaticScaling: F1, F2, F4, F4_1G\nBasicScaling or ManualScaling: B1, B2, B4, B4_1G, B8\nDefaults to F1 for AutomaticScaling and B2 for ManualScaling and BasicScaling. If no scaling is specified, AutomaticScaling is chosen."]
    pub fn set_instance_class(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().instance_class = std::option::Option::Some(v.into());
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

    #[doc =
        "Set the field `service_account`.\nThe identity that the deployed version will run as. Admin API will use the App Engine Appspot service account as default if this field is neither provided in app.yaml file nor through CLI flag."]
    pub fn set_service_account(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `threadsafe`.\nWhether multiple requests can be dispatched to this version at once."]
    pub fn set_threadsafe(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().threadsafe = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `version_id`.\nRelative name of the version within the service. For example, 'v1'. Version names can contain only lowercase letters, numbers, or hyphens. Reserved names,\"default\", \"latest\", and any name with the prefix \"ah-\"."]
    pub fn set_version_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().version_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `automatic_scaling`.\n"]
    pub fn set_automatic_scaling(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppEngineStandardAppVersionAutomaticScalingEl>>,
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

    #[doc = "Set the field `basic_scaling`.\n"]
    pub fn set_basic_scaling(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppEngineStandardAppVersionBasicScalingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().basic_scaling = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.basic_scaling = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `deployment`.\n"]
    pub fn set_deployment(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppEngineStandardAppVersionDeploymentEl>>,
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

    #[doc = "Set the field `entrypoint`.\n"]
    pub fn set_entrypoint(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppEngineStandardAppVersionEntrypointEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppEngineStandardAppVersionHandlersEl>>,
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

    #[doc = "Set the field `libraries`.\n"]
    pub fn set_libraries(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppEngineStandardAppVersionLibrariesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().libraries = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.libraries = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `manual_scaling`.\n"]
    pub fn set_manual_scaling(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppEngineStandardAppVersionManualScalingEl>>,
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<AppEngineStandardAppVersionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_access_connector`.\n"]
    pub fn set_vpc_access_connector(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppEngineStandardAppVersionVpcAccessConnectorEl>>,
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
        "Get a reference to the value of field `app_engine_apis` after provisioning.\nAllows App Engine second generation runtimes to access the legacy bundled services."]
    pub fn app_engine_apis(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_engine_apis", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_service_on_destroy` after provisioning.\nIf set to 'true', the service will be deleted if it is the last version."]
    pub fn delete_service_on_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_service_on_destroy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `env_variables` after provisioning.\nEnvironment variables available to the application."]
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
        "Get a reference to the value of field `instance_class` after provisioning.\nInstance class that is used to run this version. Valid values are\nAutomaticScaling: F1, F2, F4, F4_1G\nBasicScaling or ManualScaling: B1, B2, B4, B4_1G, B8\nDefaults to F1 for AutomaticScaling and B2 for ManualScaling and BasicScaling. If no scaling is specified, AutomaticScaling is chosen."]
    pub fn instance_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_class", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nFull path to the Version resource in the API. Example, \"v1\"."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `service` after provisioning.\nAppEngine service resource"]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nThe identity that the deployed version will run as. Admin API will use the App Engine Appspot service account as default if this field is neither provided in app.yaml file nor through CLI flag."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `threadsafe` after provisioning.\nWhether multiple requests can be dispatched to this version at once."]
    pub fn threadsafe(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threadsafe", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version_id` after provisioning.\nRelative name of the version within the service. For example, 'v1'. Version names can contain only lowercase letters, numbers, or hyphens. Reserved names,\"default\", \"latest\", and any name with the prefix \"ah-\"."]
    pub fn version_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `automatic_scaling` after provisioning.\n"]
    pub fn automatic_scaling(&self) -> terrars::ListRef<AppEngineStandardAppVersionAutomaticScalingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.automatic_scaling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `basic_scaling` after provisioning.\n"]
    pub fn basic_scaling(&self) -> terrars::ListRef<AppEngineStandardAppVersionBasicScalingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.basic_scaling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment` after provisioning.\n"]
    pub fn deployment(&self) -> terrars::ListRef<AppEngineStandardAppVersionDeploymentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entrypoint` after provisioning.\n"]
    pub fn entrypoint(&self) -> terrars::ListRef<AppEngineStandardAppVersionEntrypointElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entrypoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `handlers` after provisioning.\n"]
    pub fn handlers(&self) -> terrars::ListRef<AppEngineStandardAppVersionHandlersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.handlers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `libraries` after provisioning.\n"]
    pub fn libraries(&self) -> terrars::ListRef<AppEngineStandardAppVersionLibrariesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.libraries", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `manual_scaling` after provisioning.\n"]
    pub fn manual_scaling(&self) -> terrars::ListRef<AppEngineStandardAppVersionManualScalingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.manual_scaling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AppEngineStandardAppVersionTimeoutsElRef {
        AppEngineStandardAppVersionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_access_connector` after provisioning.\n"]
    pub fn vpc_access_connector(&self) -> terrars::ListRef<AppEngineStandardAppVersionVpcAccessConnectorElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_access_connector", self.extract_ref()))
    }
}

impl terrars::Referable for AppEngineStandardAppVersion {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AppEngineStandardAppVersion { }

impl terrars::ToListMappable for AppEngineStandardAppVersion {
    type O = terrars::ListRef<AppEngineStandardAppVersionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AppEngineStandardAppVersion_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_app_engine_standard_app_version".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppEngineStandardAppVersion {
    pub tf_id: std::string::String,
    #[doc = "Desired runtime. Example python27."]
    pub runtime: terrars::PrimField<std::string::String>,
    #[doc = "AppEngine service resource"]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildAppEngineStandardAppVersion {
    pub fn build(self, stack: &mut terrars::Stack) -> AppEngineStandardAppVersion {
        let out = AppEngineStandardAppVersion(std::rc::Rc::new(AppEngineStandardAppVersion_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AppEngineStandardAppVersionData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                app_engine_apis: core::default::Default::default(),
                delete_service_on_destroy: core::default::Default::default(),
                env_variables: core::default::Default::default(),
                id: core::default::Default::default(),
                inbound_services: core::default::Default::default(),
                instance_class: core::default::Default::default(),
                noop_on_destroy: core::default::Default::default(),
                project: core::default::Default::default(),
                runtime: self.runtime,
                runtime_api_version: core::default::Default::default(),
                service: self.service,
                service_account: core::default::Default::default(),
                threadsafe: core::default::Default::default(),
                version_id: core::default::Default::default(),
                automatic_scaling: core::default::Default::default(),
                basic_scaling: core::default::Default::default(),
                deployment: core::default::Default::default(),
                entrypoint: core::default::Default::default(),
                handlers: core::default::Default::default(),
                libraries: core::default::Default::default(),
                manual_scaling: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vpc_access_connector: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppEngineStandardAppVersionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineStandardAppVersionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AppEngineStandardAppVersionRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `app_engine_apis` after provisioning.\nAllows App Engine second generation runtimes to access the legacy bundled services."]
    pub fn app_engine_apis(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_engine_apis", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_service_on_destroy` after provisioning.\nIf set to 'true', the service will be deleted if it is the last version."]
    pub fn delete_service_on_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_service_on_destroy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `env_variables` after provisioning.\nEnvironment variables available to the application."]
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
        "Get a reference to the value of field `instance_class` after provisioning.\nInstance class that is used to run this version. Valid values are\nAutomaticScaling: F1, F2, F4, F4_1G\nBasicScaling or ManualScaling: B1, B2, B4, B4_1G, B8\nDefaults to F1 for AutomaticScaling and B2 for ManualScaling and BasicScaling. If no scaling is specified, AutomaticScaling is chosen."]
    pub fn instance_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_class", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nFull path to the Version resource in the API. Example, \"v1\"."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `service` after provisioning.\nAppEngine service resource"]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nThe identity that the deployed version will run as. Admin API will use the App Engine Appspot service account as default if this field is neither provided in app.yaml file nor through CLI flag."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `threadsafe` after provisioning.\nWhether multiple requests can be dispatched to this version at once."]
    pub fn threadsafe(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threadsafe", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version_id` after provisioning.\nRelative name of the version within the service. For example, 'v1'. Version names can contain only lowercase letters, numbers, or hyphens. Reserved names,\"default\", \"latest\", and any name with the prefix \"ah-\"."]
    pub fn version_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `automatic_scaling` after provisioning.\n"]
    pub fn automatic_scaling(&self) -> terrars::ListRef<AppEngineStandardAppVersionAutomaticScalingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.automatic_scaling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `basic_scaling` after provisioning.\n"]
    pub fn basic_scaling(&self) -> terrars::ListRef<AppEngineStandardAppVersionBasicScalingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.basic_scaling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment` after provisioning.\n"]
    pub fn deployment(&self) -> terrars::ListRef<AppEngineStandardAppVersionDeploymentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entrypoint` after provisioning.\n"]
    pub fn entrypoint(&self) -> terrars::ListRef<AppEngineStandardAppVersionEntrypointElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entrypoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `handlers` after provisioning.\n"]
    pub fn handlers(&self) -> terrars::ListRef<AppEngineStandardAppVersionHandlersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.handlers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `libraries` after provisioning.\n"]
    pub fn libraries(&self) -> terrars::ListRef<AppEngineStandardAppVersionLibrariesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.libraries", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `manual_scaling` after provisioning.\n"]
    pub fn manual_scaling(&self) -> terrars::ListRef<AppEngineStandardAppVersionManualScalingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.manual_scaling", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AppEngineStandardAppVersionTimeoutsElRef {
        AppEngineStandardAppVersionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `vpc_access_connector` after provisioning.\n"]
    pub fn vpc_access_connector(&self) -> terrars::ListRef<AppEngineStandardAppVersionVpcAccessConnectorElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_access_connector", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AppEngineStandardAppVersionAutomaticScalingElStandardSchedulerSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_instances: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_instances: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_cpu_utilization: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_throughput_utilization: std::option::Option<terrars::PrimField<f64>>,
}

impl AppEngineStandardAppVersionAutomaticScalingElStandardSchedulerSettingsEl {
    #[doc =
        "Set the field `max_instances`.\nMaximum number of instances to run for this version. Set to zero to disable maxInstances configuration."]
    pub fn set_max_instances(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_instances = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_instances`.\nMinimum number of instances to run for this version. Set to zero to disable minInstances configuration."]
    pub fn set_min_instances(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_instances = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `target_cpu_utilization`.\nTarget CPU utilization ratio to maintain when scaling. Should be a value in the range [0.50, 0.95], zero, or a negative value."]
    pub fn set_target_cpu_utilization(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_cpu_utilization = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `target_throughput_utilization`.\nTarget throughput utilization ratio to maintain when scaling. Should be a value in the range [0.50, 0.95], zero, or a negative value."]
    pub fn set_target_throughput_utilization(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_throughput_utilization = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppEngineStandardAppVersionAutomaticScalingElStandardSchedulerSettingsEl {
    type O = terrars::BlockAssignable<AppEngineStandardAppVersionAutomaticScalingElStandardSchedulerSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineStandardAppVersionAutomaticScalingElStandardSchedulerSettingsEl {}

impl BuildAppEngineStandardAppVersionAutomaticScalingElStandardSchedulerSettingsEl {
    pub fn build(self) -> AppEngineStandardAppVersionAutomaticScalingElStandardSchedulerSettingsEl {
        AppEngineStandardAppVersionAutomaticScalingElStandardSchedulerSettingsEl {
            max_instances: core::default::Default::default(),
            min_instances: core::default::Default::default(),
            target_cpu_utilization: core::default::Default::default(),
            target_throughput_utilization: core::default::Default::default(),
        }
    }
}

pub struct AppEngineStandardAppVersionAutomaticScalingElStandardSchedulerSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineStandardAppVersionAutomaticScalingElStandardSchedulerSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppEngineStandardAppVersionAutomaticScalingElStandardSchedulerSettingsElRef {
        AppEngineStandardAppVersionAutomaticScalingElStandardSchedulerSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineStandardAppVersionAutomaticScalingElStandardSchedulerSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_instances` after provisioning.\nMaximum number of instances to run for this version. Set to zero to disable maxInstances configuration."]
    pub fn max_instances(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_instances", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_instances` after provisioning.\nMinimum number of instances to run for this version. Set to zero to disable minInstances configuration."]
    pub fn min_instances(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_instances", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_cpu_utilization` after provisioning.\nTarget CPU utilization ratio to maintain when scaling. Should be a value in the range [0.50, 0.95], zero, or a negative value."]
    pub fn target_cpu_utilization(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_cpu_utilization", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_throughput_utilization` after provisioning.\nTarget throughput utilization ratio to maintain when scaling. Should be a value in the range [0.50, 0.95], zero, or a negative value."]
    pub fn target_throughput_utilization(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_throughput_utilization", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppEngineStandardAppVersionAutomaticScalingElDynamic {
    standard_scheduler_settings: std
    ::option
    ::Option<terrars::DynamicBlock<AppEngineStandardAppVersionAutomaticScalingElStandardSchedulerSettingsEl>>,
}

#[derive(Serialize)]
pub struct AppEngineStandardAppVersionAutomaticScalingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_concurrent_requests: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_idle_instances: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_pending_latency: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_idle_instances: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_pending_latency: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    standard_scheduler_settings: std
    ::option
    ::Option<std::vec::Vec<AppEngineStandardAppVersionAutomaticScalingElStandardSchedulerSettingsEl>>,
    dynamic: AppEngineStandardAppVersionAutomaticScalingElDynamic,
}

impl AppEngineStandardAppVersionAutomaticScalingEl {
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
        "Set the field `max_pending_latency`.\nMaximum amount of time that a request should wait in the pending queue before starting a new instance to handle it.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn set_max_pending_latency(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.max_pending_latency = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_idle_instances`.\nMinimum number of idle instances that should be maintained for this version. Only applicable for the default version of a service."]
    pub fn set_min_idle_instances(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_idle_instances = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_pending_latency`.\nMinimum amount of time a request should wait in the pending queue before starting a new instance to handle it.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn set_min_pending_latency(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.min_pending_latency = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `standard_scheduler_settings`.\n"]
    pub fn set_standard_scheduler_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<AppEngineStandardAppVersionAutomaticScalingElStandardSchedulerSettingsEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.standard_scheduler_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.standard_scheduler_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppEngineStandardAppVersionAutomaticScalingEl {
    type O = terrars::BlockAssignable<AppEngineStandardAppVersionAutomaticScalingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineStandardAppVersionAutomaticScalingEl {}

impl BuildAppEngineStandardAppVersionAutomaticScalingEl {
    pub fn build(self) -> AppEngineStandardAppVersionAutomaticScalingEl {
        AppEngineStandardAppVersionAutomaticScalingEl {
            max_concurrent_requests: core::default::Default::default(),
            max_idle_instances: core::default::Default::default(),
            max_pending_latency: core::default::Default::default(),
            min_idle_instances: core::default::Default::default(),
            min_pending_latency: core::default::Default::default(),
            standard_scheduler_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppEngineStandardAppVersionAutomaticScalingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineStandardAppVersionAutomaticScalingElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppEngineStandardAppVersionAutomaticScalingElRef {
        AppEngineStandardAppVersionAutomaticScalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineStandardAppVersionAutomaticScalingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
        "Get a reference to the value of field `max_pending_latency` after provisioning.\nMaximum amount of time that a request should wait in the pending queue before starting a new instance to handle it.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn max_pending_latency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_pending_latency", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_idle_instances` after provisioning.\nMinimum number of idle instances that should be maintained for this version. Only applicable for the default version of a service."]
    pub fn min_idle_instances(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_idle_instances", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_pending_latency` after provisioning.\nMinimum amount of time a request should wait in the pending queue before starting a new instance to handle it.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn min_pending_latency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_pending_latency", self.base))
    }

    #[doc = "Get a reference to the value of field `standard_scheduler_settings` after provisioning.\n"]
    pub fn standard_scheduler_settings(
        &self,
    ) -> terrars::ListRef<AppEngineStandardAppVersionAutomaticScalingElStandardSchedulerSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.standard_scheduler_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct AppEngineStandardAppVersionBasicScalingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    max_instances: terrars::PrimField<f64>,
}

impl AppEngineStandardAppVersionBasicScalingEl {
    #[doc =
        "Set the field `idle_timeout`.\nDuration of time after the last request that an instance must wait before the instance is shut down.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\". Defaults to 900s."]
    pub fn set_idle_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.idle_timeout = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppEngineStandardAppVersionBasicScalingEl {
    type O = terrars::BlockAssignable<AppEngineStandardAppVersionBasicScalingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineStandardAppVersionBasicScalingEl {
    #[doc = "Maximum number of instances to create for this version. Must be in the range [1.0, 200.0]."]
    pub max_instances: terrars::PrimField<f64>,
}

impl BuildAppEngineStandardAppVersionBasicScalingEl {
    pub fn build(self) -> AppEngineStandardAppVersionBasicScalingEl {
        AppEngineStandardAppVersionBasicScalingEl {
            idle_timeout: core::default::Default::default(),
            max_instances: self.max_instances,
        }
    }
}

pub struct AppEngineStandardAppVersionBasicScalingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineStandardAppVersionBasicScalingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppEngineStandardAppVersionBasicScalingElRef {
        AppEngineStandardAppVersionBasicScalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineStandardAppVersionBasicScalingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `idle_timeout` after provisioning.\nDuration of time after the last request that an instance must wait before the instance is shut down.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\". Defaults to 900s."]
    pub fn idle_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idle_timeout", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_instances` after provisioning.\nMaximum number of instances to create for this version. Must be in the range [1.0, 200.0]."]
    pub fn max_instances(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_instances", self.base))
    }
}

#[derive(Serialize)]
pub struct AppEngineStandardAppVersionDeploymentElFilesEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sha1_sum: std::option::Option<terrars::PrimField<std::string::String>>,
    source_url: terrars::PrimField<std::string::String>,
}

impl AppEngineStandardAppVersionDeploymentElFilesEl {
    #[doc = "Set the field `sha1_sum`.\nSHA1 checksum of the file"]
    pub fn set_sha1_sum(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sha1_sum = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppEngineStandardAppVersionDeploymentElFilesEl {
    type O = terrars::BlockAssignable<AppEngineStandardAppVersionDeploymentElFilesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineStandardAppVersionDeploymentElFilesEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "Source URL"]
    pub source_url: terrars::PrimField<std::string::String>,
}

impl BuildAppEngineStandardAppVersionDeploymentElFilesEl {
    pub fn build(self) -> AppEngineStandardAppVersionDeploymentElFilesEl {
        AppEngineStandardAppVersionDeploymentElFilesEl {
            name: self.name,
            sha1_sum: core::default::Default::default(),
            source_url: self.source_url,
        }
    }
}

pub struct AppEngineStandardAppVersionDeploymentElFilesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineStandardAppVersionDeploymentElFilesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppEngineStandardAppVersionDeploymentElFilesElRef {
        AppEngineStandardAppVersionDeploymentElFilesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineStandardAppVersionDeploymentElFilesElRef {
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
pub struct AppEngineStandardAppVersionDeploymentElZipEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    files_count: std::option::Option<terrars::PrimField<f64>>,
    source_url: terrars::PrimField<std::string::String>,
}

impl AppEngineStandardAppVersionDeploymentElZipEl {
    #[doc = "Set the field `files_count`.\nfiles count"]
    pub fn set_files_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.files_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppEngineStandardAppVersionDeploymentElZipEl {
    type O = terrars::BlockAssignable<AppEngineStandardAppVersionDeploymentElZipEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineStandardAppVersionDeploymentElZipEl {
    #[doc = "Source URL"]
    pub source_url: terrars::PrimField<std::string::String>,
}

impl BuildAppEngineStandardAppVersionDeploymentElZipEl {
    pub fn build(self) -> AppEngineStandardAppVersionDeploymentElZipEl {
        AppEngineStandardAppVersionDeploymentElZipEl {
            files_count: core::default::Default::default(),
            source_url: self.source_url,
        }
    }
}

pub struct AppEngineStandardAppVersionDeploymentElZipElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineStandardAppVersionDeploymentElZipElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppEngineStandardAppVersionDeploymentElZipElRef {
        AppEngineStandardAppVersionDeploymentElZipElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineStandardAppVersionDeploymentElZipElRef {
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
struct AppEngineStandardAppVersionDeploymentElDynamic {
    files: std::option::Option<terrars::DynamicBlock<AppEngineStandardAppVersionDeploymentElFilesEl>>,
    zip: std::option::Option<terrars::DynamicBlock<AppEngineStandardAppVersionDeploymentElZipEl>>,
}

#[derive(Serialize)]
pub struct AppEngineStandardAppVersionDeploymentEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    files: std::option::Option<std::vec::Vec<AppEngineStandardAppVersionDeploymentElFilesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zip: std::option::Option<std::vec::Vec<AppEngineStandardAppVersionDeploymentElZipEl>>,
    dynamic: AppEngineStandardAppVersionDeploymentElDynamic,
}

impl AppEngineStandardAppVersionDeploymentEl {
    #[doc = "Set the field `files`.\n"]
    pub fn set_files(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppEngineStandardAppVersionDeploymentElFilesEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppEngineStandardAppVersionDeploymentElZipEl>>,
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

impl terrars::ToListMappable for AppEngineStandardAppVersionDeploymentEl {
    type O = terrars::BlockAssignable<AppEngineStandardAppVersionDeploymentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineStandardAppVersionDeploymentEl {}

impl BuildAppEngineStandardAppVersionDeploymentEl {
    pub fn build(self) -> AppEngineStandardAppVersionDeploymentEl {
        AppEngineStandardAppVersionDeploymentEl {
            files: core::default::Default::default(),
            zip: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppEngineStandardAppVersionDeploymentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineStandardAppVersionDeploymentElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppEngineStandardAppVersionDeploymentElRef {
        AppEngineStandardAppVersionDeploymentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineStandardAppVersionDeploymentElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `zip` after provisioning.\n"]
    pub fn zip(&self) -> terrars::ListRef<AppEngineStandardAppVersionDeploymentElZipElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zip", self.base))
    }
}

#[derive(Serialize)]
pub struct AppEngineStandardAppVersionEntrypointEl {
    shell: terrars::PrimField<std::string::String>,
}

impl AppEngineStandardAppVersionEntrypointEl { }

impl terrars::ToListMappable for AppEngineStandardAppVersionEntrypointEl {
    type O = terrars::BlockAssignable<AppEngineStandardAppVersionEntrypointEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineStandardAppVersionEntrypointEl {
    #[doc = "The format should be a shell command that can be fed to bash -c."]
    pub shell: terrars::PrimField<std::string::String>,
}

impl BuildAppEngineStandardAppVersionEntrypointEl {
    pub fn build(self) -> AppEngineStandardAppVersionEntrypointEl {
        AppEngineStandardAppVersionEntrypointEl { shell: self.shell }
    }
}

pub struct AppEngineStandardAppVersionEntrypointElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineStandardAppVersionEntrypointElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppEngineStandardAppVersionEntrypointElRef {
        AppEngineStandardAppVersionEntrypointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineStandardAppVersionEntrypointElRef {
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
pub struct AppEngineStandardAppVersionHandlersElScriptEl {
    script_path: terrars::PrimField<std::string::String>,
}

impl AppEngineStandardAppVersionHandlersElScriptEl { }

impl terrars::ToListMappable for AppEngineStandardAppVersionHandlersElScriptEl {
    type O = terrars::BlockAssignable<AppEngineStandardAppVersionHandlersElScriptEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineStandardAppVersionHandlersElScriptEl {
    #[doc = "Path to the script from the application root directory."]
    pub script_path: terrars::PrimField<std::string::String>,
}

impl BuildAppEngineStandardAppVersionHandlersElScriptEl {
    pub fn build(self) -> AppEngineStandardAppVersionHandlersElScriptEl {
        AppEngineStandardAppVersionHandlersElScriptEl { script_path: self.script_path }
    }
}

pub struct AppEngineStandardAppVersionHandlersElScriptElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineStandardAppVersionHandlersElScriptElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppEngineStandardAppVersionHandlersElScriptElRef {
        AppEngineStandardAppVersionHandlersElScriptElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineStandardAppVersionHandlersElScriptElRef {
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
pub struct AppEngineStandardAppVersionHandlersElStaticFilesEl {
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

impl AppEngineStandardAppVersionHandlersElStaticFilesEl {
    #[doc =
        "Set the field `application_readable`.\nWhether files should also be uploaded as code data. By default, files declared in static file handlers are uploaded as\nstatic data and are only served to end users; they cannot be read by the application. If enabled, uploads are charged\nagainst both your code and static data storage resource quotas."]
    pub fn set_application_readable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.application_readable = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `expiration`.\nTime a static file served by this handler should be cached by web proxies and browsers.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example \"3.5s\"."]
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
        "Set the field `path`.\nPath to the static files matched by the URL pattern, from the application root directory. The path can refer to text matched in groupings in the URL pattern."]
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

impl terrars::ToListMappable for AppEngineStandardAppVersionHandlersElStaticFilesEl {
    type O = terrars::BlockAssignable<AppEngineStandardAppVersionHandlersElStaticFilesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineStandardAppVersionHandlersElStaticFilesEl {}

impl BuildAppEngineStandardAppVersionHandlersElStaticFilesEl {
    pub fn build(self) -> AppEngineStandardAppVersionHandlersElStaticFilesEl {
        AppEngineStandardAppVersionHandlersElStaticFilesEl {
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

pub struct AppEngineStandardAppVersionHandlersElStaticFilesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineStandardAppVersionHandlersElStaticFilesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppEngineStandardAppVersionHandlersElStaticFilesElRef {
        AppEngineStandardAppVersionHandlersElStaticFilesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineStandardAppVersionHandlersElStaticFilesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `application_readable` after provisioning.\nWhether files should also be uploaded as code data. By default, files declared in static file handlers are uploaded as\nstatic data and are only served to end users; they cannot be read by the application. If enabled, uploads are charged\nagainst both your code and static data storage resource quotas."]
    pub fn application_readable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_readable", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expiration` after provisioning.\nTime a static file served by this handler should be cached by web proxies and browsers.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example \"3.5s\"."]
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
        "Get a reference to the value of field `path` after provisioning.\nPath to the static files matched by the URL pattern, from the application root directory. The path can refer to text matched in groupings in the URL pattern."]
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
struct AppEngineStandardAppVersionHandlersElDynamic {
    script: std::option::Option<terrars::DynamicBlock<AppEngineStandardAppVersionHandlersElScriptEl>>,
    static_files: std::option::Option<terrars::DynamicBlock<AppEngineStandardAppVersionHandlersElStaticFilesEl>>,
}

#[derive(Serialize)]
pub struct AppEngineStandardAppVersionHandlersEl {
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
    script: std::option::Option<std::vec::Vec<AppEngineStandardAppVersionHandlersElScriptEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    static_files: std::option::Option<std::vec::Vec<AppEngineStandardAppVersionHandlersElStaticFilesEl>>,
    dynamic: AppEngineStandardAppVersionHandlersElDynamic,
}

impl AppEngineStandardAppVersionHandlersEl {
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppEngineStandardAppVersionHandlersElScriptEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<AppEngineStandardAppVersionHandlersElStaticFilesEl>>,
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

impl terrars::ToListMappable for AppEngineStandardAppVersionHandlersEl {
    type O = terrars::BlockAssignable<AppEngineStandardAppVersionHandlersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineStandardAppVersionHandlersEl {}

impl BuildAppEngineStandardAppVersionHandlersEl {
    pub fn build(self) -> AppEngineStandardAppVersionHandlersEl {
        AppEngineStandardAppVersionHandlersEl {
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

pub struct AppEngineStandardAppVersionHandlersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineStandardAppVersionHandlersElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppEngineStandardAppVersionHandlersElRef {
        AppEngineStandardAppVersionHandlersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineStandardAppVersionHandlersElRef {
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
    pub fn script(&self) -> terrars::ListRef<AppEngineStandardAppVersionHandlersElScriptElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.script", self.base))
    }

    #[doc = "Get a reference to the value of field `static_files` after provisioning.\n"]
    pub fn static_files(&self) -> terrars::ListRef<AppEngineStandardAppVersionHandlersElStaticFilesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.static_files", self.base))
    }
}

#[derive(Serialize)]
pub struct AppEngineStandardAppVersionLibrariesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppEngineStandardAppVersionLibrariesEl {
    #[doc = "Set the field `name`.\nName of the library. Example \"django\"."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\nVersion of the library to select, or \"latest\"."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppEngineStandardAppVersionLibrariesEl {
    type O = terrars::BlockAssignable<AppEngineStandardAppVersionLibrariesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineStandardAppVersionLibrariesEl {}

impl BuildAppEngineStandardAppVersionLibrariesEl {
    pub fn build(self) -> AppEngineStandardAppVersionLibrariesEl {
        AppEngineStandardAppVersionLibrariesEl {
            name: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct AppEngineStandardAppVersionLibrariesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineStandardAppVersionLibrariesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppEngineStandardAppVersionLibrariesElRef {
        AppEngineStandardAppVersionLibrariesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineStandardAppVersionLibrariesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the library. Example \"django\"."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nVersion of the library to select, or \"latest\"."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct AppEngineStandardAppVersionManualScalingEl {
    instances: terrars::PrimField<f64>,
}

impl AppEngineStandardAppVersionManualScalingEl { }

impl terrars::ToListMappable for AppEngineStandardAppVersionManualScalingEl {
    type O = terrars::BlockAssignable<AppEngineStandardAppVersionManualScalingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineStandardAppVersionManualScalingEl {
    #[doc =
        "Number of instances to assign to the service at the start.\n\n**Note:** When managing the number of instances at runtime through the App Engine Admin API or the (now deprecated) Python 2\nModules API set_num_instances() you must use 'lifecycle.ignore_changes = [\"manual_scaling\"[0].instances]' to prevent drift detection."]
    pub instances: terrars::PrimField<f64>,
}

impl BuildAppEngineStandardAppVersionManualScalingEl {
    pub fn build(self) -> AppEngineStandardAppVersionManualScalingEl {
        AppEngineStandardAppVersionManualScalingEl { instances: self.instances }
    }
}

pub struct AppEngineStandardAppVersionManualScalingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineStandardAppVersionManualScalingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppEngineStandardAppVersionManualScalingElRef {
        AppEngineStandardAppVersionManualScalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineStandardAppVersionManualScalingElRef {
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
pub struct AppEngineStandardAppVersionTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppEngineStandardAppVersionTimeoutsEl {
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

impl terrars::ToListMappable for AppEngineStandardAppVersionTimeoutsEl {
    type O = terrars::BlockAssignable<AppEngineStandardAppVersionTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineStandardAppVersionTimeoutsEl {}

impl BuildAppEngineStandardAppVersionTimeoutsEl {
    pub fn build(self) -> AppEngineStandardAppVersionTimeoutsEl {
        AppEngineStandardAppVersionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct AppEngineStandardAppVersionTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineStandardAppVersionTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppEngineStandardAppVersionTimeoutsElRef {
        AppEngineStandardAppVersionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineStandardAppVersionTimeoutsElRef {
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
pub struct AppEngineStandardAppVersionVpcAccessConnectorEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress_setting: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
}

impl AppEngineStandardAppVersionVpcAccessConnectorEl {
    #[doc =
        "Set the field `egress_setting`.\nThe egress setting for the connector, controlling what traffic is diverted through it."]
    pub fn set_egress_setting(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.egress_setting = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppEngineStandardAppVersionVpcAccessConnectorEl {
    type O = terrars::BlockAssignable<AppEngineStandardAppVersionVpcAccessConnectorEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineStandardAppVersionVpcAccessConnectorEl {
    #[doc = "Full Serverless VPC Access Connector name e.g. /projects/my-project/locations/us-central1/connectors/c1."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAppEngineStandardAppVersionVpcAccessConnectorEl {
    pub fn build(self) -> AppEngineStandardAppVersionVpcAccessConnectorEl {
        AppEngineStandardAppVersionVpcAccessConnectorEl {
            egress_setting: core::default::Default::default(),
            name: self.name,
        }
    }
}

pub struct AppEngineStandardAppVersionVpcAccessConnectorElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineStandardAppVersionVpcAccessConnectorElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppEngineStandardAppVersionVpcAccessConnectorElRef {
        AppEngineStandardAppVersionVpcAccessConnectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineStandardAppVersionVpcAccessConnectorElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `egress_setting` after provisioning.\nThe egress setting for the connector, controlling what traffic is diverted through it."]
    pub fn egress_setting(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.egress_setting", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nFull Serverless VPC Access Connector name e.g. /projects/my-project/locations/us-central1/connectors/c1."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppEngineStandardAppVersionDynamic {
    automatic_scaling: std::option::Option<terrars::DynamicBlock<AppEngineStandardAppVersionAutomaticScalingEl>>,
    basic_scaling: std::option::Option<terrars::DynamicBlock<AppEngineStandardAppVersionBasicScalingEl>>,
    deployment: std::option::Option<terrars::DynamicBlock<AppEngineStandardAppVersionDeploymentEl>>,
    entrypoint: std::option::Option<terrars::DynamicBlock<AppEngineStandardAppVersionEntrypointEl>>,
    handlers: std::option::Option<terrars::DynamicBlock<AppEngineStandardAppVersionHandlersEl>>,
    libraries: std::option::Option<terrars::DynamicBlock<AppEngineStandardAppVersionLibrariesEl>>,
    manual_scaling: std::option::Option<terrars::DynamicBlock<AppEngineStandardAppVersionManualScalingEl>>,
    vpc_access_connector: std
    ::option
    ::Option<terrars::DynamicBlock<AppEngineStandardAppVersionVpcAccessConnectorEl>>,
}
