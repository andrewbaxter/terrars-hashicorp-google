#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ApigeeAddonsConfigData_ {
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
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    org: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    addons_config: std::option::Option<std::vec::Vec<ApigeeAddonsConfig_AddonsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ApigeeAddonsConfig_Timeouts>,
    dynamic: ApigeeAddonsConfigDynamic,
}

struct ApigeeAddonsConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ApigeeAddonsConfigData_>,
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

    #[doc = "Set the field `addons_config`.\n"]
    pub fn set_addons_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApigeeAddonsConfig_AddonsConfig>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<ApigeeAddonsConfig_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
    pub fn addons_config(&self) -> terrars::ListRef<ApigeeAddonsConfig_AddonsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.addons_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeAddonsConfig_TimeoutsRef {
        ApigeeAddonsConfig_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
            data: std::cell::RefCell::new(ApigeeAddonsConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
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

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
    pub fn addons_config(&self) -> terrars::ListRef<ApigeeAddonsConfig_AddonsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.addons_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeAddonsConfig_TimeoutsRef {
        ApigeeAddonsConfig_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ApigeeAddonsConfigDynamic {
    addons_config: std::option::Option<terrars::DynamicBlock<ApigeeAddonsConfig_AddonsConfig>>,
}

#[derive(Serialize)]
pub struct ApigeeAddonsConfig_AddonsConfig_AdvancedApiOpsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl ApigeeAddonsConfig_AddonsConfig_AdvancedApiOpsConfig {
    #[doc = "Set the field `enabled`.\nFlag that specifies whether the Advanced API Ops add-on is enabled."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApigeeAddonsConfig_AddonsConfig_AdvancedApiOpsConfig {
    type O = terrars::BlockAssignable<ApigeeAddonsConfig_AddonsConfig_AdvancedApiOpsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeAddonsConfig_AddonsConfig_AdvancedApiOpsConfig {}

impl BuildApigeeAddonsConfig_AddonsConfig_AdvancedApiOpsConfig {
    pub fn build(self) -> ApigeeAddonsConfig_AddonsConfig_AdvancedApiOpsConfig {
        ApigeeAddonsConfig_AddonsConfig_AdvancedApiOpsConfig { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ApigeeAddonsConfig_AddonsConfig_AdvancedApiOpsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeAddonsConfig_AddonsConfig_AdvancedApiOpsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApigeeAddonsConfig_AddonsConfig_AdvancedApiOpsConfigRef {
        ApigeeAddonsConfig_AddonsConfig_AdvancedApiOpsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeAddonsConfig_AddonsConfig_AdvancedApiOpsConfigRef {
    #[allow(dead_code)]
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
pub struct ApigeeAddonsConfig_AddonsConfig_ApiSecurityConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl ApigeeAddonsConfig_AddonsConfig_ApiSecurityConfig {
    #[doc = "Set the field `enabled`.\nFlag that specifies whether the API security add-on is enabled."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApigeeAddonsConfig_AddonsConfig_ApiSecurityConfig {
    type O = terrars::BlockAssignable<ApigeeAddonsConfig_AddonsConfig_ApiSecurityConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeAddonsConfig_AddonsConfig_ApiSecurityConfig {}

impl BuildApigeeAddonsConfig_AddonsConfig_ApiSecurityConfig {
    pub fn build(self) -> ApigeeAddonsConfig_AddonsConfig_ApiSecurityConfig {
        ApigeeAddonsConfig_AddonsConfig_ApiSecurityConfig { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ApigeeAddonsConfig_AddonsConfig_ApiSecurityConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeAddonsConfig_AddonsConfig_ApiSecurityConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApigeeAddonsConfig_AddonsConfig_ApiSecurityConfigRef {
        ApigeeAddonsConfig_AddonsConfig_ApiSecurityConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeAddonsConfig_AddonsConfig_ApiSecurityConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nFlag that specifies whether the API security add-on is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expires_at` after provisioning.\nTime at which the API Security add-on expires in in milliseconds since epoch. If unspecified, the add-on will never expire."]
    pub fn expires_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expires_at", self.base))
    }
}

#[derive(Serialize)]
pub struct ApigeeAddonsConfig_AddonsConfig_ConnectorsPlatformConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl ApigeeAddonsConfig_AddonsConfig_ConnectorsPlatformConfig {
    #[doc = "Set the field `enabled`.\nFlag that specifies whether the Connectors Platform add-on is enabled."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApigeeAddonsConfig_AddonsConfig_ConnectorsPlatformConfig {
    type O = terrars::BlockAssignable<ApigeeAddonsConfig_AddonsConfig_ConnectorsPlatformConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeAddonsConfig_AddonsConfig_ConnectorsPlatformConfig {}

impl BuildApigeeAddonsConfig_AddonsConfig_ConnectorsPlatformConfig {
    pub fn build(self) -> ApigeeAddonsConfig_AddonsConfig_ConnectorsPlatformConfig {
        ApigeeAddonsConfig_AddonsConfig_ConnectorsPlatformConfig { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ApigeeAddonsConfig_AddonsConfig_ConnectorsPlatformConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeAddonsConfig_AddonsConfig_ConnectorsPlatformConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApigeeAddonsConfig_AddonsConfig_ConnectorsPlatformConfigRef {
        ApigeeAddonsConfig_AddonsConfig_ConnectorsPlatformConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeAddonsConfig_AddonsConfig_ConnectorsPlatformConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nFlag that specifies whether the Connectors Platform add-on is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expires_at` after provisioning.\nTime at which the Connectors Platform add-on expires in milliseconds since epoch. If unspecified, the add-on will never expire."]
    pub fn expires_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expires_at", self.base))
    }
}

#[derive(Serialize)]
pub struct ApigeeAddonsConfig_AddonsConfig_IntegrationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl ApigeeAddonsConfig_AddonsConfig_IntegrationConfig {
    #[doc = "Set the field `enabled`.\nFlag that specifies whether the Integration add-on is enabled."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApigeeAddonsConfig_AddonsConfig_IntegrationConfig {
    type O = terrars::BlockAssignable<ApigeeAddonsConfig_AddonsConfig_IntegrationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeAddonsConfig_AddonsConfig_IntegrationConfig {}

impl BuildApigeeAddonsConfig_AddonsConfig_IntegrationConfig {
    pub fn build(self) -> ApigeeAddonsConfig_AddonsConfig_IntegrationConfig {
        ApigeeAddonsConfig_AddonsConfig_IntegrationConfig { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ApigeeAddonsConfig_AddonsConfig_IntegrationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeAddonsConfig_AddonsConfig_IntegrationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApigeeAddonsConfig_AddonsConfig_IntegrationConfigRef {
        ApigeeAddonsConfig_AddonsConfig_IntegrationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeAddonsConfig_AddonsConfig_IntegrationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nFlag that specifies whether the Integration add-on is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ApigeeAddonsConfig_AddonsConfig_MonetizationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl ApigeeAddonsConfig_AddonsConfig_MonetizationConfig {
    #[doc = "Set the field `enabled`.\nFlag that specifies whether the Monetization add-on is enabled."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApigeeAddonsConfig_AddonsConfig_MonetizationConfig {
    type O = terrars::BlockAssignable<ApigeeAddonsConfig_AddonsConfig_MonetizationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeAddonsConfig_AddonsConfig_MonetizationConfig {}

impl BuildApigeeAddonsConfig_AddonsConfig_MonetizationConfig {
    pub fn build(self) -> ApigeeAddonsConfig_AddonsConfig_MonetizationConfig {
        ApigeeAddonsConfig_AddonsConfig_MonetizationConfig { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ApigeeAddonsConfig_AddonsConfig_MonetizationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeAddonsConfig_AddonsConfig_MonetizationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApigeeAddonsConfig_AddonsConfig_MonetizationConfigRef {
        ApigeeAddonsConfig_AddonsConfig_MonetizationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeAddonsConfig_AddonsConfig_MonetizationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nFlag that specifies whether the Monetization add-on is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApigeeAddonsConfig_AddonsConfigDynamic {
    advanced_api_ops_config: std
    ::option
    ::Option<terrars::DynamicBlock<ApigeeAddonsConfig_AddonsConfig_AdvancedApiOpsConfig>>,
    api_security_config: std
    ::option
    ::Option<terrars::DynamicBlock<ApigeeAddonsConfig_AddonsConfig_ApiSecurityConfig>>,
    connectors_platform_config: std
    ::option
    ::Option<terrars::DynamicBlock<ApigeeAddonsConfig_AddonsConfig_ConnectorsPlatformConfig>>,
    integration_config: std
    ::option
    ::Option<terrars::DynamicBlock<ApigeeAddonsConfig_AddonsConfig_IntegrationConfig>>,
    monetization_config: std
    ::option
    ::Option<terrars::DynamicBlock<ApigeeAddonsConfig_AddonsConfig_MonetizationConfig>>,
}

#[derive(Serialize)]
pub struct ApigeeAddonsConfig_AddonsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_api_ops_config: std::option::Option<std::vec::Vec<ApigeeAddonsConfig_AddonsConfig_AdvancedApiOpsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_security_config: std::option::Option<std::vec::Vec<ApigeeAddonsConfig_AddonsConfig_ApiSecurityConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connectors_platform_config: std
    ::option
    ::Option<std::vec::Vec<ApigeeAddonsConfig_AddonsConfig_ConnectorsPlatformConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    integration_config: std::option::Option<std::vec::Vec<ApigeeAddonsConfig_AddonsConfig_IntegrationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monetization_config: std::option::Option<std::vec::Vec<ApigeeAddonsConfig_AddonsConfig_MonetizationConfig>>,
    dynamic: ApigeeAddonsConfig_AddonsConfigDynamic,
}

impl ApigeeAddonsConfig_AddonsConfig {
    #[doc = "Set the field `advanced_api_ops_config`.\n"]
    pub fn set_advanced_api_ops_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApigeeAddonsConfig_AddonsConfig_AdvancedApiOpsConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ApigeeAddonsConfig_AddonsConfig_ApiSecurityConfig>>,
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
                    ::Into<terrars::BlockAssignable<ApigeeAddonsConfig_AddonsConfig_ConnectorsPlatformConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ApigeeAddonsConfig_AddonsConfig_IntegrationConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ApigeeAddonsConfig_AddonsConfig_MonetizationConfig>>,
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

impl terrars::ToListMappable for ApigeeAddonsConfig_AddonsConfig {
    type O = terrars::BlockAssignable<ApigeeAddonsConfig_AddonsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeAddonsConfig_AddonsConfig {}

impl BuildApigeeAddonsConfig_AddonsConfig {
    pub fn build(self) -> ApigeeAddonsConfig_AddonsConfig {
        ApigeeAddonsConfig_AddonsConfig {
            advanced_api_ops_config: core::default::Default::default(),
            api_security_config: core::default::Default::default(),
            connectors_platform_config: core::default::Default::default(),
            integration_config: core::default::Default::default(),
            monetization_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApigeeAddonsConfig_AddonsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeAddonsConfig_AddonsConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApigeeAddonsConfig_AddonsConfigRef {
        ApigeeAddonsConfig_AddonsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeAddonsConfig_AddonsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `advanced_api_ops_config` after provisioning.\n"]
    pub fn advanced_api_ops_config(&self) -> terrars::ListRef<ApigeeAddonsConfig_AddonsConfig_AdvancedApiOpsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_api_ops_config", self.base))
    }

    #[doc = "Get a reference to the value of field `api_security_config` after provisioning.\n"]
    pub fn api_security_config(&self) -> terrars::ListRef<ApigeeAddonsConfig_AddonsConfig_ApiSecurityConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_security_config", self.base))
    }

    #[doc = "Get a reference to the value of field `connectors_platform_config` after provisioning.\n"]
    pub fn connectors_platform_config(
        &self,
    ) -> terrars::ListRef<ApigeeAddonsConfig_AddonsConfig_ConnectorsPlatformConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connectors_platform_config", self.base))
    }

    #[doc = "Get a reference to the value of field `integration_config` after provisioning.\n"]
    pub fn integration_config(&self) -> terrars::ListRef<ApigeeAddonsConfig_AddonsConfig_IntegrationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.integration_config", self.base))
    }

    #[doc = "Get a reference to the value of field `monetization_config` after provisioning.\n"]
    pub fn monetization_config(&self) -> terrars::ListRef<ApigeeAddonsConfig_AddonsConfig_MonetizationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monetization_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ApigeeAddonsConfig_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApigeeAddonsConfig_Timeouts {
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

impl terrars::ToListMappable for ApigeeAddonsConfig_Timeouts {
    type O = terrars::BlockAssignable<ApigeeAddonsConfig_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeAddonsConfig_Timeouts {}

impl BuildApigeeAddonsConfig_Timeouts {
    pub fn build(self) -> ApigeeAddonsConfig_Timeouts {
        ApigeeAddonsConfig_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApigeeAddonsConfig_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeAddonsConfig_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApigeeAddonsConfig_TimeoutsRef {
        ApigeeAddonsConfig_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeAddonsConfig_TimeoutsRef {
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
