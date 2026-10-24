use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct IdentityPlatformConfigData {
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
    blocking_functions: std::option::Option<std::vec::Vec<IdentityPlatformConfigBlockingFunctionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quota: std::option::Option<std::vec::Vec<IdentityPlatformConfigQuotaEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sign_in: std::option::Option<std::vec::Vec<IdentityPlatformConfigSignInEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sms_region_config: std::option::Option<std::vec::Vec<IdentityPlatformConfigSmsRegionConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<IdentityPlatformConfigTimeoutsEl>,
    dynamic: IdentityPlatformConfigDynamic,
}

struct IdentityPlatformConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<IdentityPlatformConfigData>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<IdentityPlatformConfigBlockingFunctionsEl>>,
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

    #[doc = "Set the field `quota`.\n"]
    pub fn set_quota(self, v: impl std::convert::Into<terrars::BlockAssignable<IdentityPlatformConfigQuotaEl>>) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<IdentityPlatformConfigSignInEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<IdentityPlatformConfigSmsRegionConfigEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<IdentityPlatformConfigTimeoutsEl>) -> Self {
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
    pub fn blocking_functions(&self) -> terrars::ListRef<IdentityPlatformConfigBlockingFunctionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.blocking_functions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quota` after provisioning.\n"]
    pub fn quota(&self) -> terrars::ListRef<IdentityPlatformConfigQuotaElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.quota", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sign_in` after provisioning.\n"]
    pub fn sign_in(&self) -> terrars::ListRef<IdentityPlatformConfigSignInElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sign_in", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sms_region_config` after provisioning.\n"]
    pub fn sms_region_config(&self) -> terrars::ListRef<IdentityPlatformConfigSmsRegionConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sms_region_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IdentityPlatformConfigTimeoutsElRef {
        IdentityPlatformConfigTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
            data: std::cell::RefCell::new(IdentityPlatformConfigData {
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
    pub fn blocking_functions(&self) -> terrars::ListRef<IdentityPlatformConfigBlockingFunctionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.blocking_functions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quota` after provisioning.\n"]
    pub fn quota(&self) -> terrars::ListRef<IdentityPlatformConfigQuotaElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.quota", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sign_in` after provisioning.\n"]
    pub fn sign_in(&self) -> terrars::ListRef<IdentityPlatformConfigSignInElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sign_in", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sms_region_config` after provisioning.\n"]
    pub fn sms_region_config(&self) -> terrars::ListRef<IdentityPlatformConfigSmsRegionConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sms_region_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IdentityPlatformConfigTimeoutsElRef {
        IdentityPlatformConfigTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct IdentityPlatformConfigBlockingFunctionsElForwardInboundCredentialsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_token: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id_token: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    refresh_token: std::option::Option<terrars::PrimField<bool>>,
}

impl IdentityPlatformConfigBlockingFunctionsElForwardInboundCredentialsEl {
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

impl terrars::ToListMappable for IdentityPlatformConfigBlockingFunctionsElForwardInboundCredentialsEl {
    type O = terrars::BlockAssignable<IdentityPlatformConfigBlockingFunctionsElForwardInboundCredentialsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformConfigBlockingFunctionsElForwardInboundCredentialsEl {}

impl BuildIdentityPlatformConfigBlockingFunctionsElForwardInboundCredentialsEl {
    pub fn build(self) -> IdentityPlatformConfigBlockingFunctionsElForwardInboundCredentialsEl {
        IdentityPlatformConfigBlockingFunctionsElForwardInboundCredentialsEl {
            access_token: core::default::Default::default(),
            id_token: core::default::Default::default(),
            refresh_token: core::default::Default::default(),
        }
    }
}

pub struct IdentityPlatformConfigBlockingFunctionsElForwardInboundCredentialsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformConfigBlockingFunctionsElForwardInboundCredentialsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IdentityPlatformConfigBlockingFunctionsElForwardInboundCredentialsElRef {
        IdentityPlatformConfigBlockingFunctionsElForwardInboundCredentialsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformConfigBlockingFunctionsElForwardInboundCredentialsElRef {
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
pub struct IdentityPlatformConfigBlockingFunctionsElTriggersEl {
    event_type: terrars::PrimField<std::string::String>,
    function_uri: terrars::PrimField<std::string::String>,
}

impl IdentityPlatformConfigBlockingFunctionsElTriggersEl { }

impl terrars::ToListMappable for IdentityPlatformConfigBlockingFunctionsElTriggersEl {
    type O = terrars::BlockAssignable<IdentityPlatformConfigBlockingFunctionsElTriggersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformConfigBlockingFunctionsElTriggersEl {
    #[doc = ""]
    pub event_type: terrars::PrimField<std::string::String>,
    #[doc = "HTTP URI trigger for the Cloud Function."]
    pub function_uri: terrars::PrimField<std::string::String>,
}

impl BuildIdentityPlatformConfigBlockingFunctionsElTriggersEl {
    pub fn build(self) -> IdentityPlatformConfigBlockingFunctionsElTriggersEl {
        IdentityPlatformConfigBlockingFunctionsElTriggersEl {
            event_type: self.event_type,
            function_uri: self.function_uri,
        }
    }
}

pub struct IdentityPlatformConfigBlockingFunctionsElTriggersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformConfigBlockingFunctionsElTriggersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IdentityPlatformConfigBlockingFunctionsElTriggersElRef {
        IdentityPlatformConfigBlockingFunctionsElTriggersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformConfigBlockingFunctionsElTriggersElRef {
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
struct IdentityPlatformConfigBlockingFunctionsElDynamic {
    forward_inbound_credentials: std
    ::option
    ::Option<terrars::DynamicBlock<IdentityPlatformConfigBlockingFunctionsElForwardInboundCredentialsEl>>,
    triggers: std::option::Option<terrars::DynamicBlock<IdentityPlatformConfigBlockingFunctionsElTriggersEl>>,
}

#[derive(Serialize)]
pub struct IdentityPlatformConfigBlockingFunctionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forward_inbound_credentials: std
    ::option
    ::Option<std::vec::Vec<IdentityPlatformConfigBlockingFunctionsElForwardInboundCredentialsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    triggers: std::option::Option<std::vec::Vec<IdentityPlatformConfigBlockingFunctionsElTriggersEl>>,
    dynamic: IdentityPlatformConfigBlockingFunctionsElDynamic,
}

impl IdentityPlatformConfigBlockingFunctionsEl {
    #[doc = "Set the field `forward_inbound_credentials`.\n"]
    pub fn set_forward_inbound_credentials(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<IdentityPlatformConfigBlockingFunctionsElForwardInboundCredentialsEl>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<IdentityPlatformConfigBlockingFunctionsElTriggersEl>>,
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

impl terrars::ToListMappable for IdentityPlatformConfigBlockingFunctionsEl {
    type O = terrars::BlockAssignable<IdentityPlatformConfigBlockingFunctionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformConfigBlockingFunctionsEl {}

impl BuildIdentityPlatformConfigBlockingFunctionsEl {
    pub fn build(self) -> IdentityPlatformConfigBlockingFunctionsEl {
        IdentityPlatformConfigBlockingFunctionsEl {
            forward_inbound_credentials: core::default::Default::default(),
            triggers: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct IdentityPlatformConfigBlockingFunctionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformConfigBlockingFunctionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IdentityPlatformConfigBlockingFunctionsElRef {
        IdentityPlatformConfigBlockingFunctionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformConfigBlockingFunctionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `forward_inbound_credentials` after provisioning.\n"]
    pub fn forward_inbound_credentials(
        &self,
    ) -> terrars::ListRef<IdentityPlatformConfigBlockingFunctionsElForwardInboundCredentialsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forward_inbound_credentials", self.base))
    }
}

#[derive(Serialize)]
pub struct IdentityPlatformConfigQuotaElSignUpQuotaConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quota: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quota_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IdentityPlatformConfigQuotaElSignUpQuotaConfigEl {
    #[doc = "Set the field `quota`.\nA sign up APIs quota that customers can override temporarily."]
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

impl terrars::ToListMappable for IdentityPlatformConfigQuotaElSignUpQuotaConfigEl {
    type O = terrars::BlockAssignable<IdentityPlatformConfigQuotaElSignUpQuotaConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformConfigQuotaElSignUpQuotaConfigEl {}

impl BuildIdentityPlatformConfigQuotaElSignUpQuotaConfigEl {
    pub fn build(self) -> IdentityPlatformConfigQuotaElSignUpQuotaConfigEl {
        IdentityPlatformConfigQuotaElSignUpQuotaConfigEl {
            quota: core::default::Default::default(),
            quota_duration: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

pub struct IdentityPlatformConfigQuotaElSignUpQuotaConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformConfigQuotaElSignUpQuotaConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IdentityPlatformConfigQuotaElSignUpQuotaConfigElRef {
        IdentityPlatformConfigQuotaElSignUpQuotaConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformConfigQuotaElSignUpQuotaConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `quota` after provisioning.\nA sign up APIs quota that customers can override temporarily."]
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
struct IdentityPlatformConfigQuotaElDynamic {
    sign_up_quota_config: std
    ::option
    ::Option<terrars::DynamicBlock<IdentityPlatformConfigQuotaElSignUpQuotaConfigEl>>,
}

#[derive(Serialize)]
pub struct IdentityPlatformConfigQuotaEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sign_up_quota_config: std::option::Option<std::vec::Vec<IdentityPlatformConfigQuotaElSignUpQuotaConfigEl>>,
    dynamic: IdentityPlatformConfigQuotaElDynamic,
}

impl IdentityPlatformConfigQuotaEl {
    #[doc = "Set the field `sign_up_quota_config`.\n"]
    pub fn set_sign_up_quota_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IdentityPlatformConfigQuotaElSignUpQuotaConfigEl>>,
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

impl terrars::ToListMappable for IdentityPlatformConfigQuotaEl {
    type O = terrars::BlockAssignable<IdentityPlatformConfigQuotaEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformConfigQuotaEl {}

impl BuildIdentityPlatformConfigQuotaEl {
    pub fn build(self) -> IdentityPlatformConfigQuotaEl {
        IdentityPlatformConfigQuotaEl {
            sign_up_quota_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct IdentityPlatformConfigQuotaElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformConfigQuotaElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IdentityPlatformConfigQuotaElRef {
        IdentityPlatformConfigQuotaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformConfigQuotaElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sign_up_quota_config` after provisioning.\n"]
    pub fn sign_up_quota_config(&self) -> terrars::ListRef<IdentityPlatformConfigQuotaElSignUpQuotaConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sign_up_quota_config", self.base))
    }
}

#[derive(Serialize)]
pub struct IdentityPlatformConfigSignInElHashConfigEl {
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

impl IdentityPlatformConfigSignInElHashConfigEl {
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

impl terrars::ToListMappable for IdentityPlatformConfigSignInElHashConfigEl {
    type O = terrars::BlockAssignable<IdentityPlatformConfigSignInElHashConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformConfigSignInElHashConfigEl {}

impl BuildIdentityPlatformConfigSignInElHashConfigEl {
    pub fn build(self) -> IdentityPlatformConfigSignInElHashConfigEl {
        IdentityPlatformConfigSignInElHashConfigEl {
            algorithm: core::default::Default::default(),
            memory_cost: core::default::Default::default(),
            rounds: core::default::Default::default(),
            salt_separator: core::default::Default::default(),
            signer_key: core::default::Default::default(),
        }
    }
}

pub struct IdentityPlatformConfigSignInElHashConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformConfigSignInElHashConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IdentityPlatformConfigSignInElHashConfigElRef {
        IdentityPlatformConfigSignInElHashConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformConfigSignInElHashConfigElRef {
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
pub struct IdentityPlatformConfigSignInElAnonymousEl {
    enabled: terrars::PrimField<bool>,
}

impl IdentityPlatformConfigSignInElAnonymousEl { }

impl terrars::ToListMappable for IdentityPlatformConfigSignInElAnonymousEl {
    type O = terrars::BlockAssignable<IdentityPlatformConfigSignInElAnonymousEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformConfigSignInElAnonymousEl {
    #[doc = "Whether anonymous user auth is enabled for the project or not."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildIdentityPlatformConfigSignInElAnonymousEl {
    pub fn build(self) -> IdentityPlatformConfigSignInElAnonymousEl {
        IdentityPlatformConfigSignInElAnonymousEl { enabled: self.enabled }
    }
}

pub struct IdentityPlatformConfigSignInElAnonymousElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformConfigSignInElAnonymousElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IdentityPlatformConfigSignInElAnonymousElRef {
        IdentityPlatformConfigSignInElAnonymousElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformConfigSignInElAnonymousElRef {
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
pub struct IdentityPlatformConfigSignInElEmailEl {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_required: std::option::Option<terrars::PrimField<bool>>,
}

impl IdentityPlatformConfigSignInElEmailEl {
    #[doc =
        "Set the field `password_required`.\nWhether a password is required for email auth or not. If true, both an email and\npassword must be provided to sign in. If false, a user may sign in via either\nemail/password or email link."]
    pub fn set_password_required(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.password_required = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IdentityPlatformConfigSignInElEmailEl {
    type O = terrars::BlockAssignable<IdentityPlatformConfigSignInElEmailEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformConfigSignInElEmailEl {
    #[doc = "Whether email auth is enabled for the project or not."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildIdentityPlatformConfigSignInElEmailEl {
    pub fn build(self) -> IdentityPlatformConfigSignInElEmailEl {
        IdentityPlatformConfigSignInElEmailEl {
            enabled: self.enabled,
            password_required: core::default::Default::default(),
        }
    }
}

pub struct IdentityPlatformConfigSignInElEmailElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformConfigSignInElEmailElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IdentityPlatformConfigSignInElEmailElRef {
        IdentityPlatformConfigSignInElEmailElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformConfigSignInElEmailElRef {
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
pub struct IdentityPlatformConfigSignInElPhoneNumberEl {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    test_phone_numbers: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl IdentityPlatformConfigSignInElPhoneNumberEl {
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

impl terrars::ToListMappable for IdentityPlatformConfigSignInElPhoneNumberEl {
    type O = terrars::BlockAssignable<IdentityPlatformConfigSignInElPhoneNumberEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformConfigSignInElPhoneNumberEl {
    #[doc = "Whether phone number auth is enabled for the project or not."]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildIdentityPlatformConfigSignInElPhoneNumberEl {
    pub fn build(self) -> IdentityPlatformConfigSignInElPhoneNumberEl {
        IdentityPlatformConfigSignInElPhoneNumberEl {
            enabled: self.enabled,
            test_phone_numbers: core::default::Default::default(),
        }
    }
}

pub struct IdentityPlatformConfigSignInElPhoneNumberElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformConfigSignInElPhoneNumberElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IdentityPlatformConfigSignInElPhoneNumberElRef {
        IdentityPlatformConfigSignInElPhoneNumberElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformConfigSignInElPhoneNumberElRef {
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
struct IdentityPlatformConfigSignInElDynamic {
    anonymous: std::option::Option<terrars::DynamicBlock<IdentityPlatformConfigSignInElAnonymousEl>>,
    email: std::option::Option<terrars::DynamicBlock<IdentityPlatformConfigSignInElEmailEl>>,
    phone_number: std::option::Option<terrars::DynamicBlock<IdentityPlatformConfigSignInElPhoneNumberEl>>,
}

#[derive(Serialize)]
pub struct IdentityPlatformConfigSignInEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_duplicate_emails: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    anonymous: std::option::Option<std::vec::Vec<IdentityPlatformConfigSignInElAnonymousEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<std::vec::Vec<IdentityPlatformConfigSignInElEmailEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phone_number: std::option::Option<std::vec::Vec<IdentityPlatformConfigSignInElPhoneNumberEl>>,
    dynamic: IdentityPlatformConfigSignInElDynamic,
}

impl IdentityPlatformConfigSignInEl {
    #[doc = "Set the field `allow_duplicate_emails`.\nWhether to allow more than one account to have the same email."]
    pub fn set_allow_duplicate_emails(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_duplicate_emails = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `anonymous`.\n"]
    pub fn set_anonymous(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IdentityPlatformConfigSignInElAnonymousEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<IdentityPlatformConfigSignInElEmailEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<IdentityPlatformConfigSignInElPhoneNumberEl>>,
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

impl terrars::ToListMappable for IdentityPlatformConfigSignInEl {
    type O = terrars::BlockAssignable<IdentityPlatformConfigSignInEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformConfigSignInEl {}

impl BuildIdentityPlatformConfigSignInEl {
    pub fn build(self) -> IdentityPlatformConfigSignInEl {
        IdentityPlatformConfigSignInEl {
            allow_duplicate_emails: core::default::Default::default(),
            anonymous: core::default::Default::default(),
            email: core::default::Default::default(),
            phone_number: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct IdentityPlatformConfigSignInElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformConfigSignInElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IdentityPlatformConfigSignInElRef {
        IdentityPlatformConfigSignInElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformConfigSignInElRef {
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
    pub fn hash_config(&self) -> terrars::ListRef<IdentityPlatformConfigSignInElHashConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hash_config", self.base))
    }

    #[doc = "Get a reference to the value of field `anonymous` after provisioning.\n"]
    pub fn anonymous(&self) -> terrars::ListRef<IdentityPlatformConfigSignInElAnonymousElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.anonymous", self.base))
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> terrars::ListRef<IdentityPlatformConfigSignInElEmailElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `phone_number` after provisioning.\n"]
    pub fn phone_number(&self) -> terrars::ListRef<IdentityPlatformConfigSignInElPhoneNumberElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }
}

#[derive(Serialize)]
pub struct IdentityPlatformConfigSmsRegionConfigElAllowByDefaultEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disallowed_regions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl IdentityPlatformConfigSmsRegionConfigElAllowByDefaultEl {
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

impl terrars::ToListMappable for IdentityPlatformConfigSmsRegionConfigElAllowByDefaultEl {
    type O = terrars::BlockAssignable<IdentityPlatformConfigSmsRegionConfigElAllowByDefaultEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformConfigSmsRegionConfigElAllowByDefaultEl {}

impl BuildIdentityPlatformConfigSmsRegionConfigElAllowByDefaultEl {
    pub fn build(self) -> IdentityPlatformConfigSmsRegionConfigElAllowByDefaultEl {
        IdentityPlatformConfigSmsRegionConfigElAllowByDefaultEl {
            disallowed_regions: core::default::Default::default(),
        }
    }
}

pub struct IdentityPlatformConfigSmsRegionConfigElAllowByDefaultElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformConfigSmsRegionConfigElAllowByDefaultElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IdentityPlatformConfigSmsRegionConfigElAllowByDefaultElRef {
        IdentityPlatformConfigSmsRegionConfigElAllowByDefaultElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformConfigSmsRegionConfigElAllowByDefaultElRef {
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
pub struct IdentityPlatformConfigSmsRegionConfigElAllowlistOnlyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_regions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl IdentityPlatformConfigSmsRegionConfigElAllowlistOnlyEl {
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

impl terrars::ToListMappable for IdentityPlatformConfigSmsRegionConfigElAllowlistOnlyEl {
    type O = terrars::BlockAssignable<IdentityPlatformConfigSmsRegionConfigElAllowlistOnlyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformConfigSmsRegionConfigElAllowlistOnlyEl {}

impl BuildIdentityPlatformConfigSmsRegionConfigElAllowlistOnlyEl {
    pub fn build(self) -> IdentityPlatformConfigSmsRegionConfigElAllowlistOnlyEl {
        IdentityPlatformConfigSmsRegionConfigElAllowlistOnlyEl { allowed_regions: core::default::Default::default() }
    }
}

pub struct IdentityPlatformConfigSmsRegionConfigElAllowlistOnlyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformConfigSmsRegionConfigElAllowlistOnlyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IdentityPlatformConfigSmsRegionConfigElAllowlistOnlyElRef {
        IdentityPlatformConfigSmsRegionConfigElAllowlistOnlyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformConfigSmsRegionConfigElAllowlistOnlyElRef {
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
struct IdentityPlatformConfigSmsRegionConfigElDynamic {
    allow_by_default: std
    ::option
    ::Option<terrars::DynamicBlock<IdentityPlatformConfigSmsRegionConfigElAllowByDefaultEl>>,
    allowlist_only: std
    ::option
    ::Option<terrars::DynamicBlock<IdentityPlatformConfigSmsRegionConfigElAllowlistOnlyEl>>,
}

#[derive(Serialize)]
pub struct IdentityPlatformConfigSmsRegionConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_by_default: std::option::Option<std::vec::Vec<IdentityPlatformConfigSmsRegionConfigElAllowByDefaultEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowlist_only: std::option::Option<std::vec::Vec<IdentityPlatformConfigSmsRegionConfigElAllowlistOnlyEl>>,
    dynamic: IdentityPlatformConfigSmsRegionConfigElDynamic,
}

impl IdentityPlatformConfigSmsRegionConfigEl {
    #[doc = "Set the field `allow_by_default`.\n"]
    pub fn set_allow_by_default(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<IdentityPlatformConfigSmsRegionConfigElAllowByDefaultEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<IdentityPlatformConfigSmsRegionConfigElAllowlistOnlyEl>>,
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

impl terrars::ToListMappable for IdentityPlatformConfigSmsRegionConfigEl {
    type O = terrars::BlockAssignable<IdentityPlatformConfigSmsRegionConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformConfigSmsRegionConfigEl {}

impl BuildIdentityPlatformConfigSmsRegionConfigEl {
    pub fn build(self) -> IdentityPlatformConfigSmsRegionConfigEl {
        IdentityPlatformConfigSmsRegionConfigEl {
            allow_by_default: core::default::Default::default(),
            allowlist_only: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct IdentityPlatformConfigSmsRegionConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformConfigSmsRegionConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IdentityPlatformConfigSmsRegionConfigElRef {
        IdentityPlatformConfigSmsRegionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformConfigSmsRegionConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow_by_default` after provisioning.\n"]
    pub fn allow_by_default(&self) -> terrars::ListRef<IdentityPlatformConfigSmsRegionConfigElAllowByDefaultElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow_by_default", self.base))
    }

    #[doc = "Get a reference to the value of field `allowlist_only` after provisioning.\n"]
    pub fn allowlist_only(&self) -> terrars::ListRef<IdentityPlatformConfigSmsRegionConfigElAllowlistOnlyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowlist_only", self.base))
    }
}

#[derive(Serialize)]
pub struct IdentityPlatformConfigTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IdentityPlatformConfigTimeoutsEl {
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

impl terrars::ToListMappable for IdentityPlatformConfigTimeoutsEl {
    type O = terrars::BlockAssignable<IdentityPlatformConfigTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIdentityPlatformConfigTimeoutsEl {}

impl BuildIdentityPlatformConfigTimeoutsEl {
    pub fn build(self) -> IdentityPlatformConfigTimeoutsEl {
        IdentityPlatformConfigTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct IdentityPlatformConfigTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IdentityPlatformConfigTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IdentityPlatformConfigTimeoutsElRef {
        IdentityPlatformConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IdentityPlatformConfigTimeoutsElRef {
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
struct IdentityPlatformConfigDynamic {
    blocking_functions: std::option::Option<terrars::DynamicBlock<IdentityPlatformConfigBlockingFunctionsEl>>,
    quota: std::option::Option<terrars::DynamicBlock<IdentityPlatformConfigQuotaEl>>,
    sign_in: std::option::Option<terrars::DynamicBlock<IdentityPlatformConfigSignInEl>>,
    sms_region_config: std::option::Option<terrars::DynamicBlock<IdentityPlatformConfigSmsRegionConfigEl>>,
}
