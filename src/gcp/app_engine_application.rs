#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct AppEngineApplicationData_ {
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
    auth_domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    serving_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    feature_settings: std::option::Option<std::vec::Vec<AppEngineApplication_FeatureSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iap: std::option::Option<std::vec::Vec<AppEngineApplication_Iap>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<AppEngineApplication_Timeouts>,
    dynamic: AppEngineApplicationDynamic,
}

struct AppEngineApplication_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AppEngineApplicationData_>,
}

#[derive(Clone)]
pub struct AppEngineApplication(std::rc::Rc<AppEngineApplication_>);

impl AppEngineApplication {
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

    #[doc = "Set the field `auth_domain`.\nThe domain to authenticate users with when using App Engine's User API."]
    pub fn set_auth_domain(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().auth_domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database_type`.\n"]
    pub fn set_database_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().database_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\nThe project ID to create the application under."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `serving_status`.\nThe serving status of the app."]
    pub fn set_serving_status(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().serving_status = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ssl_policy`.\nThe SSL policy that will be applied to the application. If set to Modern it will restrict traffic with TLS \\u003c 1.2 and allow only Modern Ciphers suite"]
    pub fn set_ssl_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ssl_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `feature_settings`.\n"]
    pub fn set_feature_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppEngineApplication_FeatureSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().feature_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.feature_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `iap`.\n"]
    pub fn set_iap(self, v: impl std::convert::Into<terrars::BlockAssignable<AppEngineApplication_Iap>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().iap = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.iap = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<AppEngineApplication_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `app_id` after provisioning.\nIdentifier of the app."]
    pub fn app_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auth_domain` after provisioning.\nThe domain to authenticate users with when using App Engine's User API."]
    pub fn auth_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_domain", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `code_bucket` after provisioning.\nThe GCS bucket code is being stored in for this app."]
    pub fn code_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code_bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_type` after provisioning.\n"]
    pub fn database_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_bucket` after provisioning.\nThe GCS bucket content is being stored in for this app."]
    pub fn default_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_bucket", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_hostname` after provisioning.\nThe default hostname for this app."]
    pub fn default_hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_hostname", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gcr_domain` after provisioning.\nThe GCR domain used for storing managed Docker images for this app."]
    pub fn gcr_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcr_domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location_id` after provisioning.\nThe location to serve the app from."]
    pub fn location_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nUnique name of the app."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe project ID to create the application under."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `serving_status` after provisioning.\nThe serving status of the app."]
    pub fn serving_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serving_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ssl_policy` after provisioning.\nThe SSL policy that will be applied to the application. If set to Modern it will restrict traffic with TLS \\u003c 1.2 and allow only Modern Ciphers suite"]
    pub fn ssl_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `url_dispatch_rule` after provisioning.\nA list of dispatch rule blocks. Each block has a domain, path, and service field."]
    pub fn url_dispatch_rule(&self) -> terrars::ListRef<AppEngineApplication_UrlDispatchRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.url_dispatch_rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `feature_settings` after provisioning.\n"]
    pub fn feature_settings(&self) -> terrars::ListRef<AppEngineApplication_FeatureSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.feature_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iap` after provisioning.\n"]
    pub fn iap(&self) -> terrars::ListRef<AppEngineApplication_IapRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iap", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AppEngineApplication_TimeoutsRef {
        AppEngineApplication_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for AppEngineApplication {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AppEngineApplication { }

impl terrars::ToListMappable for AppEngineApplication {
    type O = terrars::ListRef<AppEngineApplicationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AppEngineApplication_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_app_engine_application".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppEngineApplication {
    pub tf_id: std::string::String,
    #[doc = "The location to serve the app from."]
    pub location_id: terrars::PrimField<std::string::String>,
}

impl BuildAppEngineApplication {
    pub fn build(self, stack: &mut terrars::Stack) -> AppEngineApplication {
        let out = AppEngineApplication(std::rc::Rc::new(AppEngineApplication_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AppEngineApplicationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                auth_domain: core::default::Default::default(),
                database_type: core::default::Default::default(),
                id: core::default::Default::default(),
                location_id: self.location_id,
                project: core::default::Default::default(),
                serving_status: core::default::Default::default(),
                ssl_policy: core::default::Default::default(),
                feature_settings: core::default::Default::default(),
                iap: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppEngineApplicationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineApplicationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AppEngineApplicationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app_id` after provisioning.\nIdentifier of the app."]
    pub fn app_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `auth_domain` after provisioning.\nThe domain to authenticate users with when using App Engine's User API."]
    pub fn auth_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_domain", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `code_bucket` after provisioning.\nThe GCS bucket code is being stored in for this app."]
    pub fn code_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code_bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `database_type` after provisioning.\n"]
    pub fn database_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_bucket` after provisioning.\nThe GCS bucket content is being stored in for this app."]
    pub fn default_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_bucket", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_hostname` after provisioning.\nThe default hostname for this app."]
    pub fn default_hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_hostname", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `gcr_domain` after provisioning.\nThe GCR domain used for storing managed Docker images for this app."]
    pub fn gcr_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcr_domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location_id` after provisioning.\nThe location to serve the app from."]
    pub fn location_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nUnique name of the app."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe project ID to create the application under."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `serving_status` after provisioning.\nThe serving status of the app."]
    pub fn serving_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serving_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ssl_policy` after provisioning.\nThe SSL policy that will be applied to the application. If set to Modern it will restrict traffic with TLS \\u003c 1.2 and allow only Modern Ciphers suite"]
    pub fn ssl_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `url_dispatch_rule` after provisioning.\nA list of dispatch rule blocks. Each block has a domain, path, and service field."]
    pub fn url_dispatch_rule(&self) -> terrars::ListRef<AppEngineApplication_UrlDispatchRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.url_dispatch_rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `feature_settings` after provisioning.\n"]
    pub fn feature_settings(&self) -> terrars::ListRef<AppEngineApplication_FeatureSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.feature_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iap` after provisioning.\n"]
    pub fn iap(&self) -> terrars::ListRef<AppEngineApplication_IapRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iap", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AppEngineApplication_TimeoutsRef {
        AppEngineApplication_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct AppEngineApplicationDynamic {
    feature_settings: std::option::Option<terrars::DynamicBlock<AppEngineApplication_FeatureSettings>>,
    iap: std::option::Option<terrars::DynamicBlock<AppEngineApplication_Iap>>,
}

#[derive(Serialize)]
pub struct AppEngineApplication_UrlDispatchRule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppEngineApplication_UrlDispatchRule {
    #[doc = "Set the field `domain`.\n"]
    pub fn set_domain(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service`.\n"]
    pub fn set_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppEngineApplication_UrlDispatchRule {
    type O = terrars::BlockAssignable<AppEngineApplication_UrlDispatchRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineApplication_UrlDispatchRule {}

impl BuildAppEngineApplication_UrlDispatchRule {
    pub fn build(self) -> AppEngineApplication_UrlDispatchRule {
        AppEngineApplication_UrlDispatchRule {
            domain: core::default::Default::default(),
            path: core::default::Default::default(),
            service: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppEngineApplication_UrlDispatchRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineApplication_UrlDispatchRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppEngineApplication_UrlDispatchRuleRef {
        AppEngineApplication_UrlDispatchRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineApplication_UrlDispatchRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize)]
pub struct AppEngineApplication_FeatureSettings {
    split_health_checks: terrars::PrimField<bool>,
}

impl AppEngineApplication_FeatureSettings { }

impl terrars::ToListMappable for AppEngineApplication_FeatureSettings {
    type O = terrars::BlockAssignable<AppEngineApplication_FeatureSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineApplication_FeatureSettings {
    #[doc = ""]
    pub split_health_checks: terrars::PrimField<bool>,
}

impl BuildAppEngineApplication_FeatureSettings {
    pub fn build(self) -> AppEngineApplication_FeatureSettings {
        AppEngineApplication_FeatureSettings { split_health_checks: self.split_health_checks }
    }
}

#[allow(dead_code)]
pub struct AppEngineApplication_FeatureSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineApplication_FeatureSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppEngineApplication_FeatureSettingsRef {
        AppEngineApplication_FeatureSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineApplication_FeatureSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `split_health_checks` after provisioning.\n"]
    pub fn split_health_checks(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.split_health_checks", self.base))
    }
}

#[derive(Serialize)]
pub struct AppEngineApplication_Iap {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    oauth2_client_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth2_client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth2_client_secret_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth2_client_secret_wo_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppEngineApplication_Iap {
    #[doc = "Set the field `enabled`.\nAdapted for use with the app"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `oauth2_client_secret`.\nOAuth2 client secret to use for the authentication flow. The SHA-256 hash of the value is returned in the oauth2ClientSecretSha256 field."]
    pub fn set_oauth2_client_secret(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.oauth2_client_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `oauth2_client_secret_wo`.\nOAuth2 client secret to use for the authentication flow. The SHA-256 hash of the value is returned in the oauth2ClientSecretSha256 field."]
    pub fn set_oauth2_client_secret_wo(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.oauth2_client_secret_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `oauth2_client_secret_wo_version`.\nTriggers update of `oauth2_client_secret_wo` write-only. Increment this value when an update to `oauth2_client_secret_wo` is needed. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn set_oauth2_client_secret_wo_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.oauth2_client_secret_wo_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppEngineApplication_Iap {
    type O = terrars::BlockAssignable<AppEngineApplication_Iap>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineApplication_Iap {
    #[doc = "OAuth2 client ID to use for the authentication flow."]
    pub oauth2_client_id: terrars::PrimField<std::string::String>,
}

impl BuildAppEngineApplication_Iap {
    pub fn build(self) -> AppEngineApplication_Iap {
        AppEngineApplication_Iap {
            enabled: core::default::Default::default(),
            oauth2_client_id: self.oauth2_client_id,
            oauth2_client_secret: core::default::Default::default(),
            oauth2_client_secret_wo: core::default::Default::default(),
            oauth2_client_secret_wo_version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppEngineApplication_IapRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineApplication_IapRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppEngineApplication_IapRef {
        AppEngineApplication_IapRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineApplication_IapRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nAdapted for use with the app"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `oauth2_client_id` after provisioning.\nOAuth2 client ID to use for the authentication flow."]
    pub fn oauth2_client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oauth2_client_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `oauth2_client_secret` after provisioning.\nOAuth2 client secret to use for the authentication flow. The SHA-256 hash of the value is returned in the oauth2ClientSecretSha256 field."]
    pub fn oauth2_client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oauth2_client_secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `oauth2_client_secret_sha256` after provisioning.\nHex-encoded SHA-256 hash of the client secret."]
    pub fn oauth2_client_secret_sha256(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oauth2_client_secret_sha256", self.base))
    }

    #[doc =
        "Get a reference to the value of field `oauth2_client_secret_wo` after provisioning.\nOAuth2 client secret to use for the authentication flow. The SHA-256 hash of the value is returned in the oauth2ClientSecretSha256 field."]
    pub fn oauth2_client_secret_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oauth2_client_secret_wo", self.base))
    }

    #[doc =
        "Get a reference to the value of field `oauth2_client_secret_wo_version` after provisioning.\nTriggers update of `oauth2_client_secret_wo` write-only. Increment this value when an update to `oauth2_client_secret_wo` is needed. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn oauth2_client_secret_wo_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oauth2_client_secret_wo_version", self.base))
    }
}

#[derive(Serialize)]
pub struct AppEngineApplication_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppEngineApplication_Timeouts {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppEngineApplication_Timeouts {
    type O = terrars::BlockAssignable<AppEngineApplication_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineApplication_Timeouts {}

impl BuildAppEngineApplication_Timeouts {
    pub fn build(self) -> AppEngineApplication_Timeouts {
        AppEngineApplication_Timeouts {
            create: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppEngineApplication_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineApplication_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppEngineApplication_TimeoutsRef {
        AppEngineApplication_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineApplication_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
