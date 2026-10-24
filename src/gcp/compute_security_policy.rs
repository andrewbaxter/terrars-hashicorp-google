use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeSecurityPolicyData {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    adaptive_protection_config: std::option::Option<std::vec::Vec<ComputeSecurityPolicyAdaptiveProtectionConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_options_config: std::option::Option<std::vec::Vec<ComputeSecurityPolicyAdvancedOptionsConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recaptcha_options_config: std::option::Option<std::vec::Vec<ComputeSecurityPolicyRecaptchaOptionsConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<std::vec::Vec<ComputeSecurityPolicyRuleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeSecurityPolicyTimeoutsEl>,
    dynamic: ComputeSecurityPolicyDynamic,
}

struct ComputeSecurityPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeSecurityPolicyData>,
}

#[derive(Clone)]
pub struct ComputeSecurityPolicy(std::rc::Rc<ComputeSecurityPolicy_>);

impl ComputeSecurityPolicy {
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

    #[doc = "Set the field `description`.\nAn optional description of this security policy. Max size is 2048."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project`.\nThe project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nThe type indicates the intended use of the security policy. CLOUD_ARMOR - Cloud Armor backend security policies can be configured to filter incoming HTTP requests targeting backend services. They filter requests before they hit the origin servers. CLOUD_ARMOR_EDGE - Cloud Armor edge security policies can be configured to filter incoming HTTP requests targeting backend services (including Cloud CDN-enabled) as well as backend buckets (Cloud Storage). They filter requests before the request is served from Google's cache."]
    pub fn set_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `adaptive_protection_config`.\n"]
    pub fn set_adaptive_protection_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeSecurityPolicyAdaptiveProtectionConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().adaptive_protection_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.adaptive_protection_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `advanced_options_config`.\n"]
    pub fn set_advanced_options_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeSecurityPolicyAdvancedOptionsConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().advanced_options_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.advanced_options_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `recaptcha_options_config`.\n"]
    pub fn set_recaptcha_options_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeSecurityPolicyRecaptchaOptionsConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().recaptcha_options_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.recaptcha_options_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(self, v: impl std::convert::Into<terrars::BlockAssignable<ComputeSecurityPolicyRuleEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeSecurityPolicyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this security policy. Max size is 2048."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint of this resource."]
    pub fn fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fingerprint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the security policy."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type indicates the intended use of the security policy. CLOUD_ARMOR - Cloud Armor backend security policies can be configured to filter incoming HTTP requests targeting backend services. They filter requests before they hit the origin servers. CLOUD_ARMOR_EDGE - Cloud Armor edge security policies can be configured to filter incoming HTTP requests targeting backend services (including Cloud CDN-enabled) as well as backend buckets (Cloud Storage). They filter requests before the request is served from Google's cache."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `adaptive_protection_config` after provisioning.\n"]
    pub fn adaptive_protection_config(&self) -> terrars::ListRef<ComputeSecurityPolicyAdaptiveProtectionConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.adaptive_protection_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `advanced_options_config` after provisioning.\n"]
    pub fn advanced_options_config(&self) -> terrars::ListRef<ComputeSecurityPolicyAdvancedOptionsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_options_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `recaptcha_options_config` after provisioning.\n"]
    pub fn recaptcha_options_config(&self) -> terrars::ListRef<ComputeSecurityPolicyRecaptchaOptionsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recaptcha_options_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeSecurityPolicyTimeoutsElRef {
        ComputeSecurityPolicyTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ComputeSecurityPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputeSecurityPolicy { }

impl terrars::ToListMappable for ComputeSecurityPolicy {
    type O = terrars::ListRef<ComputeSecurityPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputeSecurityPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_compute_security_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeSecurityPolicy {
    pub tf_id: std::string::String,
    #[doc = "The name of the security policy."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeSecurityPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputeSecurityPolicy {
        let out = ComputeSecurityPolicy(std::rc::Rc::new(ComputeSecurityPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputeSecurityPolicyData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                type_: core::default::Default::default(),
                adaptive_protection_config: core::default::Default::default(),
                advanced_options_config: core::default::Default::default(),
                recaptcha_options_config: core::default::Default::default(),
                rule: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeSecurityPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeSecurityPolicyRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this security policy. Max size is 2048."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint of this resource."]
    pub fn fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fingerprint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the security policy."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type indicates the intended use of the security policy. CLOUD_ARMOR - Cloud Armor backend security policies can be configured to filter incoming HTTP requests targeting backend services. They filter requests before they hit the origin servers. CLOUD_ARMOR_EDGE - Cloud Armor edge security policies can be configured to filter incoming HTTP requests targeting backend services (including Cloud CDN-enabled) as well as backend buckets (Cloud Storage). They filter requests before the request is served from Google's cache."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `adaptive_protection_config` after provisioning.\n"]
    pub fn adaptive_protection_config(&self) -> terrars::ListRef<ComputeSecurityPolicyAdaptiveProtectionConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.adaptive_protection_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `advanced_options_config` after provisioning.\n"]
    pub fn advanced_options_config(&self) -> terrars::ListRef<ComputeSecurityPolicyAdvancedOptionsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_options_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `recaptcha_options_config` after provisioning.\n"]
    pub fn recaptcha_options_config(&self) -> terrars::ListRef<ComputeSecurityPolicyRecaptchaOptionsConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recaptcha_options_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeSecurityPolicyTimeoutsElRef {
        ComputeSecurityPolicyTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_visibility: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigEl {
    #[doc = "Set the field `enable`.\nIf set to true, enables CAAP for L7 DDoS detection."]
    pub fn set_enable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule_visibility`.\nRule visibility. Supported values include: \"STANDARD\", \"PREMIUM\"."]
    pub fn set_rule_visibility(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rule_visibility = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigEl {
    type O = terrars::BlockAssignable<ComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigEl {}

impl BuildComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigEl {
    pub fn build(self) -> ComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigEl {
        ComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigEl {
            enable: core::default::Default::default(),
            rule_visibility: core::default::Default::default(),
        }
    }
}

pub struct ComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElRef {
        ComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable` after provisioning.\nIf set to true, enables CAAP for L7 DDoS detection."]
    pub fn enable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rule_visibility` after provisioning.\nRule visibility. Supported values include: \"STANDARD\", \"PREMIUM\"."]
    pub fn rule_visibility(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_visibility", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeSecurityPolicyAdaptiveProtectionConfigElDynamic {
    layer_7_ddos_defense_config: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigEl>>,
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicyAdaptiveProtectionConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    layer_7_ddos_defense_config: std
    ::option
    ::Option<std::vec::Vec<ComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigEl>>,
    dynamic: ComputeSecurityPolicyAdaptiveProtectionConfigElDynamic,
}

impl ComputeSecurityPolicyAdaptiveProtectionConfigEl {
    #[doc = "Set the field `layer_7_ddos_defense_config`.\n"]
    pub fn set_layer_7_ddos_defense_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.layer_7_ddos_defense_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.layer_7_ddos_defense_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeSecurityPolicyAdaptiveProtectionConfigEl {
    type O = terrars::BlockAssignable<ComputeSecurityPolicyAdaptiveProtectionConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicyAdaptiveProtectionConfigEl {}

impl BuildComputeSecurityPolicyAdaptiveProtectionConfigEl {
    pub fn build(self) -> ComputeSecurityPolicyAdaptiveProtectionConfigEl {
        ComputeSecurityPolicyAdaptiveProtectionConfigEl {
            layer_7_ddos_defense_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeSecurityPolicyAdaptiveProtectionConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicyAdaptiveProtectionConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeSecurityPolicyAdaptiveProtectionConfigElRef {
        ComputeSecurityPolicyAdaptiveProtectionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicyAdaptiveProtectionConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `layer_7_ddos_defense_config` after provisioning.\n"]
    pub fn layer_7_ddos_defense_config(
        &self,
    ) -> terrars::ListRef<ComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.layer_7_ddos_defense_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl {
    content_types: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl ComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl { }

impl terrars::ToListMappable for ComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl {
    type O = terrars::BlockAssignable<ComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl {
    #[doc = "A list of custom Content-Type header values to apply the JSON parsing."]
    pub content_types: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl {
    pub fn build(self) -> ComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl {
        ComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl { content_types: self.content_types }
    }
}

pub struct ComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigElRef {
        ComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `content_types` after provisioning.\nA list of custom Content-Type header values to apply the JSON parsing."]
    pub fn content_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.content_types", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeSecurityPolicyAdvancedOptionsConfigElDynamic {
    json_custom_config: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl>>,
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicyAdvancedOptionsConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json_parsing: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json_custom_config: std
    ::option
    ::Option<std::vec::Vec<ComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl>>,
    dynamic: ComputeSecurityPolicyAdvancedOptionsConfigElDynamic,
}

impl ComputeSecurityPolicyAdvancedOptionsConfigEl {
    #[doc = "Set the field `json_parsing`.\nJSON body parsing. Supported values include: \"DISABLED\", \"STANDARD\"."]
    pub fn set_json_parsing(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.json_parsing = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_level`.\nLogging level. Supported values include: \"NORMAL\", \"VERBOSE\"."]
    pub fn set_log_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `json_custom_config`.\n"]
    pub fn set_json_custom_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.json_custom_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.json_custom_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeSecurityPolicyAdvancedOptionsConfigEl {
    type O = terrars::BlockAssignable<ComputeSecurityPolicyAdvancedOptionsConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicyAdvancedOptionsConfigEl {}

impl BuildComputeSecurityPolicyAdvancedOptionsConfigEl {
    pub fn build(self) -> ComputeSecurityPolicyAdvancedOptionsConfigEl {
        ComputeSecurityPolicyAdvancedOptionsConfigEl {
            json_parsing: core::default::Default::default(),
            log_level: core::default::Default::default(),
            json_custom_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeSecurityPolicyAdvancedOptionsConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicyAdvancedOptionsConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeSecurityPolicyAdvancedOptionsConfigElRef {
        ComputeSecurityPolicyAdvancedOptionsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicyAdvancedOptionsConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `json_parsing` after provisioning.\nJSON body parsing. Supported values include: \"DISABLED\", \"STANDARD\"."]
    pub fn json_parsing(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.json_parsing", self.base))
    }

    #[doc =
        "Get a reference to the value of field `log_level` after provisioning.\nLogging level. Supported values include: \"NORMAL\", \"VERBOSE\"."]
    pub fn log_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_level", self.base))
    }

    #[doc = "Get a reference to the value of field `json_custom_config` after provisioning.\n"]
    pub fn json_custom_config(
        &self,
    ) -> terrars::ListRef<ComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.json_custom_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicyRecaptchaOptionsConfigEl {
    redirect_site_key: terrars::PrimField<std::string::String>,
}

impl ComputeSecurityPolicyRecaptchaOptionsConfigEl { }

impl terrars::ToListMappable for ComputeSecurityPolicyRecaptchaOptionsConfigEl {
    type O = terrars::BlockAssignable<ComputeSecurityPolicyRecaptchaOptionsConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicyRecaptchaOptionsConfigEl {
    #[doc =
        "A field to supply a reCAPTCHA site key to be used for all the rules using the redirect action with the type of GOOGLE_RECAPTCHA under the security policy. The specified site key needs to be created from the reCAPTCHA API. The user is responsible for the validity of the specified site key. If not specified, a Google-managed site key is used."]
    pub redirect_site_key: terrars::PrimField<std::string::String>,
}

impl BuildComputeSecurityPolicyRecaptchaOptionsConfigEl {
    pub fn build(self) -> ComputeSecurityPolicyRecaptchaOptionsConfigEl {
        ComputeSecurityPolicyRecaptchaOptionsConfigEl { redirect_site_key: self.redirect_site_key }
    }
}

pub struct ComputeSecurityPolicyRecaptchaOptionsConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicyRecaptchaOptionsConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeSecurityPolicyRecaptchaOptionsConfigElRef {
        ComputeSecurityPolicyRecaptchaOptionsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicyRecaptchaOptionsConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `redirect_site_key` after provisioning.\nA field to supply a reCAPTCHA site key to be used for all the rules using the redirect action with the type of GOOGLE_RECAPTCHA under the security policy. The specified site key needs to be created from the reCAPTCHA API. The user is responsible for the validity of the specified site key. If not specified, a Google-managed site key is used."]
    pub fn redirect_site_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redirect_site_key", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsEl {
    header_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsEl {
    #[doc = "Set the field `header_value`.\nThe value to set the named header to."]
    pub fn set_header_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.header_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsEl {
    type O = terrars::BlockAssignable<ComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsEl {
    #[doc = "The name of the header to set."]
    pub header_name: terrars::PrimField<std::string::String>,
}

impl BuildComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsEl {
    pub fn build(self) -> ComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsEl {
        ComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsEl {
            header_name: self.header_name,
            header_value: core::default::Default::default(),
        }
    }
}

pub struct ComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsElRef {
        ComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header to set."]
    pub fn header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `header_value` after provisioning.\nThe value to set the named header to."]
    pub fn header_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeSecurityPolicyRuleElHeaderActionElDynamic {
    request_headers_to_adds: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsEl>>,
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicyRuleElHeaderActionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_headers_to_adds: std
    ::option
    ::Option<std::vec::Vec<ComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsEl>>,
    dynamic: ComputeSecurityPolicyRuleElHeaderActionElDynamic,
}

impl ComputeSecurityPolicyRuleElHeaderActionEl {
    #[doc = "Set the field `request_headers_to_adds`.\n"]
    pub fn set_request_headers_to_adds(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.request_headers_to_adds = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.request_headers_to_adds = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeSecurityPolicyRuleElHeaderActionEl {
    type O = terrars::BlockAssignable<ComputeSecurityPolicyRuleElHeaderActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicyRuleElHeaderActionEl {}

impl BuildComputeSecurityPolicyRuleElHeaderActionEl {
    pub fn build(self) -> ComputeSecurityPolicyRuleElHeaderActionEl {
        ComputeSecurityPolicyRuleElHeaderActionEl {
            request_headers_to_adds: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeSecurityPolicyRuleElHeaderActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicyRuleElHeaderActionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeSecurityPolicyRuleElHeaderActionElRef {
        ComputeSecurityPolicyRuleElHeaderActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicyRuleElHeaderActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `request_headers_to_adds` after provisioning.\n"]
    pub fn request_headers_to_adds(
        &self,
    ) -> terrars::ListRef<ComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_headers_to_adds", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicyRuleElMatchElConfigEl {
    src_ip_ranges: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl ComputeSecurityPolicyRuleElMatchElConfigEl { }

impl terrars::ToListMappable for ComputeSecurityPolicyRuleElMatchElConfigEl {
    type O = terrars::BlockAssignable<ComputeSecurityPolicyRuleElMatchElConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicyRuleElMatchElConfigEl {
    #[doc =
        "Set of IP addresses or ranges (IPV4 or IPV6) in CIDR notation to match against inbound traffic. There is a limit of 10 IP ranges per rule. A value of '*' matches all IPs (can be used to override the default behavior)."]
    pub src_ip_ranges: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildComputeSecurityPolicyRuleElMatchElConfigEl {
    pub fn build(self) -> ComputeSecurityPolicyRuleElMatchElConfigEl {
        ComputeSecurityPolicyRuleElMatchElConfigEl { src_ip_ranges: self.src_ip_ranges }
    }
}

pub struct ComputeSecurityPolicyRuleElMatchElConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicyRuleElMatchElConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeSecurityPolicyRuleElMatchElConfigElRef {
        ComputeSecurityPolicyRuleElMatchElConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicyRuleElMatchElConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `src_ip_ranges` after provisioning.\nSet of IP addresses or ranges (IPV4 or IPV6) in CIDR notation to match against inbound traffic. There is a limit of 10 IP ranges per rule. A value of '*' matches all IPs (can be used to override the default behavior)."]
    pub fn src_ip_ranges(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.src_ip_ranges", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicyRuleElMatchElExprEl {
    expression: terrars::PrimField<std::string::String>,
}

impl ComputeSecurityPolicyRuleElMatchElExprEl { }

impl terrars::ToListMappable for ComputeSecurityPolicyRuleElMatchElExprEl {
    type O = terrars::BlockAssignable<ComputeSecurityPolicyRuleElMatchElExprEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicyRuleElMatchElExprEl {
    #[doc =
        "Textual representation of an expression in Common Expression Language syntax. The application context of the containing message determines which well-known feature set of CEL is supported."]
    pub expression: terrars::PrimField<std::string::String>,
}

impl BuildComputeSecurityPolicyRuleElMatchElExprEl {
    pub fn build(self) -> ComputeSecurityPolicyRuleElMatchElExprEl {
        ComputeSecurityPolicyRuleElMatchElExprEl { expression: self.expression }
    }
}

pub struct ComputeSecurityPolicyRuleElMatchElExprElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicyRuleElMatchElExprElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeSecurityPolicyRuleElMatchElExprElRef {
        ComputeSecurityPolicyRuleElMatchElExprElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicyRuleElMatchElExprElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nTextual representation of an expression in Common Expression Language syntax. The application context of the containing message determines which well-known feature set of CEL is supported."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeSecurityPolicyRuleElMatchElDynamic {
    config: std::option::Option<terrars::DynamicBlock<ComputeSecurityPolicyRuleElMatchElConfigEl>>,
    expr: std::option::Option<terrars::DynamicBlock<ComputeSecurityPolicyRuleElMatchElExprEl>>,
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicyRuleElMatchEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    versioned_expr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config: std::option::Option<std::vec::Vec<ComputeSecurityPolicyRuleElMatchElConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expr: std::option::Option<std::vec::Vec<ComputeSecurityPolicyRuleElMatchElExprEl>>,
    dynamic: ComputeSecurityPolicyRuleElMatchElDynamic,
}

impl ComputeSecurityPolicyRuleElMatchEl {
    #[doc =
        "Set the field `versioned_expr`.\nPredefined rule expression. If this field is specified, config must also be specified. Available options:   SRC_IPS_V1: Must specify the corresponding src_ip_ranges field in config."]
    pub fn set_versioned_expr(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.versioned_expr = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `config`.\n"]
    pub fn set_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeSecurityPolicyRuleElMatchElConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `expr`.\n"]
    pub fn set_expr(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeSecurityPolicyRuleElMatchElExprEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.expr = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.expr = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeSecurityPolicyRuleElMatchEl {
    type O = terrars::BlockAssignable<ComputeSecurityPolicyRuleElMatchEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicyRuleElMatchEl {}

impl BuildComputeSecurityPolicyRuleElMatchEl {
    pub fn build(self) -> ComputeSecurityPolicyRuleElMatchEl {
        ComputeSecurityPolicyRuleElMatchEl {
            versioned_expr: core::default::Default::default(),
            config: core::default::Default::default(),
            expr: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeSecurityPolicyRuleElMatchElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicyRuleElMatchElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeSecurityPolicyRuleElMatchElRef {
        ComputeSecurityPolicyRuleElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicyRuleElMatchElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `versioned_expr` after provisioning.\nPredefined rule expression. If this field is specified, config must also be specified. Available options:   SRC_IPS_V1: Must specify the corresponding src_ip_ranges field in config."]
    pub fn versioned_expr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.versioned_expr", self.base))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<ComputeSecurityPolicyRuleElMatchElConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.base))
    }

    #[doc = "Get a reference to the value of field `expr` after provisioning.\n"]
    pub fn expr(&self) -> terrars::ListRef<ComputeSecurityPolicyRuleElMatchElExprElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expr", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdEl {
    count: terrars::PrimField<f64>,
    interval_sec: terrars::PrimField<f64>,
}

impl ComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdEl { }

impl terrars::ToListMappable for ComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdEl {
    type O = terrars::BlockAssignable<ComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdEl {
    #[doc = "Number of HTTP(S) requests for calculating the threshold."]
    pub count: terrars::PrimField<f64>,
    #[doc = "Interval over which the threshold is computed."]
    pub interval_sec: terrars::PrimField<f64>,
}

impl BuildComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdEl {
    pub fn build(self) -> ComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdEl {
        ComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdEl {
            count: self.count,
            interval_sec: self.interval_sec,
        }
    }
}

pub struct ComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdElRef {
        ComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `count` after provisioning.\nNumber of HTTP(S) requests for calculating the threshold."]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `interval_sec` after provisioning.\nInterval over which the threshold is computed."]
    pub fn interval_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval_sec", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl ComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsEl {
    #[doc =
        "Set the field `target`.\nTarget for the redirect action. This is required if the type is EXTERNAL_302 and cannot be specified for GOOGLE_RECAPTCHA."]
    pub fn set_target(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsEl {
    type O = terrars::BlockAssignable<ComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsEl {
    #[doc = "Type of the redirect action."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsEl {
    pub fn build(self) -> ComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsEl {
        ComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsEl {
            target: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct ComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsElRef {
        ComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `target` after provisioning.\nTarget for the redirect action. This is required if the type is EXTERNAL_302 and cannot be specified for GOOGLE_RECAPTCHA."]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nType of the redirect action."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdEl {
    count: terrars::PrimField<f64>,
    interval_sec: terrars::PrimField<f64>,
}

impl ComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdEl { }

impl terrars::ToListMappable for ComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdEl {
    type O = terrars::BlockAssignable<ComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdEl {
    #[doc = "Number of HTTP(S) requests for calculating the threshold."]
    pub count: terrars::PrimField<f64>,
    #[doc = "Interval over which the threshold is computed."]
    pub interval_sec: terrars::PrimField<f64>,
}

impl BuildComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdEl {
    pub fn build(self) -> ComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdEl {
        ComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdEl {
            count: self.count,
            interval_sec: self.interval_sec,
        }
    }
}

pub struct ComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdElRef {
        ComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `count` after provisioning.\nNumber of HTTP(S) requests for calculating the threshold."]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `interval_sec` after provisioning.\nInterval over which the threshold is computed."]
    pub fn interval_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval_sec", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeSecurityPolicyRuleElRateLimitOptionsElDynamic {
    ban_threshold: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdEl>>,
    exceed_redirect_options: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsEl>>,
    rate_limit_threshold: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdEl>>,
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicyRuleElRateLimitOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ban_duration_sec: std::option::Option<terrars::PrimField<f64>>,
    conform_action: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce_on_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce_on_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    exceed_action: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ban_threshold: std::option::Option<std::vec::Vec<ComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exceed_redirect_options: std
    ::option
    ::Option<std::vec::Vec<ComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rate_limit_threshold: std
    ::option
    ::Option<std::vec::Vec<ComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdEl>>,
    dynamic: ComputeSecurityPolicyRuleElRateLimitOptionsElDynamic,
}

impl ComputeSecurityPolicyRuleElRateLimitOptionsEl {
    #[doc =
        "Set the field `ban_duration_sec`.\nCan only be specified if the action for the rule is \"rate_based_ban\". If specified, determines the time (in seconds) the traffic will continue to be banned by the rate limit after the rate falls below the threshold."]
    pub fn set_ban_duration_sec(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ban_duration_sec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enforce_on_key`.\nDetermines the key to enforce the rateLimitThreshold on"]
    pub fn set_enforce_on_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.enforce_on_key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enforce_on_key_name`.\nRate limit key name applicable only for the following key types: HTTP_HEADER -- Name of the HTTP header whose value is taken as the key value. HTTP_COOKIE -- Name of the HTTP cookie whose value is taken as the key value."]
    pub fn set_enforce_on_key_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.enforce_on_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ban_threshold`.\n"]
    pub fn set_ban_threshold(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ban_threshold = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ban_threshold = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `exceed_redirect_options`.\n"]
    pub fn set_exceed_redirect_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.exceed_redirect_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.exceed_redirect_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rate_limit_threshold`.\n"]
    pub fn set_rate_limit_threshold(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rate_limit_threshold = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rate_limit_threshold = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeSecurityPolicyRuleElRateLimitOptionsEl {
    type O = terrars::BlockAssignable<ComputeSecurityPolicyRuleElRateLimitOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicyRuleElRateLimitOptionsEl {
    #[doc =
        "Action to take for requests that are under the configured rate limit threshold. Valid option is \"allow\" only."]
    pub conform_action: terrars::PrimField<std::string::String>,
    #[doc =
        "Action to take for requests that are above the configured rate limit threshold, to either deny with a specified HTTP response code, or redirect to a different endpoint. Valid options are \"deny()\" where valid values for status are 403, 404, 429, and 502, and \"redirect\" where the redirect parameters come from exceedRedirectOptions below."]
    pub exceed_action: terrars::PrimField<std::string::String>,
}

impl BuildComputeSecurityPolicyRuleElRateLimitOptionsEl {
    pub fn build(self) -> ComputeSecurityPolicyRuleElRateLimitOptionsEl {
        ComputeSecurityPolicyRuleElRateLimitOptionsEl {
            ban_duration_sec: core::default::Default::default(),
            conform_action: self.conform_action,
            enforce_on_key: core::default::Default::default(),
            enforce_on_key_name: core::default::Default::default(),
            exceed_action: self.exceed_action,
            ban_threshold: core::default::Default::default(),
            exceed_redirect_options: core::default::Default::default(),
            rate_limit_threshold: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeSecurityPolicyRuleElRateLimitOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicyRuleElRateLimitOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeSecurityPolicyRuleElRateLimitOptionsElRef {
        ComputeSecurityPolicyRuleElRateLimitOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicyRuleElRateLimitOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ban_duration_sec` after provisioning.\nCan only be specified if the action for the rule is \"rate_based_ban\". If specified, determines the time (in seconds) the traffic will continue to be banned by the rate limit after the rate falls below the threshold."]
    pub fn ban_duration_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ban_duration_sec", self.base))
    }

    #[doc =
        "Get a reference to the value of field `conform_action` after provisioning.\nAction to take for requests that are under the configured rate limit threshold. Valid option is \"allow\" only."]
    pub fn conform_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.conform_action", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enforce_on_key` after provisioning.\nDetermines the key to enforce the rateLimitThreshold on"]
    pub fn enforce_on_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforce_on_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enforce_on_key_name` after provisioning.\nRate limit key name applicable only for the following key types: HTTP_HEADER -- Name of the HTTP header whose value is taken as the key value. HTTP_COOKIE -- Name of the HTTP cookie whose value is taken as the key value."]
    pub fn enforce_on_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforce_on_key_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exceed_action` after provisioning.\nAction to take for requests that are above the configured rate limit threshold, to either deny with a specified HTTP response code, or redirect to a different endpoint. Valid options are \"deny()\" where valid values for status are 403, 404, 429, and 502, and \"redirect\" where the redirect parameters come from exceedRedirectOptions below."]
    pub fn exceed_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exceed_action", self.base))
    }

    #[doc = "Get a reference to the value of field `ban_threshold` after provisioning.\n"]
    pub fn ban_threshold(&self) -> terrars::ListRef<ComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ban_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `exceed_redirect_options` after provisioning.\n"]
    pub fn exceed_redirect_options(
        &self,
    ) -> terrars::ListRef<ComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exceed_redirect_options", self.base))
    }

    #[doc = "Get a reference to the value of field `rate_limit_threshold` after provisioning.\n"]
    pub fn rate_limit_threshold(
        &self,
    ) -> terrars::ListRef<ComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rate_limit_threshold", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicyRuleElRedirectOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl ComputeSecurityPolicyRuleElRedirectOptionsEl {
    #[doc =
        "Set the field `target`.\nTarget for the redirect action. This is required if the type is EXTERNAL_302 and cannot be specified for GOOGLE_RECAPTCHA."]
    pub fn set_target(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeSecurityPolicyRuleElRedirectOptionsEl {
    type O = terrars::BlockAssignable<ComputeSecurityPolicyRuleElRedirectOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicyRuleElRedirectOptionsEl {
    #[doc =
        "Type of the redirect action. Available options: EXTERNAL_302: Must specify the corresponding target field in config. GOOGLE_RECAPTCHA: Cannot specify target field in config."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildComputeSecurityPolicyRuleElRedirectOptionsEl {
    pub fn build(self) -> ComputeSecurityPolicyRuleElRedirectOptionsEl {
        ComputeSecurityPolicyRuleElRedirectOptionsEl {
            target: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct ComputeSecurityPolicyRuleElRedirectOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicyRuleElRedirectOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeSecurityPolicyRuleElRedirectOptionsElRef {
        ComputeSecurityPolicyRuleElRedirectOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicyRuleElRedirectOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `target` after provisioning.\nTarget for the redirect action. This is required if the type is EXTERNAL_302 and cannot be specified for GOOGLE_RECAPTCHA."]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nType of the redirect action. Available options: EXTERNAL_302: Must specify the corresponding target field in config. GOOGLE_RECAPTCHA: Cannot specify target field in config."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeSecurityPolicyRuleElDynamic {
    header_action: std::option::Option<terrars::DynamicBlock<ComputeSecurityPolicyRuleElHeaderActionEl>>,
    match_: std::option::Option<terrars::DynamicBlock<ComputeSecurityPolicyRuleElMatchEl>>,
    rate_limit_options: std::option::Option<terrars::DynamicBlock<ComputeSecurityPolicyRuleElRateLimitOptionsEl>>,
    redirect_options: std::option::Option<terrars::DynamicBlock<ComputeSecurityPolicyRuleElRedirectOptionsEl>>,
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicyRuleEl {
    action: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preview: std::option::Option<terrars::PrimField<bool>>,
    priority: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_action: std::option::Option<std::vec::Vec<ComputeSecurityPolicyRuleElHeaderActionEl>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<std::vec::Vec<ComputeSecurityPolicyRuleElMatchEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rate_limit_options: std::option::Option<std::vec::Vec<ComputeSecurityPolicyRuleElRateLimitOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect_options: std::option::Option<std::vec::Vec<ComputeSecurityPolicyRuleElRedirectOptionsEl>>,
    dynamic: ComputeSecurityPolicyRuleElDynamic,
}

impl ComputeSecurityPolicyRuleEl {
    #[doc = "Set the field `description`.\nAn optional description of this rule. Max size is 64."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `preview`.\nWhen set to true, the action specified above is not enforced. Stackdriver logs for requests that trigger a preview action are annotated as such."]
    pub fn set_preview(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.preview = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `header_action`.\n"]
    pub fn set_header_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeSecurityPolicyRuleElHeaderActionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.header_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.header_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeSecurityPolicyRuleElMatchEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.match_ = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.match_ = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rate_limit_options`.\n"]
    pub fn set_rate_limit_options(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeSecurityPolicyRuleElRateLimitOptionsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rate_limit_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rate_limit_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `redirect_options`.\n"]
    pub fn set_redirect_options(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeSecurityPolicyRuleElRedirectOptionsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.redirect_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.redirect_options = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeSecurityPolicyRuleEl {
    type O = terrars::BlockAssignable<ComputeSecurityPolicyRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicyRuleEl {
    #[doc = "Action to take when match matches the request."]
    pub action: terrars::PrimField<std::string::String>,
    #[doc =
        "An unique positive integer indicating the priority of evaluation for a rule. Rules are evaluated from highest priority (lowest numerically) to lowest priority (highest numerically) in order."]
    pub priority: terrars::PrimField<f64>,
}

impl BuildComputeSecurityPolicyRuleEl {
    pub fn build(self) -> ComputeSecurityPolicyRuleEl {
        ComputeSecurityPolicyRuleEl {
            action: self.action,
            description: core::default::Default::default(),
            preview: core::default::Default::default(),
            priority: self.priority,
            header_action: core::default::Default::default(),
            match_: core::default::Default::default(),
            rate_limit_options: core::default::Default::default(),
            redirect_options: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeSecurityPolicyRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicyRuleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeSecurityPolicyRuleElRef {
        ComputeSecurityPolicyRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicyRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nAction to take when match matches the request."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this rule. Max size is 64."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `preview` after provisioning.\nWhen set to true, the action specified above is not enforced. Stackdriver logs for requests that trigger a preview action are annotated as such."]
    pub fn preview(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preview", self.base))
    }

    #[doc =
        "Get a reference to the value of field `priority` after provisioning.\nAn unique positive integer indicating the priority of evaluation for a rule. Rules are evaluated from highest priority (lowest numerically) to lowest priority (highest numerically) in order."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(&self) -> terrars::ListRef<ComputeSecurityPolicyRuleElHeaderActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header_action", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<ComputeSecurityPolicyRuleElMatchElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `rate_limit_options` after provisioning.\n"]
    pub fn rate_limit_options(&self) -> terrars::ListRef<ComputeSecurityPolicyRuleElRateLimitOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rate_limit_options", self.base))
    }

    #[doc = "Get a reference to the value of field `redirect_options` after provisioning.\n"]
    pub fn redirect_options(&self) -> terrars::ListRef<ComputeSecurityPolicyRuleElRedirectOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redirect_options", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicyTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeSecurityPolicyTimeoutsEl {
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

impl terrars::ToListMappable for ComputeSecurityPolicyTimeoutsEl {
    type O = terrars::BlockAssignable<ComputeSecurityPolicyTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicyTimeoutsEl {}

impl BuildComputeSecurityPolicyTimeoutsEl {
    pub fn build(self) -> ComputeSecurityPolicyTimeoutsEl {
        ComputeSecurityPolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ComputeSecurityPolicyTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeSecurityPolicyTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeSecurityPolicyTimeoutsElRef {
        ComputeSecurityPolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicyTimeoutsElRef {
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
struct ComputeSecurityPolicyDynamic {
    adaptive_protection_config: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeSecurityPolicyAdaptiveProtectionConfigEl>>,
    advanced_options_config: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeSecurityPolicyAdvancedOptionsConfigEl>>,
    recaptcha_options_config: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeSecurityPolicyRecaptchaOptionsConfigEl>>,
    rule: std::option::Option<terrars::DynamicBlock<ComputeSecurityPolicyRuleEl>>,
}
