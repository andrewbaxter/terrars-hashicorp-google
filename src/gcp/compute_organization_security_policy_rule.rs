#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeOrganizationSecurityPolicyRuleData_ {
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
    action: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    policy_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preview: std::option::Option<terrars::PrimField<bool>>,
    priority: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_action: std::option::Option<std::vec::Vec<ComputeOrganizationSecurityPolicyRule_HeaderAction>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<std::vec::Vec<ComputeOrganizationSecurityPolicyRule_Match>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preconfigured_waf_config: std
    ::option
    ::Option<std::vec::Vec<ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect_options: std::option::Option<std::vec::Vec<ComputeOrganizationSecurityPolicyRule_RedirectOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeOrganizationSecurityPolicyRule_Timeouts>,
    dynamic: ComputeOrganizationSecurityPolicyRuleDynamic,
}

struct ComputeOrganizationSecurityPolicyRule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeOrganizationSecurityPolicyRuleData_>,
}

#[derive(Clone)]
pub struct ComputeOrganizationSecurityPolicyRule(std::rc::Rc<ComputeOrganizationSecurityPolicyRule_>);

impl ComputeOrganizationSecurityPolicyRule {
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

    #[doc = "Set the field `description`.\nA description of the rule."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preview`.\nIf set to true, the specified action is not enforced."]
    pub fn set_preview(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().preview = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `header_action`.\n"]
    pub fn set_header_action(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeOrganizationSecurityPolicyRule_HeaderAction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().header_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.header_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeOrganizationSecurityPolicyRule_Match>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().match_ = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.match_ = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `preconfigured_waf_config`.\n"]
    pub fn set_preconfigured_waf_config(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().preconfigured_waf_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.preconfigured_waf_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `redirect_options`.\n"]
    pub fn set_redirect_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeOrganizationSecurityPolicyRule_RedirectOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().redirect_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.redirect_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeOrganizationSecurityPolicyRule_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nThe Action to perform when the client connection triggers the rule. Valid actions are:\n\"allow\": allow access to target.\n\"deny\": deny access to target.\n\"goto_next\": forward the request to the next hierarchical policy for evaluation.\n\"redirect\": redirect to a different target. Parameters for this action can be configured via redirectOptions. Only EXTERNAL_302 redirect type is supported for organization security policies."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of the rule."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `policy_id` after provisioning.\nThe ID of the OrganizationSecurityPolicy this rule applies to."]
    pub fn policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `preview` after provisioning.\nIf set to true, the specified action is not enforced."]
    pub fn preview(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preview", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `priority` after provisioning.\nAn integer indicating the priority of a rule in the list. The priority must be a value\nbetween 0 and 2147483647. Rules are evaluated from highest to lowest priority where 0 is the\nhighest priority and 2147483647 is the lowest prority."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(&self) -> terrars::ListRef<ComputeOrganizationSecurityPolicyRule_HeaderActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<ComputeOrganizationSecurityPolicyRule_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preconfigured_waf_config` after provisioning.\n"]
    pub fn preconfigured_waf_config(
        &self,
    ) -> terrars::ListRef<ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.preconfigured_waf_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `redirect_options` after provisioning.\n"]
    pub fn redirect_options(&self) -> terrars::ListRef<ComputeOrganizationSecurityPolicyRule_RedirectOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redirect_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeOrganizationSecurityPolicyRule_TimeoutsRef {
        ComputeOrganizationSecurityPolicyRule_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for ComputeOrganizationSecurityPolicyRule {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputeOrganizationSecurityPolicyRule { }

impl terrars::ToListMappable for ComputeOrganizationSecurityPolicyRule {
    type O = terrars::ListRef<ComputeOrganizationSecurityPolicyRuleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputeOrganizationSecurityPolicyRule_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_compute_organization_security_policy_rule".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeOrganizationSecurityPolicyRule {
    pub tf_id: std::string::String,
    #[doc =
        "The Action to perform when the client connection triggers the rule. Valid actions are:\n\"allow\": allow access to target.\n\"deny\": deny access to target.\n\"goto_next\": forward the request to the next hierarchical policy for evaluation.\n\"redirect\": redirect to a different target. Parameters for this action can be configured via redirectOptions. Only EXTERNAL_302 redirect type is supported for organization security policies."]
    pub action: terrars::PrimField<std::string::String>,
    #[doc = "The ID of the OrganizationSecurityPolicy this rule applies to."]
    pub policy_id: terrars::PrimField<std::string::String>,
    #[doc =
        "An integer indicating the priority of a rule in the list. The priority must be a value\nbetween 0 and 2147483647. Rules are evaluated from highest to lowest priority where 0 is the\nhighest priority and 2147483647 is the lowest prority."]
    pub priority: terrars::PrimField<f64>,
}

impl BuildComputeOrganizationSecurityPolicyRule {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputeOrganizationSecurityPolicyRule {
        let out = ComputeOrganizationSecurityPolicyRule(std::rc::Rc::new(ComputeOrganizationSecurityPolicyRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputeOrganizationSecurityPolicyRuleData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                action: self.action,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                policy_id: self.policy_id,
                preview: core::default::Default::default(),
                priority: self.priority,
                header_action: core::default::Default::default(),
                match_: core::default::Default::default(),
                preconfigured_waf_config: core::default::Default::default(),
                redirect_options: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeOrganizationSecurityPolicyRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeOrganizationSecurityPolicyRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeOrganizationSecurityPolicyRuleRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nThe Action to perform when the client connection triggers the rule. Valid actions are:\n\"allow\": allow access to target.\n\"deny\": deny access to target.\n\"goto_next\": forward the request to the next hierarchical policy for evaluation.\n\"redirect\": redirect to a different target. Parameters for this action can be configured via redirectOptions. Only EXTERNAL_302 redirect type is supported for organization security policies."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of the rule."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `policy_id` after provisioning.\nThe ID of the OrganizationSecurityPolicy this rule applies to."]
    pub fn policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `preview` after provisioning.\nIf set to true, the specified action is not enforced."]
    pub fn preview(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preview", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `priority` after provisioning.\nAn integer indicating the priority of a rule in the list. The priority must be a value\nbetween 0 and 2147483647. Rules are evaluated from highest to lowest priority where 0 is the\nhighest priority and 2147483647 is the lowest prority."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(&self) -> terrars::ListRef<ComputeOrganizationSecurityPolicyRule_HeaderActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<ComputeOrganizationSecurityPolicyRule_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `preconfigured_waf_config` after provisioning.\n"]
    pub fn preconfigured_waf_config(
        &self,
    ) -> terrars::ListRef<ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.preconfigured_waf_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `redirect_options` after provisioning.\n"]
    pub fn redirect_options(&self) -> terrars::ListRef<ComputeOrganizationSecurityPolicyRule_RedirectOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redirect_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeOrganizationSecurityPolicyRule_TimeoutsRef {
        ComputeOrganizationSecurityPolicyRule_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct ComputeOrganizationSecurityPolicyRuleDynamic {
    header_action: std::option::Option<terrars::DynamicBlock<ComputeOrganizationSecurityPolicyRule_HeaderAction>>,
    match_: std::option::Option<terrars::DynamicBlock<ComputeOrganizationSecurityPolicyRule_Match>>,
    preconfigured_waf_config: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig>>,
    redirect_options: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeOrganizationSecurityPolicyRule_RedirectOptions>>,
}

#[derive(Serialize)]
pub struct ComputeOrganizationSecurityPolicyRule_HeaderAction_RequestHeadersToAdds {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeOrganizationSecurityPolicyRule_HeaderAction_RequestHeadersToAdds {
    #[doc = "Set the field `header_name`.\nThe name of the header to set."]
    pub fn set_header_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.header_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `header_value`.\nThe value to set the named header to."]
    pub fn set_header_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.header_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeOrganizationSecurityPolicyRule_HeaderAction_RequestHeadersToAdds {
    type O = terrars::BlockAssignable<ComputeOrganizationSecurityPolicyRule_HeaderAction_RequestHeadersToAdds>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeOrganizationSecurityPolicyRule_HeaderAction_RequestHeadersToAdds {}

impl BuildComputeOrganizationSecurityPolicyRule_HeaderAction_RequestHeadersToAdds {
    pub fn build(self) -> ComputeOrganizationSecurityPolicyRule_HeaderAction_RequestHeadersToAdds {
        ComputeOrganizationSecurityPolicyRule_HeaderAction_RequestHeadersToAdds {
            header_name: core::default::Default::default(),
            header_value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeOrganizationSecurityPolicyRule_HeaderAction_RequestHeadersToAddsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeOrganizationSecurityPolicyRule_HeaderAction_RequestHeadersToAddsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeOrganizationSecurityPolicyRule_HeaderAction_RequestHeadersToAddsRef {
        ComputeOrganizationSecurityPolicyRule_HeaderAction_RequestHeadersToAddsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeOrganizationSecurityPolicyRule_HeaderAction_RequestHeadersToAddsRef {
    #[allow(dead_code)]
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
struct ComputeOrganizationSecurityPolicyRule_HeaderActionDynamic {
    request_headers_to_adds: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeOrganizationSecurityPolicyRule_HeaderAction_RequestHeadersToAdds>>,
}

#[derive(Serialize)]
pub struct ComputeOrganizationSecurityPolicyRule_HeaderAction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_headers_to_adds: std
    ::option
    ::Option<std::vec::Vec<ComputeOrganizationSecurityPolicyRule_HeaderAction_RequestHeadersToAdds>>,
    dynamic: ComputeOrganizationSecurityPolicyRule_HeaderActionDynamic,
}

impl ComputeOrganizationSecurityPolicyRule_HeaderAction {
    #[doc = "Set the field `request_headers_to_adds`.\n"]
    pub fn set_request_headers_to_adds(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ComputeOrganizationSecurityPolicyRule_HeaderAction_RequestHeadersToAdds>,
                    >,
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

impl terrars::ToListMappable for ComputeOrganizationSecurityPolicyRule_HeaderAction {
    type O = terrars::BlockAssignable<ComputeOrganizationSecurityPolicyRule_HeaderAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeOrganizationSecurityPolicyRule_HeaderAction {}

impl BuildComputeOrganizationSecurityPolicyRule_HeaderAction {
    pub fn build(self) -> ComputeOrganizationSecurityPolicyRule_HeaderAction {
        ComputeOrganizationSecurityPolicyRule_HeaderAction {
            request_headers_to_adds: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeOrganizationSecurityPolicyRule_HeaderActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeOrganizationSecurityPolicyRule_HeaderActionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeOrganizationSecurityPolicyRule_HeaderActionRef {
        ComputeOrganizationSecurityPolicyRule_HeaderActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeOrganizationSecurityPolicyRule_HeaderActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `request_headers_to_adds` after provisioning.\n"]
    pub fn request_headers_to_adds(
        &self,
    ) -> terrars::ListRef<ComputeOrganizationSecurityPolicyRule_HeaderAction_RequestHeadersToAddsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_headers_to_adds", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeOrganizationSecurityPolicyRule_Match_Config {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    src_ip_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ComputeOrganizationSecurityPolicyRule_Match_Config {
    #[doc = "Set the field `src_ip_ranges`.\nSource IP address range in CIDR format. Required for\nINGRESS rules."]
    pub fn set_src_ip_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.src_ip_ranges = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeOrganizationSecurityPolicyRule_Match_Config {
    type O = terrars::BlockAssignable<ComputeOrganizationSecurityPolicyRule_Match_Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeOrganizationSecurityPolicyRule_Match_Config {}

impl BuildComputeOrganizationSecurityPolicyRule_Match_Config {
    pub fn build(self) -> ComputeOrganizationSecurityPolicyRule_Match_Config {
        ComputeOrganizationSecurityPolicyRule_Match_Config { src_ip_ranges: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ComputeOrganizationSecurityPolicyRule_Match_ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeOrganizationSecurityPolicyRule_Match_ConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeOrganizationSecurityPolicyRule_Match_ConfigRef {
        ComputeOrganizationSecurityPolicyRule_Match_ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeOrganizationSecurityPolicyRule_Match_ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `src_ip_ranges` after provisioning.\nSource IP address range in CIDR format. Required for\nINGRESS rules."]
    pub fn src_ip_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.src_ip_ranges", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeOrganizationSecurityPolicyRule_Match_Expr {
    expression: terrars::PrimField<std::string::String>,
}

impl ComputeOrganizationSecurityPolicyRule_Match_Expr { }

impl terrars::ToListMappable for ComputeOrganizationSecurityPolicyRule_Match_Expr {
    type O = terrars::BlockAssignable<ComputeOrganizationSecurityPolicyRule_Match_Expr>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeOrganizationSecurityPolicyRule_Match_Expr {
    #[doc =
        "Textual representation of an expression in Common Expression Language syntax. The application context of the containing message determines which well-known feature set of CEL is supported."]
    pub expression: terrars::PrimField<std::string::String>,
}

impl BuildComputeOrganizationSecurityPolicyRule_Match_Expr {
    pub fn build(self) -> ComputeOrganizationSecurityPolicyRule_Match_Expr {
        ComputeOrganizationSecurityPolicyRule_Match_Expr { expression: self.expression }
    }
}

#[allow(dead_code)]
pub struct ComputeOrganizationSecurityPolicyRule_Match_ExprRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeOrganizationSecurityPolicyRule_Match_ExprRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeOrganizationSecurityPolicyRule_Match_ExprRef {
        ComputeOrganizationSecurityPolicyRule_Match_ExprRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeOrganizationSecurityPolicyRule_Match_ExprRef {
    #[allow(dead_code)]
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
struct ComputeOrganizationSecurityPolicyRule_MatchDynamic {
    config: std::option::Option<terrars::DynamicBlock<ComputeOrganizationSecurityPolicyRule_Match_Config>>,
    expr: std::option::Option<terrars::DynamicBlock<ComputeOrganizationSecurityPolicyRule_Match_Expr>>,
}

#[derive(Serialize)]
pub struct ComputeOrganizationSecurityPolicyRule_Match {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    versioned_expr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config: std::option::Option<std::vec::Vec<ComputeOrganizationSecurityPolicyRule_Match_Config>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expr: std::option::Option<std::vec::Vec<ComputeOrganizationSecurityPolicyRule_Match_Expr>>,
    dynamic: ComputeOrganizationSecurityPolicyRule_MatchDynamic,
}

impl ComputeOrganizationSecurityPolicyRule_Match {
    #[doc = "Set the field `description`.\nA description of the rule."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `versioned_expr`.\nPreconfigured versioned expression. For organization security policy rules,\nthe only supported type is \"SRC_IPS_V1\".\n**NOTE** : 'FIREWALL' type is deprecated. Please use 'google_compute_firewall_policy_rule' resource instead."]
    pub fn set_versioned_expr(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.versioned_expr = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `config`.\n"]
    pub fn set_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeOrganizationSecurityPolicyRule_Match_Config>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeOrganizationSecurityPolicyRule_Match_Expr>>,
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

impl terrars::ToListMappable for ComputeOrganizationSecurityPolicyRule_Match {
    type O = terrars::BlockAssignable<ComputeOrganizationSecurityPolicyRule_Match>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeOrganizationSecurityPolicyRule_Match {}

impl BuildComputeOrganizationSecurityPolicyRule_Match {
    pub fn build(self) -> ComputeOrganizationSecurityPolicyRule_Match {
        ComputeOrganizationSecurityPolicyRule_Match {
            description: core::default::Default::default(),
            versioned_expr: core::default::Default::default(),
            config: core::default::Default::default(),
            expr: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeOrganizationSecurityPolicyRule_MatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeOrganizationSecurityPolicyRule_MatchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeOrganizationSecurityPolicyRule_MatchRef {
        ComputeOrganizationSecurityPolicyRule_MatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeOrganizationSecurityPolicyRule_MatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of the rule."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `versioned_expr` after provisioning.\nPreconfigured versioned expression. For organization security policy rules,\nthe only supported type is \"SRC_IPS_V1\".\n**NOTE** : 'FIREWALL' type is deprecated. Please use 'google_compute_firewall_policy_rule' resource instead."]
    pub fn versioned_expr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.versioned_expr", self.base))
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<ComputeOrganizationSecurityPolicyRule_Match_ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.base))
    }

    #[doc = "Get a reference to the value of field `expr` after provisioning.\n"]
    pub fn expr(&self) -> terrars::ListRef<ComputeOrganizationSecurityPolicyRule_Match_ExprRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expr", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestCookie {
    operator: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestCookie {
    #[doc =
        "Set the field `value`.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestCookie {
    type O =
        terrars
        ::BlockAssignable<ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestCookie>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestCookie {
    #[doc =
        "You can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value."]
    pub operator: terrars::PrimField<std::string::String>,
}

impl BuildComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestCookie {
    pub fn build(self) -> ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestCookie {
        ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestCookie {
            operator: self.operator,
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestCookieRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestCookieRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestCookieRef {
        ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestCookieRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestCookieRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `operator` after provisioning.\nYou can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value."]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestHeader {
    operator: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestHeader {
    #[doc =
        "Set the field `value`.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestHeader {
    type O =
        terrars
        ::BlockAssignable<ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestHeader>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestHeader {
    #[doc =
        "You can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value."]
    pub operator: terrars::PrimField<std::string::String>,
}

impl BuildComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestHeader {
    pub fn build(self) -> ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestHeader {
        ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestHeader {
            operator: self.operator,
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestHeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestHeaderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestHeaderRef {
        ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestHeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestHeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `operator` after provisioning.\nYou can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value."]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestQueryParam {
    operator: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestQueryParam {
    #[doc =
        "Set the field `value`.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestQueryParam {
    type O =
        terrars
        ::BlockAssignable<ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestQueryParam>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestQueryParam {
    #[doc =
        "You can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value."]
    pub operator: terrars::PrimField<std::string::String>,
}

impl BuildComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestQueryParam {
    pub fn build(self) -> ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestQueryParam {
        ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestQueryParam {
            operator: self.operator,
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestQueryParamRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestQueryParamRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestQueryParamRef {
        ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestQueryParamRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestQueryParamRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `operator` after provisioning.\nYou can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value."]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestUri {
    operator: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestUri {
    #[doc =
        "Set the field `value`.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestUri {
    type O =
        terrars::BlockAssignable<ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestUri>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestUri {
    #[doc =
        "You can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value."]
    pub operator: terrars::PrimField<std::string::String>,
}

impl BuildComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestUri {
    pub fn build(self) -> ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestUri {
        ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestUri {
            operator: self.operator,
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestUriRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestUriRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestUriRef {
        ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestUriRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestUriRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `operator` after provisioning.\nYou can specify an exact match or a partial match by using a field operator and a field value.\nAvailable options:\nEQUALS: The operator matches if the field value equals the specified value.\nSTARTS_WITH: The operator matches if the field value starts with the specified value.\nENDS_WITH: The operator matches if the field value ends with the specified value.\nCONTAINS: The operator matches if the field value contains the specified value.\nEQUALS_ANY: The operator matches if the field value is any value."]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nA request field matching the specified value will be excluded from inspection during preconfigured WAF evaluation.\nThe field value must be given if the field operator is not EQUALS_ANY, and cannot be given if the field operator is EQUALS_ANY."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_ExclusionDynamic {
    request_cookie: std
    ::option
    ::Option<
        terrars::DynamicBlock<ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestCookie>,
    >,
    request_header: std
    ::option
    ::Option<
        terrars::DynamicBlock<ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestHeader>,
    >,
    request_query_param: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestQueryParam>,
    >,
    request_uri: std
    ::option
    ::Option<
        terrars::DynamicBlock<ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestUri>,
    >,
}

#[derive(Serialize)]
pub struct ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_rule_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    target_rule_set: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_cookie: std
    ::option
    ::Option<std::vec::Vec<ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestCookie>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_header: std
    ::option
    ::Option<std::vec::Vec<ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestHeader>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_query_param: std
    ::option
    ::Option<std::vec::Vec<ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestQueryParam>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_uri: std
    ::option
    ::Option<std::vec::Vec<ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestUri>>,
    dynamic: ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_ExclusionDynamic,
}

impl ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion {
    #[doc =
        "Set the field `target_rule_ids`.\nA list of target rule IDs under the WAF rule set to apply the preconfigured WAF exclusion.\nIf omitted, it refers to all the rule IDs under the WAF rule set."]
    pub fn set_target_rule_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.target_rule_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `request_cookie`.\n"]
    pub fn set_request_cookie(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestCookie,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.request_cookie = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.request_cookie = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `request_header`.\n"]
    pub fn set_request_header(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestHeader,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.request_header = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.request_header = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `request_query_param`.\n"]
    pub fn set_request_query_param(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestQueryParam,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.request_query_param = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.request_query_param = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `request_uri`.\n"]
    pub fn set_request_uri(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestUri,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.request_uri = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.request_uri = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion {
    type O = terrars::BlockAssignable<ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion {
    #[doc = "Target WAF rule set to apply the preconfigured WAF exclusion."]
    pub target_rule_set: terrars::PrimField<std::string::String>,
}

impl BuildComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion {
    pub fn build(self) -> ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion {
        ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion {
            target_rule_ids: core::default::Default::default(),
            target_rule_set: self.target_rule_set,
            request_cookie: core::default::Default::default(),
            request_header: core::default::Default::default(),
            request_query_param: core::default::Default::default(),
            request_uri: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_ExclusionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_ExclusionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_ExclusionRef {
        ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_ExclusionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_ExclusionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `target_rule_ids` after provisioning.\nA list of target rule IDs under the WAF rule set to apply the preconfigured WAF exclusion.\nIf omitted, it refers to all the rule IDs under the WAF rule set."]
    pub fn target_rule_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_rule_ids", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_rule_set` after provisioning.\nTarget WAF rule set to apply the preconfigured WAF exclusion."]
    pub fn target_rule_set(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_rule_set", self.base))
    }

    #[doc = "Get a reference to the value of field `request_cookie` after provisioning.\n"]
    pub fn request_cookie(
        &self,
    ) -> terrars::ListRef<ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestCookieRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_cookie", self.base))
    }

    #[doc = "Get a reference to the value of field `request_header` after provisioning.\n"]
    pub fn request_header(
        &self,
    ) -> terrars::ListRef<ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestHeaderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_header", self.base))
    }

    #[doc = "Get a reference to the value of field `request_query_param` after provisioning.\n"]
    pub fn request_query_param(
        &self,
    ) -> terrars
    ::ListRef<ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestQueryParamRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_query_param", self.base))
    }

