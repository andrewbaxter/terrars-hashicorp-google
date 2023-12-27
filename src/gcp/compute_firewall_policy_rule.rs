use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeFirewallPolicyRuleData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    action: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    direction: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_logging: Option<PrimField<bool>>,
    firewall_policy: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    priority: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_resources: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_service_accounts: Option<ListField<PrimField<String>>>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<Vec<ComputeFirewallPolicyRuleMatchEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeFirewallPolicyRuleTimeoutsEl>,
    dynamic: ComputeFirewallPolicyRuleDynamic,
}

struct ComputeFirewallPolicyRule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeFirewallPolicyRuleData>,
}

#[derive(Clone)]
pub struct ComputeFirewallPolicyRule(Rc<ComputeFirewallPolicyRule_>);

impl ComputeFirewallPolicyRule {
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

    #[doc= "Set the field `description`.\nAn optional description for this resource."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `disabled`.\nDenotes whether the firewall policy rule is disabled. When set to true, the firewall policy rule is not enforced and traffic behaves as if it did not exist. If this is unspecified, the firewall policy rule will be enabled."]
    pub fn set_disabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disabled = Some(v.into());
        self
    }

    #[doc= "Set the field `enable_logging`.\nDenotes whether to enable logging for a particular rule. If logging is enabled, logs will be exported to the configured export destination in Stackdriver. Logs may be exported to BigQuery or Pub/Sub. Note: you cannot enable logging on \"goto_next\" rules."]
    pub fn set_enable_logging(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_logging = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `target_resources`.\nA list of network resource URLs to which this rule applies. This field allows you to control which network's VMs get this rule. If this field is left blank, all VMs within the organization will receive the rule."]
    pub fn set_target_resources(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().target_resources = Some(v.into());
        self
    }

    #[doc= "Set the field `target_service_accounts`.\nA list of service accounts indicating the sets of instances that are applied with this rule."]
    pub fn set_target_service_accounts(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().target_service_accounts = Some(v.into());
        self
    }

    #[doc= "Set the field `match_`.\n"]
    pub fn set_match(self, v: impl Into<BlockAssignable<ComputeFirewallPolicyRuleMatchEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().match_ = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.match_ = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeFirewallPolicyRuleTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `action` after provisioning.\nThe Action to perform when the client connection triggers the rule. Valid actions are \"allow\", \"deny\" and \"goto_next\"."]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn optional description for this resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `direction` after provisioning.\nThe direction in which this rule applies. Possible values: INGRESS, EGRESS"]
    pub fn direction(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.direction", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disabled` after provisioning.\nDenotes whether the firewall policy rule is disabled. When set to true, the firewall policy rule is not enforced and traffic behaves as if it did not exist. If this is unspecified, the firewall policy rule will be enabled."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_logging` after provisioning.\nDenotes whether to enable logging for a particular rule. If logging is enabled, logs will be exported to the configured export destination in Stackdriver. Logs may be exported to BigQuery or Pub/Sub. Note: you cannot enable logging on \"goto_next\" rules."]
    pub fn enable_logging(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_logging", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `firewall_policy` after provisioning.\nThe firewall policy of the resource."]
    pub fn firewall_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.firewall_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kind` after provisioning.\nType of the resource. Always `compute#firewallPolicyRule` for firewall policy rules"]
    pub fn kind(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `priority` after provisioning.\nAn integer indicating the priority of a rule in the list. The priority must be a positive value between 0 and 2147483647. Rules are evaluated from highest to lowest priority where 0 is the highest priority and 2147483647 is the lowest prority."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rule_tuple_count` after provisioning.\nCalculation of the complexity of a single firewall policy rule."]
    pub fn rule_tuple_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_tuple_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `target_resources` after provisioning.\nA list of network resource URLs to which this rule applies. This field allows you to control which network's VMs get this rule. If this field is left blank, all VMs within the organization will receive the rule."]
    pub fn target_resources(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.target_resources", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `target_service_accounts` after provisioning.\nA list of service accounts indicating the sets of instances that are applied with this rule."]
    pub fn target_service_accounts(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.target_service_accounts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<ComputeFirewallPolicyRuleMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeFirewallPolicyRuleTimeoutsElRef {
        ComputeFirewallPolicyRuleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for ComputeFirewallPolicyRule {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ComputeFirewallPolicyRule { }

impl ToListMappable for ComputeFirewallPolicyRule {
    type O = ListRef<ComputeFirewallPolicyRuleRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ComputeFirewallPolicyRule_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_firewall_policy_rule".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeFirewallPolicyRule {
    pub tf_id: String,
    #[doc= "The Action to perform when the client connection triggers the rule. Valid actions are \"allow\", \"deny\" and \"goto_next\"."]
    pub action: PrimField<String>,
    #[doc= "The direction in which this rule applies. Possible values: INGRESS, EGRESS"]
    pub direction: PrimField<String>,
    #[doc= "The firewall policy of the resource."]
    pub firewall_policy: PrimField<String>,
    #[doc= "An integer indicating the priority of a rule in the list. The priority must be a positive value between 0 and 2147483647. Rules are evaluated from highest to lowest priority where 0 is the highest priority and 2147483647 is the lowest prority."]
    pub priority: PrimField<f64>,
}

impl BuildComputeFirewallPolicyRule {
    pub fn build(self, stack: &mut Stack) -> ComputeFirewallPolicyRule {
        let out = ComputeFirewallPolicyRule(Rc::new(ComputeFirewallPolicyRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeFirewallPolicyRuleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                action: self.action,
                description: core::default::Default::default(),
                direction: self.direction,
                disabled: core::default::Default::default(),
                enable_logging: core::default::Default::default(),
                firewall_policy: self.firewall_policy,
                id: core::default::Default::default(),
                priority: self.priority,
                target_resources: core::default::Default::default(),
                target_service_accounts: core::default::Default::default(),
                match_: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeFirewallPolicyRuleRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeFirewallPolicyRuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeFirewallPolicyRuleRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `action` after provisioning.\nThe Action to perform when the client connection triggers the rule. Valid actions are \"allow\", \"deny\" and \"goto_next\"."]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nAn optional description for this resource."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `direction` after provisioning.\nThe direction in which this rule applies. Possible values: INGRESS, EGRESS"]
    pub fn direction(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.direction", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disabled` after provisioning.\nDenotes whether the firewall policy rule is disabled. When set to true, the firewall policy rule is not enforced and traffic behaves as if it did not exist. If this is unspecified, the firewall policy rule will be enabled."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_logging` after provisioning.\nDenotes whether to enable logging for a particular rule. If logging is enabled, logs will be exported to the configured export destination in Stackdriver. Logs may be exported to BigQuery or Pub/Sub. Note: you cannot enable logging on \"goto_next\" rules."]
    pub fn enable_logging(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_logging", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `firewall_policy` after provisioning.\nThe firewall policy of the resource."]
    pub fn firewall_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.firewall_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kind` after provisioning.\nType of the resource. Always `compute#firewallPolicyRule` for firewall policy rules"]
    pub fn kind(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `priority` after provisioning.\nAn integer indicating the priority of a rule in the list. The priority must be a positive value between 0 and 2147483647. Rules are evaluated from highest to lowest priority where 0 is the highest priority and 2147483647 is the lowest prority."]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `rule_tuple_count` after provisioning.\nCalculation of the complexity of a single firewall policy rule."]
    pub fn rule_tuple_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_tuple_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `target_resources` after provisioning.\nA list of network resource URLs to which this rule applies. This field allows you to control which network's VMs get this rule. If this field is left blank, all VMs within the organization will receive the rule."]
    pub fn target_resources(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.target_resources", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `target_service_accounts` after provisioning.\nA list of service accounts indicating the sets of instances that are applied with this rule."]
    pub fn target_service_accounts(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.target_service_accounts", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<ComputeFirewallPolicyRuleMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeFirewallPolicyRuleTimeoutsElRef {
        ComputeFirewallPolicyRuleTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct ComputeFirewallPolicyRuleMatchElLayer4ConfigsEl {
    ip_protocol: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ports: Option<ListField<PrimField<String>>>,
}

impl ComputeFirewallPolicyRuleMatchElLayer4ConfigsEl {
    #[doc= "Set the field `ports`.\nAn optional list of ports to which this rule applies. This field is only applicable for UDP or TCP protocol. Each entry must be either an integer or a range. If not specified, this rule applies to connections through any port. Example inputs include: ``."]
    pub fn set_ports(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ports = Some(v.into());
        self
    }
}

impl ToListMappable for ComputeFirewallPolicyRuleMatchElLayer4ConfigsEl {
    type O = BlockAssignable<ComputeFirewallPolicyRuleMatchElLayer4ConfigsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeFirewallPolicyRuleMatchElLayer4ConfigsEl {
    #[doc= "The IP protocol to which this rule applies. The protocol type is required when creating a firewall rule. This value can either be one of the following well known protocol strings (`tcp`, `udp`, `icmp`, `esp`, `ah`, `ipip`, `sctp`), or the IP protocol number."]
    pub ip_protocol: PrimField<String>,
}

impl BuildComputeFirewallPolicyRuleMatchElLayer4ConfigsEl {
    pub fn build(self) -> ComputeFirewallPolicyRuleMatchElLayer4ConfigsEl {
        ComputeFirewallPolicyRuleMatchElLayer4ConfigsEl {
            ip_protocol: self.ip_protocol,
            ports: core::default::Default::default(),
        }
    }
}

pub struct ComputeFirewallPolicyRuleMatchElLayer4ConfigsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeFirewallPolicyRuleMatchElLayer4ConfigsElRef {
    fn new(shared: StackShared, base: String) -> ComputeFirewallPolicyRuleMatchElLayer4ConfigsElRef {
        ComputeFirewallPolicyRuleMatchElLayer4ConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeFirewallPolicyRuleMatchElLayer4ConfigsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `ip_protocol` after provisioning.\nThe IP protocol to which this rule applies. The protocol type is required when creating a firewall rule. This value can either be one of the following well known protocol strings (`tcp`, `udp`, `icmp`, `esp`, `ah`, `ipip`, `sctp`), or the IP protocol number."]
    pub fn ip_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip_protocol", self.base))
    }

    #[doc= "Get a reference to the value of field `ports` after provisioning.\nAn optional list of ports to which this rule applies. This field is only applicable for UDP or TCP protocol. Each entry must be either an integer or a range. If not specified, this rule applies to connections through any port. Example inputs include: ``."]
    pub fn ports(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeFirewallPolicyRuleMatchElDynamic {
    layer4_configs: Option<DynamicBlock<ComputeFirewallPolicyRuleMatchElLayer4ConfigsEl>>,
}

#[derive(Serialize)]
pub struct ComputeFirewallPolicyRuleMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dest_address_groups: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dest_fqdns: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dest_ip_ranges: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dest_region_codes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dest_threat_intelligences: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    src_address_groups: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    src_fqdns: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    src_ip_ranges: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    src_region_codes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    src_threat_intelligences: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    layer4_configs: Option<Vec<ComputeFirewallPolicyRuleMatchElLayer4ConfigsEl>>,
    dynamic: ComputeFirewallPolicyRuleMatchElDynamic,
}

impl ComputeFirewallPolicyRuleMatchEl {
    #[doc= "Set the field `dest_address_groups`.\nAddress groups which should be matched against the traffic destination. Maximum number of destination address groups is 10. Destination address groups is only supported in Egress rules."]
    pub fn set_dest_address_groups(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.dest_address_groups = Some(v.into());
        self
    }

    #[doc= "Set the field `dest_fqdns`.\nDomain names that will be used to match against the resolved domain name of destination of traffic. Can only be specified if DIRECTION is egress."]
    pub fn set_dest_fqdns(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.dest_fqdns = Some(v.into());
        self
    }

    #[doc= "Set the field `dest_ip_ranges`.\nCIDR IP address range. Maximum number of destination CIDR IP ranges allowed is 256."]
    pub fn set_dest_ip_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.dest_ip_ranges = Some(v.into());
        self
    }

    #[doc= "Set the field `dest_region_codes`.\nThe Unicode country codes whose IP addresses will be used to match against the source of traffic. Can only be specified if DIRECTION is egress."]
    pub fn set_dest_region_codes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.dest_region_codes = Some(v.into());
        self
    }

    #[doc= "Set the field `dest_threat_intelligences`.\nName of the Google Cloud Threat Intelligence list."]
    pub fn set_dest_threat_intelligences(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.dest_threat_intelligences = Some(v.into());
        self
    }

    #[doc= "Set the field `src_address_groups`.\nAddress groups which should be matched against the traffic source. Maximum number of source address groups is 10. Source address groups is only supported in Ingress rules."]
    pub fn set_src_address_groups(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.src_address_groups = Some(v.into());
        self
    }

    #[doc= "Set the field `src_fqdns`.\nDomain names that will be used to match against the resolved domain name of source of traffic. Can only be specified if DIRECTION is ingress."]
    pub fn set_src_fqdns(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.src_fqdns = Some(v.into());
        self
    }

    #[doc= "Set the field `src_ip_ranges`.\nCIDR IP address range. Maximum number of source CIDR IP ranges allowed is 256."]
    pub fn set_src_ip_ranges(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.src_ip_ranges = Some(v.into());
        self
    }

    #[doc= "Set the field `src_region_codes`.\nThe Unicode country codes whose IP addresses will be used to match against the source of traffic. Can only be specified if DIRECTION is ingress."]
    pub fn set_src_region_codes(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.src_region_codes = Some(v.into());
        self
    }

    #[doc= "Set the field `src_threat_intelligences`.\nName of the Google Cloud Threat Intelligence list."]
    pub fn set_src_threat_intelligences(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.src_threat_intelligences = Some(v.into());
        self
    }

    #[doc= "Set the field `layer4_configs`.\n"]
    pub fn set_layer4_configs(
        mut self,
        v: impl Into<BlockAssignable<ComputeFirewallPolicyRuleMatchElLayer4ConfigsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.layer4_configs = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.layer4_configs = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for ComputeFirewallPolicyRuleMatchEl {
    type O = BlockAssignable<ComputeFirewallPolicyRuleMatchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeFirewallPolicyRuleMatchEl {}

impl BuildComputeFirewallPolicyRuleMatchEl {
    pub fn build(self) -> ComputeFirewallPolicyRuleMatchEl {
        ComputeFirewallPolicyRuleMatchEl {
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
            dynamic: Default::default(),
        }
    }
}

pub struct ComputeFirewallPolicyRuleMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeFirewallPolicyRuleMatchElRef {
    fn new(shared: StackShared, base: String) -> ComputeFirewallPolicyRuleMatchElRef {
        ComputeFirewallPolicyRuleMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeFirewallPolicyRuleMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `dest_address_groups` after provisioning.\nAddress groups which should be matched against the traffic destination. Maximum number of destination address groups is 10. Destination address groups is only supported in Egress rules."]
    pub fn dest_address_groups(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.dest_address_groups", self.base))
    }

    #[doc= "Get a reference to the value of field `dest_fqdns` after provisioning.\nDomain names that will be used to match against the resolved domain name of destination of traffic. Can only be specified if DIRECTION is egress."]
    pub fn dest_fqdns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.dest_fqdns", self.base))
    }

    #[doc= "Get a reference to the value of field `dest_ip_ranges` after provisioning.\nCIDR IP address range. Maximum number of destination CIDR IP ranges allowed is 256."]
    pub fn dest_ip_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.dest_ip_ranges", self.base))
    }

    #[doc= "Get a reference to the value of field `dest_region_codes` after provisioning.\nThe Unicode country codes whose IP addresses will be used to match against the source of traffic. Can only be specified if DIRECTION is egress."]
    pub fn dest_region_codes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.dest_region_codes", self.base))
    }

    #[doc= "Get a reference to the value of field `dest_threat_intelligences` after provisioning.\nName of the Google Cloud Threat Intelligence list."]
    pub fn dest_threat_intelligences(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.dest_threat_intelligences", self.base))
    }

    #[doc= "Get a reference to the value of field `src_address_groups` after provisioning.\nAddress groups which should be matched against the traffic source. Maximum number of source address groups is 10. Source address groups is only supported in Ingress rules."]
    pub fn src_address_groups(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.src_address_groups", self.base))
    }

    #[doc= "Get a reference to the value of field `src_fqdns` after provisioning.\nDomain names that will be used to match against the resolved domain name of source of traffic. Can only be specified if DIRECTION is ingress."]
    pub fn src_fqdns(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.src_fqdns", self.base))
    }

    #[doc= "Get a reference to the value of field `src_ip_ranges` after provisioning.\nCIDR IP address range. Maximum number of source CIDR IP ranges allowed is 256."]
    pub fn src_ip_ranges(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.src_ip_ranges", self.base))
    }

    #[doc= "Get a reference to the value of field `src_region_codes` after provisioning.\nThe Unicode country codes whose IP addresses will be used to match against the source of traffic. Can only be specified if DIRECTION is ingress."]
    pub fn src_region_codes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.src_region_codes", self.base))
    }

    #[doc= "Get a reference to the value of field `src_threat_intelligences` after provisioning.\nName of the Google Cloud Threat Intelligence list."]
    pub fn src_threat_intelligences(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.src_threat_intelligences", self.base))
    }

    #[doc= "Get a reference to the value of field `layer4_configs` after provisioning.\n"]
    pub fn layer4_configs(&self) -> ListRef<ComputeFirewallPolicyRuleMatchElLayer4ConfigsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.layer4_configs", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeFirewallPolicyRuleTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl ComputeFirewallPolicyRuleTimeoutsEl {
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

impl ToListMappable for ComputeFirewallPolicyRuleTimeoutsEl {
    type O = BlockAssignable<ComputeFirewallPolicyRuleTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeFirewallPolicyRuleTimeoutsEl {}

impl BuildComputeFirewallPolicyRuleTimeoutsEl {
    pub fn build(self) -> ComputeFirewallPolicyRuleTimeoutsEl {
        ComputeFirewallPolicyRuleTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ComputeFirewallPolicyRuleTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeFirewallPolicyRuleTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeFirewallPolicyRuleTimeoutsElRef {
        ComputeFirewallPolicyRuleTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeFirewallPolicyRuleTimeoutsElRef {
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
struct ComputeFirewallPolicyRuleDynamic {
    match_: Option<DynamicBlock<ComputeFirewallPolicyRuleMatchEl>>,
}
