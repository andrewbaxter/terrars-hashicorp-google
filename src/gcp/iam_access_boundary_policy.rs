use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct IamAccessBoundaryPolicyData {
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
    rules: std::option::Option<std::vec::Vec<IamAccessBoundaryPolicyRulesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<IamAccessBoundaryPolicyTimeoutsEl>,
    dynamic: IamAccessBoundaryPolicyDynamic,
}

struct IamAccessBoundaryPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<IamAccessBoundaryPolicyData>,
}

#[derive(Clone)]
pub struct IamAccessBoundaryPolicy(std::rc::Rc<IamAccessBoundaryPolicy_>);

impl IamAccessBoundaryPolicy {
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
    pub fn set_rules(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IamAccessBoundaryPolicyRulesEl>>,
    ) -> Self {
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
    pub fn set_timeouts(self, v: impl std::convert::Into<IamAccessBoundaryPolicyTimeoutsEl>) -> Self {
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
    pub fn rules(&self) -> terrars::ListRef<IamAccessBoundaryPolicyRulesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamAccessBoundaryPolicyTimeoutsElRef {
        IamAccessBoundaryPolicyTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for IamAccessBoundaryPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for IamAccessBoundaryPolicy { }

impl terrars::ToListMappable for IamAccessBoundaryPolicy {
    type O = terrars::ListRef<IamAccessBoundaryPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for IamAccessBoundaryPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_iam_access_boundary_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIamAccessBoundaryPolicy {
    pub tf_id: std::string::String,
    #[doc = "The name of the policy."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The attachment point is identified by its URL-encoded full resource name."]
    pub parent: terrars::PrimField<std::string::String>,
}

impl BuildIamAccessBoundaryPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> IamAccessBoundaryPolicy {
        let out = IamAccessBoundaryPolicy(std::rc::Rc::new(IamAccessBoundaryPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(IamAccessBoundaryPolicyData {
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

pub struct IamAccessBoundaryPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamAccessBoundaryPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl IamAccessBoundaryPolicyRef {
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
    pub fn rules(&self) -> terrars::ListRef<IamAccessBoundaryPolicyRulesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamAccessBoundaryPolicyTimeoutsElRef {
        IamAccessBoundaryPolicyTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElAvailabilityConditionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    expression: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    title: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElAvailabilityConditionEl {
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

impl terrars::ToListMappable for IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElAvailabilityConditionEl {
    type O = terrars::BlockAssignable<IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElAvailabilityConditionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamAccessBoundaryPolicyRulesElAccessBoundaryRuleElAvailabilityConditionEl {
    #[doc = "Textual representation of an expression in Common Expression Language syntax."]
    pub expression: terrars::PrimField<std::string::String>,
}

impl BuildIamAccessBoundaryPolicyRulesElAccessBoundaryRuleElAvailabilityConditionEl {
    pub fn build(self) -> IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElAvailabilityConditionEl {
        IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElAvailabilityConditionEl {
            description: core::default::Default::default(),
            expression: self.expression,
            location: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}

pub struct IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElAvailabilityConditionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElAvailabilityConditionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElAvailabilityConditionElRef {
        IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElAvailabilityConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElAvailabilityConditionElRef {
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
struct IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElDynamic {
    availability_condition: std
    ::option
    ::Option<terrars::DynamicBlock<IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElAvailabilityConditionEl>>,
}

#[derive(Serialize)]
pub struct IamAccessBoundaryPolicyRulesElAccessBoundaryRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    available_permissions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    available_resource: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_condition: std
    ::option
    ::Option<std::vec::Vec<IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElAvailabilityConditionEl>>,
    dynamic: IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElDynamic,
}

impl IamAccessBoundaryPolicyRulesElAccessBoundaryRuleEl {
    #[doc =
        "Set the field `available_permissions`.\nA list of permissions that may be allowed for use on the specified resource."]
    pub fn set_available_permissions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.available_permissions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `available_resource`.\nThe full resource name of a Google Cloud resource entity."]
    pub fn set_available_resource(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.available_resource = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `availability_condition`.\n"]
    pub fn set_availability_condition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElAvailabilityConditionEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.availability_condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.availability_condition = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IamAccessBoundaryPolicyRulesElAccessBoundaryRuleEl {
    type O = terrars::BlockAssignable<IamAccessBoundaryPolicyRulesElAccessBoundaryRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamAccessBoundaryPolicyRulesElAccessBoundaryRuleEl {}

impl BuildIamAccessBoundaryPolicyRulesElAccessBoundaryRuleEl {
    pub fn build(self) -> IamAccessBoundaryPolicyRulesElAccessBoundaryRuleEl {
        IamAccessBoundaryPolicyRulesElAccessBoundaryRuleEl {
            available_permissions: core::default::Default::default(),
            available_resource: core::default::Default::default(),
            availability_condition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElRef {
        IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `available_permissions` after provisioning.\nA list of permissions that may be allowed for use on the specified resource."]
    pub fn available_permissions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.available_permissions", self.base))
    }

    #[doc =
        "Get a reference to the value of field `available_resource` after provisioning.\nThe full resource name of a Google Cloud resource entity."]
    pub fn available_resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_resource", self.base))
    }

    #[doc = "Get a reference to the value of field `availability_condition` after provisioning.\n"]
    pub fn availability_condition(
        &self,
    ) -> terrars::ListRef<IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElAvailabilityConditionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.availability_condition", self.base))
    }
}

#[derive(Serialize, Default)]
struct IamAccessBoundaryPolicyRulesElDynamic {
    access_boundary_rule: std
    ::option
    ::Option<terrars::DynamicBlock<IamAccessBoundaryPolicyRulesElAccessBoundaryRuleEl>>,
}

#[derive(Serialize)]
pub struct IamAccessBoundaryPolicyRulesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_boundary_rule: std::option::Option<std::vec::Vec<IamAccessBoundaryPolicyRulesElAccessBoundaryRuleEl>>,
    dynamic: IamAccessBoundaryPolicyRulesElDynamic,
}

impl IamAccessBoundaryPolicyRulesEl {
    #[doc = "Set the field `description`.\nThe description of the rule."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `access_boundary_rule`.\n"]
    pub fn set_access_boundary_rule(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<IamAccessBoundaryPolicyRulesElAccessBoundaryRuleEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.access_boundary_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.access_boundary_rule = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for IamAccessBoundaryPolicyRulesEl {
    type O = terrars::BlockAssignable<IamAccessBoundaryPolicyRulesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamAccessBoundaryPolicyRulesEl {}

impl BuildIamAccessBoundaryPolicyRulesEl {
    pub fn build(self) -> IamAccessBoundaryPolicyRulesEl {
        IamAccessBoundaryPolicyRulesEl {
            description: core::default::Default::default(),
            access_boundary_rule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct IamAccessBoundaryPolicyRulesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamAccessBoundaryPolicyRulesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IamAccessBoundaryPolicyRulesElRef {
        IamAccessBoundaryPolicyRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamAccessBoundaryPolicyRulesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the rule."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `access_boundary_rule` after provisioning.\n"]
    pub fn access_boundary_rule(&self) -> terrars::ListRef<IamAccessBoundaryPolicyRulesElAccessBoundaryRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_boundary_rule", self.base))
    }
}

#[derive(Serialize)]
pub struct IamAccessBoundaryPolicyTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IamAccessBoundaryPolicyTimeoutsEl {
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

impl terrars::ToListMappable for IamAccessBoundaryPolicyTimeoutsEl {
    type O = terrars::BlockAssignable<IamAccessBoundaryPolicyTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamAccessBoundaryPolicyTimeoutsEl {}

impl BuildIamAccessBoundaryPolicyTimeoutsEl {
    pub fn build(self) -> IamAccessBoundaryPolicyTimeoutsEl {
        IamAccessBoundaryPolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct IamAccessBoundaryPolicyTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamAccessBoundaryPolicyTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IamAccessBoundaryPolicyTimeoutsElRef {
        IamAccessBoundaryPolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamAccessBoundaryPolicyTimeoutsElRef {
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
struct IamAccessBoundaryPolicyDynamic {
    rules: std::option::Option<terrars::DynamicBlock<IamAccessBoundaryPolicyRulesEl>>,
}
