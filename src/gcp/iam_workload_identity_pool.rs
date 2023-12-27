use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct IamWorkloadIdentityPoolData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    workload_identity_pool_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<IamWorkloadIdentityPoolTimeoutsEl>,
}

struct IamWorkloadIdentityPool_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<IamWorkloadIdentityPoolData>,
}

#[derive(Clone)]
pub struct IamWorkloadIdentityPool(Rc<IamWorkloadIdentityPool_>);

impl IamWorkloadIdentityPool {
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

    #[doc= "Set the field `description`.\nA description of the pool. Cannot exceed 256 characters."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `disabled`.\nWhether the pool is disabled. You cannot use a disabled pool to exchange tokens, or use\nexisting tokens to access resources. If the pool is re-enabled, existing tokens grant\naccess again."]
    pub fn set_disabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disabled = Some(v.into());
        self
    }

    #[doc= "Set the field `display_name`.\nA display name for the pool. Cannot exceed 32 characters."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<IamWorkloadIdentityPoolTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA description of the pool. Cannot exceed 256 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disabled` after provisioning.\nWhether the pool is disabled. You cannot use a disabled pool to exchange tokens, or use\nexisting tokens to access resources. If the pool is re-enabled, existing tokens grant\naccess again."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `display_name` after provisioning.\nA display name for the pool. Cannot exceed 32 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe resource name of the pool as\n'projects/{project_number}/locations/global/workloadIdentityPools/{workload_identity_pool_id}'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\nThe state of the pool.\n* STATE_UNSPECIFIED: State unspecified.\n* ACTIVE: The pool is active, and may be used in Google Cloud policies.\n* DELETED: The pool is soft-deleted. Soft-deleted pools are permanently deleted after\n  approximately 30 days. You can restore a soft-deleted pool using\n  UndeleteWorkloadIdentityPool. You cannot reuse the ID of a soft-deleted pool until it is\n  permanently deleted. While a pool is deleted, you cannot use it to exchange tokens, or\n  use existing tokens to access resources. If the pool is undeleted, existing tokens grant\n  access again."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `workload_identity_pool_id` after provisioning.\nThe ID to use for the pool, which becomes the final component of the resource name. This\nvalue should be 4-32 characters, and may contain the characters [a-z0-9-]. The prefix\n'gcp-' is reserved for use by Google, and may not be specified."]
    pub fn workload_identity_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.workload_identity_pool_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamWorkloadIdentityPoolTimeoutsElRef {
        IamWorkloadIdentityPoolTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for IamWorkloadIdentityPool {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for IamWorkloadIdentityPool { }

impl ToListMappable for IamWorkloadIdentityPool {
    type O = ListRef<IamWorkloadIdentityPoolRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for IamWorkloadIdentityPool_ {
    fn extract_resource_type(&self) -> String {
        "google_iam_workload_identity_pool".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIamWorkloadIdentityPool {
    pub tf_id: String,
    #[doc= "The ID to use for the pool, which becomes the final component of the resource name. This\nvalue should be 4-32 characters, and may contain the characters [a-z0-9-]. The prefix\n'gcp-' is reserved for use by Google, and may not be specified."]
    pub workload_identity_pool_id: PrimField<String>,
}

impl BuildIamWorkloadIdentityPool {
    pub fn build(self, stack: &mut Stack) -> IamWorkloadIdentityPool {
        let out = IamWorkloadIdentityPool(Rc::new(IamWorkloadIdentityPool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(IamWorkloadIdentityPoolData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                disabled: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                workload_identity_pool_id: self.workload_identity_pool_id,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IamWorkloadIdentityPoolRef {
    shared: StackShared,
    base: String,
}

impl Ref for IamWorkloadIdentityPoolRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl IamWorkloadIdentityPoolRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nA description of the pool. Cannot exceed 256 characters."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `disabled` after provisioning.\nWhether the pool is disabled. You cannot use a disabled pool to exchange tokens, or use\nexisting tokens to access resources. If the pool is re-enabled, existing tokens grant\naccess again."]
    pub fn disabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `display_name` after provisioning.\nA display name for the pool. Cannot exceed 32 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe resource name of the pool as\n'projects/{project_number}/locations/global/workloadIdentityPools/{workload_identity_pool_id}'."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\nThe state of the pool.\n* STATE_UNSPECIFIED: State unspecified.\n* ACTIVE: The pool is active, and may be used in Google Cloud policies.\n* DELETED: The pool is soft-deleted. Soft-deleted pools are permanently deleted after\n  approximately 30 days. You can restore a soft-deleted pool using\n  UndeleteWorkloadIdentityPool. You cannot reuse the ID of a soft-deleted pool until it is\n  permanently deleted. While a pool is deleted, you cannot use it to exchange tokens, or\n  use existing tokens to access resources. If the pool is undeleted, existing tokens grant\n  access again."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `workload_identity_pool_id` after provisioning.\nThe ID to use for the pool, which becomes the final component of the resource name. This\nvalue should be 4-32 characters, and may contain the characters [a-z0-9-]. The prefix\n'gcp-' is reserved for use by Google, and may not be specified."]
    pub fn workload_identity_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.workload_identity_pool_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamWorkloadIdentityPoolTimeoutsElRef {
        IamWorkloadIdentityPoolTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct IamWorkloadIdentityPoolTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl IamWorkloadIdentityPoolTimeoutsEl {
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

impl ToListMappable for IamWorkloadIdentityPoolTimeoutsEl {
    type O = BlockAssignable<IamWorkloadIdentityPoolTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamWorkloadIdentityPoolTimeoutsEl {}

impl BuildIamWorkloadIdentityPoolTimeoutsEl {
    pub fn build(self) -> IamWorkloadIdentityPoolTimeoutsEl {
        IamWorkloadIdentityPoolTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct IamWorkloadIdentityPoolTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for IamWorkloadIdentityPoolTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> IamWorkloadIdentityPoolTimeoutsElRef {
        IamWorkloadIdentityPoolTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamWorkloadIdentityPoolTimeoutsElRef {
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
