use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DnsResponsePolicyRuleData {
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
    dns_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    response_policy: terrars::PrimField<std::string::String>,
    rule_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_data: std::option::Option<std::vec::Vec<DnsResponsePolicyRuleLocalDataEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DnsResponsePolicyRuleTimeoutsEl>,
    dynamic: DnsResponsePolicyRuleDynamic,
}

struct DnsResponsePolicyRule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DnsResponsePolicyRuleData>,
}

#[derive(Clone)]
pub struct DnsResponsePolicyRule(std::rc::Rc<DnsResponsePolicyRule_>);

impl DnsResponsePolicyRule {
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

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_data`.\n"]
    pub fn set_local_data(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DnsResponsePolicyRuleLocalDataEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().local_data = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.local_data = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DnsResponsePolicyRuleTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `dns_name` after provisioning.\nThe DNS name (wildcard or exact) to apply this rule to. Must be unique within the Response Policy Rule."]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `response_policy` after provisioning.\nIdentifies the response policy addressed by this request."]
    pub fn response_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rule_name` after provisioning.\nAn identifier for this rule. Must be unique with the ResponsePolicy."]
    pub fn rule_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `local_data` after provisioning.\n"]
    pub fn local_data(&self) -> terrars::ListRef<DnsResponsePolicyRuleLocalDataElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.local_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DnsResponsePolicyRuleTimeoutsElRef {
        DnsResponsePolicyRuleTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DnsResponsePolicyRule {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DnsResponsePolicyRule { }

impl terrars::ToListMappable for DnsResponsePolicyRule {
    type O = terrars::ListRef<DnsResponsePolicyRuleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DnsResponsePolicyRule_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_dns_response_policy_rule".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDnsResponsePolicyRule {
    pub tf_id: std::string::String,
    #[doc = "The DNS name (wildcard or exact) to apply this rule to. Must be unique within the Response Policy Rule."]
    pub dns_name: terrars::PrimField<std::string::String>,
    #[doc = "Identifies the response policy addressed by this request."]
    pub response_policy: terrars::PrimField<std::string::String>,
    #[doc = "An identifier for this rule. Must be unique with the ResponsePolicy."]
    pub rule_name: terrars::PrimField<std::string::String>,
}

impl BuildDnsResponsePolicyRule {
    pub fn build(self, stack: &mut terrars::Stack) -> DnsResponsePolicyRule {
        let out = DnsResponsePolicyRule(std::rc::Rc::new(DnsResponsePolicyRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DnsResponsePolicyRuleData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                dns_name: self.dns_name,
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                response_policy: self.response_policy,
                rule_name: self.rule_name,
                local_data: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DnsResponsePolicyRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsResponsePolicyRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DnsResponsePolicyRuleRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dns_name` after provisioning.\nThe DNS name (wildcard or exact) to apply this rule to. Must be unique within the Response Policy Rule."]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `response_policy` after provisioning.\nIdentifies the response policy addressed by this request."]
    pub fn response_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rule_name` after provisioning.\nAn identifier for this rule. Must be unique with the ResponsePolicy."]
    pub fn rule_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `local_data` after provisioning.\n"]
    pub fn local_data(&self) -> terrars::ListRef<DnsResponsePolicyRuleLocalDataElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.local_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DnsResponsePolicyRuleTimeoutsElRef {
        DnsResponsePolicyRuleTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DnsResponsePolicyRuleLocalDataElLocalDatasEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rrdatas: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ttl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl DnsResponsePolicyRuleLocalDataElLocalDatasEl {
    #[doc = "Set the field `rrdatas`.\nAs defined in RFC 1035 (section 5) and RFC 1034 (section 3.6.1)"]
    pub fn set_rrdatas(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.rrdatas = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ttl`.\nNumber of seconds that this ResourceRecordSet can be cached by\nresolvers."]
    pub fn set_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ttl = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DnsResponsePolicyRuleLocalDataElLocalDatasEl {
    type O = terrars::BlockAssignable<DnsResponsePolicyRuleLocalDataElLocalDatasEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsResponsePolicyRuleLocalDataElLocalDatasEl {
    #[doc = "For example, www.example.com."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "One of valid DNS resource types. Possible values: [\"A\", \"AAAA\", \"CAA\", \"CNAME\", \"DNSKEY\", \"DS\", \"HTTPS\", \"IPSECVPNKEY\", \"MX\", \"NAPTR\", \"NS\", \"PTR\", \"SOA\", \"SPF\", \"SRV\", \"SSHFP\", \"SVCB\", \"TLSA\", \"TXT\"]"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildDnsResponsePolicyRuleLocalDataElLocalDatasEl {
    pub fn build(self) -> DnsResponsePolicyRuleLocalDataElLocalDatasEl {
        DnsResponsePolicyRuleLocalDataElLocalDatasEl {
            name: self.name,
            rrdatas: core::default::Default::default(),
            ttl: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct DnsResponsePolicyRuleLocalDataElLocalDatasElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsResponsePolicyRuleLocalDataElLocalDatasElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DnsResponsePolicyRuleLocalDataElLocalDatasElRef {
        DnsResponsePolicyRuleLocalDataElLocalDatasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsResponsePolicyRuleLocalDataElLocalDatasElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nFor example, www.example.com."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rrdatas` after provisioning.\nAs defined in RFC 1035 (section 5) and RFC 1034 (section 3.6.1)"]
    pub fn rrdatas(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rrdatas", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ttl` after provisioning.\nNumber of seconds that this ResourceRecordSet can be cached by\nresolvers."]
    pub fn ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nOne of valid DNS resource types. Possible values: [\"A\", \"AAAA\", \"CAA\", \"CNAME\", \"DNSKEY\", \"DS\", \"HTTPS\", \"IPSECVPNKEY\", \"MX\", \"NAPTR\", \"NS\", \"PTR\", \"SOA\", \"SPF\", \"SRV\", \"SSHFP\", \"SVCB\", \"TLSA\", \"TXT\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct DnsResponsePolicyRuleLocalDataElDynamic {
    local_datas: std::option::Option<terrars::DynamicBlock<DnsResponsePolicyRuleLocalDataElLocalDatasEl>>,
}

#[derive(Serialize)]
pub struct DnsResponsePolicyRuleLocalDataEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_datas: std::option::Option<std::vec::Vec<DnsResponsePolicyRuleLocalDataElLocalDatasEl>>,
    dynamic: DnsResponsePolicyRuleLocalDataElDynamic,
}

impl DnsResponsePolicyRuleLocalDataEl {
    #[doc = "Set the field `local_datas`.\n"]
    pub fn set_local_datas(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DnsResponsePolicyRuleLocalDataElLocalDatasEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.local_datas = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.local_datas = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DnsResponsePolicyRuleLocalDataEl {
    type O = terrars::BlockAssignable<DnsResponsePolicyRuleLocalDataEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsResponsePolicyRuleLocalDataEl {}

impl BuildDnsResponsePolicyRuleLocalDataEl {
    pub fn build(self) -> DnsResponsePolicyRuleLocalDataEl {
        DnsResponsePolicyRuleLocalDataEl {
            local_datas: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DnsResponsePolicyRuleLocalDataElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsResponsePolicyRuleLocalDataElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DnsResponsePolicyRuleLocalDataElRef {
        DnsResponsePolicyRuleLocalDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsResponsePolicyRuleLocalDataElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `local_datas` after provisioning.\n"]
    pub fn local_datas(&self) -> terrars::ListRef<DnsResponsePolicyRuleLocalDataElLocalDatasElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.local_datas", self.base))
    }
}

#[derive(Serialize)]
pub struct DnsResponsePolicyRuleTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DnsResponsePolicyRuleTimeoutsEl {
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

impl terrars::ToListMappable for DnsResponsePolicyRuleTimeoutsEl {
    type O = terrars::BlockAssignable<DnsResponsePolicyRuleTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsResponsePolicyRuleTimeoutsEl {}

impl BuildDnsResponsePolicyRuleTimeoutsEl {
    pub fn build(self) -> DnsResponsePolicyRuleTimeoutsEl {
        DnsResponsePolicyRuleTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct DnsResponsePolicyRuleTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsResponsePolicyRuleTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DnsResponsePolicyRuleTimeoutsElRef {
        DnsResponsePolicyRuleTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsResponsePolicyRuleTimeoutsElRef {
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
struct DnsResponsePolicyRuleDynamic {
    local_data: std::option::Option<terrars::DynamicBlock<DnsResponsePolicyRuleLocalDataEl>>,
}
