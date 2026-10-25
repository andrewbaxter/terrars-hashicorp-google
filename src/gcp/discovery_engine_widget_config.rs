#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DiscoveryEngineWidgetConfigData_ {
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
    collection_id: std::option::Option<terrars::PrimField<std::string::String>>,
    engine_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    widget_config_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_settings: std::option::Option<std::vec::Vec<DiscoveryEngineWidgetConfig_AccessSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    homepage_setting: std::option::Option<std::vec::Vec<DiscoveryEngineWidgetConfig_HomepageSetting>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DiscoveryEngineWidgetConfig_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ui_branding: std::option::Option<std::vec::Vec<DiscoveryEngineWidgetConfig_UiBranding>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ui_settings: std::option::Option<std::vec::Vec<DiscoveryEngineWidgetConfig_UiSettings>>,
    dynamic: DiscoveryEngineWidgetConfigDynamic,
}

struct DiscoveryEngineWidgetConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DiscoveryEngineWidgetConfigData_>,
}

#[derive(Clone)]
pub struct DiscoveryEngineWidgetConfig(std::rc::Rc<DiscoveryEngineWidgetConfig_>);

impl DiscoveryEngineWidgetConfig {
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

    #[doc = "Set the field `collection_id`.\nThe collection ID."]
    pub fn set_collection_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().collection_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `widget_config_id`.\nThe unique ID to use for the WidgetConfig. Currently only accepts \"default_search_widget_config\"."]
    pub fn set_widget_config_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().widget_config_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `access_settings`.\n"]
    pub fn set_access_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DiscoveryEngineWidgetConfig_AccessSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().access_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.access_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `homepage_setting`.\n"]
    pub fn set_homepage_setting(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DiscoveryEngineWidgetConfig_HomepageSetting>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().homepage_setting = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.homepage_setting = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DiscoveryEngineWidgetConfig_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ui_branding`.\n"]
    pub fn set_ui_branding(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DiscoveryEngineWidgetConfig_UiBranding>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ui_branding = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ui_branding = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ui_settings`.\n"]
    pub fn set_ui_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DiscoveryEngineWidgetConfig_UiSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ui_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ui_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `collection_id` after provisioning.\nThe collection ID."]
    pub fn collection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collection_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `config_id` after provisioning.\nOutput only. Unique obfuscated identifier of a WidgetConfig."]
    pub fn config_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.config_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_id` after provisioning.\nThe engine ID."]
    pub fn engine_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe full resource name of the widget config. Format:\n'projects/{project}/locations/{location}/collections/{collection_id}/engines/{engine_id}/widgetConfigs/{widget_config_id}'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `widget_config_id` after provisioning.\nThe unique ID to use for the WidgetConfig. Currently only accepts \"default_search_widget_config\"."]
    pub fn widget_config_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.widget_config_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `access_settings` after provisioning.\n"]
    pub fn access_settings(&self) -> terrars::ListRef<DiscoveryEngineWidgetConfig_AccessSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `homepage_setting` after provisioning.\n"]
    pub fn homepage_setting(&self) -> terrars::ListRef<DiscoveryEngineWidgetConfig_HomepageSettingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.homepage_setting", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineWidgetConfig_TimeoutsRef {
        DiscoveryEngineWidgetConfig_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ui_branding` after provisioning.\n"]
    pub fn ui_branding(&self) -> terrars::ListRef<DiscoveryEngineWidgetConfig_UiBrandingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ui_branding", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ui_settings` after provisioning.\n"]
    pub fn ui_settings(&self) -> terrars::ListRef<DiscoveryEngineWidgetConfig_UiSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ui_settings", self.extract_ref()))
    }
}

impl terrars::Referable for DiscoveryEngineWidgetConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DiscoveryEngineWidgetConfig { }

impl terrars::ToListMappable for DiscoveryEngineWidgetConfig {
    type O = terrars::ListRef<DiscoveryEngineWidgetConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DiscoveryEngineWidgetConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_discovery_engine_widget_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDiscoveryEngineWidgetConfig {
    pub tf_id: std::string::String,
    #[doc = "The engine ID."]
    pub engine_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildDiscoveryEngineWidgetConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> DiscoveryEngineWidgetConfig {
        let out = DiscoveryEngineWidgetConfig(std::rc::Rc::new(DiscoveryEngineWidgetConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DiscoveryEngineWidgetConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                collection_id: core::default::Default::default(),
                engine_id: self.engine_id,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                widget_config_id: core::default::Default::default(),
                access_settings: core::default::Default::default(),
                homepage_setting: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                ui_branding: core::default::Default::default(),
                ui_settings: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DiscoveryEngineWidgetConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineWidgetConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DiscoveryEngineWidgetConfigRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `collection_id` after provisioning.\nThe collection ID."]
    pub fn collection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collection_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `config_id` after provisioning.\nOutput only. Unique obfuscated identifier of a WidgetConfig."]
    pub fn config_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.config_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_id` after provisioning.\nThe engine ID."]
    pub fn engine_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe full resource name of the widget config. Format:\n'projects/{project}/locations/{location}/collections/{collection_id}/engines/{engine_id}/widgetConfigs/{widget_config_id}'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `widget_config_id` after provisioning.\nThe unique ID to use for the WidgetConfig. Currently only accepts \"default_search_widget_config\"."]
    pub fn widget_config_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.widget_config_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `access_settings` after provisioning.\n"]
    pub fn access_settings(&self) -> terrars::ListRef<DiscoveryEngineWidgetConfig_AccessSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `homepage_setting` after provisioning.\n"]
    pub fn homepage_setting(&self) -> terrars::ListRef<DiscoveryEngineWidgetConfig_HomepageSettingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.homepage_setting", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineWidgetConfig_TimeoutsRef {
        DiscoveryEngineWidgetConfig_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ui_branding` after provisioning.\n"]
    pub fn ui_branding(&self) -> terrars::ListRef<DiscoveryEngineWidgetConfig_UiBrandingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ui_branding", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ui_settings` after provisioning.\n"]
    pub fn ui_settings(&self) -> terrars::ListRef<DiscoveryEngineWidgetConfig_UiSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ui_settings", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DiscoveryEngineWidgetConfigDynamic {
    access_settings: std::option::Option<terrars::DynamicBlock<DiscoveryEngineWidgetConfig_AccessSettings>>,
    homepage_setting: std::option::Option<terrars::DynamicBlock<DiscoveryEngineWidgetConfig_HomepageSetting>>,
    ui_branding: std::option::Option<terrars::DynamicBlock<DiscoveryEngineWidgetConfig_UiBranding>>,
    ui_settings: std::option::Option<terrars::DynamicBlock<DiscoveryEngineWidgetConfig_UiSettings>>,
}

#[derive(Serialize)]
pub struct DiscoveryEngineWidgetConfig_AccessSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_public_access: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowlisted_domains: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_web_app: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    language_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workforce_identity_pool_provider: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DiscoveryEngineWidgetConfig_AccessSettings {
    #[doc = "Set the field `allow_public_access`.\nWhether public unauthenticated access is allowed."]
    pub fn set_allow_public_access(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_public_access = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allowlisted_domains`.\nList of domains that are allowed to integrate the search widget."]
    pub fn set_allowlisted_domains(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowlisted_domains = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_web_app`.\nWhether web app access is enabled."]
    pub fn set_enable_web_app(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_web_app = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `language_code`.\nLanguage code for user interface. Use language tags defined by\n[BCP47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt). If unset, the\ndefault language code is \"en-US\"."]
    pub fn set_language_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.language_code = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `workforce_identity_pool_provider`.\nThe workforce identity pool provider used to access the widget."]
    pub fn set_workforce_identity_pool_provider(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.workforce_identity_pool_provider = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineWidgetConfig_AccessSettings {
    type O = terrars::BlockAssignable<DiscoveryEngineWidgetConfig_AccessSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineWidgetConfig_AccessSettings {}

impl BuildDiscoveryEngineWidgetConfig_AccessSettings {
    pub fn build(self) -> DiscoveryEngineWidgetConfig_AccessSettings {
        DiscoveryEngineWidgetConfig_AccessSettings {
            allow_public_access: core::default::Default::default(),
            allowlisted_domains: core::default::Default::default(),
            enable_web_app: core::default::Default::default(),
            language_code: core::default::Default::default(),
            workforce_identity_pool_provider: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineWidgetConfig_AccessSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineWidgetConfig_AccessSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DiscoveryEngineWidgetConfig_AccessSettingsRef {
        DiscoveryEngineWidgetConfig_AccessSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineWidgetConfig_AccessSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_public_access` after provisioning.\nWhether public unauthenticated access is allowed."]
    pub fn allow_public_access(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_public_access", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allowlisted_domains` after provisioning.\nList of domains that are allowed to integrate the search widget."]
    pub fn allowlisted_domains(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowlisted_domains", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_web_app` after provisioning.\nWhether web app access is enabled."]
    pub fn enable_web_app(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_web_app", self.base))
    }

    #[doc =
        "Get a reference to the value of field `language_code` after provisioning.\nLanguage code for user interface. Use language tags defined by\n[BCP47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt). If unset, the\ndefault language code is \"en-US\"."]
    pub fn language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `workforce_identity_pool_provider` after provisioning.\nThe workforce identity pool provider used to access the widget."]
    pub fn workforce_identity_pool_provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workforce_identity_pool_provider", self.base))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineWidgetConfig_HomepageSetting_Shortcuts_Icon {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DiscoveryEngineWidgetConfig_HomepageSetting_Shortcuts_Icon {
    #[doc = "Set the field `url`.\nImage URL."]
    pub fn set_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineWidgetConfig_HomepageSetting_Shortcuts_Icon {
    type O = terrars::BlockAssignable<DiscoveryEngineWidgetConfig_HomepageSetting_Shortcuts_Icon>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineWidgetConfig_HomepageSetting_Shortcuts_Icon {}

impl BuildDiscoveryEngineWidgetConfig_HomepageSetting_Shortcuts_Icon {
    pub fn build(self) -> DiscoveryEngineWidgetConfig_HomepageSetting_Shortcuts_Icon {
        DiscoveryEngineWidgetConfig_HomepageSetting_Shortcuts_Icon { url: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineWidgetConfig_HomepageSetting_Shortcuts_IconRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineWidgetConfig_HomepageSetting_Shortcuts_IconRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineWidgetConfig_HomepageSetting_Shortcuts_IconRef {
        DiscoveryEngineWidgetConfig_HomepageSetting_Shortcuts_IconRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineWidgetConfig_HomepageSetting_Shortcuts_IconRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\nImage URL."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize, Default)]
struct DiscoveryEngineWidgetConfig_HomepageSetting_ShortcutsDynamic {
    icon: std::option::Option<terrars::DynamicBlock<DiscoveryEngineWidgetConfig_HomepageSetting_Shortcuts_Icon>>,
}

#[derive(Serialize)]
pub struct DiscoveryEngineWidgetConfig_HomepageSetting_Shortcuts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    title: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    icon: std::option::Option<std::vec::Vec<DiscoveryEngineWidgetConfig_HomepageSetting_Shortcuts_Icon>>,
    dynamic: DiscoveryEngineWidgetConfig_HomepageSetting_ShortcutsDynamic,
}

impl DiscoveryEngineWidgetConfig_HomepageSetting_Shortcuts {
    #[doc = "Set the field `destination_uri`.\nDestination URL of shortcut."]
    pub fn set_destination_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.destination_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `title`.\nTitle of the shortcut."]
    pub fn set_title(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.title = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `icon`.\n"]
    pub fn set_icon(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DiscoveryEngineWidgetConfig_HomepageSetting_Shortcuts_Icon>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.icon = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.icon = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineWidgetConfig_HomepageSetting_Shortcuts {
    type O = terrars::BlockAssignable<DiscoveryEngineWidgetConfig_HomepageSetting_Shortcuts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineWidgetConfig_HomepageSetting_Shortcuts {}

impl BuildDiscoveryEngineWidgetConfig_HomepageSetting_Shortcuts {
    pub fn build(self) -> DiscoveryEngineWidgetConfig_HomepageSetting_Shortcuts {
        DiscoveryEngineWidgetConfig_HomepageSetting_Shortcuts {
            destination_uri: core::default::Default::default(),
            title: core::default::Default::default(),
            icon: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineWidgetConfig_HomepageSetting_ShortcutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineWidgetConfig_HomepageSetting_ShortcutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineWidgetConfig_HomepageSetting_ShortcutsRef {
        DiscoveryEngineWidgetConfig_HomepageSetting_ShortcutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineWidgetConfig_HomepageSetting_ShortcutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_uri` after provisioning.\nDestination URL of shortcut."]
    pub fn destination_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `title` after provisioning.\nTitle of the shortcut."]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }

    #[doc = "Get a reference to the value of field `icon` after provisioning.\n"]
    pub fn icon(&self) -> terrars::ListRef<DiscoveryEngineWidgetConfig_HomepageSetting_Shortcuts_IconRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.icon", self.base))
    }
}

#[derive(Serialize, Default)]
struct DiscoveryEngineWidgetConfig_HomepageSettingDynamic {
    shortcuts: std::option::Option<terrars::DynamicBlock<DiscoveryEngineWidgetConfig_HomepageSetting_Shortcuts>>,
}

#[derive(Serialize)]
pub struct DiscoveryEngineWidgetConfig_HomepageSetting {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shortcuts: std::option::Option<std::vec::Vec<DiscoveryEngineWidgetConfig_HomepageSetting_Shortcuts>>,
    dynamic: DiscoveryEngineWidgetConfig_HomepageSettingDynamic,
}

impl DiscoveryEngineWidgetConfig_HomepageSetting {
    #[doc = "Set the field `shortcuts`.\n"]
    pub fn set_shortcuts(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DiscoveryEngineWidgetConfig_HomepageSetting_Shortcuts>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.shortcuts = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.shortcuts = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineWidgetConfig_HomepageSetting {
    type O = terrars::BlockAssignable<DiscoveryEngineWidgetConfig_HomepageSetting>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineWidgetConfig_HomepageSetting {}

impl BuildDiscoveryEngineWidgetConfig_HomepageSetting {
    pub fn build(self) -> DiscoveryEngineWidgetConfig_HomepageSetting {
        DiscoveryEngineWidgetConfig_HomepageSetting {
            shortcuts: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineWidgetConfig_HomepageSettingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineWidgetConfig_HomepageSettingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineWidgetConfig_HomepageSettingRef {
        DiscoveryEngineWidgetConfig_HomepageSettingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineWidgetConfig_HomepageSettingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `shortcuts` after provisioning.\n"]
    pub fn shortcuts(&self) -> terrars::ListRef<DiscoveryEngineWidgetConfig_HomepageSetting_ShortcutsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shortcuts", self.base))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineWidgetConfig_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DiscoveryEngineWidgetConfig_Timeouts {
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

impl terrars::ToListMappable for DiscoveryEngineWidgetConfig_Timeouts {
    type O = terrars::BlockAssignable<DiscoveryEngineWidgetConfig_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineWidgetConfig_Timeouts {}

impl BuildDiscoveryEngineWidgetConfig_Timeouts {
    pub fn build(self) -> DiscoveryEngineWidgetConfig_Timeouts {
        DiscoveryEngineWidgetConfig_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineWidgetConfig_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineWidgetConfig_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DiscoveryEngineWidgetConfig_TimeoutsRef {
        DiscoveryEngineWidgetConfig_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineWidgetConfig_TimeoutsRef {
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
pub struct DiscoveryEngineWidgetConfig_UiBranding_Logo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DiscoveryEngineWidgetConfig_UiBranding_Logo {
    #[doc = "Set the field `url`.\nImage URL."]
    pub fn set_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineWidgetConfig_UiBranding_Logo {
    type O = terrars::BlockAssignable<DiscoveryEngineWidgetConfig_UiBranding_Logo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineWidgetConfig_UiBranding_Logo {}

impl BuildDiscoveryEngineWidgetConfig_UiBranding_Logo {
    pub fn build(self) -> DiscoveryEngineWidgetConfig_UiBranding_Logo {
        DiscoveryEngineWidgetConfig_UiBranding_Logo { url: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineWidgetConfig_UiBranding_LogoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineWidgetConfig_UiBranding_LogoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineWidgetConfig_UiBranding_LogoRef {
        DiscoveryEngineWidgetConfig_UiBranding_LogoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineWidgetConfig_UiBranding_LogoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\nImage URL."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize, Default)]
struct DiscoveryEngineWidgetConfig_UiBrandingDynamic {
    logo: std::option::Option<terrars::DynamicBlock<DiscoveryEngineWidgetConfig_UiBranding_Logo>>,
}

#[derive(Serialize)]
pub struct DiscoveryEngineWidgetConfig_UiBranding {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logo: std::option::Option<std::vec::Vec<DiscoveryEngineWidgetConfig_UiBranding_Logo>>,
    dynamic: DiscoveryEngineWidgetConfig_UiBrandingDynamic,
}

impl DiscoveryEngineWidgetConfig_UiBranding {
    #[doc = "Set the field `logo`.\n"]
    pub fn set_logo(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DiscoveryEngineWidgetConfig_UiBranding_Logo>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.logo = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.logo = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineWidgetConfig_UiBranding {
    type O = terrars::BlockAssignable<DiscoveryEngineWidgetConfig_UiBranding>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineWidgetConfig_UiBranding {}

impl BuildDiscoveryEngineWidgetConfig_UiBranding {
    pub fn build(self) -> DiscoveryEngineWidgetConfig_UiBranding {
        DiscoveryEngineWidgetConfig_UiBranding {
            logo: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineWidgetConfig_UiBrandingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineWidgetConfig_UiBrandingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DiscoveryEngineWidgetConfig_UiBrandingRef {
        DiscoveryEngineWidgetConfig_UiBrandingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineWidgetConfig_UiBrandingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `logo` after provisioning.\n"]
    pub fn logo(&self) -> terrars::ListRef<DiscoveryEngineWidgetConfig_UiBranding_LogoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logo", self.base))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs_FacetField {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    field: terrars::PrimField<std::string::String>,
}

impl DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs_FacetField {
    #[doc = "Set the field `display_name`.\nThe field name that end users will see."]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs_FacetField {
    type O = terrars::BlockAssignable<DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs_FacetField>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs_FacetField {
    #[doc = "Registered field name. The format is 'field.abc'."]
    pub field: terrars::PrimField<std::string::String>,
}

impl BuildDiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs_FacetField {
    pub fn build(self) -> DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs_FacetField {
        DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs_FacetField {
            display_name: core::default::Default::default(),
            field: self.field,
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs_FacetFieldRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs_FacetFieldRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs_FacetFieldRef {
        DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs_FacetFieldRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs_FacetFieldRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe field name that end users will see."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `field` after provisioning.\nRegistered field name. The format is 'field.abc'."]
    pub fn field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs_FieldsUiComponentsMap {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_visibility: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_template: std::option::Option<terrars::PrimField<std::string::String>>,
    field: terrars::PrimField<std::string::String>,
    ui_component: terrars::PrimField<std::string::String>,
}

impl DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs_FieldsUiComponentsMap {
    #[doc = "Set the field `device_visibility`.\n Possible values: [\"MOBILE\", \"DESKTOP\"]"]
    pub fn set_device_visibility(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.device_visibility = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `display_template`.\nThe template to customize how the field is displayed.\nAn example value would be a string that looks like: \"Price: {value}\"."]
    pub fn set_display_template(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_template = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs_FieldsUiComponentsMap {
    type O =
        terrars::BlockAssignable<DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs_FieldsUiComponentsMap>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs_FieldsUiComponentsMap {
    #[doc = "Registered field name. The format is 'field.abc'."]
    pub field: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub ui_component: terrars::PrimField<std::string::String>,
}

impl BuildDiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs_FieldsUiComponentsMap {
    pub fn build(self) -> DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs_FieldsUiComponentsMap {
        DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs_FieldsUiComponentsMap {
            device_visibility: core::default::Default::default(),
            display_template: core::default::Default::default(),
            field: self.field,
            ui_component: self.ui_component,
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs_FieldsUiComponentsMapRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs_FieldsUiComponentsMapRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs_FieldsUiComponentsMapRef {
        DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs_FieldsUiComponentsMapRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs_FieldsUiComponentsMapRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `device_visibility` after provisioning.\n Possible values: [\"MOBILE\", \"DESKTOP\"]"]
    pub fn device_visibility(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.device_visibility", self.base))
    }

    #[doc =
        "Get a reference to the value of field `display_template` after provisioning.\nThe template to customize how the field is displayed.\nAn example value would be a string that looks like: \"Price: {value}\"."]
    pub fn display_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_template", self.base))
    }

    #[doc =
        "Get a reference to the value of field `field` after provisioning.\nRegistered field name. The format is 'field.abc'."]
    pub fn field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }

    #[doc = "Get a reference to the value of field `ui_component` after provisioning.\n"]
    pub fn ui_component(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ui_component", self.base))
    }
}

#[derive(Serialize, Default)]
struct DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigsDynamic {
    facet_field: std
    ::option
    ::Option<terrars::DynamicBlock<DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs_FacetField>>,
    fields_ui_components_map: std
    ::option
    ::Option<terrars::DynamicBlock<DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs_FieldsUiComponentsMap>>,
}

#[derive(Serialize)]
pub struct DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    facet_field: std::option::Option<std::vec::Vec<DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs_FacetField>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fields_ui_components_map: std
    ::option
    ::Option<std::vec::Vec<DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs_FieldsUiComponentsMap>>,
    dynamic: DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigsDynamic,
}

impl DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs {
    #[doc =
        "Set the field `name`.\nThe name of the data store. It should be data store resource name. Format:\n'projects/{project}/locations/{location}/collections/{collectionId}/dataStores/{dataStoreId}'.\nFor APIs under 'WidgetService', such as [WidgetService.LookUpWidgetConfig][],\nthe project number and location part is erased in this field."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `facet_field`.\n"]
    pub fn set_facet_field(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs_FacetField>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.facet_field = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.facet_field = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `fields_ui_components_map`.\n"]
    pub fn set_fields_ui_components_map(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs_FieldsUiComponentsMap,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fields_ui_components_map = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fields_ui_components_map = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs {
    type O = terrars::BlockAssignable<DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs {}

impl BuildDiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs {
    pub fn build(self) -> DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs {
        DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs {
            name: core::default::Default::default(),
            facet_field: core::default::Default::default(),
            fields_ui_components_map: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigsRef {
        DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the data store. It should be data store resource name. Format:\n'projects/{project}/locations/{location}/collections/{collectionId}/dataStores/{dataStoreId}'.\nFor APIs under 'WidgetService', such as [WidgetService.LookUpWidgetConfig][],\nthe project number and location part is erased in this field."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `facet_field` after provisioning.\n"]
    pub fn facet_field(
        &self,
    ) -> terrars::ListRef<DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs_FacetFieldRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.facet_field", self.base))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineWidgetConfig_UiSettings_GenerativeAnswerConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_related_questions: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_adversarial_query: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_low_relevant_content: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_non_answer_seeking_query: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    language_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_rephrase_steps: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_prompt_preamble: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    result_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DiscoveryEngineWidgetConfig_UiSettings_GenerativeAnswerConfig {
    #[doc =
        "Set the field `disable_related_questions`.\nWhether generated answer contains suggested related questions."]
    pub fn set_disable_related_questions(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_related_questions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ignore_adversarial_query`.\nSpecifies whether to filter out queries that are adversarial."]
    pub fn set_ignore_adversarial_query(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ignore_adversarial_query = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ignore_low_relevant_content`.\nSpecifies whether to filter out queries that are not relevant to the content."]
    pub fn set_ignore_low_relevant_content(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ignore_low_relevant_content = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ignore_non_answer_seeking_query`.\nSpecifies whether to filter out queries that are not answer-seeking.\nThe default value is 'false'. No answer is returned if the search query\nis classified as a non-answer seeking query.\nIf this field is set to 'true', we skip generating answers for\nnon-answer seeking queries and return fallback messages instead."]
    pub fn set_ignore_non_answer_seeking_query(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ignore_non_answer_seeking_query = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `image_source`.\nSource of image returned in the answer. Possible values: [\"ALL_AVAILABLE_SOURCES\", \"CORPUS_IMAGE_ONLY\", \"FIGURE_GENERATION_ONLY\"]"]
    pub fn set_image_source(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image_source = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `language_code`.\nLanguage code for Summary. Use language tags defined by\n[BCP47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt). Note: This\nis an experimental feature."]
    pub fn set_language_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.language_code = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_rephrase_steps`.\nMax rephrase steps. The max number is 5 steps. If not set or\nset to < 1, it will be set to 1 by default."]
    pub fn set_max_rephrase_steps(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_rephrase_steps = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `model_prompt_preamble`.\nText at the beginning of the prompt that instructs the model that generates the answer."]
    pub fn set_model_prompt_preamble(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.model_prompt_preamble = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_version`.\nThe model version used to generate the answer."]
    pub fn set_model_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `result_count`.\nThe number of top results to generate the answer from. Up to 10."]
    pub fn set_result_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.result_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineWidgetConfig_UiSettings_GenerativeAnswerConfig {
    type O = terrars::BlockAssignable<DiscoveryEngineWidgetConfig_UiSettings_GenerativeAnswerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineWidgetConfig_UiSettings_GenerativeAnswerConfig {}

impl BuildDiscoveryEngineWidgetConfig_UiSettings_GenerativeAnswerConfig {
    pub fn build(self) -> DiscoveryEngineWidgetConfig_UiSettings_GenerativeAnswerConfig {
        DiscoveryEngineWidgetConfig_UiSettings_GenerativeAnswerConfig {
            disable_related_questions: core::default::Default::default(),
            ignore_adversarial_query: core::default::Default::default(),
            ignore_low_relevant_content: core::default::Default::default(),
            ignore_non_answer_seeking_query: core::default::Default::default(),
            image_source: core::default::Default::default(),
            language_code: core::default::Default::default(),
            max_rephrase_steps: core::default::Default::default(),
            model_prompt_preamble: core::default::Default::default(),
            model_version: core::default::Default::default(),
            result_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineWidgetConfig_UiSettings_GenerativeAnswerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineWidgetConfig_UiSettings_GenerativeAnswerConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineWidgetConfig_UiSettings_GenerativeAnswerConfigRef {
        DiscoveryEngineWidgetConfig_UiSettings_GenerativeAnswerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineWidgetConfig_UiSettings_GenerativeAnswerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disable_related_questions` after provisioning.\nWhether generated answer contains suggested related questions."]
    pub fn disable_related_questions(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_related_questions", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ignore_adversarial_query` after provisioning.\nSpecifies whether to filter out queries that are adversarial."]
    pub fn ignore_adversarial_query(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_adversarial_query", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ignore_low_relevant_content` after provisioning.\nSpecifies whether to filter out queries that are not relevant to the content."]
    pub fn ignore_low_relevant_content(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_low_relevant_content", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ignore_non_answer_seeking_query` after provisioning.\nSpecifies whether to filter out queries that are not answer-seeking.\nThe default value is 'false'. No answer is returned if the search query\nis classified as a non-answer seeking query.\nIf this field is set to 'true', we skip generating answers for\nnon-answer seeking queries and return fallback messages instead."]
    pub fn ignore_non_answer_seeking_query(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_non_answer_seeking_query", self.base))
    }

    #[doc =
        "Get a reference to the value of field `image_source` after provisioning.\nSource of image returned in the answer. Possible values: [\"ALL_AVAILABLE_SOURCES\", \"CORPUS_IMAGE_ONLY\", \"FIGURE_GENERATION_ONLY\"]"]
    pub fn image_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_source", self.base))
    }

    #[doc =
        "Get a reference to the value of field `language_code` after provisioning.\nLanguage code for Summary. Use language tags defined by\n[BCP47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt). Note: This\nis an experimental feature."]
    pub fn language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_rephrase_steps` after provisioning.\nMax rephrase steps. The max number is 5 steps. If not set or\nset to < 1, it will be set to 1 by default."]
    pub fn max_rephrase_steps(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_rephrase_steps", self.base))
    }

    #[doc =
        "Get a reference to the value of field `model_prompt_preamble` after provisioning.\nText at the beginning of the prompt that instructs the model that generates the answer."]
    pub fn model_prompt_preamble(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_prompt_preamble", self.base))
    }

    #[doc =
        "Get a reference to the value of field `model_version` after provisioning.\nThe model version used to generate the answer."]
    pub fn model_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `result_count` after provisioning.\nThe number of top results to generate the answer from. Up to 10."]
    pub fn result_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.result_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineWidgetConfig_UiSettings_SearchAddonSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generative_answer_add_on_disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kpi_personalization_add_on_disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    semantic_add_on_disabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DiscoveryEngineWidgetConfig_UiSettings_SearchAddonSpec {
    #[doc =
        "Set the field `generative_answer_add_on_disabled`.\nIf true, generative answer add-on is disabled. Generative answer\nadd-on includes natural language to filters and simple answers."]
    pub fn set_generative_answer_add_on_disabled(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.generative_answer_add_on_disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kpi_personalization_add_on_disabled`.\nIf true, disables event re-ranking and personalization to optimize KPIs\n& personalize results."]
    pub fn set_kpi_personalization_add_on_disabled(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.kpi_personalization_add_on_disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `semantic_add_on_disabled`.\nIf true, semantic add-on is disabled. Semantic add-on includes\nembeddings and jetstream."]
    pub fn set_semantic_add_on_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.semantic_add_on_disabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineWidgetConfig_UiSettings_SearchAddonSpec {
    type O = terrars::BlockAssignable<DiscoveryEngineWidgetConfig_UiSettings_SearchAddonSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineWidgetConfig_UiSettings_SearchAddonSpec {}

impl BuildDiscoveryEngineWidgetConfig_UiSettings_SearchAddonSpec {
    pub fn build(self) -> DiscoveryEngineWidgetConfig_UiSettings_SearchAddonSpec {
        DiscoveryEngineWidgetConfig_UiSettings_SearchAddonSpec {
            generative_answer_add_on_disabled: core::default::Default::default(),
            kpi_personalization_add_on_disabled: core::default::Default::default(),
            semantic_add_on_disabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineWidgetConfig_UiSettings_SearchAddonSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineWidgetConfig_UiSettings_SearchAddonSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineWidgetConfig_UiSettings_SearchAddonSpecRef {
        DiscoveryEngineWidgetConfig_UiSettings_SearchAddonSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineWidgetConfig_UiSettings_SearchAddonSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `generative_answer_add_on_disabled` after provisioning.\nIf true, generative answer add-on is disabled. Generative answer\nadd-on includes natural language to filters and simple answers."]
    pub fn generative_answer_add_on_disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generative_answer_add_on_disabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kpi_personalization_add_on_disabled` after provisioning.\nIf true, disables event re-ranking and personalization to optimize KPIs\n& personalize results."]
    pub fn kpi_personalization_add_on_disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kpi_personalization_add_on_disabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `semantic_add_on_disabled` after provisioning.\nIf true, semantic add-on is disabled. Semantic add-on includes\nembeddings and jetstream."]
    pub fn semantic_add_on_disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.semantic_add_on_disabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct DiscoveryEngineWidgetConfig_UiSettingsDynamic {
    data_store_ui_configs: std
    ::option
    ::Option<terrars::DynamicBlock<DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs>>,
    generative_answer_config: std
    ::option
    ::Option<terrars::DynamicBlock<DiscoveryEngineWidgetConfig_UiSettings_GenerativeAnswerConfig>>,
    search_addon_spec: std
    ::option
    ::Option<terrars::DynamicBlock<DiscoveryEngineWidgetConfig_UiSettings_SearchAddonSpec>>,
}

#[derive(Serialize)]
pub struct DiscoveryEngineWidgetConfig_UiSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_search_request_order_by: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_user_events_collection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_autocomplete: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_create_agent_button: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_people_search: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_quality_feedback: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_safe_search: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_search_as_you_type: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_visual_content_summary: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interaction_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    result_description_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_admin_display_name_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_store_ui_configs: std::option::Option<std::vec::Vec<DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generative_answer_config: std
    ::option
    ::Option<std::vec::Vec<DiscoveryEngineWidgetConfig_UiSettings_GenerativeAnswerConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    search_addon_spec: std::option::Option<std::vec::Vec<DiscoveryEngineWidgetConfig_UiSettings_SearchAddonSpec>>,
    dynamic: DiscoveryEngineWidgetConfig_UiSettingsDynamic,
}

impl DiscoveryEngineWidgetConfig_UiSettings {
    #[doc =
        "Set the field `default_search_request_order_by`.\nThe default ordering for search results if specified.\nUsed to set SearchRequest#orderBy on applicable requests."]
    pub fn set_default_search_request_order_by(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.default_search_request_order_by = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_user_events_collection`.\nIf set to true, the widget will not collect user events."]
    pub fn set_disable_user_events_collection(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_user_events_collection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_autocomplete`.\nWhether or not to enable autocomplete."]
    pub fn set_enable_autocomplete(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_autocomplete = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_create_agent_button`.\nIf set to true, the widget will enable the create agent button."]
    pub fn set_enable_create_agent_button(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_create_agent_button = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_people_search`.\nIf set to true, the widget will enable people search."]
    pub fn set_enable_people_search(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_people_search = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_quality_feedback`.\nTurn on or off collecting the search result quality feedback from end users."]
    pub fn set_enable_quality_feedback(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_quality_feedback = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_safe_search`.\nWhether to enable safe search."]
    pub fn set_enable_safe_search(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_safe_search = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_search_as_you_type`.\nWhether to enable search-as-you-type behavior for the search widget."]
    pub fn set_enable_search_as_you_type(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_search_as_you_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_visual_content_summary`.\nIf set to true, the widget will enable visual content summary on applicable\nsearch requests. Only used by healthcare search."]
    pub fn set_enable_visual_content_summary(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_visual_content_summary = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `interaction_type`.\nDescribes widget (or web app) interaction type Possible values: [\"SEARCH_ONLY\", \"SEARCH_WITH_ANSWER\", \"SEARCH_WITH_FOLLOW_UPS\"]"]
    pub fn set_interaction_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.interaction_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `result_description_type`.\nControls whether result extract is display and how (snippet or extractive answer).\nDefault to no result if unspecified. Possible values: [\"SNIPPET\", \"EXTRACTIVE_ANSWER\"]"]
    pub fn set_result_description_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.result_description_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `source_admin_display_name_enabled`.\nWhether to show the admin-configured display name for data connectors in\nthe widget sources UI (instead of the connector kind). Opt-in; defaults\nto false."]
    pub fn set_source_admin_display_name_enabled(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.source_admin_display_name_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_store_ui_configs`.\n"]
    pub fn set_data_store_ui_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.data_store_ui_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.data_store_ui_configs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `generative_answer_config`.\n"]
    pub fn set_generative_answer_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DiscoveryEngineWidgetConfig_UiSettings_GenerativeAnswerConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.generative_answer_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.generative_answer_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `search_addon_spec`.\n"]
    pub fn set_search_addon_spec(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DiscoveryEngineWidgetConfig_UiSettings_SearchAddonSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.search_addon_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.search_addon_spec = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineWidgetConfig_UiSettings {
    type O = terrars::BlockAssignable<DiscoveryEngineWidgetConfig_UiSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineWidgetConfig_UiSettings {}

impl BuildDiscoveryEngineWidgetConfig_UiSettings {
    pub fn build(self) -> DiscoveryEngineWidgetConfig_UiSettings {
        DiscoveryEngineWidgetConfig_UiSettings {
            default_search_request_order_by: core::default::Default::default(),
            disable_user_events_collection: core::default::Default::default(),
            enable_autocomplete: core::default::Default::default(),
            enable_create_agent_button: core::default::Default::default(),
            enable_people_search: core::default::Default::default(),
            enable_quality_feedback: core::default::Default::default(),
            enable_safe_search: core::default::Default::default(),
            enable_search_as_you_type: core::default::Default::default(),
            enable_visual_content_summary: core::default::Default::default(),
            interaction_type: core::default::Default::default(),
            result_description_type: core::default::Default::default(),
            source_admin_display_name_enabled: core::default::Default::default(),
            data_store_ui_configs: core::default::Default::default(),
            generative_answer_config: core::default::Default::default(),
            search_addon_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineWidgetConfig_UiSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineWidgetConfig_UiSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DiscoveryEngineWidgetConfig_UiSettingsRef {
        DiscoveryEngineWidgetConfig_UiSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineWidgetConfig_UiSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `default_search_request_order_by` after provisioning.\nThe default ordering for search results if specified.\nUsed to set SearchRequest#orderBy on applicable requests."]
    pub fn default_search_request_order_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_search_request_order_by", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disable_user_events_collection` after provisioning.\nIf set to true, the widget will not collect user events."]
    pub fn disable_user_events_collection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_user_events_collection", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_autocomplete` after provisioning.\nWhether or not to enable autocomplete."]
    pub fn enable_autocomplete(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_autocomplete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_create_agent_button` after provisioning.\nIf set to true, the widget will enable the create agent button."]
    pub fn enable_create_agent_button(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_create_agent_button", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_people_search` after provisioning.\nIf set to true, the widget will enable people search."]
    pub fn enable_people_search(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_people_search", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_quality_feedback` after provisioning.\nTurn on or off collecting the search result quality feedback from end users."]
    pub fn enable_quality_feedback(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_quality_feedback", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_safe_search` after provisioning.\nWhether to enable safe search."]
    pub fn enable_safe_search(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_safe_search", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_search_as_you_type` after provisioning.\nWhether to enable search-as-you-type behavior for the search widget."]
    pub fn enable_search_as_you_type(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_search_as_you_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_visual_content_summary` after provisioning.\nIf set to true, the widget will enable visual content summary on applicable\nsearch requests. Only used by healthcare search."]
    pub fn enable_visual_content_summary(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_visual_content_summary", self.base))
    }

    #[doc =
        "Get a reference to the value of field `interaction_type` after provisioning.\nDescribes widget (or web app) interaction type Possible values: [\"SEARCH_ONLY\", \"SEARCH_WITH_ANSWER\", \"SEARCH_WITH_FOLLOW_UPS\"]"]
    pub fn interaction_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interaction_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `result_description_type` after provisioning.\nControls whether result extract is display and how (snippet or extractive answer).\nDefault to no result if unspecified. Possible values: [\"SNIPPET\", \"EXTRACTIVE_ANSWER\"]"]
    pub fn result_description_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.result_description_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_admin_display_name_enabled` after provisioning.\nWhether to show the admin-configured display name for data connectors in\nthe widget sources UI (instead of the connector kind). Opt-in; defaults\nto false."]
    pub fn source_admin_display_name_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_admin_display_name_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `data_store_ui_configs` after provisioning.\n"]
    pub fn data_store_ui_configs(
        &self,
    ) -> terrars::ListRef<DiscoveryEngineWidgetConfig_UiSettings_DataStoreUiConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_store_ui_configs", self.base))
    }

    #[doc = "Get a reference to the value of field `generative_answer_config` after provisioning.\n"]
    pub fn generative_answer_config(
        &self,
    ) -> terrars::ListRef<DiscoveryEngineWidgetConfig_UiSettings_GenerativeAnswerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.generative_answer_config", self.base))
    }

    #[doc = "Get a reference to the value of field `search_addon_spec` after provisioning.\n"]
    pub fn search_addon_spec(&self) -> terrars::ListRef<DiscoveryEngineWidgetConfig_UiSettings_SearchAddonSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.search_addon_spec", self.base))
    }
}
