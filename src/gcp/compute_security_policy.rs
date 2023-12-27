use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeSecurityPolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    adaptive_protection_config: Option<Vec<ComputeSecurityPolicyAdaptiveProtectionConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    advanced_options_config: Option<Vec<ComputeSecurityPolicyAdvancedOptionsConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recaptcha_options_config: Option<Vec<ComputeSecurityPolicyRecaptchaOptionsConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule: Option<Vec<ComputeSecurityPolicyRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeSecurityPolicyTimeoutsEl>,
    dynamic: ComputeSecurityPolicyDynamic,
}

struct ComputeSecurityPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeSecurityPolicyData>,
}

#[derive(Clone)]
pub struct ComputeSecurityPolicy(Rc<ComputeSecurityPolicy_>);

impl ComputeSecurityPolicy {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `description`.\nAn optional description of this security policy. Max size is 2048."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\nThe project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\nThe type indicates the intended use of the security policy. CLOUD_ARMOR - Cloud Armor backend security policies can be configured to filter incoming HTTP requests targeting backend services. They filter requests before they hit the origin servers. CLOUD_ARMOR_EDGE - Cloud Armor edge security policies can be configured to filter incoming HTTP requests targeting backend services (including Cloud CDN-enabled) as well as backend buckets (Cloud Storage). They filter requests before the request is served from Google's cache."]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
        self
    }

    #[doc= "Set the field `adaptive_protection_config`.\n"]
    pub fn set_adaptive_protection_config(
        self,
        v: impl Into<BlockAssignable<ComputeSecurityPolicyAdaptiveProtectionConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().adaptive_protection_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.adaptive_protection_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `advanced_options_config`.\n"]
    pub fn set_advanced_options_config(
        self,
        v: impl Into<BlockAssignable<ComputeSecurityPolicyAdvancedOptionsConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().advanced_options_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.advanced_options_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `recaptcha_options_config`.\n"]
    pub fn set_recaptcha_options_config(
        self,
        v: impl Into<BlockAssignable<ComputeSecurityPolicyRecaptchaOptionsConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().recaptcha_options_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.recaptcha_options_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `rule`.\n"]
    pub fn set_rule(self, v: impl Into<BlockAssignable<ComputeSecurityPolicyRuleEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rule = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rule = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeSecurityPolicyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn optional description of this security policy. Max size is 2048."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint of this resource."]
    pub fn fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fingerprint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the security policy."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe type indicates the intended use of the security policy. CLOUD_ARMOR - Cloud Armor backend security policies can be configured to filter incoming HTTP requests targeting backend services. They filter requests before they hit the origin servers. CLOUD_ARMOR_EDGE - Cloud Armor edge security policies can be configured to filter incoming HTTP requests targeting backend services (including Cloud CDN-enabled) as well as backend buckets (Cloud Storage). They filter requests before the request is served from Google's cache."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `adaptive_protection_config` after provisioning.\n"]
    pub fn adaptive_protection_config(&self) -> ListRef<ComputeSecurityPolicyAdaptiveProtectionConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.adaptive_protection_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `advanced_options_config` after provisioning.\n"]
    pub fn advanced_options_config(&self) -> ListRef<ComputeSecurityPolicyAdvancedOptionsConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.advanced_options_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `recaptcha_options_config` after provisioning.\n"]
    pub fn recaptcha_options_config(&self) -> ListRef<ComputeSecurityPolicyRecaptchaOptionsConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.recaptcha_options_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeSecurityPolicyTimeoutsElRef {
        ComputeSecurityPolicyTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for ComputeSecurityPolicy {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ComputeSecurityPolicy { }

impl ToListMappable for ComputeSecurityPolicy {
    type O = ListRef<ComputeSecurityPolicyRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ComputeSecurityPolicy_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_security_policy".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeSecurityPolicy {
    pub tf_id: String,
    #[doc= "The name of the security policy."]
    pub name: PrimField<String>,
}

impl BuildComputeSecurityPolicy {
    pub fn build(self, stack: &mut Stack) -> ComputeSecurityPolicy {
        let out = ComputeSecurityPolicy(Rc::new(ComputeSecurityPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeSecurityPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
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
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeSecurityPolicyRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeSecurityPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeSecurityPolicyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn optional description of this security policy. Max size is 2048."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint of this resource."]
    pub fn fingerprint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.fingerprint", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the security policy."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\nThe project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\nThe URI of the created resource."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe type indicates the intended use of the security policy. CLOUD_ARMOR - Cloud Armor backend security policies can be configured to filter incoming HTTP requests targeting backend services. They filter requests before they hit the origin servers. CLOUD_ARMOR_EDGE - Cloud Armor edge security policies can be configured to filter incoming HTTP requests targeting backend services (including Cloud CDN-enabled) as well as backend buckets (Cloud Storage). They filter requests before the request is served from Google's cache."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `adaptive_protection_config` after provisioning.\n"]
    pub fn adaptive_protection_config(&self) -> ListRef<ComputeSecurityPolicyAdaptiveProtectionConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.adaptive_protection_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `advanced_options_config` after provisioning.\n"]
    pub fn advanced_options_config(&self) -> ListRef<ComputeSecurityPolicyAdvancedOptionsConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.advanced_options_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `recaptcha_options_config` after provisioning.\n"]
    pub fn recaptcha_options_config(&self) -> ListRef<ComputeSecurityPolicyRecaptchaOptionsConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.recaptcha_options_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeSecurityPolicyTimeoutsElRef {
        ComputeSecurityPolicyTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule_visibility: Option<PrimField<String>>,
}

impl ComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigEl {
    #[doc= "Set the field `enable`.\nIf set to true, enables CAAP for L7 DDoS detection."]
    pub fn set_enable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable = Some(v.into());
        self
    }

    #[doc= "Set the field `rule_visibility`.\nRule visibility. Supported values include: \"STANDARD\", \"PREMIUM\"."]
    pub fn set_rule_visibility(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rule_visibility = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigEl {
    type O = BlockAssignable<ComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for ComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElRef {
        ComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enable` after provisioning.\nIf set to true, enables CAAP for L7 DDoS detection."]
    pub fn enable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }

    #[doc= "Get a reference to the value of field `rule_visibility` after provisioning.\nRule visibility. Supported values include: \"STANDARD\", \"PREMIUM\"."]
    pub fn rule_visibility(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_visibility", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeSecurityPolicyAdaptiveProtectionConfigElDynamic {
    layer_7_ddos_defense_config: Option<
        DynamicBlock<ComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigEl>,
    >,
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicyAdaptiveProtectionConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    layer_7_ddos_defense_config: Option<Vec<ComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigEl>>,
    dynamic: ComputeSecurityPolicyAdaptiveProtectionConfigElDynamic,
}

impl ComputeSecurityPolicyAdaptiveProtectionConfigEl {
    #[doc= "Set the field `layer_7_ddos_defense_config`.\n"]
    pub fn set_layer_7_ddos_defense_config(
        mut self,
        v: impl Into<BlockAssignable<ComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.layer_7_ddos_defense_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.layer_7_ddos_defense_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeSecurityPolicyAdaptiveProtectionConfigEl {
    type O = BlockAssignable<ComputeSecurityPolicyAdaptiveProtectionConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeSecurityPolicyAdaptiveProtectionConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeSecurityPolicyAdaptiveProtectionConfigElRef {
    fn new(shared: StackShared, base: String) -> ComputeSecurityPolicyAdaptiveProtectionConfigElRef {
        ComputeSecurityPolicyAdaptiveProtectionConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicyAdaptiveProtectionConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `layer_7_ddos_defense_config` after provisioning.\n"]
    pub fn layer_7_ddos_defense_config(
        &self,
    ) -> ListRef<ComputeSecurityPolicyAdaptiveProtectionConfigElLayer7DdosDefenseConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.layer_7_ddos_defense_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl {
    content_types: SetField<PrimField<String>>,
}

impl ComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl { }

impl ToListMappable for ComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl {
    type O = BlockAssignable<ComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl {
    #[doc= "A list of custom Content-Type header values to apply the JSON parsing."]
    pub content_types: SetField<PrimField<String>>,
}

impl BuildComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl {
    pub fn build(self) -> ComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl {
        ComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl { content_types: self.content_types }
    }
}

pub struct ComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigElRef {
    fn new(shared: StackShared, base: String) -> ComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigElRef {
        ComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `content_types` after provisioning.\nA list of custom Content-Type header values to apply the JSON parsing."]
    pub fn content_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.content_types", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeSecurityPolicyAdvancedOptionsConfigElDynamic {
    json_custom_config: Option<DynamicBlock<ComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl>>,
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicyAdvancedOptionsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    json_parsing: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    json_custom_config: Option<Vec<ComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl>>,
    dynamic: ComputeSecurityPolicyAdvancedOptionsConfigElDynamic,
}

impl ComputeSecurityPolicyAdvancedOptionsConfigEl {
    #[doc= "Set the field `json_parsing`.\nJSON body parsing. Supported values include: \"DISABLED\", \"STANDARD\"."]
    pub fn set_json_parsing(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.json_parsing = Some(v.into());
        self
    }

    #[doc= "Set the field `log_level`.\nLogging level. Supported values include: \"NORMAL\", \"VERBOSE\"."]
    pub fn set_log_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_level = Some(v.into());
        self
    }

    #[doc= "Set the field `json_custom_config`.\n"]
    pub fn set_json_custom_config(
        mut self,
        v: impl Into<BlockAssignable<ComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.json_custom_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.json_custom_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeSecurityPolicyAdvancedOptionsConfigEl {
    type O = BlockAssignable<ComputeSecurityPolicyAdvancedOptionsConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeSecurityPolicyAdvancedOptionsConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeSecurityPolicyAdvancedOptionsConfigElRef {
    fn new(shared: StackShared, base: String) -> ComputeSecurityPolicyAdvancedOptionsConfigElRef {
        ComputeSecurityPolicyAdvancedOptionsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicyAdvancedOptionsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `json_parsing` after provisioning.\nJSON body parsing. Supported values include: \"DISABLED\", \"STANDARD\"."]
    pub fn json_parsing(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.json_parsing", self.base))
    }

    #[doc= "Get a reference to the value of field `log_level` after provisioning.\nLogging level. Supported values include: \"NORMAL\", \"VERBOSE\"."]
    pub fn log_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_level", self.base))
    }

    #[doc= "Get a reference to the value of field `json_custom_config` after provisioning.\n"]
    pub fn json_custom_config(&self) -> ListRef<ComputeSecurityPolicyAdvancedOptionsConfigElJsonCustomConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.json_custom_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicyRecaptchaOptionsConfigEl {
    redirect_site_key: PrimField<String>,
}

impl ComputeSecurityPolicyRecaptchaOptionsConfigEl { }

impl ToListMappable for ComputeSecurityPolicyRecaptchaOptionsConfigEl {
    type O = BlockAssignable<ComputeSecurityPolicyRecaptchaOptionsConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicyRecaptchaOptionsConfigEl {
    #[doc= "A field to supply a reCAPTCHA site key to be used for all the rules using the redirect action with the type of GOOGLE_RECAPTCHA under the security policy. The specified site key needs to be created from the reCAPTCHA API. The user is responsible for the validity of the specified site key. If not specified, a Google-managed site key is used."]
    pub redirect_site_key: PrimField<String>,
}

impl BuildComputeSecurityPolicyRecaptchaOptionsConfigEl {
    pub fn build(self) -> ComputeSecurityPolicyRecaptchaOptionsConfigEl {
        ComputeSecurityPolicyRecaptchaOptionsConfigEl { redirect_site_key: self.redirect_site_key }
    }
}

pub struct ComputeSecurityPolicyRecaptchaOptionsConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeSecurityPolicyRecaptchaOptionsConfigElRef {
    fn new(shared: StackShared, base: String) -> ComputeSecurityPolicyRecaptchaOptionsConfigElRef {
        ComputeSecurityPolicyRecaptchaOptionsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicyRecaptchaOptionsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `redirect_site_key` after provisioning.\nA field to supply a reCAPTCHA site key to be used for all the rules using the redirect action with the type of GOOGLE_RECAPTCHA under the security policy. The specified site key needs to be created from the reCAPTCHA API. The user is responsible for the validity of the specified site key. If not specified, a Google-managed site key is used."]
    pub fn redirect_site_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.redirect_site_key", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsEl {
    header_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_value: Option<PrimField<String>>,
}

impl ComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsEl {
    #[doc= "Set the field `header_value`.\nThe value to set the named header to."]
    pub fn set_header_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.header_value = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsEl {
    type O = BlockAssignable<ComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsEl {
    #[doc= "The name of the header to set."]
    pub header_name: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for ComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsElRef {
    fn new(shared: StackShared, base: String) -> ComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsElRef {
        ComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header to set."]
    pub fn header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }

    #[doc= "Get a reference to the value of field `header_value` after provisioning.\nThe value to set the named header to."]
    pub fn header_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeSecurityPolicyRuleElHeaderActionElDynamic {
    request_headers_to_adds: Option<DynamicBlock<ComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsEl>>,
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicyRuleElHeaderActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    request_headers_to_adds: Option<Vec<ComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsEl>>,
    dynamic: ComputeSecurityPolicyRuleElHeaderActionElDynamic,
}

impl ComputeSecurityPolicyRuleElHeaderActionEl {
    #[doc= "Set the field `request_headers_to_adds`.\n"]
    pub fn set_request_headers_to_adds(
        mut self,
        v: impl Into<BlockAssignable<ComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.request_headers_to_adds = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.request_headers_to_adds = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeSecurityPolicyRuleElHeaderActionEl {
    type O = BlockAssignable<ComputeSecurityPolicyRuleElHeaderActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeSecurityPolicyRuleElHeaderActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeSecurityPolicyRuleElHeaderActionElRef {
    fn new(shared: StackShared, base: String) -> ComputeSecurityPolicyRuleElHeaderActionElRef {
        ComputeSecurityPolicyRuleElHeaderActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicyRuleElHeaderActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `request_headers_to_adds` after provisioning.\n"]
    pub fn request_headers_to_adds(&self) -> ListRef<ComputeSecurityPolicyRuleElHeaderActionElRequestHeadersToAddsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.request_headers_to_adds", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicyRuleElMatchElConfigEl {
    src_ip_ranges: SetField<PrimField<String>>,
}

impl ComputeSecurityPolicyRuleElMatchElConfigEl { }

impl ToListMappable for ComputeSecurityPolicyRuleElMatchElConfigEl {
    type O = BlockAssignable<ComputeSecurityPolicyRuleElMatchElConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicyRuleElMatchElConfigEl {
    #[doc= "Set of IP addresses or ranges (IPV4 or IPV6) in CIDR notation to match against inbound traffic. There is a limit of 10 IP ranges per rule. A value of '*' matches all IPs (can be used to override the default behavior)."]
    pub src_ip_ranges: SetField<PrimField<String>>,
}

impl BuildComputeSecurityPolicyRuleElMatchElConfigEl {
    pub fn build(self) -> ComputeSecurityPolicyRuleElMatchElConfigEl {
        ComputeSecurityPolicyRuleElMatchElConfigEl { src_ip_ranges: self.src_ip_ranges }
    }
}

pub struct ComputeSecurityPolicyRuleElMatchElConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeSecurityPolicyRuleElMatchElConfigElRef {
    fn new(shared: StackShared, base: String) -> ComputeSecurityPolicyRuleElMatchElConfigElRef {
        ComputeSecurityPolicyRuleElMatchElConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicyRuleElMatchElConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `src_ip_ranges` after provisioning.\nSet of IP addresses or ranges (IPV4 or IPV6) in CIDR notation to match against inbound traffic. There is a limit of 10 IP ranges per rule. A value of '*' matches all IPs (can be used to override the default behavior)."]
    pub fn src_ip_ranges(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.src_ip_ranges", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicyRuleElMatchElExprEl {
    expression: PrimField<String>,
}

impl ComputeSecurityPolicyRuleElMatchElExprEl { }

impl ToListMappable for ComputeSecurityPolicyRuleElMatchElExprEl {
    type O = BlockAssignable<ComputeSecurityPolicyRuleElMatchElExprEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicyRuleElMatchElExprEl {
    #[doc= "Textual representation of an expression in Common Expression Language syntax. The application context of the containing message determines which well-known feature set of CEL is supported."]
    pub expression: PrimField<String>,
}

impl BuildComputeSecurityPolicyRuleElMatchElExprEl {
    pub fn build(self) -> ComputeSecurityPolicyRuleElMatchElExprEl {
        ComputeSecurityPolicyRuleElMatchElExprEl { expression: self.expression }
    }
}

pub struct ComputeSecurityPolicyRuleElMatchElExprElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeSecurityPolicyRuleElMatchElExprElRef {
    fn new(shared: StackShared, base: String) -> ComputeSecurityPolicyRuleElMatchElExprElRef {
        ComputeSecurityPolicyRuleElMatchElExprElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicyRuleElMatchElExprElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `expression` after provisioning.\nTextual representation of an expression in Common Expression Language syntax. The application context of the containing message determines which well-known feature set of CEL is supported."]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeSecurityPolicyRuleElMatchElDynamic {
    config: Option<DynamicBlock<ComputeSecurityPolicyRuleElMatchElConfigEl>>,
    expr: Option<DynamicBlock<ComputeSecurityPolicyRuleElMatchElExprEl>>,
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicyRuleElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    versioned_expr: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    config: Option<Vec<ComputeSecurityPolicyRuleElMatchElConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expr: Option<Vec<ComputeSecurityPolicyRuleElMatchElExprEl>>,
    dynamic: ComputeSecurityPolicyRuleElMatchElDynamic,
}

impl ComputeSecurityPolicyRuleElMatchEl {
    #[doc= "Set the field `versioned_expr`.\nPredefined rule expression. If this field is specified, config must also be specified. Available options:   SRC_IPS_V1: Must specify the corresponding src_ip_ranges field in config."]
    pub fn set_versioned_expr(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.versioned_expr = Some(v.into());
        self
    }

    #[doc= "Set the field `config`.\n"]
    pub fn set_config(mut self, v: impl Into<BlockAssignable<ComputeSecurityPolicyRuleElMatchElConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `expr`.\n"]
    pub fn set_expr(mut self, v: impl Into<BlockAssignable<ComputeSecurityPolicyRuleElMatchElExprEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.expr = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.expr = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeSecurityPolicyRuleElMatchEl {
    type O = BlockAssignable<ComputeSecurityPolicyRuleElMatchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeSecurityPolicyRuleElMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeSecurityPolicyRuleElMatchElRef {
    fn new(shared: StackShared, base: String) -> ComputeSecurityPolicyRuleElMatchElRef {
        ComputeSecurityPolicyRuleElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicyRuleElMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `versioned_expr` after provisioning.\nPredefined rule expression. If this field is specified, config must also be specified. Available options:   SRC_IPS_V1: Must specify the corresponding src_ip_ranges field in config."]
    pub fn versioned_expr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.versioned_expr", self.base))
    }

    #[doc= "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> ListRef<ComputeSecurityPolicyRuleElMatchElConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.config", self.base))
    }

    #[doc= "Get a reference to the value of field `expr` after provisioning.\n"]
    pub fn expr(&self) -> ListRef<ComputeSecurityPolicyRuleElMatchElExprElRef> {
        ListRef::new(self.shared().clone(), format!("{}.expr", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdEl {
    count: PrimField<f64>,
    interval_sec: PrimField<f64>,
}

impl ComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdEl { }

impl ToListMappable for ComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdEl {
    type O = BlockAssignable<ComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdEl {
    #[doc= "Number of HTTP(S) requests for calculating the threshold."]
    pub count: PrimField<f64>,
    #[doc= "Interval over which the threshold is computed."]
    pub interval_sec: PrimField<f64>,
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
    shared: StackShared,
    base: String,
}

impl Ref for ComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdElRef {
    fn new(shared: StackShared, base: String) -> ComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdElRef {
        ComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `count` after provisioning.\nNumber of HTTP(S) requests for calculating the threshold."]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc= "Get a reference to the value of field `interval_sec` after provisioning.\nInterval over which the threshold is computed."]
    pub fn interval_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval_sec", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl ComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsEl {
    #[doc= "Set the field `target`.\nTarget for the redirect action. This is required if the type is EXTERNAL_302 and cannot be specified for GOOGLE_RECAPTCHA."]
    pub fn set_target(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsEl {
    type O = BlockAssignable<ComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsEl {
    #[doc= "Type of the redirect action."]
    pub type_: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for ComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsElRef {
        ComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `target` after provisioning.\nTarget for the redirect action. This is required if the type is EXTERNAL_302 and cannot be specified for GOOGLE_RECAPTCHA."]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nType of the redirect action."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdEl {
    count: PrimField<f64>,
    interval_sec: PrimField<f64>,
}

impl ComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdEl { }

impl ToListMappable for ComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdEl {
    type O = BlockAssignable<ComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdEl {
    #[doc= "Number of HTTP(S) requests for calculating the threshold."]
    pub count: PrimField<f64>,
    #[doc= "Interval over which the threshold is computed."]
    pub interval_sec: PrimField<f64>,
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
    shared: StackShared,
    base: String,
}

impl Ref for ComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdElRef {
        ComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `count` after provisioning.\nNumber of HTTP(S) requests for calculating the threshold."]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc= "Get a reference to the value of field `interval_sec` after provisioning.\nInterval over which the threshold is computed."]
    pub fn interval_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval_sec", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeSecurityPolicyRuleElRateLimitOptionsElDynamic {
    ban_threshold: Option<DynamicBlock<ComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdEl>>,
    exceed_redirect_options: Option<
        DynamicBlock<ComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsEl>,
    >,
    rate_limit_threshold: Option<DynamicBlock<ComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdEl>>,
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicyRuleElRateLimitOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ban_duration_sec: Option<PrimField<f64>>,
    conform_action: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce_on_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enforce_on_key_name: Option<PrimField<String>>,
    exceed_action: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ban_threshold: Option<Vec<ComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exceed_redirect_options: Option<Vec<ComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rate_limit_threshold: Option<Vec<ComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdEl>>,
    dynamic: ComputeSecurityPolicyRuleElRateLimitOptionsElDynamic,
}

impl ComputeSecurityPolicyRuleElRateLimitOptionsEl {
    #[doc= "Set the field `ban_duration_sec`.\nCan only be specified if the action for the rule is \"rate_based_ban\". If specified, determines the time (in seconds) the traffic will continue to be banned by the rate limit after the rate falls below the threshold."]
    pub fn set_ban_duration_sec(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.ban_duration_sec = Some(v.into());
        self
    }

    #[doc= "Set the field `enforce_on_key`.\nDetermines the key to enforce the rateLimitThreshold on"]
    pub fn set_enforce_on_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.enforce_on_key = Some(v.into());
        self
    }

    #[doc= "Set the field `enforce_on_key_name`.\nRate limit key name applicable only for the following key types: HTTP_HEADER -- Name of the HTTP header whose value is taken as the key value. HTTP_COOKIE -- Name of the HTTP cookie whose value is taken as the key value."]
    pub fn set_enforce_on_key_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.enforce_on_key_name = Some(v.into());
        self
    }

    #[doc= "Set the field `ban_threshold`.\n"]
    pub fn set_ban_threshold(
        mut self,
        v: impl Into<BlockAssignable<ComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ban_threshold = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ban_threshold = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `exceed_redirect_options`.\n"]
    pub fn set_exceed_redirect_options(
        mut self,
        v: impl Into<BlockAssignable<ComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.exceed_redirect_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.exceed_redirect_options = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `rate_limit_threshold`.\n"]
    pub fn set_rate_limit_threshold(
        mut self,
        v: impl Into<BlockAssignable<ComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rate_limit_threshold = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rate_limit_threshold = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeSecurityPolicyRuleElRateLimitOptionsEl {
    type O = BlockAssignable<ComputeSecurityPolicyRuleElRateLimitOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicyRuleElRateLimitOptionsEl {
    #[doc= "Action to take for requests that are under the configured rate limit threshold. Valid option is \"allow\" only."]
    pub conform_action: PrimField<String>,
    #[doc= "Action to take for requests that are above the configured rate limit threshold, to either deny with a specified HTTP response code, or redirect to a different endpoint. Valid options are \"deny()\" where valid values for status are 403, 404, 429, and 502, and \"redirect\" where the redirect parameters come from exceedRedirectOptions below."]
    pub exceed_action: PrimField<String>,
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
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeSecurityPolicyRuleElRateLimitOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeSecurityPolicyRuleElRateLimitOptionsElRef {
    fn new(shared: StackShared, base: String) -> ComputeSecurityPolicyRuleElRateLimitOptionsElRef {
        ComputeSecurityPolicyRuleElRateLimitOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicyRuleElRateLimitOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `ban_duration_sec` after provisioning.\nCan only be specified if the action for the rule is \"rate_based_ban\". If specified, determines the time (in seconds) the traffic will continue to be banned by the rate limit after the rate falls below the threshold."]
    pub fn ban_duration_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ban_duration_sec", self.base))
    }

    #[doc= "Get a reference to the value of field `conform_action` after provisioning.\nAction to take for requests that are under the configured rate limit threshold. Valid option is \"allow\" only."]
    pub fn conform_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.conform_action", self.base))
    }

    #[doc= "Get a reference to the value of field `enforce_on_key` after provisioning.\nDetermines the key to enforce the rateLimitThreshold on"]
    pub fn enforce_on_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.enforce_on_key", self.base))
    }

    #[doc= "Get a reference to the value of field `enforce_on_key_name` after provisioning.\nRate limit key name applicable only for the following key types: HTTP_HEADER -- Name of the HTTP header whose value is taken as the key value. HTTP_COOKIE -- Name of the HTTP cookie whose value is taken as the key value."]
    pub fn enforce_on_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.enforce_on_key_name", self.base))
    }

    #[doc= "Get a reference to the value of field `exceed_action` after provisioning.\nAction to take for requests that are above the configured rate limit threshold, to either deny with a specified HTTP response code, or redirect to a different endpoint. Valid options are \"deny()\" where valid values for status are 403, 404, 429, and 502, and \"redirect\" where the redirect parameters come from exceedRedirectOptions below."]
    pub fn exceed_action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exceed_action", self.base))
    }

    #[doc= "Get a reference to the value of field `ban_threshold` after provisioning.\n"]
    pub fn ban_threshold(&self) -> ListRef<ComputeSecurityPolicyRuleElRateLimitOptionsElBanThresholdElRef> {
        ListRef::new(self.shared().clone(), format!("{}.ban_threshold", self.base))
    }

    #[doc= "Get a reference to the value of field `exceed_redirect_options` after provisioning.\n"]
    pub fn exceed_redirect_options(
        &self,
    ) -> ListRef<ComputeSecurityPolicyRuleElRateLimitOptionsElExceedRedirectOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.exceed_redirect_options", self.base))
    }

    #[doc= "Get a reference to the value of field `rate_limit_threshold` after provisioning.\n"]
    pub fn rate_limit_threshold(&self) -> ListRef<ComputeSecurityPolicyRuleElRateLimitOptionsElRateLimitThresholdElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rate_limit_threshold", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicyRuleElRedirectOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl ComputeSecurityPolicyRuleElRedirectOptionsEl {
    #[doc= "Set the field `target`.\nTarget for the redirect action. This is required if the type is EXTERNAL_302 and cannot be specified for GOOGLE_RECAPTCHA."]
    pub fn set_target(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeSecurityPolicyRuleElRedirectOptionsEl {
    type O = BlockAssignable<ComputeSecurityPolicyRuleElRedirectOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicyRuleElRedirectOptionsEl {
    #[doc= "Type of the redirect action. Available options: EXTERNAL_302: Must specify the corresponding target field in config. GOOGLE_RECAPTCHA: Cannot specify target field in config."]
    pub type_: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for ComputeSecurityPolicyRuleElRedirectOptionsElRef {
    fn new(shared: StackShared, base: String) -> ComputeSecurityPolicyRuleElRedirectOptionsElRef {
        ComputeSecurityPolicyRuleElRedirectOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicyRuleElRedirectOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `target` after provisioning.\nTarget for the redirect action. This is required if the type is EXTERNAL_302 and cannot be specified for GOOGLE_RECAPTCHA."]
    pub fn target(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nType of the redirect action. Available options: EXTERNAL_302: Must specify the corresponding target field in config. GOOGLE_RECAPTCHA: Cannot specify target field in config."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeSecurityPolicyRuleElDynamic {
    header_action: Option<DynamicBlock<ComputeSecurityPolicyRuleElHeaderActionEl>>,
    match_: Option<DynamicBlock<ComputeSecurityPolicyRuleElMatchEl>>,
    rate_limit_options: Option<DynamicBlock<ComputeSecurityPolicyRuleElRateLimitOptionsEl>>,
    redirect_options: Option<DynamicBlock<ComputeSecurityPolicyRuleElRedirectOptionsEl>>,
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicyRuleEl {
    action: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preview: Option<PrimField<bool>>,
    priority: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header_action: Option<Vec<ComputeSecurityPolicyRuleElHeaderActionEl>>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<Vec<ComputeSecurityPolicyRuleElMatchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rate_limit_options: Option<Vec<ComputeSecurityPolicyRuleElRateLimitOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redirect_options: Option<Vec<ComputeSecurityPolicyRuleElRedirectOptionsEl>>,
    dynamic: ComputeSecurityPolicyRuleElDynamic,
}

impl ComputeSecurityPolicyRuleEl {
    #[doc= "Set the field `description`.\nAn optional description of this rule. Max size is 64."]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc= "Set the field `preview`.\nWhen set to true, the action specified above is not enforced. Stackdriver logs for requests that trigger a preview action are annotated as such."]
    pub fn set_preview(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.preview = Some(v.into());
        self
    }

    #[doc= "Set the field `header_action`.\n"]
    pub fn set_header_action(
        mut self,
        v: impl Into<BlockAssignable<ComputeSecurityPolicyRuleElHeaderActionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.header_action = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.header_action = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `match_`.\n"]
    pub fn set_match(mut self, v: impl Into<BlockAssignable<ComputeSecurityPolicyRuleElMatchEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.match_ = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.match_ = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `rate_limit_options`.\n"]
    pub fn set_rate_limit_options(
        mut self,
        v: impl Into<BlockAssignable<ComputeSecurityPolicyRuleElRateLimitOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rate_limit_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rate_limit_options = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `redirect_options`.\n"]
    pub fn set_redirect_options(
        mut self,
        v: impl Into<BlockAssignable<ComputeSecurityPolicyRuleElRedirectOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.redirect_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.redirect_options = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeSecurityPolicyRuleEl {
    type O = BlockAssignable<ComputeSecurityPolicyRuleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeSecurityPolicyRuleEl {
    #[doc= "Action to take when match matches the request."]
    pub action: PrimField<String>,
    #[doc= "An unique positive integer indicating the priority of evaluation for a rule. Rules are evaluated from highest priority (lowest numerically) to lowest priority (highest numerically) in order."]
    pub priority: PrimField<f64>,
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
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeSecurityPolicyRuleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeSecurityPolicyRuleElRef {
    fn new(shared: StackShared, base: String) -> ComputeSecurityPolicyRuleElRef {
        ComputeSecurityPolicyRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicyRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `action` after provisioning.\nAction to take when match matches the request."]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn optional description of this rule. Max size is 64."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc= "Get a reference to the value of field `preview` after provisioning.\nWhen set to true, the action specified above is not enforced. Stackdriver logs for requests that trigger a preview action are annotated as such."]
    pub fn preview(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.preview", self.base))
    }

    #[doc= "Get a reference to the value of field `priority` after provisioning.\nAn unique positive integer indicating the priority of evaluation for a rule. Rules are evaluated from highest priority (lowest numerically) to lowest priority (highest numerically) in order."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc= "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(&self) -> ListRef<ComputeSecurityPolicyRuleElHeaderActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.header_action", self.base))
    }

    #[doc= "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<ComputeSecurityPolicyRuleElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc= "Get a reference to the value of field `rate_limit_options` after provisioning.\n"]
    pub fn rate_limit_options(&self) -> ListRef<ComputeSecurityPolicyRuleElRateLimitOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rate_limit_options", self.base))
    }

    #[doc= "Get a reference to the value of field `redirect_options` after provisioning.\n"]
    pub fn redirect_options(&self) -> ListRef<ComputeSecurityPolicyRuleElRedirectOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.redirect_options", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeSecurityPolicyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl ComputeSecurityPolicyTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc= "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc= "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeSecurityPolicyTimeoutsEl {
    type O = BlockAssignable<ComputeSecurityPolicyTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for ComputeSecurityPolicyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeSecurityPolicyTimeoutsElRef {
        ComputeSecurityPolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeSecurityPolicyTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc= "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc= "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeSecurityPolicyDynamic {
    adaptive_protection_config: Option<DynamicBlock<ComputeSecurityPolicyAdaptiveProtectionConfigEl>>,
    advanced_options_config: Option<DynamicBlock<ComputeSecurityPolicyAdvancedOptionsConfigEl>>,
    recaptcha_options_config: Option<DynamicBlock<ComputeSecurityPolicyRecaptchaOptionsConfigEl>>,
    rule: Option<DynamicBlock<ComputeSecurityPolicyRuleEl>>,
}
