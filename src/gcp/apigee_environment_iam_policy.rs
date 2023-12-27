use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ApigeeEnvironmentIamPolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    env_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    org_id: PrimField<String>,
    policy_data: PrimField<String>,
}

struct ApigeeEnvironmentIamPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApigeeEnvironmentIamPolicyData>,
}

#[derive(Clone)]
pub struct ApigeeEnvironmentIamPolicy(Rc<ApigeeEnvironmentIamPolicy_>);

impl ApigeeEnvironmentIamPolicy {
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

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `env_id` after provisioning.\n"]
    pub fn env_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.env_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `org_id` after provisioning.\n"]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.org_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `policy_data` after provisioning.\n"]
    pub fn policy_data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_data", self.extract_ref()))
    }
}

impl Referable for ApigeeEnvironmentIamPolicy {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ApigeeEnvironmentIamPolicy { }

impl ToListMappable for ApigeeEnvironmentIamPolicy {
    type O = ListRef<ApigeeEnvironmentIamPolicyRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ApigeeEnvironmentIamPolicy_ {
    fn extract_resource_type(&self) -> String {
        "google_apigee_environment_iam_policy".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApigeeEnvironmentIamPolicy {
    pub tf_id: String,
    #[doc= ""]
    pub env_id: PrimField<String>,
    #[doc= ""]
    pub org_id: PrimField<String>,
    #[doc= ""]
    pub policy_data: PrimField<String>,
}

impl BuildApigeeEnvironmentIamPolicy {
    pub fn build(self, stack: &mut Stack) -> ApigeeEnvironmentIamPolicy {
        let out = ApigeeEnvironmentIamPolicy(Rc::new(ApigeeEnvironmentIamPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApigeeEnvironmentIamPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                env_id: self.env_id,
                id: core::default::Default::default(),
                org_id: self.org_id,
                policy_data: self.policy_data,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApigeeEnvironmentIamPolicyRef {
    shared: StackShared,
    base: String,
}

impl Ref for ApigeeEnvironmentIamPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ApigeeEnvironmentIamPolicyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `env_id` after provisioning.\n"]
    pub fn env_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.env_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `org_id` after provisioning.\n"]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.org_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `policy_data` after provisioning.\n"]
    pub fn policy_data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.policy_data", self.extract_ref()))
    }
}
