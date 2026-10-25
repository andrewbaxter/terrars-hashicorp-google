#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct AppEngineServiceNetworkSettingsData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    service: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_settings: std::option::Option<std::vec::Vec<AppEngineServiceNetworkSettings_NetworkSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<AppEngineServiceNetworkSettings_Timeouts>,
    dynamic: AppEngineServiceNetworkSettingsDynamic,
}

struct AppEngineServiceNetworkSettings_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AppEngineServiceNetworkSettingsData_>,
}

#[derive(Clone)]
pub struct AppEngineServiceNetworkSettings(std::rc::Rc<AppEngineServiceNetworkSettings_>);

impl AppEngineServiceNetworkSettings {
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

    #[doc = "Set the field `network_settings`.\n"]
    pub fn set_network_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppEngineServiceNetworkSettings_NetworkSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<AppEngineServiceNetworkSettings_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nThe name of the service these settings apply to."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_settings` after provisioning.\n"]
    pub fn network_settings(&self) -> terrars::ListRef<AppEngineServiceNetworkSettings_NetworkSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AppEngineServiceNetworkSettings_TimeoutsRef {
        AppEngineServiceNetworkSettings_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for AppEngineServiceNetworkSettings {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AppEngineServiceNetworkSettings { }

impl terrars::ToListMappable for AppEngineServiceNetworkSettings {
    type O = terrars::ListRef<AppEngineServiceNetworkSettingsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AppEngineServiceNetworkSettings_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_app_engine_service_network_settings".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppEngineServiceNetworkSettings {
    pub tf_id: std::string::String,
    #[doc = "The name of the service these settings apply to."]
    pub service: terrars::PrimField<std::string::String>,
}

impl BuildAppEngineServiceNetworkSettings {
    pub fn build(self, stack: &mut terrars::Stack) -> AppEngineServiceNetworkSettings {
        let out = AppEngineServiceNetworkSettings(std::rc::Rc::new(AppEngineServiceNetworkSettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AppEngineServiceNetworkSettingsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                service: self.service,
                network_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppEngineServiceNetworkSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineServiceNetworkSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AppEngineServiceNetworkSettingsRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nThe name of the service these settings apply to."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_settings` after provisioning.\n"]
    pub fn network_settings(&self) -> terrars::ListRef<AppEngineServiceNetworkSettings_NetworkSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AppEngineServiceNetworkSettings_TimeoutsRef {
        AppEngineServiceNetworkSettings_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct AppEngineServiceNetworkSettingsDynamic {
    network_settings: std::option::Option<terrars::DynamicBlock<AppEngineServiceNetworkSettings_NetworkSettings>>,
}

#[derive(Serialize)]
pub struct AppEngineServiceNetworkSettings_NetworkSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress_traffic_allowed: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppEngineServiceNetworkSettings_NetworkSettings {
    #[doc =
        "Set the field `ingress_traffic_allowed`.\nThe ingress settings for version or service. Default value: \"INGRESS_TRAFFIC_ALLOWED_UNSPECIFIED\" Possible values: [\"INGRESS_TRAFFIC_ALLOWED_UNSPECIFIED\", \"INGRESS_TRAFFIC_ALLOWED_ALL\", \"INGRESS_TRAFFIC_ALLOWED_INTERNAL_ONLY\", \"INGRESS_TRAFFIC_ALLOWED_INTERNAL_AND_LB\"]"]
    pub fn set_ingress_traffic_allowed(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ingress_traffic_allowed = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppEngineServiceNetworkSettings_NetworkSettings {
    type O = terrars::BlockAssignable<AppEngineServiceNetworkSettings_NetworkSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineServiceNetworkSettings_NetworkSettings {}

impl BuildAppEngineServiceNetworkSettings_NetworkSettings {
    pub fn build(self) -> AppEngineServiceNetworkSettings_NetworkSettings {
        AppEngineServiceNetworkSettings_NetworkSettings {
            ingress_traffic_allowed: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppEngineServiceNetworkSettings_NetworkSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineServiceNetworkSettings_NetworkSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppEngineServiceNetworkSettings_NetworkSettingsRef {
        AppEngineServiceNetworkSettings_NetworkSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineServiceNetworkSettings_NetworkSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ingress_traffic_allowed` after provisioning.\nThe ingress settings for version or service. Default value: \"INGRESS_TRAFFIC_ALLOWED_UNSPECIFIED\" Possible values: [\"INGRESS_TRAFFIC_ALLOWED_UNSPECIFIED\", \"INGRESS_TRAFFIC_ALLOWED_ALL\", \"INGRESS_TRAFFIC_ALLOWED_INTERNAL_ONLY\", \"INGRESS_TRAFFIC_ALLOWED_INTERNAL_AND_LB\"]"]
    pub fn ingress_traffic_allowed(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ingress_traffic_allowed", self.base))
    }
}

#[derive(Serialize)]
pub struct AppEngineServiceNetworkSettings_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppEngineServiceNetworkSettings_Timeouts {
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

impl terrars::ToListMappable for AppEngineServiceNetworkSettings_Timeouts {
    type O = terrars::BlockAssignable<AppEngineServiceNetworkSettings_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineServiceNetworkSettings_Timeouts {}

impl BuildAppEngineServiceNetworkSettings_Timeouts {
    pub fn build(self) -> AppEngineServiceNetworkSettings_Timeouts {
        AppEngineServiceNetworkSettings_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppEngineServiceNetworkSettings_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineServiceNetworkSettings_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppEngineServiceNetworkSettings_TimeoutsRef {
        AppEngineServiceNetworkSettings_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineServiceNetworkSettings_TimeoutsRef {
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
