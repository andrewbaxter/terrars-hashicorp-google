#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct IdentityPlatformConfigData_ {
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
    authorized_domains: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autodelete_anonymous_users: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    blocking_functions: std::option::Option<std::vec::Vec<IdentityPlatformConfig_BlockingFunctions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client: std::option::Option<std::vec::Vec<IdentityPlatformConfig_Client>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mfa: std::option::Option<std::vec::Vec<IdentityPlatformConfig_Mfa>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitoring: std::option::Option<std::vec::Vec<IdentityPlatformConfig_Monitoring>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multi_tenant: std::option::Option<std::vec::Vec<IdentityPlatformConfig_MultiTenant>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quota: std::option::Option<std::vec::Vec<IdentityPlatformConfig_Quota>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sign_in: std::option::Option<std::vec::Vec<IdentityPlatformConfig_SignIn>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sms_region_config: std::option::Option<std::vec::Vec<IdentityPlatformConfig_SmsRegionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<IdentityPlatformConfig_Timeouts>,
    dynamic: IdentityPlatformConfigDynamic,
}

struct IdentityPlatformConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<IdentityPlatformConfigData_>,
}

#[derive(Clone)]
pub struct IdentityPlatformConfig(std::rc::Rc<IdentityPlatformConfig_>);

impl IdentityPlatformConfig {
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

