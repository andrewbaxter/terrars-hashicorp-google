use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeNetworkFirewallPolicyRuleData {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    direction: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_logging: std::option::Option<terrars::PrimField<bool>>,
    firewall_policy: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    priority: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_service_accounts: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<std::vec::Vec<ComputeNetworkFirewallPolicyRuleMatchEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_secure_tags: std::option::Option<std::vec::Vec<ComputeNetworkFirewallPolicyRuleTargetSecureTagsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeNetworkFirewallPolicyRuleTimeoutsEl>,
    dynamic: ComputeNetworkFirewallPolicyRuleDynamic,
}

struct ComputeNetworkFirewallPolicyRule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeNetworkFirewallPolicyRuleData>,
}

#[derive(Clone)]
pub struct ComputeNetworkFirewallPolicyRule(std::rc::Rc<ComputeNetworkFirewallPolicyRule_>);

impl ComputeNetworkFirewallPolicyRule {
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

    #[doc = "Set the field `description`.\nAn optional description for this resource."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disabled`.\nDenotes whether the firewall policy rule is disabled. When set to true, the firewall policy rule is not enforced and traffic behaves as if it did not exist. If this is unspecified, the firewall policy rule will be enabled."]
    pub fn set_disabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_logging`.\nDenotes whether to enable logging for a particular rule. If logging is enabled, logs will be exported to the configured export destination in Stackdriver. Logs may be exported to BigQuery or Pub/Sub. Note: you cannot enable logging on \"goto_next\" rules."]
    pub fn set_enable_logging(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_logging = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\nThe project for the resource"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `rule_name`.\nAn optional name for the rule. This field is not a unique identifier and can be updated."]
    pub fn set_rule_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().rule_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `target_service_accounts`.\nA list of service accounts indicating the sets of instances that are applied with this rule."]
    pub fn set_target_service_accounts(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().target_service_accounts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeNetworkFirewallPolicyRuleMatchEl>>,
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

    #[doc = "Set the field `target_secure_tags`.\n"]
    pub fn set_target_secure_tags(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeNetworkFirewallPolicyRuleTargetSecureTagsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target_secure_tags = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target_secure_tags = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeNetworkFirewallPolicyRuleTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nThe Action to perform when the client connection triggers the rule. Valid actions are \"allow\", \"deny\" and \"goto_next\"."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description for this resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nThe direction in which this rule applies. Possible values: INGRESS, EGRESS"]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nDenotes whether the firewall policy rule is disabled. When set to true, the firewall policy rule is not enforced and traffic behaves as if it did not exist. If this is unspecified, the firewall policy rule will be enabled."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_logging` after provisioning.\nDenotes whether to enable logging for a particular rule. If logging is enabled, logs will be exported to the configured export destination in Stackdriver. Logs may be exported to BigQuery or Pub/Sub. Note: you cannot enable logging on \"goto_next\" rules."]
    pub fn enable_logging(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_logging", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `firewall_policy` after provisioning.\nThe firewall policy of the resource."]
    pub fn firewall_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.firewall_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kind` after provisioning.\nType of the resource. Always `compute#firewallPolicyRule` for firewall policy rules"]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `priority` after provisioning.\nAn integer indicating the priority of a rule in the list. The priority must be a positive value between 0 and 2147483647. Rules are evaluated from highest to lowest priority where 0 is the highest priority and 2147483647 is the lowest prority."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rule_name` after provisioning.\nAn optional name for the rule. This field is not a unique identifier and can be updated."]
    pub fn rule_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rule_tuple_count` after provisioning.\nCalculation of the complexity of a single firewall policy rule."]
    pub fn rule_tuple_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_tuple_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_service_accounts` after provisioning.\nA list of service accounts indicating the sets of instances that are applied with this rule."]
    pub fn target_service_accounts(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_service_accounts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<ComputeNetworkFirewallPolicyRuleMatchElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_secure_tags` after provisioning.\n"]
    pub fn target_secure_tags(&self) -> terrars::ListRef<ComputeNetworkFirewallPolicyRuleTargetSecureTagsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_secure_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeNetworkFirewallPolicyRuleTimeoutsElRef {
        ComputeNetworkFirewallPolicyRuleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for ComputeNetworkFirewallPolicyRule {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputeNetworkFirewallPolicyRule { }

impl terrars::ToListMappable for ComputeNetworkFirewallPolicyRule {
    type O = terrars::ListRef<ComputeNetworkFirewallPolicyRuleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputeNetworkFirewallPolicyRule_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_compute_network_firewall_policy_rule".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeNetworkFirewallPolicyRule {
    pub tf_id: std::string::String,
    #[doc =
        "The Action to perform when the client connection triggers the rule. Valid actions are \"allow\", \"deny\" and \"goto_next\"."]
    pub action: terrars::PrimField<std::string::String>,
    #[doc = "The direction in which this rule applies. Possible values: INGRESS, EGRESS"]
    pub direction: terrars::PrimField<std::string::String>,
    #[doc = "The firewall policy of the resource."]
    pub firewall_policy: terrars::PrimField<std::string::String>,
    #[doc =
        "An integer indicating the priority of a rule in the list. The priority must be a positive value between 0 and 2147483647. Rules are evaluated from highest to lowest priority where 0 is the highest priority and 2147483647 is the lowest prority."]
    pub priority: terrars::PrimField<f64>,
}

impl BuildComputeNetworkFirewallPolicyRule {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputeNetworkFirewallPolicyRule {
        let out = ComputeNetworkFirewallPolicyRule(std::rc::Rc::new(ComputeNetworkFirewallPolicyRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputeNetworkFirewallPolicyRuleData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                action: self.action,
                description: core::default::Default::default(),
                direction: self.direction,
                disabled: core::default::Default::default(),
                enable_logging: core::default::Default::default(),
                firewall_policy: self.firewall_policy,
                id: core::default::Default::default(),
                priority: self.priority,
                project: core::default::Default::default(),
                rule_name: core::default::Default::default(),
                target_service_accounts: core::default::Default::default(),
                match_: core::default::Default::default(),
                target_secure_tags: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeNetworkFirewallPolicyRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeNetworkFirewallPolicyRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeNetworkFirewallPolicyRuleRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nThe Action to perform when the client connection triggers the rule. Valid actions are \"allow\", \"deny\" and \"goto_next\"."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description for this resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nThe direction in which this rule applies. Possible values: INGRESS, EGRESS"]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nDenotes whether the firewall policy rule is disabled. When set to true, the firewall policy rule is not enforced and traffic behaves as if it did not exist. If this is unspecified, the firewall policy rule will be enabled."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_logging` after provisioning.\nDenotes whether to enable logging for a particular rule. If logging is enabled, logs will be exported to the configured export destination in Stackdriver. Logs may be exported to BigQuery or Pub/Sub. Note: you cannot enable logging on \"goto_next\" rules."]
    pub fn enable_logging(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_logging", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `firewall_policy` after provisioning.\nThe firewall policy of the resource."]
    pub fn firewall_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.firewall_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kind` after provisioning.\nType of the resource. Always `compute#firewallPolicyRule` for firewall policy rules"]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `priority` after provisioning.\nAn integer indicating the priority of a rule in the list. The priority must be a positive value between 0 and 2147483647. Rules are evaluated from highest to lowest priority where 0 is the highest priority and 2147483647 is the lowest prority."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rule_name` after provisioning.\nAn optional name for the rule. This field is not a unique identifier and can be updated."]
    pub fn rule_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rule_tuple_count` after provisioning.\nCalculation of the complexity of a single firewall policy rule."]
    pub fn rule_tuple_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_tuple_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `target_service_accounts` after provisioning.\nA list of service accounts indicating the sets of instances that are applied with this rule."]
    pub fn target_service_accounts(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_service_accounts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<ComputeNetworkFirewallPolicyRuleMatchElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_secure_tags` after provisioning.\n"]
    pub fn target_secure_tags(&self) -> terrars::ListRef<ComputeNetworkFirewallPolicyRuleTargetSecureTagsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_secure_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeNetworkFirewallPolicyRuleTimeoutsElRef {
        ComputeNetworkFirewallPolicyRuleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct ComputeNetworkFirewallPolicyRuleMatchElLayer4ConfigsEl {
    ip_protocol: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ports: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ComputeNetworkFirewallPolicyRuleMatchElLayer4ConfigsEl {
    #[doc =
        "Set the field `ports`.\nAn optional list of ports to which this rule applies. This field is only applicable for UDP or TCP protocol. Each entry must be either an integer or a range. If not specified, this rule applies to connections through any port. Example inputs include: ``."]
    pub fn set_ports(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ports = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeNetworkFirewallPolicyRuleMatchElLayer4ConfigsEl {
    type O = terrars::BlockAssignable<ComputeNetworkFirewallPolicyRuleMatchElLayer4ConfigsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeNetworkFirewallPolicyRuleMatchElLayer4ConfigsEl {
    #[doc =
        "The IP protocol to which this rule applies. The protocol type is required when creating a firewall rule. This value can either be one of the following well known protocol strings (`tcp`, `udp`, `icmp`, `esp`, `ah`, `ipip`, `sctp`), or the IP protocol number."]
    pub ip_protocol: terrars::PrimField<std::string::String>,
}

impl BuildComputeNetworkFirewallPolicyRuleMatchElLayer4ConfigsEl {
    pub fn build(self) -> ComputeNetworkFirewallPolicyRuleMatchElLayer4ConfigsEl {
        ComputeNetworkFirewallPolicyRuleMatchElLayer4ConfigsEl {
            ip_protocol: self.ip_protocol,
            ports: core::default::Default::default(),
        }
    }
}

pub struct ComputeNetworkFirewallPolicyRuleMatchElLayer4ConfigsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeNetworkFirewallPolicyRuleMatchElLayer4ConfigsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeNetworkFirewallPolicyRuleMatchElLayer4ConfigsElRef {
        ComputeNetworkFirewallPolicyRuleMatchElLayer4ConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeNetworkFirewallPolicyRuleMatchElLayer4ConfigsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ip_protocol` after provisioning.\nThe IP protocol to which this rule applies. The protocol type is required when creating a firewall rule. This value can either be one of the following well known protocol strings (`tcp`, `udp`, `icmp`, `esp`, `ah`, `ipip`, `sctp`), or the IP protocol number."]
    pub fn ip_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_protocol", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ports` after provisioning.\nAn optional list of ports to which this rule applies. This field is only applicable for UDP or TCP protocol. Each entry must be either an integer or a range. If not specified, this rule applies to connections through any port. Example inputs include: ``."]
    pub fn ports(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeNetworkFirewallPolicyRuleMatchElSrcSecureTagsEl {
    name: terrars::PrimField<std::string::String>,
}

impl ComputeNetworkFirewallPolicyRuleMatchElSrcSecureTagsEl { }

impl terrars::ToListMappable for ComputeNetworkFirewallPolicyRuleMatchElSrcSecureTagsEl {
    type O = terrars::BlockAssignable<ComputeNetworkFirewallPolicyRuleMatchElSrcSecureTagsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeNetworkFirewallPolicyRuleMatchElSrcSecureTagsEl {
    #[doc = "Name of the secure tag, created with TagManager's TagValue API. @pattern tagValues/[0-9]+"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeNetworkFirewallPolicyRuleMatchElSrcSecureTagsEl {
    pub fn build(self) -> ComputeNetworkFirewallPolicyRuleMatchElSrcSecureTagsEl {
        ComputeNetworkFirewallPolicyRuleMatchElSrcSecureTagsEl { name: self.name }
    }
}

pub struct ComputeNetworkFirewallPolicyRuleMatchElSrcSecureTagsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeNetworkFirewallPolicyRuleMatchElSrcSecureTagsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeNetworkFirewallPolicyRuleMatchElSrcSecureTagsElRef {
        ComputeNetworkFirewallPolicyRuleMatchElSrcSecureTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeNetworkFirewallPolicyRuleMatchElSrcSecureTagsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the secure tag, created with TagManager's TagValue API. @pattern tagValues/[0-9]+"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\n[Output Only] State of the secure tag, either `EFFECTIVE` or `INEFFECTIVE`. A secure tag is `INEFFECTIVE` when it is deleted or its network is deleted."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeNetworkFirewallPolicyRuleMatchElDynamic {
    layer4_configs: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeNetworkFirewallPolicyRuleMatchElLayer4ConfigsEl>>,
    src_secure_tags: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeNetworkFirewallPolicyRuleMatchElSrcSecureTagsEl>>,
}

#[derive(Serialize)]
pub struct ComputeNetworkFirewallPolicyRuleMatchEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dest_address_groups: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dest_fqdns: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dest_ip_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dest_region_codes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dest_threat_intelligences: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    src_address_groups: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    src_fqdns: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    src_ip_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    src_region_codes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    src_threat_intelligences: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    layer4_configs: std::option::Option<std::vec::Vec<ComputeNetworkFirewallPolicyRuleMatchElLayer4ConfigsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    src_secure_tags: std::option::Option<std::vec::Vec<ComputeNetworkFirewallPolicyRuleMatchElSrcSecureTagsEl>>,
    dynamic: ComputeNetworkFirewallPolicyRuleMatchElDynamic,
}

impl ComputeNetworkFirewallPolicyRuleMatchEl {
    #[doc =
        "Set the field `dest_address_groups`.\nAddress groups which should be matched against the traffic destination. Maximum number of destination address groups is 10. Destination address groups is only supported in Egress rules."]
    pub fn set_dest_address_groups(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dest_address_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `dest_fqdns`.\nDomain names that will be used to match against the resolved domain name of destination of traffic. Can only be specified if DIRECTION is egress."]
    pub fn set_dest_fqdns(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dest_fqdns = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `dest_ip_ranges`.\nCIDR IP address range. Maximum number of destination CIDR IP ranges allowed is 5000."]
    pub fn set_dest_ip_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dest_ip_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `dest_region_codes`.\nThe Unicode country codes whose IP addresses will be used to match against the source of traffic. Can only be specified if DIRECTION is egress."]
    pub fn set_dest_region_codes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dest_region_codes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dest_threat_intelligences`.\nName of the Google Cloud Threat Intelligence list."]
    pub fn set_dest_threat_intelligences(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dest_threat_intelligences = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `src_address_groups`.\nAddress groups which should be matched against the traffic source. Maximum number of source address groups is 10. Source address groups is only supported in Ingress rules."]
    pub fn set_src_address_groups(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.src_address_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `src_fqdns`.\nDomain names that will be used to match against the resolved domain name of source of traffic. Can only be specified if DIRECTION is ingress."]
    pub fn set_src_fqdns(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.src_fqdns = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `src_ip_ranges`.\nCIDR IP address range. Maximum number of source CIDR IP ranges allowed is 5000."]
    pub fn set_src_ip_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.src_ip_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `src_region_codes`.\nThe Unicode country codes whose IP addresses will be used to match against the source of traffic. Can only be specified if DIRECTION is ingress."]
    pub fn set_src_region_codes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.src_region_codes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `src_threat_intelligences`.\nName of the Google Cloud Threat Intelligence list."]
    pub fn set_src_threat_intelligences(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.src_threat_intelligences = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `layer4_configs`.\n"]
    pub fn set_layer4_configs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeNetworkFirewallPolicyRuleMatchElLayer4ConfigsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.layer4_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.layer4_configs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `src_secure_tags`.\n"]
    pub fn set_src_secure_tags(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeNetworkFirewallPolicyRuleMatchElSrcSecureTagsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.src_secure_tags = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.src_secure_tags = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeNetworkFirewallPolicyRuleMatchEl {
    type O = terrars::BlockAssignable<ComputeNetworkFirewallPolicyRuleMatchEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeNetworkFirewallPolicyRuleMatchEl {}

impl BuildComputeNetworkFirewallPolicyRuleMatchEl {
    pub fn build(self) -> ComputeNetworkFirewallPolicyRuleMatchEl {
        ComputeNetworkFirewallPolicyRuleMatchEl {
            dest_address_groups: core::default::Default::default(),
            dest_fqdns: core::default::Default::default(),
            dest_ip_ranges: core::default::Default::default(),
            dest_region_codes: core::default::Default::default(),
            dest_threat_intelligences: core::default::Default::default(),
            src_address_groups: core::default::Default::default(),
            src_fqdns: core::default::Default::default(),
            src_ip_ranges: core::default::Default::default(),
            src_region_codes: core::default::Default::default(),
            src_threat_intelligences: core::default::Default::default(),
            layer4_configs: core::default::Default::default(),
            src_secure_tags: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ComputeNetworkFirewallPolicyRuleMatchElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeNetworkFirewallPolicyRuleMatchElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeNetworkFirewallPolicyRuleMatchElRef {
        ComputeNetworkFirewallPolicyRuleMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeNetworkFirewallPolicyRuleMatchElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dest_address_groups` after provisioning.\nAddress groups which should be matched against the traffic destination. Maximum number of destination address groups is 10. Destination address groups is only supported in Egress rules."]
    pub fn dest_address_groups(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dest_address_groups", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dest_fqdns` after provisioning.\nDomain names that will be used to match against the resolved domain name of destination of traffic. Can only be specified if DIRECTION is egress."]
    pub fn dest_fqdns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dest_fqdns", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dest_ip_ranges` after provisioning.\nCIDR IP address range. Maximum number of destination CIDR IP ranges allowed is 5000."]
    pub fn dest_ip_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dest_ip_ranges", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dest_region_codes` after provisioning.\nThe Unicode country codes whose IP addresses will be used to match against the source of traffic. Can only be specified if DIRECTION is egress."]
    pub fn dest_region_codes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dest_region_codes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dest_threat_intelligences` after provisioning.\nName of the Google Cloud Threat Intelligence list."]
    pub fn dest_threat_intelligences(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dest_threat_intelligences", self.base))
    }

    #[doc =
        "Get a reference to the value of field `src_address_groups` after provisioning.\nAddress groups which should be matched against the traffic source. Maximum number of source address groups is 10. Source address groups is only supported in Ingress rules."]
    pub fn src_address_groups(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.src_address_groups", self.base))
    }

    #[doc =
        "Get a reference to the value of field `src_fqdns` after provisioning.\nDomain names that will be used to match against the resolved domain name of source of traffic. Can only be specified if DIRECTION is ingress."]
    pub fn src_fqdns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.src_fqdns", self.base))
    }

    #[doc =
        "Get a reference to the value of field `src_ip_ranges` after provisioning.\nCIDR IP address range. Maximum number of source CIDR IP ranges allowed is 5000."]
    pub fn src_ip_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.src_ip_ranges", self.base))
    }

    #[doc =
        "Get a reference to the value of field `src_region_codes` after provisioning.\nThe Unicode country codes whose IP addresses will be used to match against the source of traffic. Can only be specified if DIRECTION is ingress."]
    pub fn src_region_codes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.src_region_codes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `src_threat_intelligences` after provisioning.\nName of the Google Cloud Threat Intelligence list."]
    pub fn src_threat_intelligences(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.src_threat_intelligences", self.base))
    }

    #[doc = "Get a reference to the value of field `layer4_configs` after provisioning.\n"]
    pub fn layer4_configs(&self) -> terrars::ListRef<ComputeNetworkFirewallPolicyRuleMatchElLayer4ConfigsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.layer4_configs", self.base))
    }

    #[doc = "Get a reference to the value of field `src_secure_tags` after provisioning.\n"]
    pub fn src_secure_tags(&self) -> terrars::ListRef<ComputeNetworkFirewallPolicyRuleMatchElSrcSecureTagsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.src_secure_tags", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeNetworkFirewallPolicyRuleTargetSecureTagsEl {
    name: terrars::PrimField<std::string::String>,
}

impl ComputeNetworkFirewallPolicyRuleTargetSecureTagsEl { }

impl terrars::ToListMappable for ComputeNetworkFirewallPolicyRuleTargetSecureTagsEl {
    type O = terrars::BlockAssignable<ComputeNetworkFirewallPolicyRuleTargetSecureTagsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeNetworkFirewallPolicyRuleTargetSecureTagsEl {
    #[doc = "Name of the secure tag, created with TagManager's TagValue API. @pattern tagValues/[0-9]+"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeNetworkFirewallPolicyRuleTargetSecureTagsEl {
    pub fn build(self) -> ComputeNetworkFirewallPolicyRuleTargetSecureTagsEl {
        ComputeNetworkFirewallPolicyRuleTargetSecureTagsEl { name: self.name }
    }
}

pub struct ComputeNetworkFirewallPolicyRuleTargetSecureTagsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeNetworkFirewallPolicyRuleTargetSecureTagsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeNetworkFirewallPolicyRuleTargetSecureTagsElRef {
        ComputeNetworkFirewallPolicyRuleTargetSecureTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeNetworkFirewallPolicyRuleTargetSecureTagsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the secure tag, created with TagManager's TagValue API. @pattern tagValues/[0-9]+"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\n[Output Only] State of the secure tag, either `EFFECTIVE` or `INEFFECTIVE`. A secure tag is `INEFFECTIVE` when it is deleted or its network is deleted."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeNetworkFirewallPolicyRuleTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeNetworkFirewallPolicyRuleTimeoutsEl {
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

impl terrars::ToListMappable for ComputeNetworkFirewallPolicyRuleTimeoutsEl {
    type O = terrars::BlockAssignable<ComputeNetworkFirewallPolicyRuleTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeNetworkFirewallPolicyRuleTimeoutsEl {}

impl BuildComputeNetworkFirewallPolicyRuleTimeoutsEl {
    pub fn build(self) -> ComputeNetworkFirewallPolicyRuleTimeoutsEl {
        ComputeNetworkFirewallPolicyRuleTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ComputeNetworkFirewallPolicyRuleTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeNetworkFirewallPolicyRuleTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeNetworkFirewallPolicyRuleTimeoutsElRef {
        ComputeNetworkFirewallPolicyRuleTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeNetworkFirewallPolicyRuleTimeoutsElRef {
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
struct ComputeNetworkFirewallPolicyRuleDynamic {
    match_: std::option::Option<terrars::DynamicBlock<ComputeNetworkFirewallPolicyRuleMatchEl>>,
    target_secure_tags: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeNetworkFirewallPolicyRuleTargetSecureTagsEl>>,
}
