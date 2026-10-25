#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct OrgPolicyPolicyData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    parent: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dry_run_spec: std::option::Option<std::vec::Vec<OrgPolicyPolicy_DryRunSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spec: std::option::Option<std::vec::Vec<OrgPolicyPolicy_Spec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<OrgPolicyPolicy_Timeouts>,
    dynamic: OrgPolicyPolicyDynamic,
}

struct OrgPolicyPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<OrgPolicyPolicyData_>,
}

#[derive(Clone)]
pub struct OrgPolicyPolicy(std::rc::Rc<OrgPolicyPolicy_>);

impl OrgPolicyPolicy {
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

    #[doc = "Set the field `dry_run_spec`.\n"]
    pub fn set_dry_run_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OrgPolicyPolicy_DryRunSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dry_run_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dry_run_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `spec`.\n"]
    pub fn set_spec(self, v: impl std::convert::Into<terrars::BlockAssignable<OrgPolicyPolicy_Spec>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<OrgPolicyPolicy_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nOptional. An opaque tag indicating the current state of the policy, used for concurrency control. This 'etag' is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nImmutable. The resource name of the Policy. Must be one of the following forms, where constraint_name is the name of the constraint which this Policy configures: * 'projects/{project_number}/policies/{constraint_name}' * 'folders/{folder_id}/policies/{constraint_name}' * 'organizations/{organization_id}/policies/{constraint_name}' For example, \"projects/123/policies/compute.disableSerialPortAccess\". Note: 'projects/{project_id}/policies/{constraint_name}' is also an acceptable name for API requests, but responses will return the name using the equivalent project number."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe parent of the resource."]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dry_run_spec` after provisioning.\n"]
    pub fn dry_run_spec(&self) -> terrars::ListRef<OrgPolicyPolicy_DryRunSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dry_run_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> terrars::ListRef<OrgPolicyPolicy_SpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OrgPolicyPolicy_TimeoutsRef {
        OrgPolicyPolicy_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for OrgPolicyPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for OrgPolicyPolicy { }

impl terrars::ToListMappable for OrgPolicyPolicy {
    type O = terrars::ListRef<OrgPolicyPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for OrgPolicyPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_org_policy_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOrgPolicyPolicy {
    pub tf_id: std::string::String,
    #[doc =
        "Immutable. The resource name of the Policy. Must be one of the following forms, where constraint_name is the name of the constraint which this Policy configures: * 'projects/{project_number}/policies/{constraint_name}' * 'folders/{folder_id}/policies/{constraint_name}' * 'organizations/{organization_id}/policies/{constraint_name}' For example, \"projects/123/policies/compute.disableSerialPortAccess\". Note: 'projects/{project_id}/policies/{constraint_name}' is also an acceptable name for API requests, but responses will return the name using the equivalent project number."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The parent of the resource."]
    pub parent: terrars::PrimField<std::string::String>,
}

impl BuildOrgPolicyPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> OrgPolicyPolicy {
        let out = OrgPolicyPolicy(std::rc::Rc::new(OrgPolicyPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(OrgPolicyPolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                parent: self.parent,
                dry_run_spec: core::default::Default::default(),
                spec: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OrgPolicyPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OrgPolicyPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OrgPolicyPolicyRef {
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
        "Get a reference to the value of field `etag` after provisioning.\nOptional. An opaque tag indicating the current state of the policy, used for concurrency control. This 'etag' is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nImmutable. The resource name of the Policy. Must be one of the following forms, where constraint_name is the name of the constraint which this Policy configures: * 'projects/{project_number}/policies/{constraint_name}' * 'folders/{folder_id}/policies/{constraint_name}' * 'organizations/{organization_id}/policies/{constraint_name}' For example, \"projects/123/policies/compute.disableSerialPortAccess\". Note: 'projects/{project_id}/policies/{constraint_name}' is also an acceptable name for API requests, but responses will return the name using the equivalent project number."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe parent of the resource."]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dry_run_spec` after provisioning.\n"]
    pub fn dry_run_spec(&self) -> terrars::ListRef<OrgPolicyPolicy_DryRunSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dry_run_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> terrars::ListRef<OrgPolicyPolicy_SpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OrgPolicyPolicy_TimeoutsRef {
        OrgPolicyPolicy_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct OrgPolicyPolicyDynamic {
    dry_run_spec: std::option::Option<terrars::DynamicBlock<OrgPolicyPolicy_DryRunSpec>>,
    spec: std::option::Option<terrars::DynamicBlock<OrgPolicyPolicy_Spec>>,
}

#[derive(Serialize)]
pub struct OrgPolicyPolicy_Spec_Rules_Condition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    title: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OrgPolicyPolicy_Spec_Rules_Condition {
    #[doc =
        "Set the field `description`.\nOptional. Description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `expression`.\nTextual representation of an expression in Common Expression Language syntax."]
    pub fn set_expression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expression = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `location`.\nOptional. String indicating the location of the expression for error reporting, e.g. a file name and a position in the file."]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `title`.\nOptional. Title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression."]
    pub fn set_title(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.title = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OrgPolicyPolicy_Spec_Rules_Condition {
    type O = terrars::BlockAssignable<OrgPolicyPolicy_Spec_Rules_Condition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOrgPolicyPolicy_Spec_Rules_Condition {}

impl BuildOrgPolicyPolicy_Spec_Rules_Condition {
    pub fn build(self) -> OrgPolicyPolicy_Spec_Rules_Condition {
        OrgPolicyPolicy_Spec_Rules_Condition {
            description: core::default::Default::default(),
            expression: core::default::Default::default(),
            location: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OrgPolicyPolicy_Spec_Rules_ConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OrgPolicyPolicy_Spec_Rules_ConditionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OrgPolicyPolicy_Spec_Rules_ConditionRef {
        OrgPolicyPolicy_Spec_Rules_ConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OrgPolicyPolicy_Spec_Rules_ConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional. Description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nTextual representation of an expression in Common Expression Language syntax."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nOptional. String indicating the location of the expression for error reporting, e.g. a file name and a position in the file."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc =
        "Get a reference to the value of field `title` after provisioning.\nOptional. Title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression."]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}

#[derive(Serialize)]
pub struct OrgPolicyPolicy_Spec_Rules_Values {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    denied_values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl OrgPolicyPolicy_Spec_Rules_Values {
    #[doc = "Set the field `allowed_values`.\nList of values allowed at this resource."]
    pub fn set_allowed_values(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_values = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `denied_values`.\nList of values denied at this resource."]
    pub fn set_denied_values(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.denied_values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OrgPolicyPolicy_Spec_Rules_Values {
    type O = terrars::BlockAssignable<OrgPolicyPolicy_Spec_Rules_Values>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOrgPolicyPolicy_Spec_Rules_Values {}

impl BuildOrgPolicyPolicy_Spec_Rules_Values {
    pub fn build(self) -> OrgPolicyPolicy_Spec_Rules_Values {
        OrgPolicyPolicy_Spec_Rules_Values {
            allowed_values: core::default::Default::default(),
            denied_values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OrgPolicyPolicy_Spec_Rules_ValuesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OrgPolicyPolicy_Spec_Rules_ValuesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OrgPolicyPolicy_Spec_Rules_ValuesRef {
        OrgPolicyPolicy_Spec_Rules_ValuesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OrgPolicyPolicy_Spec_Rules_ValuesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_values` after provisioning.\nList of values allowed at this resource."]
    pub fn allowed_values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_values", self.base))
    }

    #[doc =
        "Get a reference to the value of field `denied_values` after provisioning.\nList of values denied at this resource."]
    pub fn denied_values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.denied_values", self.base))
    }
}

#[derive(Serialize, Default)]
struct OrgPolicyPolicy_Spec_RulesDynamic {
    condition: std::option::Option<terrars::DynamicBlock<OrgPolicyPolicy_Spec_Rules_Condition>>,
    values: std::option::Option<terrars::DynamicBlock<OrgPolicyPolicy_Spec_Rules_Values>>,
}

#[derive(Serialize)]
pub struct OrgPolicyPolicy_Spec_Rules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_all: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deny_all: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<std::vec::Vec<OrgPolicyPolicy_Spec_Rules_Condition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<std::vec::Vec<OrgPolicyPolicy_Spec_Rules_Values>>,
    dynamic: OrgPolicyPolicy_Spec_RulesDynamic,
}

impl OrgPolicyPolicy_Spec_Rules {
    #[doc =
        "Set the field `allow_all`.\nSetting this to '\"TRUE\"' means that all values are allowed. This field can be set only in Policies for list constraints."]
    pub fn set_allow_all(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.allow_all = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deny_all`.\nSetting this to '\"TRUE\"' means that all values are denied. This field can be set only in Policies for list constraints."]
    pub fn set_deny_all(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.deny_all = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enforce`.\nIf '\"TRUE\"', then the 'Policy' is enforced. If '\"FALSE\"', then any configuration is acceptable. This field can be set only in Policies for boolean constraints."]
    pub fn set_enforce(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.enforce = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `parameters`.\nOptional. Required for Managed Constraints if parameters defined in constraints. Pass parameter values when policy enforcement is enabled. Ensure that parameter value types match those defined in the constraint definition. For example: { \\\"allowedLocations\\\" : [\\\"us-east1\\\", \\\"us-west1\\\"], \\\"allowAll\\\" : true }"]
    pub fn set_parameters(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<OrgPolicyPolicy_Spec_Rules_Condition>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.condition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<OrgPolicyPolicy_Spec_Rules_Values>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.values = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.values = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OrgPolicyPolicy_Spec_Rules {
    type O = terrars::BlockAssignable<OrgPolicyPolicy_Spec_Rules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOrgPolicyPolicy_Spec_Rules {}

impl BuildOrgPolicyPolicy_Spec_Rules {
    pub fn build(self) -> OrgPolicyPolicy_Spec_Rules {
        OrgPolicyPolicy_Spec_Rules {
            allow_all: core::default::Default::default(),
            deny_all: core::default::Default::default(),
            enforce: core::default::Default::default(),
            parameters: core::default::Default::default(),
            condition: core::default::Default::default(),
            values: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OrgPolicyPolicy_Spec_RulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OrgPolicyPolicy_Spec_RulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OrgPolicyPolicy_Spec_RulesRef {
        OrgPolicyPolicy_Spec_RulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OrgPolicyPolicy_Spec_RulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_all` after provisioning.\nSetting this to '\"TRUE\"' means that all values are allowed. This field can be set only in Policies for list constraints."]
    pub fn allow_all(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_all", self.base))
    }

    #[doc =
        "Get a reference to the value of field `deny_all` after provisioning.\nSetting this to '\"TRUE\"' means that all values are denied. This field can be set only in Policies for list constraints."]
    pub fn deny_all(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deny_all", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enforce` after provisioning.\nIf '\"TRUE\"', then the 'Policy' is enforced. If '\"FALSE\"', then any configuration is acceptable. This field can be set only in Policies for boolean constraints."]
    pub fn enforce(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforce", self.base))
    }

    #[doc =
        "Get a reference to the value of field `parameters` after provisioning.\nOptional. Required for Managed Constraints if parameters defined in constraints. Pass parameter values when policy enforcement is enabled. Ensure that parameter value types match those defined in the constraint definition. For example: { \\\"allowedLocations\\\" : [\\\"us-east1\\\", \\\"us-west1\\\"], \\\"allowAll\\\" : true }"]
    pub fn parameters(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> terrars::ListRef<OrgPolicyPolicy_Spec_Rules_ConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<OrgPolicyPolicy_Spec_Rules_ValuesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct OrgPolicyPolicy_DryRunSpecDynamic {
    rules: std::option::Option<terrars::DynamicBlock<OrgPolicyPolicy_Spec_Rules>>,
}

#[derive(Serialize)]
pub struct OrgPolicyPolicy_DryRunSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inherit_from_parent: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reset: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules: std::option::Option<std::vec::Vec<OrgPolicyPolicy_Spec_Rules>>,
    dynamic: OrgPolicyPolicy_DryRunSpecDynamic,
}

impl OrgPolicyPolicy_DryRunSpec {
    #[doc =
        "Set the field `inherit_from_parent`.\nDetermines the inheritance behavior for this policy. If 'inherit_from_parent' is true, policy rules set higher up in the hierarchy (up to the closest root) are inherited and present in the effective policy. If it is false, then no rules are inherited, and this policy becomes the new root for evaluation. This field can be set only for policies which configure list constraints."]
    pub fn set_inherit_from_parent(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.inherit_from_parent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `reset`.\nIgnores policies set above this resource and restores the 'constraint_default' enforcement behavior of the specific constraint at this resource. This field can be set in policies for either list or boolean constraints. If set, 'rules' must be empty and 'inherit_from_parent' must be set to false."]
    pub fn set_reset(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.reset = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<OrgPolicyPolicy_Spec_Rules>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rules = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OrgPolicyPolicy_DryRunSpec {
    type O = terrars::BlockAssignable<OrgPolicyPolicy_DryRunSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOrgPolicyPolicy_DryRunSpec {}

impl BuildOrgPolicyPolicy_DryRunSpec {
    pub fn build(self) -> OrgPolicyPolicy_DryRunSpec {
        OrgPolicyPolicy_DryRunSpec {
            inherit_from_parent: core::default::Default::default(),
            reset: core::default::Default::default(),
            rules: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OrgPolicyPolicy_DryRunSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OrgPolicyPolicy_DryRunSpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OrgPolicyPolicy_DryRunSpecRef {
        OrgPolicyPolicy_DryRunSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OrgPolicyPolicy_DryRunSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nAn opaque tag indicating the current version of the policy, used for concurrency control. This field is ignored if used in a 'CreatePolicy' request. When the policy' is returned from either a 'GetPolicy' or a 'ListPolicies' request, this 'etag' indicates the version of the current policy to use when executing a read-modify-write loop. When the policy is returned from a 'GetEffectivePolicy' request, the 'etag' will be unset."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.base))
    }

    #[doc =
        "Get a reference to the value of field `inherit_from_parent` after provisioning.\nDetermines the inheritance behavior for this policy. If 'inherit_from_parent' is true, policy rules set higher up in the hierarchy (up to the closest root) are inherited and present in the effective policy. If it is false, then no rules are inherited, and this policy becomes the new root for evaluation. This field can be set only for policies which configure list constraints."]
    pub fn inherit_from_parent(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inherit_from_parent", self.base))
    }

    #[doc =
        "Get a reference to the value of field `reset` after provisioning.\nIgnores policies set above this resource and restores the 'constraint_default' enforcement behavior of the specific constraint at this resource. This field can be set in policies for either list or boolean constraints. If set, 'rules' must be empty and 'inherit_from_parent' must be set to false."]
    pub fn reset(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reset", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time stamp this was previously updated. This represents the last time a call to 'CreatePolicy' or 'UpdatePolicy' was made for that policy."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<OrgPolicyPolicy_Spec_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.base))
    }
}

#[derive(Serialize, Default)]
struct OrgPolicyPolicy_SpecDynamic {
    rules: std::option::Option<terrars::DynamicBlock<OrgPolicyPolicy_Spec_Rules>>,
}

#[derive(Serialize)]
pub struct OrgPolicyPolicy_Spec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inherit_from_parent: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reset: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules: std::option::Option<std::vec::Vec<OrgPolicyPolicy_Spec_Rules>>,
    dynamic: OrgPolicyPolicy_SpecDynamic,
}

impl OrgPolicyPolicy_Spec {
    #[doc =
        "Set the field `inherit_from_parent`.\nDetermines the inheritance behavior for this 'Policy'. If 'inherit_from_parent' is true, PolicyRules set higher up in the hierarchy (up to the closest root) are inherited and present in the effective policy. If it is false, then no rules are inherited, and this Policy becomes the new root for evaluation. This field can be set only for Policies which configure list constraints."]
    pub fn set_inherit_from_parent(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.inherit_from_parent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `reset`.\nIgnores policies set above this resource and restores the 'constraint_default' enforcement behavior of the specific 'Constraint' at this resource. This field can be set in policies for either list or boolean constraints. If set, 'rules' must be empty and 'inherit_from_parent' must be set to false."]
    pub fn set_reset(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.reset = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<OrgPolicyPolicy_Spec_Rules>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rules = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OrgPolicyPolicy_Spec {
    type O = terrars::BlockAssignable<OrgPolicyPolicy_Spec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOrgPolicyPolicy_Spec {}

impl BuildOrgPolicyPolicy_Spec {
    pub fn build(self) -> OrgPolicyPolicy_Spec {
        OrgPolicyPolicy_Spec {
            inherit_from_parent: core::default::Default::default(),
            reset: core::default::Default::default(),
            rules: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OrgPolicyPolicy_SpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OrgPolicyPolicy_SpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OrgPolicyPolicy_SpecRef {
        OrgPolicyPolicy_SpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OrgPolicyPolicy_SpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nAn opaque tag indicating the current version of the 'Policy', used for concurrency control. This field is ignored if used in a 'CreatePolicy' request. When the 'Policy' is returned from either a 'GetPolicy' or a 'ListPolicies' request, this 'etag' indicates the version of the current 'Policy' to use when executing a read-modify-write loop. When the 'Policy' is returned from a 'GetEffectivePolicy' request, the 'etag' will be unset."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.base))
    }

    #[doc =
        "Get a reference to the value of field `inherit_from_parent` after provisioning.\nDetermines the inheritance behavior for this 'Policy'. If 'inherit_from_parent' is true, PolicyRules set higher up in the hierarchy (up to the closest root) are inherited and present in the effective policy. If it is false, then no rules are inherited, and this Policy becomes the new root for evaluation. This field can be set only for Policies which configure list constraints."]
    pub fn inherit_from_parent(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inherit_from_parent", self.base))
    }

    #[doc =
        "Get a reference to the value of field `reset` after provisioning.\nIgnores policies set above this resource and restores the 'constraint_default' enforcement behavior of the specific 'Constraint' at this resource. This field can be set in policies for either list or boolean constraints. If set, 'rules' must be empty and 'inherit_from_parent' must be set to false."]
    pub fn reset(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reset", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The time stamp this was previously updated. This represents the last time a call to 'CreatePolicy' or 'UpdatePolicy' was made for that 'Policy'."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<OrgPolicyPolicy_Spec_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.base))
    }
}

#[derive(Serialize)]
pub struct OrgPolicyPolicy_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OrgPolicyPolicy_Timeouts {
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

impl terrars::ToListMappable for OrgPolicyPolicy_Timeouts {
    type O = terrars::BlockAssignable<OrgPolicyPolicy_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOrgPolicyPolicy_Timeouts {}

impl BuildOrgPolicyPolicy_Timeouts {
    pub fn build(self) -> OrgPolicyPolicy_Timeouts {
        OrgPolicyPolicy_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct OrgPolicyPolicy_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OrgPolicyPolicy_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OrgPolicyPolicy_TimeoutsRef {
        OrgPolicyPolicy_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OrgPolicyPolicy_TimeoutsRef {
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
