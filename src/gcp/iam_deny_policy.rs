use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct IamDenyPolicyData {
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
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    parent: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules: std::option::Option<std::vec::Vec<IamDenyPolicyRulesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<IamDenyPolicyTimeoutsEl>,
    dynamic: IamDenyPolicyDynamic,
}

struct IamDenyPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<IamDenyPolicyData>,
}

#[derive(Clone)]
pub struct IamDenyPolicy(std::rc::Rc<IamDenyPolicy_>);

impl IamDenyPolicy {
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

    #[doc = "Set the field `display_name`.\nThe display name of the rule."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(self, v: impl std::convert::Into<terrars::BlockAssignable<IamDenyPolicyRulesEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rules = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<IamDenyPolicyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the rule."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThe hash of the resource. Used internally during updates."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the policy."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe attachment point is identified by its URL-encoded full resource name."]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<IamDenyPolicyRulesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamDenyPolicyTimeoutsElRef {
        IamDenyPolicyTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for IamDenyPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for IamDenyPolicy { }

impl terrars::ToListMappable for IamDenyPolicy {
    type O = terrars::ListRef<IamDenyPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for IamDenyPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_iam_deny_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIamDenyPolicy {
    pub tf_id: std::string::String,
    #[doc = "The name of the policy."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The attachment point is identified by its URL-encoded full resource name."]
    pub parent: terrars::PrimField<std::string::String>,
}

impl BuildIamDenyPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> IamDenyPolicy {
        let out = IamDenyPolicy(std::rc::Rc::new(IamDenyPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(IamDenyPolicyData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                parent: self.parent,
                rules: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IamDenyPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamDenyPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl IamDenyPolicyRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the rule."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThe hash of the resource. Used internally during updates."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the policy."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe attachment point is identified by its URL-encoded full resource name."]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<IamDenyPolicyRulesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamDenyPolicyTimeoutsElRef {
        IamDenyPolicyTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct IamDenyPolicyRulesElDenyRuleElDenialConditionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    expression: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    title: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IamDenyPolicyRulesElDenyRuleElDenialConditionEl {
    #[doc =
        "Set the field `description`.\nDescription of the expression. This is a longer text which describes the expression,\ne.g. when hovered over it in a UI."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `location`.\nString indicating the location of the expression for error reporting,\ne.g. a file name and a position in the file."]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `title`.\nTitle for the expression, i.e. a short string describing its purpose.\nThis can be used e.g. in UIs which allow to enter the expression."]
    pub fn set_title(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.title = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IamDenyPolicyRulesElDenyRuleElDenialConditionEl {
    type O = terrars::BlockAssignable<IamDenyPolicyRulesElDenyRuleElDenialConditionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamDenyPolicyRulesElDenyRuleElDenialConditionEl {
    #[doc = "Textual representation of an expression in Common Expression Language syntax."]
    pub expression: terrars::PrimField<std::string::String>,
}

impl BuildIamDenyPolicyRulesElDenyRuleElDenialConditionEl {
    pub fn build(self) -> IamDenyPolicyRulesElDenyRuleElDenialConditionEl {
        IamDenyPolicyRulesElDenyRuleElDenialConditionEl {
            description: core::default::Default::default(),
            expression: self.expression,
            location: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}

pub struct IamDenyPolicyRulesElDenyRuleElDenialConditionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamDenyPolicyRulesElDenyRuleElDenialConditionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IamDenyPolicyRulesElDenyRuleElDenialConditionElRef {
        IamDenyPolicyRulesElDenyRuleElDenialConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamDenyPolicyRulesElDenyRuleElDenialConditionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the expression. This is a longer text which describes the expression,\ne.g. when hovered over it in a UI."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nTextual representation of an expression in Common Expression Language syntax."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nString indicating the location of the expression for error reporting,\ne.g. a file name and a position in the file."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc =
        "Get a reference to the value of field `title` after provisioning.\nTitle for the expression, i.e. a short string describing its purpose.\nThis can be used e.g. in UIs which allow to enter the expression."]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}

#[derive(Serialize, Default)]
struct IamDenyPolicyRulesElDenyRuleElDynamic {
    denial_condition: std::option::Option<terrars::DynamicBlock<IamDenyPolicyRulesElDenyRuleElDenialConditionEl>>,
}

#[derive(Serialize)]
pub struct IamDenyPolicyRulesElDenyRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    denied_permissions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    denied_principals: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exception_permissions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exception_principals: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    denial_condition: std::option::Option<std::vec::Vec<IamDenyPolicyRulesElDenyRuleElDenialConditionEl>>,
    dynamic: IamDenyPolicyRulesElDenyRuleElDynamic,
}

impl IamDenyPolicyRulesElDenyRuleEl {
    #[doc =
        "Set the field `denied_permissions`.\nThe permissions that are explicitly denied by this rule. Each permission uses the format '{service-fqdn}/{resource}.{verb}',\nwhere '{service-fqdn}' is the fully qualified domain name for the service. For example, 'iam.googleapis.com/roles.list'."]
    pub fn set_denied_permissions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.denied_permissions = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `denied_principals`.\nThe identities that are prevented from using one or more permissions on Google Cloud resources."]
    pub fn set_denied_principals(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.denied_principals = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `exception_permissions`.\nSpecifies the permissions that this rule excludes from the set of denied permissions given by deniedPermissions.\nIf a permission appears in deniedPermissions and in exceptionPermissions then it will not be denied.\nThe excluded permissions can be specified using the same syntax as deniedPermissions."]
    pub fn set_exception_permissions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exception_permissions = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `exception_principals`.\nThe identities that are excluded from the deny rule, even if they are listed in the deniedPrincipals.\nFor example, you could add a Google group to the deniedPrincipals, then exclude specific users who belong to that group."]
    pub fn set_exception_principals(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exception_principals = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `denial_condition`.\n"]
    pub fn set_denial_condition(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IamDenyPolicyRulesElDenyRuleElDenialConditionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.denial_condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.denial_condition = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IamDenyPolicyRulesElDenyRuleEl {
    type O = terrars::BlockAssignable<IamDenyPolicyRulesElDenyRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamDenyPolicyRulesElDenyRuleEl {}

impl BuildIamDenyPolicyRulesElDenyRuleEl {
    pub fn build(self) -> IamDenyPolicyRulesElDenyRuleEl {
        IamDenyPolicyRulesElDenyRuleEl {
            denied_permissions: core::default::Default::default(),
            denied_principals: core::default::Default::default(),
            exception_permissions: core::default::Default::default(),
            exception_principals: core::default::Default::default(),
            denial_condition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct IamDenyPolicyRulesElDenyRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamDenyPolicyRulesElDenyRuleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IamDenyPolicyRulesElDenyRuleElRef {
        IamDenyPolicyRulesElDenyRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamDenyPolicyRulesElDenyRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `denied_permissions` after provisioning.\nThe permissions that are explicitly denied by this rule. Each permission uses the format '{service-fqdn}/{resource}.{verb}',\nwhere '{service-fqdn}' is the fully qualified domain name for the service. For example, 'iam.googleapis.com/roles.list'."]
    pub fn denied_permissions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.denied_permissions", self.base))
    }

    #[doc =
        "Get a reference to the value of field `denied_principals` after provisioning.\nThe identities that are prevented from using one or more permissions on Google Cloud resources."]
    pub fn denied_principals(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.denied_principals", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exception_permissions` after provisioning.\nSpecifies the permissions that this rule excludes from the set of denied permissions given by deniedPermissions.\nIf a permission appears in deniedPermissions and in exceptionPermissions then it will not be denied.\nThe excluded permissions can be specified using the same syntax as deniedPermissions."]
    pub fn exception_permissions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exception_permissions", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exception_principals` after provisioning.\nThe identities that are excluded from the deny rule, even if they are listed in the deniedPrincipals.\nFor example, you could add a Google group to the deniedPrincipals, then exclude specific users who belong to that group."]
    pub fn exception_principals(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exception_principals", self.base))
    }

    #[doc = "Get a reference to the value of field `denial_condition` after provisioning.\n"]
    pub fn denial_condition(&self) -> terrars::ListRef<IamDenyPolicyRulesElDenyRuleElDenialConditionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.denial_condition", self.base))
    }
}

#[derive(Serialize, Default)]
struct IamDenyPolicyRulesElDynamic {
    deny_rule: std::option::Option<terrars::DynamicBlock<IamDenyPolicyRulesElDenyRuleEl>>,
}

#[derive(Serialize)]
pub struct IamDenyPolicyRulesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deny_rule: std::option::Option<std::vec::Vec<IamDenyPolicyRulesElDenyRuleEl>>,
    dynamic: IamDenyPolicyRulesElDynamic,
}

impl IamDenyPolicyRulesEl {
    #[doc = "Set the field `description`.\nThe description of the rule."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deny_rule`.\n"]
    pub fn set_deny_rule(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IamDenyPolicyRulesElDenyRuleEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.deny_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.deny_rule = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IamDenyPolicyRulesEl {
    type O = terrars::BlockAssignable<IamDenyPolicyRulesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamDenyPolicyRulesEl {}

impl BuildIamDenyPolicyRulesEl {
    pub fn build(self) -> IamDenyPolicyRulesEl {
        IamDenyPolicyRulesEl {
            description: core::default::Default::default(),
            deny_rule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct IamDenyPolicyRulesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamDenyPolicyRulesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IamDenyPolicyRulesElRef {
        IamDenyPolicyRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamDenyPolicyRulesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the rule."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `deny_rule` after provisioning.\n"]
    pub fn deny_rule(&self) -> terrars::ListRef<IamDenyPolicyRulesElDenyRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deny_rule", self.base))
    }
}

#[derive(Serialize)]
pub struct IamDenyPolicyTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IamDenyPolicyTimeoutsEl {
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

impl terrars::ToListMappable for IamDenyPolicyTimeoutsEl {
    type O = terrars::BlockAssignable<IamDenyPolicyTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamDenyPolicyTimeoutsEl {}

impl BuildIamDenyPolicyTimeoutsEl {
    pub fn build(self) -> IamDenyPolicyTimeoutsEl {
        IamDenyPolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct IamDenyPolicyTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamDenyPolicyTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IamDenyPolicyTimeoutsElRef {
        IamDenyPolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamDenyPolicyTimeoutsElRef {
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
struct IamDenyPolicyDynamic {
    rules: std::option::Option<terrars::DynamicBlock<IamDenyPolicyRulesEl>>,
}
