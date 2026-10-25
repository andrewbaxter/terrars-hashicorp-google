#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DnsResponsePolicyRuleData_ {
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
    dns_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    response_policy: terrars::PrimField<std::string::String>,
    rule_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_data: std::option::Option<std::vec::Vec<DnsResponsePolicyRule_LocalData>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DnsResponsePolicyRule_Timeouts>,
    dynamic: DnsResponsePolicyRuleDynamic,
}

struct DnsResponsePolicyRule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DnsResponsePolicyRuleData_>,
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

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_data`.\n"]
    pub fn set_local_data(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DnsResponsePolicyRule_LocalData>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<DnsResponsePolicyRule_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
    pub fn local_data(&self) -> terrars::ListRef<DnsResponsePolicyRule_LocalDataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.local_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DnsResponsePolicyRule_TimeoutsRef {
        DnsResponsePolicyRule_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
            data: std::cell::RefCell::new(DnsResponsePolicyRuleData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
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
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
    pub fn local_data(&self) -> terrars::ListRef<DnsResponsePolicyRule_LocalDataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.local_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DnsResponsePolicyRule_TimeoutsRef {
        DnsResponsePolicyRule_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DnsResponsePolicyRuleDynamic {
    local_data: std::option::Option<terrars::DynamicBlock<DnsResponsePolicyRule_LocalData>>,
}

#[derive(Serialize)]
pub struct DnsResponsePolicyRule_LocalData_LocalDatas {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rrdatas: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ttl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl DnsResponsePolicyRule_LocalData_LocalDatas {
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

impl terrars::ToListMappable for DnsResponsePolicyRule_LocalData_LocalDatas {
    type O = terrars::BlockAssignable<DnsResponsePolicyRule_LocalData_LocalDatas>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsResponsePolicyRule_LocalData_LocalDatas {
    #[doc = "For example, www.example.com."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "One of valid DNS resource types. Possible values: [\"A\", \"AAAA\", \"CAA\", \"CNAME\", \"DNSKEY\", \"DS\", \"HTTPS\", \"IPSECVPNKEY\", \"MX\", \"NAPTR\", \"NS\", \"PTR\", \"SOA\", \"SPF\", \"SRV\", \"SSHFP\", \"SVCB\", \"TLSA\", \"TXT\"]"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildDnsResponsePolicyRule_LocalData_LocalDatas {
    pub fn build(self) -> DnsResponsePolicyRule_LocalData_LocalDatas {
        DnsResponsePolicyRule_LocalData_LocalDatas {
            name: self.name,
            rrdatas: core::default::Default::default(),
            ttl: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct DnsResponsePolicyRule_LocalData_LocalDatasRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsResponsePolicyRule_LocalData_LocalDatasRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DnsResponsePolicyRule_LocalData_LocalDatasRef {
        DnsResponsePolicyRule_LocalData_LocalDatasRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsResponsePolicyRule_LocalData_LocalDatasRef {
    #[allow(dead_code)]
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
struct DnsResponsePolicyRule_LocalDataDynamic {
    local_datas: std::option::Option<terrars::DynamicBlock<DnsResponsePolicyRule_LocalData_LocalDatas>>,
}

#[derive(Serialize)]
pub struct DnsResponsePolicyRule_LocalData {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_datas: std::option::Option<std::vec::Vec<DnsResponsePolicyRule_LocalData_LocalDatas>>,
    dynamic: DnsResponsePolicyRule_LocalDataDynamic,
}

impl DnsResponsePolicyRule_LocalData {
    #[doc = "Set the field `local_datas`.\n"]
    pub fn set_local_datas(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DnsResponsePolicyRule_LocalData_LocalDatas>>,
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

impl terrars::ToListMappable for DnsResponsePolicyRule_LocalData {
    type O = terrars::BlockAssignable<DnsResponsePolicyRule_LocalData>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsResponsePolicyRule_LocalData {}

impl BuildDnsResponsePolicyRule_LocalData {
    pub fn build(self) -> DnsResponsePolicyRule_LocalData {
        DnsResponsePolicyRule_LocalData {
            local_datas: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DnsResponsePolicyRule_LocalDataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsResponsePolicyRule_LocalDataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DnsResponsePolicyRule_LocalDataRef {
        DnsResponsePolicyRule_LocalDataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsResponsePolicyRule_LocalDataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `local_datas` after provisioning.\n"]
    pub fn local_datas(&self) -> terrars::ListRef<DnsResponsePolicyRule_LocalData_LocalDatasRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.local_datas", self.base))
    }
}

#[derive(Serialize)]
pub struct DnsResponsePolicyRule_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DnsResponsePolicyRule_Timeouts {
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

impl terrars::ToListMappable for DnsResponsePolicyRule_Timeouts {
    type O = terrars::BlockAssignable<DnsResponsePolicyRule_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDnsResponsePolicyRule_Timeouts {}

impl BuildDnsResponsePolicyRule_Timeouts {
    pub fn build(self) -> DnsResponsePolicyRule_Timeouts {
        DnsResponsePolicyRule_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DnsResponsePolicyRule_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DnsResponsePolicyRule_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DnsResponsePolicyRule_TimeoutsRef {
        DnsResponsePolicyRule_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DnsResponsePolicyRule_TimeoutsRef {
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