    #[doc = "Get a reference to the value of field `request_uri` after provisioning.\n"]
    pub fn request_uri(
        &self,
    ) -> terrars::ListRef<ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion_RequestUriRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfigDynamic {
    exclusion: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion>>,
}

#[derive(Serialize)]
pub struct ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclusion: std::option::Option<std::vec::Vec<ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion>>,
    dynamic: ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfigDynamic,
}

impl ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig {
    #[doc = "Set the field `exclusion`.\n"]
    pub fn set_exclusion(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_Exclusion>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.exclusion = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.exclusion = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig {
    type O = terrars::BlockAssignable<ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig {}

impl BuildComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig {
    pub fn build(self) -> ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig {
        ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig {
            exclusion: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfigRef {
        ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exclusion` after provisioning.\n"]
    pub fn exclusion(
        &self,
    ) -> terrars::ListRef<ComputeOrganizationSecurityPolicyRule_PreconfiguredWafConfig_ExclusionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclusion", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeOrganizationSecurityPolicyRule_RedirectOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl ComputeOrganizationSecurityPolicyRule_RedirectOptions {
    #[doc = "Set the field `target`.\nTarget for the redirect action. This is required if the type is EXTERNAL_302."]
    pub fn set_target(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeOrganizationSecurityPolicyRule_RedirectOptions {
    type O = terrars::BlockAssignable<ComputeOrganizationSecurityPolicyRule_RedirectOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeOrganizationSecurityPolicyRule_RedirectOptions {
    #[doc = "Type of the redirect action. For organization security policies, only EXTERNAL_302 is supported."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildComputeOrganizationSecurityPolicyRule_RedirectOptions {
    pub fn build(self) -> ComputeOrganizationSecurityPolicyRule_RedirectOptions {
        ComputeOrganizationSecurityPolicyRule_RedirectOptions {
            target: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct ComputeOrganizationSecurityPolicyRule_RedirectOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeOrganizationSecurityPolicyRule_RedirectOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeOrganizationSecurityPolicyRule_RedirectOptionsRef {
        ComputeOrganizationSecurityPolicyRule_RedirectOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeOrganizationSecurityPolicyRule_RedirectOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `target` after provisioning.\nTarget for the redirect action. This is required if the type is EXTERNAL_302."]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nType of the redirect action. For organization security policies, only EXTERNAL_302 is supported."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeOrganizationSecurityPolicyRule_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeOrganizationSecurityPolicyRule_Timeouts {
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

impl terrars::ToListMappable for ComputeOrganizationSecurityPolicyRule_Timeouts {
    type O = terrars::BlockAssignable<ComputeOrganizationSecurityPolicyRule_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeOrganizationSecurityPolicyRule_Timeouts {}

impl BuildComputeOrganizationSecurityPolicyRule_Timeouts {
    pub fn build(self) -> ComputeOrganizationSecurityPolicyRule_Timeouts {
        ComputeOrganizationSecurityPolicyRule_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeOrganizationSecurityPolicyRule_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeOrganizationSecurityPolicyRule_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeOrganizationSecurityPolicyRule_TimeoutsRef {
        ComputeOrganizationSecurityPolicyRule_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeOrganizationSecurityPolicyRule_TimeoutsRef {
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