    #[doc = "Set the field `authorized_domains`.\nList of domains authorized for OAuth redirects."]
    pub fn set_authorized_domains(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().authorized_domains = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `autodelete_anonymous_users`.\nWhether anonymous users will be auto-deleted after a period of 30 days"]
    pub fn set_autodelete_anonymous_users(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().autodelete_anonymous_users = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `blocking_functions`.\n"]
    pub fn set_blocking_functions(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IdentityPlatformConfig_BlockingFunctions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().blocking_functions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.blocking_functions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `client`.\n"]
    pub fn set_client(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IdentityPlatformConfig_Client>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().client = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.client = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mfa`.\n"]
    pub fn set_mfa(self, v: impl std::convert::Into<terrars::BlockAssignable<IdentityPlatformConfig_Mfa>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().mfa = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.mfa = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `monitoring`.\n"]
    pub fn set_monitoring(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IdentityPlatformConfig_Monitoring>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().monitoring = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.monitoring = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `multi_tenant`.\n"]
    pub fn set_multi_tenant(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IdentityPlatformConfig_MultiTenant>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().multi_tenant = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.multi_tenant = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `quota`.\n"]
    pub fn set_quota(self, v: impl std::convert::Into<terrars::BlockAssignable<IdentityPlatformConfig_Quota>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().quota = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.quota = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sign_in`.\n"]
    pub fn set_sign_in(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IdentityPlatformConfig_SignIn>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sign_in = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sign_in = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sms_region_config`.\n"]
    pub fn set_sms_region_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IdentityPlatformConfig_SmsRegionConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sms_region_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sms_region_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<IdentityPlatformConfig_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `authorized_domains` after provisioning.\nList of domains authorized for OAuth redirects."]
    pub fn authorized_domains(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorized_domains", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `autodelete_anonymous_users` after provisioning.\nWhether anonymous users will be auto-deleted after a period of 30 days"]
    pub fn autodelete_anonymous_users(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autodelete_anonymous_users", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the Config resource"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `blocking_functions` after provisioning.\n"]
    pub fn blocking_functions(&self) -> terrars::ListRef<IdentityPlatformConfig_BlockingFunctionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.blocking_functions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client` after provisioning.\n"]
    pub fn client(&self) -> terrars::ListRef<IdentityPlatformConfig_ClientRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mfa` after provisioning.\n"]
    pub fn mfa(&self) -> terrars::ListRef<IdentityPlatformConfig_MfaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mfa", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring` after provisioning.\n"]
    pub fn monitoring(&self) -> terrars::ListRef<IdentityPlatformConfig_MonitoringRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multi_tenant` after provisioning.\n"]
    pub fn multi_tenant(&self) -> terrars::ListRef<IdentityPlatformConfig_MultiTenantRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.multi_tenant", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quota` after provisioning.\n"]
    pub fn quota(&self) -> terrars::ListRef<IdentityPlatformConfig_QuotaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.quota", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sign_in` after provisioning.\n"]
    pub fn sign_in(&self) -> terrars::ListRef<IdentityPlatformConfig_SignInRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sign_in", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sms_region_config` after provisioning.\n"]
    pub fn sms_region_config(&self) -> terrars::ListRef<IdentityPlatformConfig_SmsRegionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sms_region_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IdentityPlatformConfig_TimeoutsRef {
        IdentityPlatformConfig_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for IdentityPlatformConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for IdentityPlatformConfig { }

impl terrars::ToListMappable for IdentityPlatformConfig {
    type O = terrars::ListRef<IdentityPlatformConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for IdentityPlatformConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_identity_platform_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIdentityPlatformConfig {
    pub tf_id: std::string::String,
}

impl BuildIdentityPlatformConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> IdentityPlatformConfig {
        let out = IdentityPlatformConfig(std::rc::Rc::new(IdentityPlatformConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(IdentityPlatformConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                authorized_domains: core::default::Default::default(),
                autodelete_anonymous_users: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                blocking_functions: core::default::Default::default(),
                client: core::default::Default::default(),
                mfa: core::default::Default::default(),
                monitoring: core::default::Default::default(),
                multi_tenant: core::default::Default::default(),
                quota: core::default::Default::default(),
                sign_in: core::default::Default::default(),
                sms_region_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IdentityPlatformConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl IdentityPlatformConfigRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `authorized_domains` after provisioning.\nList of domains authorized for OAuth redirects."]
    pub fn authorized_domains(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorized_domains", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `autodelete_anonymous_users` after provisioning.\nWhether anonymous users will be auto-deleted after a period of 30 days"]
    pub fn autodelete_anonymous_users(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autodelete_anonymous_users", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the Config resource"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `blocking_functions` after provisioning.\n"]
    pub fn blocking_functions(&self) -> terrars::ListRef<IdentityPlatformConfig_BlockingFunctionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.blocking_functions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client` after provisioning.\n"]
    pub fn client(&self) -> terrars::ListRef<IdentityPlatformConfig_ClientRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mfa` after provisioning.\n"]
    pub fn mfa(&self) -> terrars::ListRef<IdentityPlatformConfig_MfaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mfa", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring` after provisioning.\n"]
    pub fn monitoring(&self) -> terrars::ListRef<IdentityPlatformConfig_MonitoringRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multi_tenant` after provisioning.\n"]
    pub fn multi_tenant(&self) -> terrars::ListRef<IdentityPlatformConfig_MultiTenantRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.multi_tenant", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quota` after provisioning.\n"]
    pub fn quota(&self) -> terrars::ListRef<IdentityPlatformConfig_QuotaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.quota", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sign_in` after provisioning.\n"]
    pub fn sign_in(&self) -> terrars::ListRef<IdentityPlatformConfig_SignInRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sign_in", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sms_region_config` after provisioning.\n"]
    pub fn sms_region_config(&self) -> terrars::ListRef<IdentityPlatformConfig_SmsRegionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sms_region_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IdentityPlatformConfig_TimeoutsRef {
        IdentityPlatformConfig_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct IdentityPlatformConfigDynamic {
    blocking_functions: std::option::Option<terrars::DynamicBlock<IdentityPlatformConfig_BlockingFunctions>>,
    client: std::option::Option<terrars::DynamicBlock<IdentityPlatformConfig_Client>>,
    mfa: std::option::Option<terrars::DynamicBlock<IdentityPlatformConfig_Mfa>>,
    monitoring: std::option::Option<terrars::DynamicBlock<IdentityPlatformConfig_Monitoring>>,
    multi_tenant: std::option::Option<terrars::DynamicBlock<IdentityPlatformConfig_MultiTenant>>,
    quota: std::option::Option<terrars::DynamicBlock<IdentityPlatformConfig_Quota>>,
    sign_in: std::option::Option<terrars::DynamicBlock<IdentityPlatformConfig_SignIn>>,
    sms_region_config: std::option::Option<terrars::DynamicBlock<IdentityPlatformConfig_SmsRegionConfig>>,
}

#[derive(Serialize)]
pub struct IdentityPlatformConfig_BlockingFunctions_ForwardInboundCredentials {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_token: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id_token: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    refresh_token: std::option::Option<terrars::PrimField<bool>>,
}

impl IdentityPlatformConfig_BlockingFunctions_ForwardInboundCredentials {
    #[doc = "Set the field `access_token`.\nWhether to pass the user's OAuth identity provider's access token."]
    pub fn set_access_token(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.access_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id_token`.\nWhether to pass the user's OIDC identity provider's ID token."]
    pub fn set_id_token(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.id_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `refresh_token`.\nWhether to pass the user's OAuth identity provider's refresh token."]
    pub fn set_refresh_token(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.refresh_token = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IdentityPlatformConfig_BlockingFunctions_ForwardInboundCredentials {
    type O = terrars::BlockAssignable<IdentityPlatformConfig_BlockingFunctions_ForwardInboundCredentials>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformConfig_BlockingFunctions_ForwardInboundCredentials {}

impl BuildIdentityPlatformConfig_BlockingFunctions_ForwardInboundCredentials {
    pub fn build(self) -> IdentityPlatformConfig_BlockingFunctions_ForwardInboundCredentials {
        IdentityPlatformConfig_BlockingFunctions_ForwardInboundCredentials {
            access_token: core::default::Default::default(),
            id_token: core::default::Default::default(),
            refresh_token: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IdentityPlatformConfig_BlockingFunctions_ForwardInboundCredentialsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformConfig_BlockingFunctions_ForwardInboundCredentialsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IdentityPlatformConfig_BlockingFunctions_ForwardInboundCredentialsRef {
        IdentityPlatformConfig_BlockingFunctions_ForwardInboundCredentialsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformConfig_BlockingFunctions_ForwardInboundCredentialsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_token` after provisioning.\nWhether to pass the user's OAuth identity provider's access token."]
    pub fn access_token(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_token", self.base))
    }

    #[doc =
        "Get a reference to the value of field `id_token` after provisioning.\nWhether to pass the user's OIDC identity provider's ID token."]
    pub fn id_token(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id_token", self.base))
    }

    #[doc =
        "Get a reference to the value of field `refresh_token` after provisioning.\nWhether to pass the user's OAuth identity provider's refresh token."]
    pub fn refresh_token(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refresh_token", self.base))
    }
}

#[derive(Serialize)]
pub struct IdentityPlatformConfig_BlockingFunctions_Triggers {
    event_type: terrars::PrimField<std::string::String>,
    function_uri: terrars::PrimField<std::string::String>,
}

impl IdentityPlatformConfig_BlockingFunctions_Triggers { }

impl terrars::ToListMappable for IdentityPlatformConfig_BlockingFunctions_Triggers {
    type O = terrars::BlockAssignable<IdentityPlatformConfig_BlockingFunctions_Triggers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformConfig_BlockingFunctions_Triggers {
    #[doc = ""]
    pub event_type: terrars::PrimField<std::string::String>,
    #[doc = "HTTP URI trigger for the Cloud Function."]
    pub function_uri: terrars::PrimField<std::string::String>,
}

impl BuildIdentityPlatformConfig_BlockingFunctions_Triggers {
    pub fn build(self) -> IdentityPlatformConfig_BlockingFunctions_Triggers {
        IdentityPlatformConfig_BlockingFunctions_Triggers {
            event_type: self.event_type,
            function_uri: self.function_uri,
        }
    }
}

#[allow(dead_code)]
pub struct IdentityPlatformConfig_BlockingFunctions_TriggersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformConfig_BlockingFunctions_TriggersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IdentityPlatformConfig_BlockingFunctions_TriggersRef {
        IdentityPlatformConfig_BlockingFunctions_TriggersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformConfig_BlockingFunctions_TriggersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `event_type` after provisioning.\n"]
    pub fn event_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `function_uri` after provisioning.\nHTTP URI trigger for the Cloud Function."]
    pub fn function_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nWhen the trigger was changed."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}

#[derive(Serialize, Default)]
struct IdentityPlatformConfig_BlockingFunctionsDynamic {
    forward_inbound_credentials: std
    ::option
    ::Option<terrars::DynamicBlock<IdentityPlatformConfig_BlockingFunctions_ForwardInboundCredentials>>,
    triggers: std::option::Option<terrars::DynamicBlock<IdentityPlatformConfig_BlockingFunctions_Triggers>>,
}

#[derive(Serialize)]
pub struct IdentityPlatformConfig_BlockingFunctions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forward_inbound_credentials: std
    ::option
    ::Option<std::vec::Vec<IdentityPlatformConfig_BlockingFunctions_ForwardInboundCredentials>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    triggers: std::option::Option<std::vec::Vec<IdentityPlatformConfig_BlockingFunctions_Triggers>>,
    dynamic: IdentityPlatformConfig_BlockingFunctionsDynamic,
}

impl IdentityPlatformConfig_BlockingFunctions {
    #[doc = "Set the field `forward_inbound_credentials`.\n"]
    pub fn set_forward_inbound_credentials(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<IdentityPlatformConfig_BlockingFunctions_ForwardInboundCredentials>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.forward_inbound_credentials = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.forward_inbound_credentials = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `triggers`.\n"]
    pub fn set_triggers(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IdentityPlatformConfig_BlockingFunctions_Triggers>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.triggers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.triggers = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IdentityPlatformConfig_BlockingFunctions {
    type O = terrars::BlockAssignable<IdentityPlatformConfig_BlockingFunctions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformConfig_BlockingFunctions {}

impl BuildIdentityPlatformConfig_BlockingFunctions {
    pub fn build(self) -> IdentityPlatformConfig_BlockingFunctions {
        IdentityPlatformConfig_BlockingFunctions {
            forward_inbound_credentials: core::default::Default::default(),
            triggers: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IdentityPlatformConfig_BlockingFunctionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformConfig_BlockingFunctionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IdentityPlatformConfig_BlockingFunctionsRef {
        IdentityPlatformConfig_BlockingFunctionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformConfig_BlockingFunctionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `forward_inbound_credentials` after provisioning.\n"]
    pub fn forward_inbound_credentials(
        &self,
    ) -> terrars::ListRef<IdentityPlatformConfig_BlockingFunctions_ForwardInboundCredentialsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forward_inbound_credentials", self.base))
    }
}

#[derive(Serialize)]
pub struct IdentityPlatformConfig_Client_Permissions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled_user_deletion: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled_user_signup: std::option::Option<terrars::PrimField<bool>>,
}

impl IdentityPlatformConfig_Client_Permissions {
    #[doc =
        "Set the field `disabled_user_deletion`.\nWhen true, end users cannot delete their account on the associated project through any of our API methods"]
    pub fn set_disabled_user_deletion(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled_user_deletion = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disabled_user_signup`.\nWhen true, end users cannot sign up for a new account on the associated project through any of our API methods"]
    pub fn set_disabled_user_signup(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled_user_signup = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IdentityPlatformConfig_Client_Permissions {
    type O = terrars::BlockAssignable<IdentityPlatformConfig_Client_Permissions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformConfig_Client_Permissions {}

impl BuildIdentityPlatformConfig_Client_Permissions {
    pub fn build(self) -> IdentityPlatformConfig_Client_Permissions {
        IdentityPlatformConfig_Client_Permissions {
            disabled_user_deletion: core::default::Default::default(),
            disabled_user_signup: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IdentityPlatformConfig_Client_PermissionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformConfig_Client_PermissionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IdentityPlatformConfig_Client_PermissionsRef {
        IdentityPlatformConfig_Client_PermissionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformConfig_Client_PermissionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disabled_user_deletion` after provisioning.\nWhen true, end users cannot delete their account on the associated project through any of our API methods"]
    pub fn disabled_user_deletion(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled_user_deletion", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disabled_user_signup` after provisioning.\nWhen true, end users cannot sign up for a new account on the associated project through any of our API methods"]
    pub fn disabled_user_signup(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled_user_signup", self.base))
    }
}

#[derive(Serialize, Default)]
struct IdentityPlatformConfig_ClientDynamic {
    permissions: std::option::Option<terrars::DynamicBlock<IdentityPlatformConfig_Client_Permissions>>,
}

#[derive(Serialize)]
pub struct IdentityPlatformConfig_Client {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permissions: std::option::Option<std::vec::Vec<IdentityPlatformConfig_Client_Permissions>>,
    dynamic: IdentityPlatformConfig_ClientDynamic,
}

impl IdentityPlatformConfig_Client {
    #[doc = "Set the field `permissions`.\n"]
    pub fn set_permissions(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IdentityPlatformConfig_Client_Permissions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.permissions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.permissions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IdentityPlatformConfig_Client {
    type O = terrars::BlockAssignable<IdentityPlatformConfig_Client>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformConfig_Client {}

impl BuildIdentityPlatformConfig_Client {
    pub fn build(self) -> IdentityPlatformConfig_Client {
        IdentityPlatformConfig_Client {
            permissions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IdentityPlatformConfig_ClientRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformConfig_ClientRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IdentityPlatformConfig_ClientRef {
        IdentityPlatformConfig_ClientRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformConfig_ClientRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `api_key` after provisioning.\nAPI key that can be used when making requests for this project."]
    pub fn api_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key", self.base))
    }

    #[doc = "Get a reference to the value of field `firebase_subdomain` after provisioning.\nFirebase subdomain."]
    pub fn firebase_subdomain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.firebase_subdomain", self.base))
    }

    #[doc = "Get a reference to the value of field `permissions` after provisioning.\n"]
    pub fn permissions(&self) -> terrars::ListRef<IdentityPlatformConfig_Client_PermissionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permissions", self.base))
    }
}

#[derive(Serialize)]
pub struct IdentityPlatformConfig_Mfa_ProviderConfigs_TotpProviderConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    adjacent_intervals: std::option::Option<terrars::PrimField<f64>>,
}

impl IdentityPlatformConfig_Mfa_ProviderConfigs_TotpProviderConfig {
    #[doc =
        "Set the field `adjacent_intervals`.\nThe allowed number of adjacent intervals that will be used for verification to avoid clock skew."]
    pub fn set_adjacent_intervals(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.adjacent_intervals = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IdentityPlatformConfig_Mfa_ProviderConfigs_TotpProviderConfig {
    type O = terrars::BlockAssignable<IdentityPlatformConfig_Mfa_ProviderConfigs_TotpProviderConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformConfig_Mfa_ProviderConfigs_TotpProviderConfig {}

impl BuildIdentityPlatformConfig_Mfa_ProviderConfigs_TotpProviderConfig {
    pub fn build(self) -> IdentityPlatformConfig_Mfa_ProviderConfigs_TotpProviderConfig {
        IdentityPlatformConfig_Mfa_ProviderConfigs_TotpProviderConfig {
            adjacent_intervals: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IdentityPlatformConfig_Mfa_ProviderConfigs_TotpProviderConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformConfig_Mfa_ProviderConfigs_TotpProviderConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IdentityPlatformConfig_Mfa_ProviderConfigs_TotpProviderConfigRef {
        IdentityPlatformConfig_Mfa_ProviderConfigs_TotpProviderConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformConfig_Mfa_ProviderConfigs_TotpProviderConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `adjacent_intervals` after provisioning.\nThe allowed number of adjacent intervals that will be used for verification to avoid clock skew."]
    pub fn adjacent_intervals(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.adjacent_intervals", self.base))
    }
}

#[derive(Serialize, Default)]
struct IdentityPlatformConfig_Mfa_ProviderConfigsDynamic {
    totp_provider_config: std
    ::option
    ::Option<terrars::DynamicBlock<IdentityPlatformConfig_Mfa_ProviderConfigs_TotpProviderConfig>>,
}

#[derive(Serialize)]
pub struct IdentityPlatformConfig_Mfa_ProviderConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    totp_provider_config: std
    ::option
    ::Option<std::vec::Vec<IdentityPlatformConfig_Mfa_ProviderConfigs_TotpProviderConfig>>,
    dynamic: IdentityPlatformConfig_Mfa_ProviderConfigsDynamic,
}

impl IdentityPlatformConfig_Mfa_ProviderConfigs {
    #[doc =
        "Set the field `state`.\nWhether MultiFactor Authentication has been enabled for this project. Possible values: [\"DISABLED\", \"ENABLED\", \"MANDATORY\"]"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `totp_provider_config`.\n"]
    pub fn set_totp_provider_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<IdentityPlatformConfig_Mfa_ProviderConfigs_TotpProviderConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.totp_provider_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.totp_provider_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IdentityPlatformConfig_Mfa_ProviderConfigs {
    type O = terrars::BlockAssignable<IdentityPlatformConfig_Mfa_ProviderConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformConfig_Mfa_ProviderConfigs {}

impl BuildIdentityPlatformConfig_Mfa_ProviderConfigs {
    pub fn build(self) -> IdentityPlatformConfig_Mfa_ProviderConfigs {
        IdentityPlatformConfig_Mfa_ProviderConfigs {
            state: core::default::Default::default(),
            totp_provider_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IdentityPlatformConfig_Mfa_ProviderConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformConfig_Mfa_ProviderConfigsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IdentityPlatformConfig_Mfa_ProviderConfigsRef {
        IdentityPlatformConfig_Mfa_ProviderConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformConfig_Mfa_ProviderConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nWhether MultiFactor Authentication has been enabled for this project. Possible values: [\"DISABLED\", \"ENABLED\", \"MANDATORY\"]"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `totp_provider_config` after provisioning.\n"]
    pub fn totp_provider_config(
        &self,
    ) -> terrars::ListRef<IdentityPlatformConfig_Mfa_ProviderConfigs_TotpProviderConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.totp_provider_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct IdentityPlatformConfig_MfaDynamic {
    provider_configs: std::option::Option<terrars::DynamicBlock<IdentityPlatformConfig_Mfa_ProviderConfigs>>,
}

#[derive(Serialize)]
pub struct IdentityPlatformConfig_Mfa {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled_providers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider_configs: std::option::Option<std::vec::Vec<IdentityPlatformConfig_Mfa_ProviderConfigs>>,
    dynamic: IdentityPlatformConfig_MfaDynamic,
}

impl IdentityPlatformConfig_Mfa {
    #[doc =
        "Set the field `enabled_providers`.\nA list of usable second factors for this project. Possible values: [\"PHONE_SMS\"]"]
    pub fn set_enabled_providers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.enabled_providers = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `state`.\nWhether MultiFactor Authentication has been enabled for this project. Possible values: [\"DISABLED\", \"ENABLED\", \"MANDATORY\"]"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `provider_configs`.\n"]
    pub fn set_provider_configs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IdentityPlatformConfig_Mfa_ProviderConfigs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.provider_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.provider_configs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IdentityPlatformConfig_Mfa {
    type O = terrars::BlockAssignable<IdentityPlatformConfig_Mfa>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformConfig_Mfa {}

impl BuildIdentityPlatformConfig_Mfa {
    pub fn build(self) -> IdentityPlatformConfig_Mfa {
        IdentityPlatformConfig_Mfa {
            enabled_providers: core::default::Default::default(),
            state: core::default::Default::default(),
            provider_configs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IdentityPlatformConfig_MfaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformConfig_MfaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IdentityPlatformConfig_MfaRef {
        IdentityPlatformConfig_MfaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformConfig_MfaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled_providers` after provisioning.\nA list of usable second factors for this project. Possible values: [\"PHONE_SMS\"]"]
    pub fn enabled_providers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enabled_providers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nWhether MultiFactor Authentication has been enabled for this project. Possible values: [\"DISABLED\", \"ENABLED\", \"MANDATORY\"]"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `provider_configs` after provisioning.\n"]
    pub fn provider_configs(&self) -> terrars::ListRef<IdentityPlatformConfig_Mfa_ProviderConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.provider_configs", self.base))
    }
}

#[derive(Serialize)]
pub struct IdentityPlatformConfig_Monitoring_RequestLogging {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl IdentityPlatformConfig_Monitoring_RequestLogging {
    #[doc = "Set the field `enabled`.\nWhether logging is enabled for this project or not."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IdentityPlatformConfig_Monitoring_RequestLogging {
    type O = terrars::BlockAssignable<IdentityPlatformConfig_Monitoring_RequestLogging>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformConfig_Monitoring_RequestLogging {}

impl BuildIdentityPlatformConfig_Monitoring_RequestLogging {
    pub fn build(self) -> IdentityPlatformConfig_Monitoring_RequestLogging {
        IdentityPlatformConfig_Monitoring_RequestLogging { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct IdentityPlatformConfig_Monitoring_RequestLoggingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformConfig_Monitoring_RequestLoggingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IdentityPlatformConfig_Monitoring_RequestLoggingRef {
        IdentityPlatformConfig_Monitoring_RequestLoggingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformConfig_Monitoring_RequestLoggingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether logging is enabled for this project or not."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct IdentityPlatformConfig_MonitoringDynamic {
    request_logging: std::option::Option<terrars::DynamicBlock<IdentityPlatformConfig_Monitoring_RequestLogging>>,
}

#[derive(Serialize)]
pub struct IdentityPlatformConfig_Monitoring {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_logging: std::option::Option<std::vec::Vec<IdentityPlatformConfig_Monitoring_RequestLogging>>,
    dynamic: IdentityPlatformConfig_MonitoringDynamic,
}

impl IdentityPlatformConfig_Monitoring {
    #[doc = "Set the field `request_logging`.\n"]
    pub fn set_request_logging(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IdentityPlatformConfig_Monitoring_RequestLogging>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.request_logging = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.request_logging = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IdentityPlatformConfig_Monitoring {
    type O = terrars::BlockAssignable<IdentityPlatformConfig_Monitoring>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformConfig_Monitoring {}

impl BuildIdentityPlatformConfig_Monitoring {
    pub fn build(self) -> IdentityPlatformConfig_Monitoring {
        IdentityPlatformConfig_Monitoring {
            request_logging: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IdentityPlatformConfig_MonitoringRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformConfig_MonitoringRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IdentityPlatformConfig_MonitoringRef {
        IdentityPlatformConfig_MonitoringRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformConfig_MonitoringRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `request_logging` after provisioning.\n"]
    pub fn request_logging(&self) -> terrars::ListRef<IdentityPlatformConfig_Monitoring_RequestLoggingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_logging", self.base))
    }
}

#[derive(Serialize)]
pub struct IdentityPlatformConfig_MultiTenant {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_tenants: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_tenant_location: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IdentityPlatformConfig_MultiTenant {
    #[doc = "Set the field `allow_tenants`.\nWhether this project can have tenants or not."]
    pub fn set_allow_tenants(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_tenants = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `default_tenant_location`.\nThe default cloud parent org or folder that the tenant project should be created under.\nThe parent resource name should be in the format of \"/\", such as \"folders/123\" or \"organizations/456\".\nIf the value is not set, the tenant will be created under the same organization or folder as the agent project."]
    pub fn set_default_tenant_location(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.default_tenant_location = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IdentityPlatformConfig_MultiTenant {
    type O = terrars::BlockAssignable<IdentityPlatformConfig_MultiTenant>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformConfig_MultiTenant {}

impl BuildIdentityPlatformConfig_MultiTenant {
    pub fn build(self) -> IdentityPlatformConfig_MultiTenant {
        IdentityPlatformConfig_MultiTenant {
            allow_tenants: core::default::Default::default(),
            default_tenant_location: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IdentityPlatformConfig_MultiTenantRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformConfig_MultiTenantRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IdentityPlatformConfig_MultiTenantRef {
        IdentityPlatformConfig_MultiTenantRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformConfig_MultiTenantRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_tenants` after provisioning.\nWhether this project can have tenants or not."]
    pub fn allow_tenants(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_tenants", self.base))
    }

    #[doc =
        "Get a reference to the value of field `default_tenant_location` after provisioning.\nThe default cloud parent org or folder that the tenant project should be created under.\nThe parent resource name should be in the format of \"/\", such as \"folders/123\" or \"organizations/456\".\nIf the value is not set, the tenant will be created under the same organization or folder as the agent project."]
    pub fn default_tenant_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_tenant_location", self.base))
    }
}

#[derive(Serialize)]
pub struct IdentityPlatformConfig_Quota_SignUpQuotaConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quota: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quota_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IdentityPlatformConfig_Quota_SignUpQuotaConfig {
    #[doc =
        "Set the field `quota`.\nA sign up APIs quota that customers can override temporarily. Value can be in between 1 and 1000."]
    pub fn set_quota(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.quota = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `quota_duration`.\nHow long this quota will be active for. It is measurred in seconds, e.g., Example: \"9.615s\"."]
    pub fn set_quota_duration(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.quota_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\nWhen this quota will take affect."]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IdentityPlatformConfig_Quota_SignUpQuotaConfig {
    type O = terrars::BlockAssignable<IdentityPlatformConfig_Quota_SignUpQuotaConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformConfig_Quota_SignUpQuotaConfig {}

impl BuildIdentityPlatformConfig_Quota_SignUpQuotaConfig {
    pub fn build(self) -> IdentityPlatformConfig_Quota_SignUpQuotaConfig {
        IdentityPlatformConfig_Quota_SignUpQuotaConfig {
            quota: core::default::Default::default(),
            quota_duration: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IdentityPlatformConfig_Quota_SignUpQuotaConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformConfig_Quota_SignUpQuotaConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IdentityPlatformConfig_Quota_SignUpQuotaConfigRef {
        IdentityPlatformConfig_Quota_SignUpQuotaConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformConfig_Quota_SignUpQuotaConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `quota` after provisioning.\nA sign up APIs quota that customers can override temporarily. Value can be in between 1 and 1000."]
    pub fn quota(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quota", self.base))
    }

    #[doc =
        "Get a reference to the value of field `quota_duration` after provisioning.\nHow long this quota will be active for. It is measurred in seconds, e.g., Example: \"9.615s\"."]
    pub fn quota_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quota_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\nWhen this quota will take affect."]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize, Default)]
struct IdentityPlatformConfig_QuotaDynamic {
    sign_up_quota_config: std::option::Option<terrars::DynamicBlock<IdentityPlatformConfig_Quota_SignUpQuotaConfig>>,
}

#[derive(Serialize)]
pub struct IdentityPlatformConfig_Quota {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sign_up_quota_config: std::option::Option<std::vec::Vec<IdentityPlatformConfig_Quota_SignUpQuotaConfig>>,
    dynamic: IdentityPlatformConfig_QuotaDynamic,
}

impl IdentityPlatformConfig_Quota {
    #[doc = "Set the field `sign_up_quota_config`.\n"]
    pub fn set_sign_up_quota_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IdentityPlatformConfig_Quota_SignUpQuotaConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sign_up_quota_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sign_up_quota_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IdentityPlatformConfig_Quota {
    type O = terrars::BlockAssignable<IdentityPlatformConfig_Quota>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformConfig_Quota {}

impl BuildIdentityPlatformConfig_Quota {
    pub fn build(self) -> IdentityPlatformConfig_Quota {
        IdentityPlatformConfig_Quota {
            sign_up_quota_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IdentityPlatformConfig_QuotaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformConfig_QuotaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IdentityPlatformConfig_QuotaRef {
        IdentityPlatformConfig_QuotaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformConfig_QuotaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sign_up_quota_config` after provisioning.\n"]
    pub fn sign_up_quota_config(&self) -> terrars::ListRef<IdentityPlatformConfig_Quota_SignUpQuotaConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sign_up_quota_config", self.base))
    }
}

#[derive(Serialize)]
pub struct IdentityPlatformConfig_SignIn_HashConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    algorithm: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_cost: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rounds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    salt_separator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    signer_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IdentityPlatformConfig_SignIn_HashConfig {
    #[doc = "Set the field `algorithm`.\n"]
    pub fn set_algorithm(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.algorithm = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_cost`.\n"]
    pub fn set_memory_cost(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_cost = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rounds`.\n"]
    pub fn set_rounds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.rounds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `salt_separator`.\n"]
    pub fn set_salt_separator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.salt_separator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `signer_key`.\n"]
    pub fn set_signer_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.signer_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IdentityPlatformConfig_SignIn_HashConfig {
    type O = terrars::BlockAssignable<IdentityPlatformConfig_SignIn_HashConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformConfig_SignIn_HashConfig {}

impl BuildIdentityPlatformConfig_SignIn_HashConfig {
    pub fn build(self) -> IdentityPlatformConfig_SignIn_HashConfig {
        IdentityPlatformConfig_SignIn_HashConfig {
            algorithm: core::default::Default::default(),
            memory_cost: core::default::Default::default(),
            rounds: core::default::Default::default(),
            salt_separator: core::default::Default::default(),
            signer_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IdentityPlatformConfig_SignIn_HashConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformConfig_SignIn_HashConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IdentityPlatformConfig_SignIn_HashConfigRef {
        IdentityPlatformConfig_SignIn_HashConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformConfig_SignIn_HashConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `algorithm` after provisioning.\n"]
    pub fn algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_cost` after provisioning.\n"]
    pub fn memory_cost(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_cost", self.base))
    }

    #[doc = "Get a reference to the value of field `rounds` after provisioning.\n"]
    pub fn rounds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rounds", self.base))
    }

    #[doc = "Get a reference to the value of field `salt_separator` after provisioning.\n"]
    pub fn salt_separator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.salt_separator", self.base))
    }

    #[doc = "Get a reference to the value of field `signer_key` after provisioning.\n"]
    pub fn signer_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.signer_key", self.base))
    }
}

#[derive(Serialize)]
pub struct IdentityPlatformConfig_SignIn_Anonymous {
    enabled: terrars::PrimField<bool>,
}

impl IdentityPlatformConfig_SignIn_Anonymous { }

impl terrars::ToListMappable for IdentityPlatformConfig_SignIn_Anonymous {
    type O = terrars::BlockAssignable<IdentityPlatformConfig_SignIn_Anonymous>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformConfig_SignIn_Anonymous {
    #[doc = "Whether anonymous user auth is enabled for the project or not."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildIdentityPlatformConfig_SignIn_Anonymous {
    pub fn build(self) -> IdentityPlatformConfig_SignIn_Anonymous {
        IdentityPlatformConfig_SignIn_Anonymous { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct IdentityPlatformConfig_SignIn_AnonymousRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformConfig_SignIn_AnonymousRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IdentityPlatformConfig_SignIn_AnonymousRef {
        IdentityPlatformConfig_SignIn_AnonymousRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformConfig_SignIn_AnonymousRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether anonymous user auth is enabled for the project or not."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct IdentityPlatformConfig_SignIn_Email {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_required: std::option::Option<terrars::PrimField<bool>>,
}

impl IdentityPlatformConfig_SignIn_Email {
    #[doc =
        "Set the field `password_required`.\nWhether a password is required for email auth or not. If true, both an email and\npassword must be provided to sign in. If false, a user may sign in via either\nemail/password or email link."]
    pub fn set_password_required(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.password_required = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IdentityPlatformConfig_SignIn_Email {
    type O = terrars::BlockAssignable<IdentityPlatformConfig_SignIn_Email>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformConfig_SignIn_Email {
    #[doc = "Whether email auth is enabled for the project or not."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildIdentityPlatformConfig_SignIn_Email {
    pub fn build(self) -> IdentityPlatformConfig_SignIn_Email {
        IdentityPlatformConfig_SignIn_Email {
            enabled: self.enabled,
            password_required: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IdentityPlatformConfig_SignIn_EmailRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformConfig_SignIn_EmailRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IdentityPlatformConfig_SignIn_EmailRef {
        IdentityPlatformConfig_SignIn_EmailRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformConfig_SignIn_EmailRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether email auth is enabled for the project or not."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password_required` after provisioning.\nWhether a password is required for email auth or not. If true, both an email and\npassword must be provided to sign in. If false, a user may sign in via either\nemail/password or email link."]
    pub fn password_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_required", self.base))
    }
}

#[derive(Serialize)]
pub struct IdentityPlatformConfig_SignIn_PhoneNumber {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    test_phone_numbers: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl IdentityPlatformConfig_SignIn_PhoneNumber {
    #[doc =
        "Set the field `test_phone_numbers`.\nA map of <test phone number, fake code> that can be used for phone auth testing."]
    pub fn set_test_phone_numbers(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.test_phone_numbers = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IdentityPlatformConfig_SignIn_PhoneNumber {
    type O = terrars::BlockAssignable<IdentityPlatformConfig_SignIn_PhoneNumber>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformConfig_SignIn_PhoneNumber {
    #[doc = "Whether phone number auth is enabled for the project or not."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildIdentityPlatformConfig_SignIn_PhoneNumber {
    pub fn build(self) -> IdentityPlatformConfig_SignIn_PhoneNumber {
        IdentityPlatformConfig_SignIn_PhoneNumber {
            enabled: self.enabled,
            test_phone_numbers: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IdentityPlatformConfig_SignIn_PhoneNumberRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformConfig_SignIn_PhoneNumberRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IdentityPlatformConfig_SignIn_PhoneNumberRef {
        IdentityPlatformConfig_SignIn_PhoneNumberRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformConfig_SignIn_PhoneNumberRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether phone number auth is enabled for the project or not."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `test_phone_numbers` after provisioning.\nA map of <test phone number, fake code> that can be used for phone auth testing."]
    pub fn test_phone_numbers(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.test_phone_numbers", self.base))
    }
}

#[derive(Serialize, Default)]
struct IdentityPlatformConfig_SignInDynamic {
    anonymous: std::option::Option<terrars::DynamicBlock<IdentityPlatformConfig_SignIn_Anonymous>>,
    email: std::option::Option<terrars::DynamicBlock<IdentityPlatformConfig_SignIn_Email>>,
    phone_number: std::option::Option<terrars::DynamicBlock<IdentityPlatformConfig_SignIn_PhoneNumber>>,
}

#[derive(Serialize)]
pub struct IdentityPlatformConfig_SignIn {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_duplicate_emails: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    anonymous: std::option::Option<std::vec::Vec<IdentityPlatformConfig_SignIn_Anonymous>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<std::vec::Vec<IdentityPlatformConfig_SignIn_Email>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phone_number: std::option::Option<std::vec::Vec<IdentityPlatformConfig_SignIn_PhoneNumber>>,
    dynamic: IdentityPlatformConfig_SignInDynamic,
}

impl IdentityPlatformConfig_SignIn {
    #[doc = "Set the field `allow_duplicate_emails`.\nWhether to allow more than one account to have the same email."]
    pub fn set_allow_duplicate_emails(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_duplicate_emails = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `anonymous`.\n"]
    pub fn set_anonymous(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IdentityPlatformConfig_SignIn_Anonymous>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.anonymous = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.anonymous = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `email`.\n"]
    pub fn set_email(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IdentityPlatformConfig_SignIn_Email>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.email = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.email = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `phone_number`.\n"]
    pub fn set_phone_number(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IdentityPlatformConfig_SignIn_PhoneNumber>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.phone_number = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.phone_number = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IdentityPlatformConfig_SignIn {
    type O = terrars::BlockAssignable<IdentityPlatformConfig_SignIn>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformConfig_SignIn {}

impl BuildIdentityPlatformConfig_SignIn {
    pub fn build(self) -> IdentityPlatformConfig_SignIn {
        IdentityPlatformConfig_SignIn {
            allow_duplicate_emails: core::default::Default::default(),
            anonymous: core::default::Default::default(),
            email: core::default::Default::default(),
            phone_number: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IdentityPlatformConfig_SignInRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformConfig_SignInRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IdentityPlatformConfig_SignInRef {
        IdentityPlatformConfig_SignInRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformConfig_SignInRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_duplicate_emails` after provisioning.\nWhether to allow more than one account to have the same email."]
    pub fn allow_duplicate_emails(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_duplicate_emails", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hash_config` after provisioning.\nOutput only. Hash config information."]
    pub fn hash_config(&self) -> terrars::ListRef<IdentityPlatformConfig_SignIn_HashConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hash_config", self.base))
    }

    #[doc = "Get a reference to the value of field `anonymous` after provisioning.\n"]
    pub fn anonymous(&self) -> terrars::ListRef<IdentityPlatformConfig_SignIn_AnonymousRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.anonymous", self.base))
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> terrars::ListRef<IdentityPlatformConfig_SignIn_EmailRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `phone_number` after provisioning.\n"]
    pub fn phone_number(&self) -> terrars::ListRef<IdentityPlatformConfig_SignIn_PhoneNumberRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }
}

#[derive(Serialize)]
pub struct IdentityPlatformConfig_SmsRegionConfig_AllowByDefault {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disallowed_regions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl IdentityPlatformConfig_SmsRegionConfig_AllowByDefault {
    #[doc =
        "Set the field `disallowed_regions`.\nTwo letter unicode region codes to disallow as defined by https://cldr.unicode.org/ The full list of these region codes is here: https://github.com/unicode-cldr/cldr-localenames-full/blob/master/main/en/territories.json"]
    pub fn set_disallowed_regions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.disallowed_regions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IdentityPlatformConfig_SmsRegionConfig_AllowByDefault {
    type O = terrars::BlockAssignable<IdentityPlatformConfig_SmsRegionConfig_AllowByDefault>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformConfig_SmsRegionConfig_AllowByDefault {}

impl BuildIdentityPlatformConfig_SmsRegionConfig_AllowByDefault {
    pub fn build(self) -> IdentityPlatformConfig_SmsRegionConfig_AllowByDefault {
        IdentityPlatformConfig_SmsRegionConfig_AllowByDefault {
            disallowed_regions: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IdentityPlatformConfig_SmsRegionConfig_AllowByDefaultRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformConfig_SmsRegionConfig_AllowByDefaultRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IdentityPlatformConfig_SmsRegionConfig_AllowByDefaultRef {
        IdentityPlatformConfig_SmsRegionConfig_AllowByDefaultRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformConfig_SmsRegionConfig_AllowByDefaultRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disallowed_regions` after provisioning.\nTwo letter unicode region codes to disallow as defined by https://cldr.unicode.org/ The full list of these region codes is here: https://github.com/unicode-cldr/cldr-localenames-full/blob/master/main/en/territories.json"]
    pub fn disallowed_regions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disallowed_regions", self.base))
    }
}

#[derive(Serialize)]
pub struct IdentityPlatformConfig_SmsRegionConfig_AllowlistOnly {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_regions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl IdentityPlatformConfig_SmsRegionConfig_AllowlistOnly {
    #[doc =
        "Set the field `allowed_regions`.\nTwo letter unicode region codes to allow as defined by https://cldr.unicode.org/ The full list of these region codes is here: https://github.com/unicode-cldr/cldr-localenames-full/blob/master/main/en/territories.json"]
    pub fn set_allowed_regions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_regions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IdentityPlatformConfig_SmsRegionConfig_AllowlistOnly {
    type O = terrars::BlockAssignable<IdentityPlatformConfig_SmsRegionConfig_AllowlistOnly>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformConfig_SmsRegionConfig_AllowlistOnly {}

impl BuildIdentityPlatformConfig_SmsRegionConfig_AllowlistOnly {
    pub fn build(self) -> IdentityPlatformConfig_SmsRegionConfig_AllowlistOnly {
        IdentityPlatformConfig_SmsRegionConfig_AllowlistOnly { allowed_regions: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct IdentityPlatformConfig_SmsRegionConfig_AllowlistOnlyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformConfig_SmsRegionConfig_AllowlistOnlyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IdentityPlatformConfig_SmsRegionConfig_AllowlistOnlyRef {
        IdentityPlatformConfig_SmsRegionConfig_AllowlistOnlyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformConfig_SmsRegionConfig_AllowlistOnlyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_regions` after provisioning.\nTwo letter unicode region codes to allow as defined by https://cldr.unicode.org/ The full list of these region codes is here: https://github.com/unicode-cldr/cldr-localenames-full/blob/master/main/en/territories.json"]
    pub fn allowed_regions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_regions", self.base))
    }
}

#[derive(Serialize, Default)]
struct IdentityPlatformConfig_SmsRegionConfigDynamic {
    allow_by_default: std
    ::option
    ::Option<terrars::DynamicBlock<IdentityPlatformConfig_SmsRegionConfig_AllowByDefault>>,
    allowlist_only: std
    ::option
    ::Option<terrars::DynamicBlock<IdentityPlatformConfig_SmsRegionConfig_AllowlistOnly>>,
}

#[derive(Serialize)]
pub struct IdentityPlatformConfig_SmsRegionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_by_default: std::option::Option<std::vec::Vec<IdentityPlatformConfig_SmsRegionConfig_AllowByDefault>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowlist_only: std::option::Option<std::vec::Vec<IdentityPlatformConfig_SmsRegionConfig_AllowlistOnly>>,
    dynamic: IdentityPlatformConfig_SmsRegionConfigDynamic,
}

impl IdentityPlatformConfig_SmsRegionConfig {
    #[doc = "Set the field `allow_by_default`.\n"]
    pub fn set_allow_by_default(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IdentityPlatformConfig_SmsRegionConfig_AllowByDefault>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.allow_by_default = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.allow_by_default = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `allowlist_only`.\n"]
    pub fn set_allowlist_only(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IdentityPlatformConfig_SmsRegionConfig_AllowlistOnly>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.allowlist_only = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.allowlist_only = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IdentityPlatformConfig_SmsRegionConfig {
    type O = terrars::BlockAssignable<IdentityPlatformConfig_SmsRegionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformConfig_SmsRegionConfig {}

impl BuildIdentityPlatformConfig_SmsRegionConfig {
    pub fn build(self) -> IdentityPlatformConfig_SmsRegionConfig {
        IdentityPlatformConfig_SmsRegionConfig {
            allow_by_default: core::default::Default::default(),
            allowlist_only: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IdentityPlatformConfig_SmsRegionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformConfig_SmsRegionConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IdentityPlatformConfig_SmsRegionConfigRef {
        IdentityPlatformConfig_SmsRegionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformConfig_SmsRegionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_by_default` after provisioning.\n"]
    pub fn allow_by_default(&self) -> terrars::ListRef<IdentityPlatformConfig_SmsRegionConfig_AllowByDefaultRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow_by_default", self.base))
    }

    #[doc = "Get a reference to the value of field `allowlist_only` after provisioning.\n"]
    pub fn allowlist_only(&self) -> terrars::ListRef<IdentityPlatformConfig_SmsRegionConfig_AllowlistOnlyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowlist_only", self.base))
    }
}

#[derive(Serialize)]
pub struct IdentityPlatformConfig_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IdentityPlatformConfig_Timeouts {
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

impl terrars::ToListMappable for IdentityPlatformConfig_Timeouts {
    type O = terrars::BlockAssignable<IdentityPlatformConfig_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformConfig_Timeouts {}

impl BuildIdentityPlatformConfig_Timeouts {
    pub fn build(self) -> IdentityPlatformConfig_Timeouts {
        IdentityPlatformConfig_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IdentityPlatformConfig_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformConfig_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IdentityPlatformConfig_TimeoutsRef {
        IdentityPlatformConfig_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformConfig_TimeoutsRef {
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
