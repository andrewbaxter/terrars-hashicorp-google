use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ApigeeAddonsConfigData {
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
    org: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    addons_config: std::option::Option<std::vec::Vec<ApigeeAddonsConfigAddonsConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ApigeeAddonsConfigTimeoutsEl>,
    dynamic: ApigeeAddonsConfigDynamic,
}

struct ApigeeAddonsConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ApigeeAddonsConfigData>,
}

#[derive(Clone)]
pub struct ApigeeAddonsConfig(std::rc::Rc<ApigeeAddonsConfig_>);

impl ApigeeAddonsConfig {
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

    #[doc = "Set the field `addons_config`.\n"]
    pub fn set_addons_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApigeeAddonsConfigAddonsConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().addons_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.addons_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ApigeeAddonsConfigTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `org` after provisioning.\nName of the Apigee organization."]
    pub fn org(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.org", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `addons_config` after provisioning.\n"]
    pub fn addons_config(&self) -> terrars::ListRef<ApigeeAddonsConfigAddonsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.addons_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeAddonsConfigTimeoutsElRef {
        ApigeeAddonsConfigTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ApigeeAddonsConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ApigeeAddonsConfig { }

impl terrars::ToListMappable for ApigeeAddonsConfig {
    type O = terrars::ListRef<ApigeeAddonsConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ApigeeAddonsConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_apigee_addons_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApigeeAddonsConfig {
    pub tf_id: std::string::String,
    #[doc = "Name of the Apigee organization."]
    pub org: terrars::PrimField<std::string::String>,
}

impl BuildApigeeAddonsConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> ApigeeAddonsConfig {
        let out = ApigeeAddonsConfig(std::rc::Rc::new(ApigeeAddonsConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ApigeeAddonsConfigData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                org: self.org,
                addons_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApigeeAddonsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeAddonsConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ApigeeAddonsConfigRef {
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

    #[doc = "Get a reference to the value of field `org` after provisioning.\nName of the Apigee organization."]
    pub fn org(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.org", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `addons_config` after provisioning.\n"]
    pub fn addons_config(&self) -> terrars::ListRef<ApigeeAddonsConfigAddonsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.addons_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeAddonsConfigTimeoutsElRef {
        ApigeeAddonsConfigTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ApigeeAddonsConfigAddonsConfigElAdvancedApiOpsConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl ApigeeAddonsConfigAddonsConfigElAdvancedApiOpsConfigEl {
    #[doc = "Set the field `enabled`.\nFlag that specifies whether the Advanced API Ops add-on is enabled."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApigeeAddonsConfigAddonsConfigElAdvancedApiOpsConfigEl {
    type O = terrars::BlockAssignable<ApigeeAddonsConfigAddonsConfigElAdvancedApiOpsConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeAddonsConfigAddonsConfigElAdvancedApiOpsConfigEl {}

impl BuildApigeeAddonsConfigAddonsConfigElAdvancedApiOpsConfigEl {
    pub fn build(self) -> ApigeeAddonsConfigAddonsConfigElAdvancedApiOpsConfigEl {
        ApigeeAddonsConfigAddonsConfigElAdvancedApiOpsConfigEl { enabled: core::default::Default::default() }
    }
}

pub struct ApigeeAddonsConfigAddonsConfigElAdvancedApiOpsConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeAddonsConfigAddonsConfigElAdvancedApiOpsConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApigeeAddonsConfigAddonsConfigElAdvancedApiOpsConfigElRef {
        ApigeeAddonsConfigAddonsConfigElAdvancedApiOpsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeAddonsConfigAddonsConfigElAdvancedApiOpsConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nFlag that specifies whether the Advanced API Ops add-on is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ApigeeAddonsConfigAddonsConfigElApiSecurityConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl ApigeeAddonsConfigAddonsConfigElApiSecurityConfigEl {
    #[doc = "Set the field `enabled`.\nFlag that specifies whether the Advanced API Ops add-on is enabled."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApigeeAddonsConfigAddonsConfigElApiSecurityConfigEl {
    type O = terrars::BlockAssignable<ApigeeAddonsConfigAddonsConfigElApiSecurityConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeAddonsConfigAddonsConfigElApiSecurityConfigEl {}

impl BuildApigeeAddonsConfigAddonsConfigElApiSecurityConfigEl {
    pub fn build(self) -> ApigeeAddonsConfigAddonsConfigElApiSecurityConfigEl {
        ApigeeAddonsConfigAddonsConfigElApiSecurityConfigEl { enabled: core::default::Default::default() }
    }
}

pub struct ApigeeAddonsConfigAddonsConfigElApiSecurityConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeAddonsConfigAddonsConfigElApiSecurityConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApigeeAddonsConfigAddonsConfigElApiSecurityConfigElRef {
        ApigeeAddonsConfigAddonsConfigElApiSecurityConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeAddonsConfigAddonsConfigElApiSecurityConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nFlag that specifies whether the Advanced API Ops add-on is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expires_at` after provisioning.\nFlag that specifies whether the Advanced API Ops add-on is enabled."]
    pub fn expires_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expires_at", self.base))
    }
}

#[derive(Serialize)]
pub struct ApigeeAddonsConfigAddonsConfigElConnectorsPlatformConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl ApigeeAddonsConfigAddonsConfigElConnectorsPlatformConfigEl {
    #[doc = "Set the field `enabled`.\nFlag that specifies whether the Advanced API Ops add-on is enabled."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApigeeAddonsConfigAddonsConfigElConnectorsPlatformConfigEl {
    type O = terrars::BlockAssignable<ApigeeAddonsConfigAddonsConfigElConnectorsPlatformConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeAddonsConfigAddonsConfigElConnectorsPlatformConfigEl {}

impl BuildApigeeAddonsConfigAddonsConfigElConnectorsPlatformConfigEl {
    pub fn build(self) -> ApigeeAddonsConfigAddonsConfigElConnectorsPlatformConfigEl {
        ApigeeAddonsConfigAddonsConfigElConnectorsPlatformConfigEl { enabled: core::default::Default::default() }
    }
}

pub struct ApigeeAddonsConfigAddonsConfigElConnectorsPlatformConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeAddonsConfigAddonsConfigElConnectorsPlatformConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApigeeAddonsConfigAddonsConfigElConnectorsPlatformConfigElRef {
        ApigeeAddonsConfigAddonsConfigElConnectorsPlatformConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeAddonsConfigAddonsConfigElConnectorsPlatformConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nFlag that specifies whether the Advanced API Ops add-on is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expires_at` after provisioning.\nFlag that specifies whether the Advanced API Ops add-on is enabled."]
    pub fn expires_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expires_at", self.base))
    }
}

#[derive(Serialize)]
pub struct ApigeeAddonsConfigAddonsConfigElIntegrationConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl ApigeeAddonsConfigAddonsConfigElIntegrationConfigEl {
    #[doc = "Set the field `enabled`.\nFlag that specifies whether the Advanced API Ops add-on is enabled."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApigeeAddonsConfigAddonsConfigElIntegrationConfigEl {
    type O = terrars::BlockAssignable<ApigeeAddonsConfigAddonsConfigElIntegrationConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeAddonsConfigAddonsConfigElIntegrationConfigEl {}

impl BuildApigeeAddonsConfigAddonsConfigElIntegrationConfigEl {
    pub fn build(self) -> ApigeeAddonsConfigAddonsConfigElIntegrationConfigEl {
        ApigeeAddonsConfigAddonsConfigElIntegrationConfigEl { enabled: core::default::Default::default() }
    }
}

pub struct ApigeeAddonsConfigAddonsConfigElIntegrationConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeAddonsConfigAddonsConfigElIntegrationConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApigeeAddonsConfigAddonsConfigElIntegrationConfigElRef {
        ApigeeAddonsConfigAddonsConfigElIntegrationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeAddonsConfigAddonsConfigElIntegrationConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nFlag that specifies whether the Advanced API Ops add-on is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ApigeeAddonsConfigAddonsConfigElMonetizationConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl ApigeeAddonsConfigAddonsConfigElMonetizationConfigEl {
    #[doc = "Set the field `enabled`.\nFlag that specifies whether the Advanced API Ops add-on is enabled."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApigeeAddonsConfigAddonsConfigElMonetizationConfigEl {
    type O = terrars::BlockAssignable<ApigeeAddonsConfigAddonsConfigElMonetizationConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeAddonsConfigAddonsConfigElMonetizationConfigEl {}

impl BuildApigeeAddonsConfigAddonsConfigElMonetizationConfigEl {
    pub fn build(self) -> ApigeeAddonsConfigAddonsConfigElMonetizationConfigEl {
        ApigeeAddonsConfigAddonsConfigElMonetizationConfigEl { enabled: core::default::Default::default() }
    }
}

pub struct ApigeeAddonsConfigAddonsConfigElMonetizationConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeAddonsConfigAddonsConfigElMonetizationConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApigeeAddonsConfigAddonsConfigElMonetizationConfigElRef {
        ApigeeAddonsConfigAddonsConfigElMonetizationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeAddonsConfigAddonsConfigElMonetizationConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nFlag that specifies whether the Advanced API Ops add-on is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApigeeAddonsConfigAddonsConfigElDynamic {
    advanced_api_ops_config: std
    ::option
    ::Option<terrars::DynamicBlock<ApigeeAddonsConfigAddonsConfigElAdvancedApiOpsConfigEl>>,
    api_security_config: std
    ::option
    ::Option<terrars::DynamicBlock<ApigeeAddonsConfigAddonsConfigElApiSecurityConfigEl>>,
    connectors_platform_config: std
    ::option
    ::Option<terrars::DynamicBlock<ApigeeAddonsConfigAddonsConfigElConnectorsPlatformConfigEl>>,
    integration_config: std
    ::option
    ::Option<terrars::DynamicBlock<ApigeeAddonsConfigAddonsConfigElIntegrationConfigEl>>,
    monetization_config: std
    ::option
    ::Option<terrars::DynamicBlock<ApigeeAddonsConfigAddonsConfigElMonetizationConfigEl>>,
}

#[derive(Serialize)]
pub struct ApigeeAddonsConfigAddonsConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_api_ops_config: std::option::Option<std::vec::Vec<ApigeeAddonsConfigAddonsConfigElAdvancedApiOpsConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_security_config: std::option::Option<std::vec::Vec<ApigeeAddonsConfigAddonsConfigElApiSecurityConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connectors_platform_config: std
    ::option
    ::Option<std::vec::Vec<ApigeeAddonsConfigAddonsConfigElConnectorsPlatformConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    integration_config: std::option::Option<std::vec::Vec<ApigeeAddonsConfigAddonsConfigElIntegrationConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monetization_config: std::option::Option<std::vec::Vec<ApigeeAddonsConfigAddonsConfigElMonetizationConfigEl>>,
    dynamic: ApigeeAddonsConfigAddonsConfigElDynamic,
}

impl ApigeeAddonsConfigAddonsConfigEl {
    #[doc = "Set the field `advanced_api_ops_config`.\n"]
    pub fn set_advanced_api_ops_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApigeeAddonsConfigAddonsConfigElAdvancedApiOpsConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.advanced_api_ops_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.advanced_api_ops_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `api_security_config`.\n"]
    pub fn set_api_security_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApigeeAddonsConfigAddonsConfigElApiSecurityConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.api_security_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.api_security_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `connectors_platform_config`.\n"]
    pub fn set_connectors_platform_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ApigeeAddonsConfigAddonsConfigElConnectorsPlatformConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.connectors_platform_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.connectors_platform_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `integration_config`.\n"]
    pub fn set_integration_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApigeeAddonsConfigAddonsConfigElIntegrationConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.integration_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.integration_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `monetization_config`.\n"]
    pub fn set_monetization_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApigeeAddonsConfigAddonsConfigElMonetizationConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.monetization_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.monetization_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ApigeeAddonsConfigAddonsConfigEl {
    type O = terrars::BlockAssignable<ApigeeAddonsConfigAddonsConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeAddonsConfigAddonsConfigEl {}

impl BuildApigeeAddonsConfigAddonsConfigEl {
    pub fn build(self) -> ApigeeAddonsConfigAddonsConfigEl {
        ApigeeAddonsConfigAddonsConfigEl {
            advanced_api_ops_config: core::default::Default::default(),
            api_security_config: core::default::Default::default(),
            connectors_platform_config: core::default::Default::default(),
            integration_config: core::default::Default::default(),
            monetization_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ApigeeAddonsConfigAddonsConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeAddonsConfigAddonsConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApigeeAddonsConfigAddonsConfigElRef {
        ApigeeAddonsConfigAddonsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeAddonsConfigAddonsConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `advanced_api_ops_config` after provisioning.\n"]
    pub fn advanced_api_ops_config(&self) -> terrars::ListRef<ApigeeAddonsConfigAddonsConfigElAdvancedApiOpsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_api_ops_config", self.base))
    }

    #[doc = "Get a reference to the value of field `api_security_config` after provisioning.\n"]
    pub fn api_security_config(&self) -> terrars::ListRef<ApigeeAddonsConfigAddonsConfigElApiSecurityConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_security_config", self.base))
    }

    #[doc = "Get a reference to the value of field `connectors_platform_config` after provisioning.\n"]
    pub fn connectors_platform_config(
        &self,
    ) -> terrars::ListRef<ApigeeAddonsConfigAddonsConfigElConnectorsPlatformConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connectors_platform_config", self.base))
    }

    #[doc = "Get a reference to the value of field `integration_config` after provisioning.\n"]
    pub fn integration_config(&self) -> terrars::ListRef<ApigeeAddonsConfigAddonsConfigElIntegrationConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.integration_config", self.base))
    }

    #[doc = "Get a reference to the value of field `monetization_config` after provisioning.\n"]
    pub fn monetization_config(&self) -> terrars::ListRef<ApigeeAddonsConfigAddonsConfigElMonetizationConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monetization_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ApigeeAddonsConfigTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApigeeAddonsConfigTimeoutsEl {
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

impl terrars::ToListMappable for ApigeeAddonsConfigTimeoutsEl {
    type O = terrars::BlockAssignable<ApigeeAddonsConfigTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeAddonsConfigTimeoutsEl {}

impl BuildApigeeAddonsConfigTimeoutsEl {
    pub fn build(self) -> ApigeeAddonsConfigTimeoutsEl {
        ApigeeAddonsConfigTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ApigeeAddonsConfigTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeAddonsConfigTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApigeeAddonsConfigTimeoutsElRef {
        ApigeeAddonsConfigTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeAddonsConfigTimeoutsElRef {
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
struct ApigeeAddonsConfigDynamic {
    addons_config: std::option::Option<terrars::DynamicBlock<ApigeeAddonsConfigAddonsConfigEl>>,
}
