#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeNetworkFirewallPolicyWithRulesData_ {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<std::vec::Vec<ComputeNetworkFirewallPolicyWithRules_Rule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeNetworkFirewallPolicyWithRules_Timeouts>,
    dynamic: ComputeNetworkFirewallPolicyWithRulesDynamic,
}

struct ComputeNetworkFirewallPolicyWithRules_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeNetworkFirewallPolicyWithRulesData_>,
}

#[derive(Clone)]
pub struct ComputeNetworkFirewallPolicyWithRules(std::rc::Rc<ComputeNetworkFirewallPolicyWithRules_>);

impl ComputeNetworkFirewallPolicyWithRules {
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

    #[doc = "Set the field `description`.\nAn optional description of this resource."]
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
        "Set the field `policy_type`.\nPolicy type is used to determine which resources (networks) the policy can be associated with.\nA policy can be associated with a network only if the network has the matching policyType in its network profile.\nDifferent policy types may support some of the Firewall Rules features. Possible values: [\"VPC_POLICY\"]"]
    pub fn set_policy_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().policy_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeNetworkFirewallPolicyWithRules_Rule>>,
    ) -> Self {
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
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeNetworkFirewallPolicyWithRules_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint of the resource. This field is used internally during updates of this resource."]
    pub fn fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fingerprint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nUser-provided name of the Network firewall policy.\nThe name should be unique in the project in which the firewall policy is created.\nThe name must be 1-63 characters long, and comply with RFC1035. Specifically,\nthe name must be 1-63 characters long and match the regular expression [a-z]([-a-z0-9]*[a-z0-9])?\nwhich means the first character must be a lowercase letter, and all following characters must be a dash,\nlowercase letter, or digit, except the last character, which cannot be a dash."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network_firewall_policy_id` after provisioning.\nThe unique identifier for the resource. This identifier is defined by the server."]
    pub fn network_firewall_policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_firewall_policy_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `policy_type` after provisioning.\nPolicy type is used to determine which resources (networks) the policy can be associated with.\nA policy can be associated with a network only if the network has the matching policyType in its network profile.\nDifferent policy types may support some of the Firewall Rules features. Possible values: [\"VPC_POLICY\"]"]
    pub fn policy_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `predefined_rules` after provisioning.\nA list of firewall policy pre-defined rules."]
    pub fn predefined_rules(&self) -> terrars::ListRef<ComputeNetworkFirewallPolicyWithRules_PredefinedRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.predefined_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rule_tuple_count` after provisioning.\nTotal count of all firewall policy rule tuples. A firewall policy can not exceed a set number of tuples."]
    pub fn rule_tuple_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_tuple_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `self_link` after provisioning.\nServer-defined URL for the resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `self_link_with_id` after provisioning.\nServer-defined URL for this resource with the resource id."]
    pub fn self_link_with_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link_with_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<ComputeNetworkFirewallPolicyWithRules_RuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeNetworkFirewallPolicyWithRules_TimeoutsRef {
        ComputeNetworkFirewallPolicyWithRules_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for ComputeNetworkFirewallPolicyWithRules {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputeNetworkFirewallPolicyWithRules { }

impl terrars::ToListMappable for ComputeNetworkFirewallPolicyWithRules {
    type O = terrars::ListRef<ComputeNetworkFirewallPolicyWithRulesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputeNetworkFirewallPolicyWithRules_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_compute_network_firewall_policy_with_rules".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeNetworkFirewallPolicyWithRules {
    pub tf_id: std::string::String,
    #[doc =
        "User-provided name of the Network firewall policy.\nThe name should be unique in the project in which the firewall policy is created.\nThe name must be 1-63 characters long, and comply with RFC1035. Specifically,\nthe name must be 1-63 characters long and match the regular expression [a-z]([-a-z0-9]*[a-z0-9])?\nwhich means the first character must be a lowercase letter, and all following characters must be a dash,\nlowercase letter, or digit, except the last character, which cannot be a dash."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeNetworkFirewallPolicyWithRules {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputeNetworkFirewallPolicyWithRules {
        let out = ComputeNetworkFirewallPolicyWithRules(std::rc::Rc::new(ComputeNetworkFirewallPolicyWithRules_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputeNetworkFirewallPolicyWithRulesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                policy_type: core::default::Default::default(),
                project: core::default::Default::default(),
                rule: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeNetworkFirewallPolicyWithRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeNetworkFirewallPolicyWithRulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeNetworkFirewallPolicyWithRulesRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional description of this resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fingerprint` after provisioning.\nFingerprint of the resource. This field is used internally during updates of this resource."]
    pub fn fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fingerprint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nUser-provided name of the Network firewall policy.\nThe name should be unique in the project in which the firewall policy is created.\nThe name must be 1-63 characters long, and comply with RFC1035. Specifically,\nthe name must be 1-63 characters long and match the regular expression [a-z]([-a-z0-9]*[a-z0-9])?\nwhich means the first character must be a lowercase letter, and all following characters must be a dash,\nlowercase letter, or digit, except the last character, which cannot be a dash."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network_firewall_policy_id` after provisioning.\nThe unique identifier for the resource. This identifier is defined by the server."]
    pub fn network_firewall_policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_firewall_policy_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `policy_type` after provisioning.\nPolicy type is used to determine which resources (networks) the policy can be associated with.\nA policy can be associated with a network only if the network has the matching policyType in its network profile.\nDifferent policy types may support some of the Firewall Rules features. Possible values: [\"VPC_POLICY\"]"]
    pub fn policy_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `predefined_rules` after provisioning.\nA list of firewall policy pre-defined rules."]
    pub fn predefined_rules(&self) -> terrars::ListRef<ComputeNetworkFirewallPolicyWithRules_PredefinedRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.predefined_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rule_tuple_count` after provisioning.\nTotal count of all firewall policy rule tuples. A firewall policy can not exceed a set number of tuples."]
    pub fn rule_tuple_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_tuple_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `self_link` after provisioning.\nServer-defined URL for the resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `self_link_with_id` after provisioning.\nServer-defined URL for this resource with the resource id."]
    pub fn self_link_with_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link_with_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<ComputeNetworkFirewallPolicyWithRules_RuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeNetworkFirewallPolicyWithRules_TimeoutsRef {
        ComputeNetworkFirewallPolicyWithRules_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct ComputeNetworkFirewallPolicyWithRulesDynamic {
    rule: std::option::Option<terrars::DynamicBlock<ComputeNetworkFirewallPolicyWithRules_Rule>>,
}

#[derive(Serialize)]
pub struct ComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match_Layer4Config {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ports: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match_Layer4Config {
    #[doc = "Set the field `ip_protocol`.\n"]
    pub fn set_ip_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ports`.\n"]
    pub fn set_ports(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ports = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match_Layer4Config {
    type O = terrars::BlockAssignable<ComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match_Layer4Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match_Layer4Config {}

impl BuildComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match_Layer4Config {
    pub fn build(self) -> ComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match_Layer4Config {
        ComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match_Layer4Config {
            ip_protocol: core::default::Default::default(),
            ports: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match_Layer4ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match_Layer4ConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match_Layer4ConfigRef {
        ComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match_Layer4ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match_Layer4ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip_protocol` after provisioning.\n"]
    pub fn ip_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `ports` after provisioning.\n"]
    pub fn ports(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match_SrcSecureTag {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match_SrcSecureTag {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match_SrcSecureTag {
    type O = terrars::BlockAssignable<ComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match_SrcSecureTag>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match_SrcSecureTag {}

impl BuildComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match_SrcSecureTag {
    pub fn build(self) -> ComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match_SrcSecureTag {
        ComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match_SrcSecureTag {
            name: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match_SrcSecureTagRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match_SrcSecureTagRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match_SrcSecureTagRef {
        ComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match_SrcSecureTagRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match_SrcSecureTagRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match {
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
    layer4_config: std
    ::option
    ::Option<terrars::ListField<ComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match_Layer4Config>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    src_address_groups: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    src_fqdns: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    src_ip_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    src_region_codes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    src_secure_tag: std
    ::option
    ::Option<terrars::ListField<ComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match_SrcSecureTag>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    src_threat_intelligences: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match {
    #[doc = "Set the field `dest_address_groups`.\n"]
    pub fn set_dest_address_groups(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dest_address_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dest_fqdns`.\n"]
    pub fn set_dest_fqdns(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dest_fqdns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dest_ip_ranges`.\n"]
    pub fn set_dest_ip_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dest_ip_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dest_region_codes`.\n"]
    pub fn set_dest_region_codes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dest_region_codes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dest_threat_intelligences`.\n"]
    pub fn set_dest_threat_intelligences(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dest_threat_intelligences = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `layer4_config`.\n"]
    pub fn set_layer4_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<ComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match_Layer4Config>,
                    >,
    ) -> Self {
        self.layer4_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `src_address_groups`.\n"]
    pub fn set_src_address_groups(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.src_address_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `src_fqdns`.\n"]
    pub fn set_src_fqdns(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.src_fqdns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `src_ip_ranges`.\n"]
    pub fn set_src_ip_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.src_ip_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `src_region_codes`.\n"]
    pub fn set_src_region_codes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.src_region_codes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `src_secure_tag`.\n"]
    pub fn set_src_secure_tag(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<ComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match_SrcSecureTag>,
                    >,
    ) -> Self {
        self.src_secure_tag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `src_threat_intelligences`.\n"]
    pub fn set_src_threat_intelligences(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.src_threat_intelligences = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match {
    type O = terrars::BlockAssignable<ComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match {}

impl BuildComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match {
    pub fn build(self) -> ComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match {
        ComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match {
            dest_address_groups: core::default::Default::default(),
            dest_fqdns: core::default::Default::default(),
            dest_ip_ranges: core::default::Default::default(),
            dest_region_codes: core::default::Default::default(),
            dest_threat_intelligences: core::default::Default::default(),
            layer4_config: core::default::Default::default(),
            src_address_groups: core::default::Default::default(),
            src_fqdns: core::default::Default::default(),
            src_ip_ranges: core::default::Default::default(),
            src_region_codes: core::default::Default::default(),
            src_secure_tag: core::default::Default::default(),
            src_threat_intelligences: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeNetworkFirewallPolicyWithRules_PredefinedRules_MatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeNetworkFirewallPolicyWithRules_PredefinedRules_MatchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeNetworkFirewallPolicyWithRules_PredefinedRules_MatchRef {
        ComputeNetworkFirewallPolicyWithRules_PredefinedRules_MatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeNetworkFirewallPolicyWithRules_PredefinedRules_MatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dest_address_groups` after provisioning.\n"]
    pub fn dest_address_groups(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dest_address_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `dest_fqdns` after provisioning.\n"]
    pub fn dest_fqdns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dest_fqdns", self.base))
    }

    #[doc = "Get a reference to the value of field `dest_ip_ranges` after provisioning.\n"]
    pub fn dest_ip_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dest_ip_ranges", self.base))
    }

    #[doc = "Get a reference to the value of field `dest_region_codes` after provisioning.\n"]
    pub fn dest_region_codes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dest_region_codes", self.base))
    }

    #[doc = "Get a reference to the value of field `dest_threat_intelligences` after provisioning.\n"]
    pub fn dest_threat_intelligences(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dest_threat_intelligences", self.base))
    }

    #[doc = "Get a reference to the value of field `layer4_config` after provisioning.\n"]
    pub fn layer4_config(
        &self,
    ) -> terrars::ListRef<ComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match_Layer4ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.layer4_config", self.base))
    }

    #[doc = "Get a reference to the value of field `src_address_groups` after provisioning.\n"]
    pub fn src_address_groups(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.src_address_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `src_fqdns` after provisioning.\n"]
    pub fn src_fqdns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.src_fqdns", self.base))
    }

    #[doc = "Get a reference to the value of field `src_ip_ranges` after provisioning.\n"]
    pub fn src_ip_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.src_ip_ranges", self.base))
    }

    #[doc = "Get a reference to the value of field `src_region_codes` after provisioning.\n"]
    pub fn src_region_codes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.src_region_codes", self.base))
    }

    #[doc = "Get a reference to the value of field `src_secure_tag` after provisioning.\n"]
    pub fn src_secure_tag(
        &self,
    ) -> terrars::ListRef<ComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match_SrcSecureTagRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.src_secure_tag", self.base))
    }

    #[doc = "Get a reference to the value of field `src_threat_intelligences` after provisioning.\n"]
    pub fn src_threat_intelligences(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.src_threat_intelligences", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeNetworkFirewallPolicyWithRules_PredefinedRules_TargetSecureTag {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeNetworkFirewallPolicyWithRules_PredefinedRules_TargetSecureTag {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeNetworkFirewallPolicyWithRules_PredefinedRules_TargetSecureTag {
    type O = terrars::BlockAssignable<ComputeNetworkFirewallPolicyWithRules_PredefinedRules_TargetSecureTag>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeNetworkFirewallPolicyWithRules_PredefinedRules_TargetSecureTag {}

impl BuildComputeNetworkFirewallPolicyWithRules_PredefinedRules_TargetSecureTag {
    pub fn build(self) -> ComputeNetworkFirewallPolicyWithRules_PredefinedRules_TargetSecureTag {
        ComputeNetworkFirewallPolicyWithRules_PredefinedRules_TargetSecureTag {
            name: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeNetworkFirewallPolicyWithRules_PredefinedRules_TargetSecureTagRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeNetworkFirewallPolicyWithRules_PredefinedRules_TargetSecureTagRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeNetworkFirewallPolicyWithRules_PredefinedRules_TargetSecureTagRef {
        ComputeNetworkFirewallPolicyWithRules_PredefinedRules_TargetSecureTagRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeNetworkFirewallPolicyWithRules_PredefinedRules_TargetSecureTagRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeNetworkFirewallPolicyWithRules_PredefinedRules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_logging: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<terrars::ListField<ComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_profile_group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_secure_tag: std
    ::option
    ::Option<terrars::ListField<ComputeNetworkFirewallPolicyWithRules_PredefinedRules_TargetSecureTag>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_service_accounts: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_inspect: std::option::Option<terrars::PrimField<bool>>,
}

impl ComputeNetworkFirewallPolicyWithRules_PredefinedRules {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `direction`.\n"]
    pub fn set_direction(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.direction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_logging`.\n"]
    pub fn set_enable_logging(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_logging = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ComputeNetworkFirewallPolicyWithRules_PredefinedRules_Match>>,
    ) -> Self {
        self.match_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.priority = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule_name`.\n"]
    pub fn set_rule_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rule_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_profile_group`.\n"]
    pub fn set_security_profile_group(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.security_profile_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_secure_tag`.\n"]
    pub fn set_target_secure_tag(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<ComputeNetworkFirewallPolicyWithRules_PredefinedRules_TargetSecureTag>>,
    ) -> Self {
        self.target_secure_tag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_service_accounts`.\n"]
    pub fn set_target_service_accounts(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.target_service_accounts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tls_inspect`.\n"]
    pub fn set_tls_inspect(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.tls_inspect = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeNetworkFirewallPolicyWithRules_PredefinedRules {
    type O = terrars::BlockAssignable<ComputeNetworkFirewallPolicyWithRules_PredefinedRules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeNetworkFirewallPolicyWithRules_PredefinedRules {}

impl BuildComputeNetworkFirewallPolicyWithRules_PredefinedRules {
    pub fn build(self) -> ComputeNetworkFirewallPolicyWithRules_PredefinedRules {
        ComputeNetworkFirewallPolicyWithRules_PredefinedRules {
            action: core::default::Default::default(),
            description: core::default::Default::default(),
            direction: core::default::Default::default(),
            disabled: core::default::Default::default(),
            enable_logging: core::default::Default::default(),
            match_: core::default::Default::default(),
            priority: core::default::Default::default(),
            rule_name: core::default::Default::default(),
            security_profile_group: core::default::Default::default(),
            target_secure_tag: core::default::Default::default(),
            target_service_accounts: core::default::Default::default(),
            tls_inspect: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeNetworkFirewallPolicyWithRules_PredefinedRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeNetworkFirewallPolicyWithRules_PredefinedRulesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeNetworkFirewallPolicyWithRules_PredefinedRulesRef {
        ComputeNetworkFirewallPolicyWithRules_PredefinedRulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeNetworkFirewallPolicyWithRules_PredefinedRulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `direction` after provisioning.\n"]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.base))
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_logging` after provisioning.\n"]
    pub fn enable_logging(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_logging", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<ComputeNetworkFirewallPolicyWithRules_PredefinedRules_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_name` after provisioning.\n"]
    pub fn rule_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_name", self.base))
    }

    #[doc = "Get a reference to the value of field `security_profile_group` after provisioning.\n"]
    pub fn security_profile_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_profile_group", self.base))
    }

    #[doc = "Get a reference to the value of field `target_secure_tag` after provisioning.\n"]
    pub fn target_secure_tag(
        &self,
    ) -> terrars::ListRef<ComputeNetworkFirewallPolicyWithRules_PredefinedRules_TargetSecureTagRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_secure_tag", self.base))
    }

    #[doc = "Get a reference to the value of field `target_service_accounts` after provisioning.\n"]
    pub fn target_service_accounts(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_service_accounts", self.base))
    }

    #[doc = "Get a reference to the value of field `tls_inspect` after provisioning.\n"]
    pub fn tls_inspect(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls_inspect", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeNetworkFirewallPolicyWithRules_Rule_Match_Layer4Config {
    ip_protocol: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ports: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ComputeNetworkFirewallPolicyWithRules_Rule_Match_Layer4Config {
    #[doc =
        "Set the field `ports`.\nAn optional list of ports to which this rule applies. This field\nis only applicable for UDP or TCP protocol. Each entry must be\neither an integer or a range. If not specified, this rule\napplies to connections through any port.\nExample inputs include: [\"22\"], [\"80\",\"443\"], and\n[\"12345-12349\"]."]
    pub fn set_ports(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ports = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeNetworkFirewallPolicyWithRules_Rule_Match_Layer4Config {
    type O = terrars::BlockAssignable<ComputeNetworkFirewallPolicyWithRules_Rule_Match_Layer4Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeNetworkFirewallPolicyWithRules_Rule_Match_Layer4Config {
    #[doc =
        "The IP protocol to which this rule applies. The protocol\ntype is required when creating a firewall rule.\nThis value can either be one of the following well\nknown protocol strings (tcp, udp, icmp, esp, ah, ipip, sctp),\nor the IP protocol number."]
    pub ip_protocol: terrars::PrimField<std::string::String>,
}

impl BuildComputeNetworkFirewallPolicyWithRules_Rule_Match_Layer4Config {
    pub fn build(self) -> ComputeNetworkFirewallPolicyWithRules_Rule_Match_Layer4Config {
        ComputeNetworkFirewallPolicyWithRules_Rule_Match_Layer4Config {
            ip_protocol: self.ip_protocol,
            ports: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeNetworkFirewallPolicyWithRules_Rule_Match_Layer4ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeNetworkFirewallPolicyWithRules_Rule_Match_Layer4ConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeNetworkFirewallPolicyWithRules_Rule_Match_Layer4ConfigRef {
        ComputeNetworkFirewallPolicyWithRules_Rule_Match_Layer4ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeNetworkFirewallPolicyWithRules_Rule_Match_Layer4ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ip_protocol` after provisioning.\nThe IP protocol to which this rule applies. The protocol\ntype is required when creating a firewall rule.\nThis value can either be one of the following well\nknown protocol strings (tcp, udp, icmp, esp, ah, ipip, sctp),\nor the IP protocol number."]
    pub fn ip_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_protocol", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ports` after provisioning.\nAn optional list of ports to which this rule applies. This field\nis only applicable for UDP or TCP protocol. Each entry must be\neither an integer or a range. If not specified, this rule\napplies to connections through any port.\nExample inputs include: [\"22\"], [\"80\",\"443\"], and\n[\"12345-12349\"]."]
    pub fn ports(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeNetworkFirewallPolicyWithRules_Rule_Match_SrcSecureTag {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeNetworkFirewallPolicyWithRules_Rule_Match_SrcSecureTag {
    #[doc =
        "Set the field `name`.\nName of the secure tag, created with TagManager's TagValue API.\n@pattern tagValues/[0-9]+"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeNetworkFirewallPolicyWithRules_Rule_Match_SrcSecureTag {
    type O = terrars::BlockAssignable<ComputeNetworkFirewallPolicyWithRules_Rule_Match_SrcSecureTag>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeNetworkFirewallPolicyWithRules_Rule_Match_SrcSecureTag {}

impl BuildComputeNetworkFirewallPolicyWithRules_Rule_Match_SrcSecureTag {
    pub fn build(self) -> ComputeNetworkFirewallPolicyWithRules_Rule_Match_SrcSecureTag {
        ComputeNetworkFirewallPolicyWithRules_Rule_Match_SrcSecureTag { name: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ComputeNetworkFirewallPolicyWithRules_Rule_Match_SrcSecureTagRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeNetworkFirewallPolicyWithRules_Rule_Match_SrcSecureTagRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeNetworkFirewallPolicyWithRules_Rule_Match_SrcSecureTagRef {
        ComputeNetworkFirewallPolicyWithRules_Rule_Match_SrcSecureTagRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeNetworkFirewallPolicyWithRules_Rule_Match_SrcSecureTagRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the secure tag, created with TagManager's TagValue API.\n@pattern tagValues/[0-9]+"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\n[Output Only] State of the secure tag, either 'EFFECTIVE' or\n'INEFFECTIVE'. A secure tag is 'INEFFECTIVE' when it is deleted\nor its network is deleted."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeNetworkFirewallPolicyWithRules_Rule_MatchDynamic {
    layer4_config: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeNetworkFirewallPolicyWithRules_Rule_Match_Layer4Config>>,
    src_secure_tag: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeNetworkFirewallPolicyWithRules_Rule_Match_SrcSecureTag>>,
}

#[derive(Serialize)]
pub struct ComputeNetworkFirewallPolicyWithRules_Rule_Match {
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
    layer4_config: std::option::Option<std::vec::Vec<ComputeNetworkFirewallPolicyWithRules_Rule_Match_Layer4Config>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    src_secure_tag: std::option::Option<std::vec::Vec<ComputeNetworkFirewallPolicyWithRules_Rule_Match_SrcSecureTag>>,
    dynamic: ComputeNetworkFirewallPolicyWithRules_Rule_MatchDynamic,
}

impl ComputeNetworkFirewallPolicyWithRules_Rule_Match {
    #[doc =
        "Set the field `dest_address_groups`.\nAddress groups which should be matched against the traffic destination.\nMaximum number of destination address groups is 10."]
    pub fn set_dest_address_groups(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dest_address_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `dest_fqdns`.\nFully Qualified Domain Name (FQDN) which should be matched against\ntraffic destination. Maximum number of destination fqdn allowed is 100."]
    pub fn set_dest_fqdns(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dest_fqdns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dest_ip_ranges`.\nDestination IP address range in CIDR format. Required for\nEGRESS rules."]
    pub fn set_dest_ip_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dest_ip_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `dest_region_codes`.\nRegion codes whose IP addresses will be used to match for destination\nof traffic. Should be specified as 2 letter country code defined as per\nISO 3166 alpha-2 country codes. ex.\"US\"\nMaximum number of destination region codes allowed is 5000."]
    pub fn set_dest_region_codes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dest_region_codes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `dest_threat_intelligences`.\nNames of Network Threat Intelligence lists.\nThe IPs in these lists will be matched against traffic destination."]
    pub fn set_dest_threat_intelligences(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dest_threat_intelligences = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `src_address_groups`.\nAddress groups which should be matched against the traffic source.\nMaximum number of source address groups is 10."]
    pub fn set_src_address_groups(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.src_address_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `src_fqdns`.\nFully Qualified Domain Name (FQDN) which should be matched against\ntraffic source. Maximum number of source fqdn allowed is 100."]
    pub fn set_src_fqdns(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.src_fqdns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `src_ip_ranges`.\nSource IP address range in CIDR format. Required for\nINGRESS rules."]
    pub fn set_src_ip_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.src_ip_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `src_region_codes`.\nRegion codes whose IP addresses will be used to match for source\nof traffic. Should be specified as 2 letter country code defined as per\nISO 3166 alpha-2 country codes. ex.\"US\"\nMaximum number of source region codes allowed is 5000."]
    pub fn set_src_region_codes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.src_region_codes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `src_threat_intelligences`.\nNames of Network Threat Intelligence lists.\nThe IPs in these lists will be matched against traffic source."]
    pub fn set_src_threat_intelligences(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.src_threat_intelligences = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `layer4_config`.\n"]
    pub fn set_layer4_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeNetworkFirewallPolicyWithRules_Rule_Match_Layer4Config>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.layer4_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.layer4_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `src_secure_tag`.\n"]
    pub fn set_src_secure_tag(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeNetworkFirewallPolicyWithRules_Rule_Match_SrcSecureTag>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.src_secure_tag = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.src_secure_tag = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeNetworkFirewallPolicyWithRules_Rule_Match {
    type O = terrars::BlockAssignable<ComputeNetworkFirewallPolicyWithRules_Rule_Match>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeNetworkFirewallPolicyWithRules_Rule_Match {}

impl BuildComputeNetworkFirewallPolicyWithRules_Rule_Match {
    pub fn build(self) -> ComputeNetworkFirewallPolicyWithRules_Rule_Match {
        ComputeNetworkFirewallPolicyWithRules_Rule_Match {
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
            layer4_config: core::default::Default::default(),
            src_secure_tag: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeNetworkFirewallPolicyWithRules_Rule_MatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeNetworkFirewallPolicyWithRules_Rule_MatchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeNetworkFirewallPolicyWithRules_Rule_MatchRef {
        ComputeNetworkFirewallPolicyWithRules_Rule_MatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeNetworkFirewallPolicyWithRules_Rule_MatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dest_address_groups` after provisioning.\nAddress groups which should be matched against the traffic destination.\nMaximum number of destination address groups is 10."]
    pub fn dest_address_groups(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dest_address_groups", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dest_fqdns` after provisioning.\nFully Qualified Domain Name (FQDN) which should be matched against\ntraffic destination. Maximum number of destination fqdn allowed is 100."]
    pub fn dest_fqdns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dest_fqdns", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dest_ip_ranges` after provisioning.\nDestination IP address range in CIDR format. Required for\nEGRESS rules."]
    pub fn dest_ip_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dest_ip_ranges", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dest_region_codes` after provisioning.\nRegion codes whose IP addresses will be used to match for destination\nof traffic. Should be specified as 2 letter country code defined as per\nISO 3166 alpha-2 country codes. ex.\"US\"\nMaximum number of destination region codes allowed is 5000."]
    pub fn dest_region_codes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dest_region_codes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dest_threat_intelligences` after provisioning.\nNames of Network Threat Intelligence lists.\nThe IPs in these lists will be matched against traffic destination."]
    pub fn dest_threat_intelligences(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dest_threat_intelligences", self.base))
    }

    #[doc =
        "Get a reference to the value of field `src_address_groups` after provisioning.\nAddress groups which should be matched against the traffic source.\nMaximum number of source address groups is 10."]
    pub fn src_address_groups(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.src_address_groups", self.base))
    }

    #[doc =
        "Get a reference to the value of field `src_fqdns` after provisioning.\nFully Qualified Domain Name (FQDN) which should be matched against\ntraffic source. Maximum number of source fqdn allowed is 100."]
    pub fn src_fqdns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.src_fqdns", self.base))
    }

    #[doc =
        "Get a reference to the value of field `src_ip_ranges` after provisioning.\nSource IP address range in CIDR format. Required for\nINGRESS rules."]
    pub fn src_ip_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.src_ip_ranges", self.base))
    }

    #[doc =
        "Get a reference to the value of field `src_region_codes` after provisioning.\nRegion codes whose IP addresses will be used to match for source\nof traffic. Should be specified as 2 letter country code defined as per\nISO 3166 alpha-2 country codes. ex.\"US\"\nMaximum number of source region codes allowed is 5000."]
    pub fn src_region_codes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.src_region_codes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `src_threat_intelligences` after provisioning.\nNames of Network Threat Intelligence lists.\nThe IPs in these lists will be matched against traffic source."]
    pub fn src_threat_intelligences(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.src_threat_intelligences", self.base))
    }

    #[doc = "Get a reference to the value of field `layer4_config` after provisioning.\n"]
    pub fn layer4_config(&self) -> terrars::ListRef<ComputeNetworkFirewallPolicyWithRules_Rule_Match_Layer4ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.layer4_config", self.base))
    }

    #[doc = "Get a reference to the value of field `src_secure_tag` after provisioning.\n"]
    pub fn src_secure_tag(&self) -> terrars::ListRef<ComputeNetworkFirewallPolicyWithRules_Rule_Match_SrcSecureTagRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.src_secure_tag", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeNetworkFirewallPolicyWithRules_Rule_TargetSecureTag {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeNetworkFirewallPolicyWithRules_Rule_TargetSecureTag {
    #[doc =
        "Set the field `name`.\nName of the secure tag, created with TagManager's TagValue API.\n@pattern tagValues/[0-9]+"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeNetworkFirewallPolicyWithRules_Rule_TargetSecureTag {
    type O = terrars::BlockAssignable<ComputeNetworkFirewallPolicyWithRules_Rule_TargetSecureTag>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeNetworkFirewallPolicyWithRules_Rule_TargetSecureTag {}

impl BuildComputeNetworkFirewallPolicyWithRules_Rule_TargetSecureTag {
    pub fn build(self) -> ComputeNetworkFirewallPolicyWithRules_Rule_TargetSecureTag {
        ComputeNetworkFirewallPolicyWithRules_Rule_TargetSecureTag { name: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ComputeNetworkFirewallPolicyWithRules_Rule_TargetSecureTagRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeNetworkFirewallPolicyWithRules_Rule_TargetSecureTagRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeNetworkFirewallPolicyWithRules_Rule_TargetSecureTagRef {
        ComputeNetworkFirewallPolicyWithRules_Rule_TargetSecureTagRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeNetworkFirewallPolicyWithRules_Rule_TargetSecureTagRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the secure tag, created with TagManager's TagValue API.\n@pattern tagValues/[0-9]+"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\n[Output Only] State of the secure tag, either 'EFFECTIVE' or\n'INEFFECTIVE'. A secure tag is 'INEFFECTIVE' when it is deleted\nor its network is deleted."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeNetworkFirewallPolicyWithRules_RuleDynamic {
    match_: std::option::Option<terrars::DynamicBlock<ComputeNetworkFirewallPolicyWithRules_Rule_Match>>,
    target_secure_tag: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeNetworkFirewallPolicyWithRules_Rule_TargetSecureTag>>,
}

#[derive(Serialize)]
pub struct ComputeNetworkFirewallPolicyWithRules_Rule {
    action: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_logging: std::option::Option<terrars::PrimField<bool>>,
    priority: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_profile_group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_service_accounts: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_inspect: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<std::vec::Vec<ComputeNetworkFirewallPolicyWithRules_Rule_Match>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_secure_tag: std::option::Option<std::vec::Vec<ComputeNetworkFirewallPolicyWithRules_Rule_TargetSecureTag>>,
    dynamic: ComputeNetworkFirewallPolicyWithRules_RuleDynamic,
}

impl ComputeNetworkFirewallPolicyWithRules_Rule {
    #[doc = "Set the field `description`.\nA description of the rule."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `direction`.\nThe direction in which this rule applies. If unspecified an INGRESS rule is created. Possible values: [\"INGRESS\", \"EGRESS\"]"]
    pub fn set_direction(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.direction = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disabled`.\nDenotes whether the firewall policy rule is disabled. When set to true,\nthe firewall policy rule is not enforced and traffic behaves as if it did\nnot exist. If this is unspecified, the firewall policy rule will be\nenabled."]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_logging`.\nDenotes whether to enable logging for a particular rule.\nIf logging is enabled, logs will be exported to the\nconfigured export destination in Stackdriver."]
    pub fn set_enable_logging(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_logging = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `rule_name`.\nAn optional name for the rule. This field is not a unique identifier\nand can be updated."]
    pub fn set_rule_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rule_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `security_profile_group`.\nA fully-qualified URL of a SecurityProfile resource instance.\nExample:\nhttps://networksecurity.googleapis.com/v1/projects/{project}/locations/{location}/securityProfileGroups/my-security-profile-group\nMust be specified if action is 'apply_security_profile_group'."]
    pub fn set_security_profile_group(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.security_profile_group = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `target_service_accounts`.\nA list of service accounts indicating the sets of\ninstances that are applied with this rule."]
    pub fn set_target_service_accounts(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.target_service_accounts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tls_inspect`.\nBoolean flag indicating if the traffic should be TLS decrypted.\nIt can be set only if action = 'apply_security_profile_group' and cannot be set for other actions."]
    pub fn set_tls_inspect(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.tls_inspect = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeNetworkFirewallPolicyWithRules_Rule_Match>>,
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

    #[doc = "Set the field `target_secure_tag`.\n"]
    pub fn set_target_secure_tag(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeNetworkFirewallPolicyWithRules_Rule_TargetSecureTag>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.target_secure_tag = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.target_secure_tag = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeNetworkFirewallPolicyWithRules_Rule {
    type O = terrars::BlockAssignable<ComputeNetworkFirewallPolicyWithRules_Rule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeNetworkFirewallPolicyWithRules_Rule {
    #[doc =
        "The Action to perform when the client connection triggers the rule. Can currently be either\n\"allow\", \"deny\", \"apply_security_profile_group\" or \"goto_next\"."]
    pub action: terrars::PrimField<std::string::String>,
    #[doc =
        "An integer indicating the priority of a rule in the list. The priority must be a value\nbetween 0 and 2147483647. Rules are evaluated from highest to lowest priority where 0 is the\nhighest priority and 2147483647 is the lowest priority."]
    pub priority: terrars::PrimField<f64>,
}

impl BuildComputeNetworkFirewallPolicyWithRules_Rule {
    pub fn build(self) -> ComputeNetworkFirewallPolicyWithRules_Rule {
        ComputeNetworkFirewallPolicyWithRules_Rule {
            action: self.action,
            description: core::default::Default::default(),
            direction: core::default::Default::default(),
            disabled: core::default::Default::default(),
            enable_logging: core::default::Default::default(),
            priority: self.priority,
            rule_name: core::default::Default::default(),
            security_profile_group: core::default::Default::default(),
            target_service_accounts: core::default::Default::default(),
            tls_inspect: core::default::Default::default(),
            match_: core::default::Default::default(),
            target_secure_tag: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeNetworkFirewallPolicyWithRules_RuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeNetworkFirewallPolicyWithRules_RuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeNetworkFirewallPolicyWithRules_RuleRef {
        ComputeNetworkFirewallPolicyWithRules_RuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeNetworkFirewallPolicyWithRules_RuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action` after provisioning.\nThe Action to perform when the client connection triggers the rule. Can currently be either\n\"allow\", \"deny\", \"apply_security_profile_group\" or \"goto_next\"."]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of the rule."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `direction` after provisioning.\nThe direction in which this rule applies. If unspecified an INGRESS rule is created. Possible values: [\"INGRESS\", \"EGRESS\"]"]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nDenotes whether the firewall policy rule is disabled. When set to true,\nthe firewall policy rule is not enforced and traffic behaves as if it did\nnot exist. If this is unspecified, the firewall policy rule will be\nenabled."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_logging` after provisioning.\nDenotes whether to enable logging for a particular rule.\nIf logging is enabled, logs will be exported to the\nconfigured export destination in Stackdriver."]
    pub fn enable_logging(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_logging", self.base))
    }

    #[doc =
        "Get a reference to the value of field `priority` after provisioning.\nAn integer indicating the priority of a rule in the list. The priority must be a value\nbetween 0 and 2147483647. Rules are evaluated from highest to lowest priority where 0 is the\nhighest priority and 2147483647 is the lowest priority."]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rule_name` after provisioning.\nAn optional name for the rule. This field is not a unique identifier\nand can be updated."]
    pub fn rule_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `security_profile_group` after provisioning.\nA fully-qualified URL of a SecurityProfile resource instance.\nExample:\nhttps://networksecurity.googleapis.com/v1/projects/{project}/locations/{location}/securityProfileGroups/my-security-profile-group\nMust be specified if action is 'apply_security_profile_group'."]
    pub fn security_profile_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_profile_group", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_service_accounts` after provisioning.\nA list of service accounts indicating the sets of\ninstances that are applied with this rule."]
    pub fn target_service_accounts(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_service_accounts", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tls_inspect` after provisioning.\nBoolean flag indicating if the traffic should be TLS decrypted.\nIt can be set only if action = 'apply_security_profile_group' and cannot be set for other actions."]
    pub fn tls_inspect(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls_inspect", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<ComputeNetworkFirewallPolicyWithRules_Rule_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `target_secure_tag` after provisioning.\n"]
    pub fn target_secure_tag(&self) -> terrars::ListRef<ComputeNetworkFirewallPolicyWithRules_Rule_TargetSecureTagRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_secure_tag", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeNetworkFirewallPolicyWithRules_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeNetworkFirewallPolicyWithRules_Timeouts {
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

impl terrars::ToListMappable for ComputeNetworkFirewallPolicyWithRules_Timeouts {
    type O = terrars::BlockAssignable<ComputeNetworkFirewallPolicyWithRules_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeNetworkFirewallPolicyWithRules_Timeouts {}

impl BuildComputeNetworkFirewallPolicyWithRules_Timeouts {
    pub fn build(self) -> ComputeNetworkFirewallPolicyWithRules_Timeouts {
        ComputeNetworkFirewallPolicyWithRules_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeNetworkFirewallPolicyWithRules_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeNetworkFirewallPolicyWithRules_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeNetworkFirewallPolicyWithRules_TimeoutsRef {
        ComputeNetworkFirewallPolicyWithRules_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeNetworkFirewallPolicyWithRules_TimeoutsRef {
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
