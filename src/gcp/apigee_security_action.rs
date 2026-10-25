#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ApigeeSecurityActionData_ {
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
    api_proxies: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    env_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expire_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    org_id: terrars::PrimField<std::string::String>,
    security_action_id: terrars::PrimField<std::string::String>,
    state: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ttl: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow: std::option::Option<std::vec::Vec<ApigeeSecurityAction_Allow>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition_config: std::option::Option<std::vec::Vec<ApigeeSecurityAction_ConditionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deny: std::option::Option<std::vec::Vec<ApigeeSecurityAction_Deny>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flag: std::option::Option<std::vec::Vec<ApigeeSecurityAction_Flag>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ApigeeSecurityAction_Timeouts>,
    dynamic: ApigeeSecurityActionDynamic,
}

struct ApigeeSecurityAction_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ApigeeSecurityActionData_>,
}

#[derive(Clone)]
pub struct ApigeeSecurityAction(std::rc::Rc<ApigeeSecurityAction_>);

impl ApigeeSecurityAction {
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
        "Set the field `api_proxies`.\nIf unset, this would apply to all proxies in the environment.\nIf set, this action is enforced only if at least one proxy in the repeated\nlist is deployed at the time of enforcement. If set, several restrictions are enforced on SecurityActions.\nThere can be at most 100 enabled actions with proxies set in an env.\nSeveral other restrictions apply on conditions and are detailed later."]
    pub fn set_api_proxies(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().api_proxies = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nAn optional user provided description of the SecurityAction."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `expire_time`.\nThe expiration for this SecurityAction.\nUses RFC 3339, where generated output will always be Z-normalized and uses 0, 3, 6 or 9\nfractional digits. Offsets other than \"Z\" are also accepted.\nExamples: \"2014-10-02T15:01:23Z\", \"2014-10-02T15:01:23.045123456Z\" or \"2014-10-02T15:01:23+05:30\"."]
    pub fn set_expire_time(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().expire_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ttl`.\nThe TTL for this SecurityAction.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn set_ttl(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow`.\n"]
    pub fn set_allow(self, v: impl std::convert::Into<terrars::BlockAssignable<ApigeeSecurityAction_Allow>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().allow = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.allow = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `condition_config`.\n"]
    pub fn set_condition_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApigeeSecurityAction_ConditionConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().condition_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.condition_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `deny`.\n"]
    pub fn set_deny(self, v: impl std::convert::Into<terrars::BlockAssignable<ApigeeSecurityAction_Deny>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().deny = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.deny = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `flag`.\n"]
    pub fn set_flag(self, v: impl std::convert::Into<terrars::BlockAssignable<ApigeeSecurityAction_Flag>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().flag = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.flag = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ApigeeSecurityAction_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `api_proxies` after provisioning.\nIf unset, this would apply to all proxies in the environment.\nIf set, this action is enforced only if at least one proxy in the repeated\nlist is deployed at the time of enforcement. If set, several restrictions are enforced on SecurityActions.\nThere can be at most 100 enabled actions with proxies set in an env.\nSeveral other restrictions apply on conditions and are detailed later."]
    pub fn api_proxies(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_proxies", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe create time for this SecurityAction.\nUses RFC 3339, where generated output will always be Z-normalized and uses 0, 3, 6 or 9 fractional digits.\nOffsets other than \"Z\" are also accepted. Examples: \"2014-10-02T15:01:23Z\", \"2014-10-02T15:01:23.045123456Z\" or \"2014-10-02T15:01:23+05:30\"."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional user provided description of the SecurityAction."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `env_id` after provisioning.\nThe Apigee environment that this security action applies to."]
    pub fn env_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.env_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expire_time` after provisioning.\nThe expiration for this SecurityAction.\nUses RFC 3339, where generated output will always be Z-normalized and uses 0, 3, 6 or 9\nfractional digits. Offsets other than \"Z\" are also accepted.\nExamples: \"2014-10-02T15:01:23Z\", \"2014-10-02T15:01:23.045123456Z\" or \"2014-10-02T15:01:23+05:30\"."]
    pub fn expire_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expire_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `org_id` after provisioning.\nThe organization that this security action applies to."]
    pub fn org_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.org_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `security_action_id` after provisioning.\nThe ID to use for the SecurityAction, which will become the final component of the action's resource name.\nThis value should be 0-61 characters, and valid format is (^a-z?$)."]
    pub fn security_action_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_action_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOnly an ENABLED SecurityAction is enforced. An ENABLED SecurityAction past its expiration time will not be enforced. Possible values: [\"ENABLED\", \"DISABLED\"]"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ttl` after provisioning.\nThe TTL for this SecurityAction.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn ttl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe update time for this SecurityAction. This reflects when this SecurityAction changed states.\nUses RFC 3339, where generated output will always be Z-normalized and uses 0, 3, 6 or 9 fractional digits.\nOffsets other than \"Z\" are also accepted. Examples: \"2014-10-02T15:01:23Z\", \"2014-10-02T15:01:23.045123456Z\" or \"2014-10-02T15:01:23+05:30\"."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allow` after provisioning.\n"]
    pub fn allow(&self) -> terrars::ListRef<ApigeeSecurityAction_AllowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `condition_config` after provisioning.\n"]
    pub fn condition_config(&self) -> terrars::ListRef<ApigeeSecurityAction_ConditionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deny` after provisioning.\n"]
    pub fn deny(&self) -> terrars::ListRef<ApigeeSecurityAction_DenyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deny", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `flag` after provisioning.\n"]
    pub fn flag(&self) -> terrars::ListRef<ApigeeSecurityAction_FlagRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.flag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeSecurityAction_TimeoutsRef {
        ApigeeSecurityAction_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ApigeeSecurityAction {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ApigeeSecurityAction { }

impl terrars::ToListMappable for ApigeeSecurityAction {
    type O = terrars::ListRef<ApigeeSecurityActionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ApigeeSecurityAction_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_apigee_security_action".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApigeeSecurityAction {
    pub tf_id: std::string::String,
    #[doc = "The Apigee environment that this security action applies to."]
    pub env_id: terrars::PrimField<std::string::String>,
    #[doc = "The organization that this security action applies to."]
    pub org_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The ID to use for the SecurityAction, which will become the final component of the action's resource name.\nThis value should be 0-61 characters, and valid format is (^a-z?$)."]
    pub security_action_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Only an ENABLED SecurityAction is enforced. An ENABLED SecurityAction past its expiration time will not be enforced. Possible values: [\"ENABLED\", \"DISABLED\"]"]
    pub state: terrars::PrimField<std::string::String>,
}

impl BuildApigeeSecurityAction {
    pub fn build(self, stack: &mut terrars::Stack) -> ApigeeSecurityAction {
        let out = ApigeeSecurityAction(std::rc::Rc::new(ApigeeSecurityAction_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ApigeeSecurityActionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                api_proxies: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                env_id: self.env_id,
                expire_time: core::default::Default::default(),
                id: core::default::Default::default(),
                org_id: self.org_id,
                security_action_id: self.security_action_id,
                state: self.state,
                ttl: core::default::Default::default(),
                allow: core::default::Default::default(),
                condition_config: core::default::Default::default(),
                deny: core::default::Default::default(),
                flag: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApigeeSecurityActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeSecurityActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ApigeeSecurityActionRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `api_proxies` after provisioning.\nIf unset, this would apply to all proxies in the environment.\nIf set, this action is enforced only if at least one proxy in the repeated\nlist is deployed at the time of enforcement. If set, several restrictions are enforced on SecurityActions.\nThere can be at most 100 enabled actions with proxies set in an env.\nSeveral other restrictions apply on conditions and are detailed later."]
    pub fn api_proxies(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_proxies", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe create time for this SecurityAction.\nUses RFC 3339, where generated output will always be Z-normalized and uses 0, 3, 6 or 9 fractional digits.\nOffsets other than \"Z\" are also accepted. Examples: \"2014-10-02T15:01:23Z\", \"2014-10-02T15:01:23.045123456Z\" or \"2014-10-02T15:01:23+05:30\"."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional user provided description of the SecurityAction."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `env_id` after provisioning.\nThe Apigee environment that this security action applies to."]
    pub fn env_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.env_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expire_time` after provisioning.\nThe expiration for this SecurityAction.\nUses RFC 3339, where generated output will always be Z-normalized and uses 0, 3, 6 or 9\nfractional digits. Offsets other than \"Z\" are also accepted.\nExamples: \"2014-10-02T15:01:23Z\", \"2014-10-02T15:01:23.045123456Z\" or \"2014-10-02T15:01:23+05:30\"."]
    pub fn expire_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expire_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `org_id` after provisioning.\nThe organization that this security action applies to."]
    pub fn org_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.org_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `security_action_id` after provisioning.\nThe ID to use for the SecurityAction, which will become the final component of the action's resource name.\nThis value should be 0-61 characters, and valid format is (^a-z?$)."]
    pub fn security_action_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_action_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOnly an ENABLED SecurityAction is enforced. An ENABLED SecurityAction past its expiration time will not be enforced. Possible values: [\"ENABLED\", \"DISABLED\"]"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ttl` after provisioning.\nThe TTL for this SecurityAction.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn ttl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe update time for this SecurityAction. This reflects when this SecurityAction changed states.\nUses RFC 3339, where generated output will always be Z-normalized and uses 0, 3, 6 or 9 fractional digits.\nOffsets other than \"Z\" are also accepted. Examples: \"2014-10-02T15:01:23Z\", \"2014-10-02T15:01:23.045123456Z\" or \"2014-10-02T15:01:23+05:30\"."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allow` after provisioning.\n"]
    pub fn allow(&self) -> terrars::ListRef<ApigeeSecurityAction_AllowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `condition_config` after provisioning.\n"]
    pub fn condition_config(&self) -> terrars::ListRef<ApigeeSecurityAction_ConditionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deny` after provisioning.\n"]
    pub fn deny(&self) -> terrars::ListRef<ApigeeSecurityAction_DenyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deny", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `flag` after provisioning.\n"]
    pub fn flag(&self) -> terrars::ListRef<ApigeeSecurityAction_FlagRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.flag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeSecurityAction_TimeoutsRef {
        ApigeeSecurityAction_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ApigeeSecurityActionDynamic {
    allow: std::option::Option<terrars::DynamicBlock<ApigeeSecurityAction_Allow>>,
    condition_config: std::option::Option<terrars::DynamicBlock<ApigeeSecurityAction_ConditionConfig>>,
    deny: std::option::Option<terrars::DynamicBlock<ApigeeSecurityAction_Deny>>,
    flag: std::option::Option<terrars::DynamicBlock<ApigeeSecurityAction_Flag>>,
}

#[derive(Serialize)]
pub struct ApigeeSecurityAction_Allow {}

impl ApigeeSecurityAction_Allow { }

impl terrars::ToListMappable for ApigeeSecurityAction_Allow {
    type O = terrars::BlockAssignable<ApigeeSecurityAction_Allow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeSecurityAction_Allow {}

impl BuildApigeeSecurityAction_Allow {
    pub fn build(self) -> ApigeeSecurityAction_Allow {
        ApigeeSecurityAction_Allow {}
    }
}

#[allow(dead_code)]
pub struct ApigeeSecurityAction_AllowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeSecurityAction_AllowRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApigeeSecurityAction_AllowRef {
        ApigeeSecurityAction_AllowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeSecurityAction_AllowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct ApigeeSecurityAction_ConditionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_tokens: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_keys: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_products: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    asns: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bot_reasons: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    developer_apps: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    developers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_methods: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_address_ranges: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region_codes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_agents: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ApigeeSecurityAction_ConditionConfig {
    #[doc = "Set the field `access_tokens`.\nA list of accessTokens. Limit 1000 per action."]
    pub fn set_access_tokens(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.access_tokens = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `api_keys`.\nA list of API keys. Limit 1000 per action."]
    pub fn set_api_keys(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.api_keys = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `api_products`.\nA list of API Products. Limit 1000 per action."]
    pub fn set_api_products(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.api_products = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `asns`.\nA list of ASN numbers to act on, e.g. 23. https://en.wikipedia.org/wiki/Autonomous_system_(Internet)\nThis uses int64 instead of uint32 because of https://linter.aip.dev/141/forbidden-types."]
    pub fn set_asns(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.asns = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `bot_reasons`.\nA list of Bot Reasons. Current options: Flooder, Brute Guessor, Static Content Scraper,\nOAuth Abuser, Robot Abuser, TorListRule, Advanced Anomaly Detection, Advanced API Scraper,\nSearch Engine Crawlers, Public Clouds, Public Cloud AWS, Public Cloud Azure, and Public Cloud Google."]
    pub fn set_bot_reasons(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.bot_reasons = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `developer_apps`.\nA list of developer apps. Limit 1000 per action."]
    pub fn set_developer_apps(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.developer_apps = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `developers`.\nA list of developers. Limit 1000 per action."]
    pub fn set_developers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.developers = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `http_methods`.\nAct only on particular HTTP methods. E.g. A read-only API can block POST/PUT/DELETE methods.\nAccepted values are: GET, HEAD, POST, PUT, DELETE, CONNECT, OPTIONS, TRACE and PATCH."]
    pub fn set_http_methods(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.http_methods = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ip_address_ranges`.\nA list of IP addresses. This could be either IPv4 or IPv6. Limited to 100 per action."]
    pub fn set_ip_address_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ip_address_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region_codes`.\nA list of countries/region codes to act on, e.g. US. This follows https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2."]
    pub fn set_region_codes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.region_codes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `user_agents`.\nA list of user agents to deny. We look for exact matches. Limit 50 per action."]
    pub fn set_user_agents(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.user_agents = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApigeeSecurityAction_ConditionConfig {
    type O = terrars::BlockAssignable<ApigeeSecurityAction_ConditionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeSecurityAction_ConditionConfig {}

impl BuildApigeeSecurityAction_ConditionConfig {
    pub fn build(self) -> ApigeeSecurityAction_ConditionConfig {
        ApigeeSecurityAction_ConditionConfig {
            access_tokens: core::default::Default::default(),
            api_keys: core::default::Default::default(),
            api_products: core::default::Default::default(),
            asns: core::default::Default::default(),
            bot_reasons: core::default::Default::default(),
            developer_apps: core::default::Default::default(),
            developers: core::default::Default::default(),
            http_methods: core::default::Default::default(),
            ip_address_ranges: core::default::Default::default(),
            region_codes: core::default::Default::default(),
            user_agents: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApigeeSecurityAction_ConditionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeSecurityAction_ConditionConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApigeeSecurityAction_ConditionConfigRef {
        ApigeeSecurityAction_ConditionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeSecurityAction_ConditionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_tokens` after provisioning.\nA list of accessTokens. Limit 1000 per action."]
    pub fn access_tokens(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_tokens", self.base))
    }

    #[doc =
        "Get a reference to the value of field `api_keys` after provisioning.\nA list of API keys. Limit 1000 per action."]
    pub fn api_keys(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_keys", self.base))
    }

    #[doc =
        "Get a reference to the value of field `api_products` after provisioning.\nA list of API Products. Limit 1000 per action."]
    pub fn api_products(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_products", self.base))
    }

    #[doc =
        "Get a reference to the value of field `asns` after provisioning.\nA list of ASN numbers to act on, e.g. 23. https://en.wikipedia.org/wiki/Autonomous_system_(Internet)\nThis uses int64 instead of uint32 because of https://linter.aip.dev/141/forbidden-types."]
    pub fn asns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.asns", self.base))
    }

    #[doc =
        "Get a reference to the value of field `bot_reasons` after provisioning.\nA list of Bot Reasons. Current options: Flooder, Brute Guessor, Static Content Scraper,\nOAuth Abuser, Robot Abuser, TorListRule, Advanced Anomaly Detection, Advanced API Scraper,\nSearch Engine Crawlers, Public Clouds, Public Cloud AWS, Public Cloud Azure, and Public Cloud Google."]
    pub fn bot_reasons(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bot_reasons", self.base))
    }

    #[doc =
        "Get a reference to the value of field `developer_apps` after provisioning.\nA list of developer apps. Limit 1000 per action."]
    pub fn developer_apps(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.developer_apps", self.base))
    }

    #[doc =
        "Get a reference to the value of field `developers` after provisioning.\nA list of developers. Limit 1000 per action."]
    pub fn developers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.developers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `http_methods` after provisioning.\nAct only on particular HTTP methods. E.g. A read-only API can block POST/PUT/DELETE methods.\nAccepted values are: GET, HEAD, POST, PUT, DELETE, CONNECT, OPTIONS, TRACE and PATCH."]
    pub fn http_methods(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_methods", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ip_address_ranges` after provisioning.\nA list of IP addresses. This could be either IPv4 or IPv6. Limited to 100 per action."]
    pub fn ip_address_ranges(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_address_ranges", self.base))
    }

    #[doc =
        "Get a reference to the value of field `region_codes` after provisioning.\nA list of countries/region codes to act on, e.g. US. This follows https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2."]
    pub fn region_codes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.region_codes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `user_agents` after provisioning.\nA list of user agents to deny. We look for exact matches. Limit 50 per action."]
    pub fn user_agents(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_agents", self.base))
    }
}

#[derive(Serialize)]
pub struct ApigeeSecurityAction_Deny {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_code: std::option::Option<terrars::PrimField<f64>>,
}

impl ApigeeSecurityAction_Deny {
    #[doc = "Set the field `response_code`.\nThe HTTP response code if the Action = DENY."]
    pub fn set_response_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.response_code = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApigeeSecurityAction_Deny {
    type O = terrars::BlockAssignable<ApigeeSecurityAction_Deny>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeSecurityAction_Deny {}

impl BuildApigeeSecurityAction_Deny {
    pub fn build(self) -> ApigeeSecurityAction_Deny {
        ApigeeSecurityAction_Deny { response_code: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ApigeeSecurityAction_DenyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeSecurityAction_DenyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApigeeSecurityAction_DenyRef {
        ApigeeSecurityAction_DenyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeSecurityAction_DenyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `response_code` after provisioning.\nThe HTTP response code if the Action = DENY."]
    pub fn response_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response_code", self.base))
    }
}

#[derive(Serialize)]
pub struct ApigeeSecurityAction_Flag_Headers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApigeeSecurityAction_Flag_Headers {
    #[doc = "Set the field `name`.\nThe header name to be sent to the target."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nThe header value to be sent to the target."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApigeeSecurityAction_Flag_Headers {
    type O = terrars::BlockAssignable<ApigeeSecurityAction_Flag_Headers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeSecurityAction_Flag_Headers {}

impl BuildApigeeSecurityAction_Flag_Headers {
    pub fn build(self) -> ApigeeSecurityAction_Flag_Headers {
        ApigeeSecurityAction_Flag_Headers {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApigeeSecurityAction_Flag_HeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeSecurityAction_Flag_HeadersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApigeeSecurityAction_Flag_HeadersRef {
        ApigeeSecurityAction_Flag_HeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeSecurityAction_Flag_HeadersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe header name to be sent to the target."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nThe header value to be sent to the target."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApigeeSecurityAction_FlagDynamic {
    headers: std::option::Option<terrars::DynamicBlock<ApigeeSecurityAction_Flag_Headers>>,
}

#[derive(Serialize)]
pub struct ApigeeSecurityAction_Flag {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    headers: std::option::Option<std::vec::Vec<ApigeeSecurityAction_Flag_Headers>>,
    dynamic: ApigeeSecurityAction_FlagDynamic,
}

impl ApigeeSecurityAction_Flag {
    #[doc = "Set the field `headers`.\n"]
    pub fn set_headers(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApigeeSecurityAction_Flag_Headers>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.headers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.headers = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ApigeeSecurityAction_Flag {
    type O = terrars::BlockAssignable<ApigeeSecurityAction_Flag>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeSecurityAction_Flag {}

impl BuildApigeeSecurityAction_Flag {
    pub fn build(self) -> ApigeeSecurityAction_Flag {
        ApigeeSecurityAction_Flag {
            headers: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApigeeSecurityAction_FlagRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeSecurityAction_FlagRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApigeeSecurityAction_FlagRef {
        ApigeeSecurityAction_FlagRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeSecurityAction_FlagRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `headers` after provisioning.\n"]
    pub fn headers(&self) -> terrars::ListRef<ApigeeSecurityAction_Flag_HeadersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }
}

#[derive(Serialize)]
pub struct ApigeeSecurityAction_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApigeeSecurityAction_Timeouts {
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

impl terrars::ToListMappable for ApigeeSecurityAction_Timeouts {
    type O = terrars::BlockAssignable<ApigeeSecurityAction_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeSecurityAction_Timeouts {}

impl BuildApigeeSecurityAction_Timeouts {
    pub fn build(self) -> ApigeeSecurityAction_Timeouts {
        ApigeeSecurityAction_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApigeeSecurityAction_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApigeeSecurityAction_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApigeeSecurityAction_TimeoutsRef {
        ApigeeSecurityAction_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeSecurityAction_TimeoutsRef {
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
