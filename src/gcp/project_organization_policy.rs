use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ProjectOrganizationPolicyData {
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
    constraint: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    project: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boolean_policy: std::option::Option<std::vec::Vec<ProjectOrganizationPolicyBooleanPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    list_policy: std::option::Option<std::vec::Vec<ProjectOrganizationPolicyListPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restore_policy: std::option::Option<std::vec::Vec<ProjectOrganizationPolicyRestorePolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ProjectOrganizationPolicyTimeoutsEl>,
    dynamic: ProjectOrganizationPolicyDynamic,
}

struct ProjectOrganizationPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ProjectOrganizationPolicyData>,
}

#[derive(Clone)]
pub struct ProjectOrganizationPolicy(std::rc::Rc<ProjectOrganizationPolicy_>);

impl ProjectOrganizationPolicy {
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

    #[doc = "Set the field `version`.\nVersion of the Policy. Default version is 0."]
    pub fn set_version(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `boolean_policy`.\n"]
    pub fn set_boolean_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ProjectOrganizationPolicyBooleanPolicyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().boolean_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.boolean_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `list_policy`.\n"]
    pub fn set_list_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ProjectOrganizationPolicyListPolicyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().list_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.list_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `restore_policy`.\n"]
    pub fn set_restore_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ProjectOrganizationPolicyRestorePolicyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().restore_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.restore_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ProjectOrganizationPolicyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `constraint` after provisioning.\nThe name of the Constraint the Policy is configuring, for example, serviceuser.services."]
    pub fn constraint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.constraint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThe etag of the organization policy. etag is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project ID."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds, representing when the variable was last updated. Example: \"2016-10-09T12:33:37.578138407Z\"."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nVersion of the Policy. Default version is 0."]
    pub fn version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `boolean_policy` after provisioning.\n"]
    pub fn boolean_policy(&self) -> terrars::ListRef<ProjectOrganizationPolicyBooleanPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.boolean_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `list_policy` after provisioning.\n"]
    pub fn list_policy(&self) -> terrars::ListRef<ProjectOrganizationPolicyListPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.list_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restore_policy` after provisioning.\n"]
    pub fn restore_policy(&self) -> terrars::ListRef<ProjectOrganizationPolicyRestorePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restore_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ProjectOrganizationPolicyTimeoutsElRef {
        ProjectOrganizationPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for ProjectOrganizationPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ProjectOrganizationPolicy { }

impl terrars::ToListMappable for ProjectOrganizationPolicy {
    type O = terrars::ListRef<ProjectOrganizationPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ProjectOrganizationPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_project_organization_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildProjectOrganizationPolicy {
    pub tf_id: std::string::String,
    #[doc = "The name of the Constraint the Policy is configuring, for example, serviceuser.services."]
    pub constraint: terrars::PrimField<std::string::String>,
    #[doc = "The project ID."]
    pub project: terrars::PrimField<std::string::String>,
}

impl BuildProjectOrganizationPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> ProjectOrganizationPolicy {
        let out = ProjectOrganizationPolicy(std::rc::Rc::new(ProjectOrganizationPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ProjectOrganizationPolicyData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                constraint: self.constraint,
                id: core::default::Default::default(),
                project: self.project,
                version: core::default::Default::default(),
                boolean_policy: core::default::Default::default(),
                list_policy: core::default::Default::default(),
                restore_policy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ProjectOrganizationPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ProjectOrganizationPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ProjectOrganizationPolicyRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `constraint` after provisioning.\nThe name of the Constraint the Policy is configuring, for example, serviceuser.services."]
    pub fn constraint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.constraint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThe etag of the organization policy. etag is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project ID."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp in RFC3339 UTC \"Zulu\" format, accurate to nanoseconds, representing when the variable was last updated. Example: \"2016-10-09T12:33:37.578138407Z\"."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nVersion of the Policy. Default version is 0."]
    pub fn version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `boolean_policy` after provisioning.\n"]
    pub fn boolean_policy(&self) -> terrars::ListRef<ProjectOrganizationPolicyBooleanPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.boolean_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `list_policy` after provisioning.\n"]
    pub fn list_policy(&self) -> terrars::ListRef<ProjectOrganizationPolicyListPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.list_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restore_policy` after provisioning.\n"]
    pub fn restore_policy(&self) -> terrars::ListRef<ProjectOrganizationPolicyRestorePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restore_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ProjectOrganizationPolicyTimeoutsElRef {
        ProjectOrganizationPolicyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct ProjectOrganizationPolicyBooleanPolicyEl {
    enforced: terrars::PrimField<bool>,
}

impl ProjectOrganizationPolicyBooleanPolicyEl { }

impl terrars::ToListMappable for ProjectOrganizationPolicyBooleanPolicyEl {
    type O = terrars::BlockAssignable<ProjectOrganizationPolicyBooleanPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildProjectOrganizationPolicyBooleanPolicyEl {
    #[doc = "If true, then the Policy is enforced. If false, then any configuration is acceptable."]
    pub enforced: terrars::PrimField<bool>,
}

impl BuildProjectOrganizationPolicyBooleanPolicyEl {
    pub fn build(self) -> ProjectOrganizationPolicyBooleanPolicyEl {
        ProjectOrganizationPolicyBooleanPolicyEl { enforced: self.enforced }
    }
}

pub struct ProjectOrganizationPolicyBooleanPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ProjectOrganizationPolicyBooleanPolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ProjectOrganizationPolicyBooleanPolicyElRef {
        ProjectOrganizationPolicyBooleanPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ProjectOrganizationPolicyBooleanPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enforced` after provisioning.\nIf true, then the Policy is enforced. If false, then any configuration is acceptable."]
    pub fn enforced(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforced", self.base))
    }
}

#[derive(Serialize)]
pub struct ProjectOrganizationPolicyListPolicyElAllowEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl ProjectOrganizationPolicyListPolicyElAllowEl {
    #[doc = "Set the field `all`.\nThe policy allows or denies all values."]
    pub fn set_all(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\nThe policy can define specific values that are allowed or denied."]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ProjectOrganizationPolicyListPolicyElAllowEl {
    type O = terrars::BlockAssignable<ProjectOrganizationPolicyListPolicyElAllowEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildProjectOrganizationPolicyListPolicyElAllowEl {}

impl BuildProjectOrganizationPolicyListPolicyElAllowEl {
    pub fn build(self) -> ProjectOrganizationPolicyListPolicyElAllowEl {
        ProjectOrganizationPolicyListPolicyElAllowEl {
            all: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct ProjectOrganizationPolicyListPolicyElAllowElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ProjectOrganizationPolicyListPolicyElAllowElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ProjectOrganizationPolicyListPolicyElAllowElRef {
        ProjectOrganizationPolicyListPolicyElAllowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ProjectOrganizationPolicyListPolicyElAllowElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `all` after provisioning.\nThe policy allows or denies all values."]
    pub fn all(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.all", self.base))
    }

    #[doc =
        "Get a reference to the value of field `values` after provisioning.\nThe policy can define specific values that are allowed or denied."]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct ProjectOrganizationPolicyListPolicyElDenyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl ProjectOrganizationPolicyListPolicyElDenyEl {
    #[doc = "Set the field `all`.\nThe policy allows or denies all values."]
    pub fn set_all(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\nThe policy can define specific values that are allowed or denied."]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ProjectOrganizationPolicyListPolicyElDenyEl {
    type O = terrars::BlockAssignable<ProjectOrganizationPolicyListPolicyElDenyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildProjectOrganizationPolicyListPolicyElDenyEl {}

impl BuildProjectOrganizationPolicyListPolicyElDenyEl {
    pub fn build(self) -> ProjectOrganizationPolicyListPolicyElDenyEl {
        ProjectOrganizationPolicyListPolicyElDenyEl {
            all: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct ProjectOrganizationPolicyListPolicyElDenyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ProjectOrganizationPolicyListPolicyElDenyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ProjectOrganizationPolicyListPolicyElDenyElRef {
        ProjectOrganizationPolicyListPolicyElDenyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ProjectOrganizationPolicyListPolicyElDenyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `all` after provisioning.\nThe policy allows or denies all values."]
    pub fn all(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.all", self.base))
    }

    #[doc =
        "Get a reference to the value of field `values` after provisioning.\nThe policy can define specific values that are allowed or denied."]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct ProjectOrganizationPolicyListPolicyElDynamic {
    allow: std::option::Option<terrars::DynamicBlock<ProjectOrganizationPolicyListPolicyElAllowEl>>,
    deny: std::option::Option<terrars::DynamicBlock<ProjectOrganizationPolicyListPolicyElDenyEl>>,
}

#[derive(Serialize)]
pub struct ProjectOrganizationPolicyListPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inherit_from_parent: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suggested_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow: std::option::Option<std::vec::Vec<ProjectOrganizationPolicyListPolicyElAllowEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deny: std::option::Option<std::vec::Vec<ProjectOrganizationPolicyListPolicyElDenyEl>>,
    dynamic: ProjectOrganizationPolicyListPolicyElDynamic,
}

impl ProjectOrganizationPolicyListPolicyEl {
    #[doc =
        "Set the field `inherit_from_parent`.\nIf set to true, the values from the effective Policy of the parent resource are inherited, meaning the values set in this Policy are added to the values inherited up the hierarchy."]
    pub fn set_inherit_from_parent(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.inherit_from_parent = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `suggested_value`.\nThe Google Cloud Console will try to default to a configuration that matches the value specified in this field."]
    pub fn set_suggested_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.suggested_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allow`.\n"]
    pub fn set_allow(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ProjectOrganizationPolicyListPolicyElAllowEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.allow = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.allow = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `deny`.\n"]
    pub fn set_deny(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ProjectOrganizationPolicyListPolicyElDenyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.deny = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.deny = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ProjectOrganizationPolicyListPolicyEl {
    type O = terrars::BlockAssignable<ProjectOrganizationPolicyListPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildProjectOrganizationPolicyListPolicyEl {}

impl BuildProjectOrganizationPolicyListPolicyEl {
    pub fn build(self) -> ProjectOrganizationPolicyListPolicyEl {
        ProjectOrganizationPolicyListPolicyEl {
            inherit_from_parent: core::default::Default::default(),
            suggested_value: core::default::Default::default(),
            allow: core::default::Default::default(),
            deny: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ProjectOrganizationPolicyListPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ProjectOrganizationPolicyListPolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ProjectOrganizationPolicyListPolicyElRef {
        ProjectOrganizationPolicyListPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ProjectOrganizationPolicyListPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `inherit_from_parent` after provisioning.\nIf set to true, the values from the effective Policy of the parent resource are inherited, meaning the values set in this Policy are added to the values inherited up the hierarchy."]
    pub fn inherit_from_parent(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inherit_from_parent", self.base))
    }

    #[doc =
        "Get a reference to the value of field `suggested_value` after provisioning.\nThe Google Cloud Console will try to default to a configuration that matches the value specified in this field."]
    pub fn suggested_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suggested_value", self.base))
    }

    #[doc = "Get a reference to the value of field `allow` after provisioning.\n"]
    pub fn allow(&self) -> terrars::ListRef<ProjectOrganizationPolicyListPolicyElAllowElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow", self.base))
    }

    #[doc = "Get a reference to the value of field `deny` after provisioning.\n"]
    pub fn deny(&self) -> terrars::ListRef<ProjectOrganizationPolicyListPolicyElDenyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deny", self.base))
    }
}

#[derive(Serialize)]
pub struct ProjectOrganizationPolicyRestorePolicyEl {
    default: terrars::PrimField<bool>,
}

impl ProjectOrganizationPolicyRestorePolicyEl { }

impl terrars::ToListMappable for ProjectOrganizationPolicyRestorePolicyEl {
    type O = terrars::BlockAssignable<ProjectOrganizationPolicyRestorePolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildProjectOrganizationPolicyRestorePolicyEl {
    #[doc = "May only be set to true. If set, then the default Policy is restored."]
    pub default: terrars::PrimField<bool>,
}

impl BuildProjectOrganizationPolicyRestorePolicyEl {
    pub fn build(self) -> ProjectOrganizationPolicyRestorePolicyEl {
        ProjectOrganizationPolicyRestorePolicyEl { default: self.default }
    }
}

pub struct ProjectOrganizationPolicyRestorePolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ProjectOrganizationPolicyRestorePolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ProjectOrganizationPolicyRestorePolicyElRef {
        ProjectOrganizationPolicyRestorePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ProjectOrganizationPolicyRestorePolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `default` after provisioning.\nMay only be set to true. If set, then the default Policy is restored."]
    pub fn default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default", self.base))
    }
}

#[derive(Serialize)]
pub struct ProjectOrganizationPolicyTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ProjectOrganizationPolicyTimeoutsEl {
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

    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.read = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ProjectOrganizationPolicyTimeoutsEl {
    type O = terrars::BlockAssignable<ProjectOrganizationPolicyTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildProjectOrganizationPolicyTimeoutsEl {}

impl BuildProjectOrganizationPolicyTimeoutsEl {
    pub fn build(self) -> ProjectOrganizationPolicyTimeoutsEl {
        ProjectOrganizationPolicyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            read: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ProjectOrganizationPolicyTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ProjectOrganizationPolicyTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ProjectOrganizationPolicyTimeoutsElRef {
        ProjectOrganizationPolicyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ProjectOrganizationPolicyTimeoutsElRef {
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

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct ProjectOrganizationPolicyDynamic {
    boolean_policy: std::option::Option<terrars::DynamicBlock<ProjectOrganizationPolicyBooleanPolicyEl>>,
    list_policy: std::option::Option<terrars::DynamicBlock<ProjectOrganizationPolicyListPolicyEl>>,
    restore_policy: std::option::Option<terrars::DynamicBlock<ProjectOrganizationPolicyRestorePolicyEl>>,
}
