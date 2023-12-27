use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeBackendServiceSignedUrlKeyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    backend_service: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    key_value: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeBackendServiceSignedUrlKeyTimeoutsEl>,
}

struct ComputeBackendServiceSignedUrlKey_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeBackendServiceSignedUrlKeyData>,
}

#[derive(Clone)]
pub struct ComputeBackendServiceSignedUrlKey(Rc<ComputeBackendServiceSignedUrlKey_>);

impl ComputeBackendServiceSignedUrlKey {
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

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeBackendServiceSignedUrlKeyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `backend_service` after provisioning.\nThe backend service this signed URL key belongs."]
    pub fn backend_service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.backend_service", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `key_value` after provisioning.\n128-bit key value used for signing the URL. The key value must be a\nvalid RFC 4648 Section 5 base64url encoded string."]
    pub fn key_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_value", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the signed URL key."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeBackendServiceSignedUrlKeyTimeoutsElRef {
        ComputeBackendServiceSignedUrlKeyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for ComputeBackendServiceSignedUrlKey {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ComputeBackendServiceSignedUrlKey { }

impl ToListMappable for ComputeBackendServiceSignedUrlKey {
    type O = ListRef<ComputeBackendServiceSignedUrlKeyRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ComputeBackendServiceSignedUrlKey_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_backend_service_signed_url_key".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeBackendServiceSignedUrlKey {
    pub tf_id: String,
    #[doc= "The backend service this signed URL key belongs."]
    pub backend_service: PrimField<String>,
    #[doc= "128-bit key value used for signing the URL. The key value must be a\nvalid RFC 4648 Section 5 base64url encoded string."]
    pub key_value: PrimField<String>,
    #[doc= "Name of the signed URL key."]
    pub name: PrimField<String>,
}

impl BuildComputeBackendServiceSignedUrlKey {
    pub fn build(self, stack: &mut Stack) -> ComputeBackendServiceSignedUrlKey {
        let out = ComputeBackendServiceSignedUrlKey(Rc::new(ComputeBackendServiceSignedUrlKey_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeBackendServiceSignedUrlKeyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                backend_service: self.backend_service,
                id: core::default::Default::default(),
                key_value: self.key_value,
                name: self.name,
                project: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeBackendServiceSignedUrlKeyRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeBackendServiceSignedUrlKeyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeBackendServiceSignedUrlKeyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `backend_service` after provisioning.\nThe backend service this signed URL key belongs."]
    pub fn backend_service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.backend_service", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `key_value` after provisioning.\n128-bit key value used for signing the URL. The key value must be a\nvalid RFC 4648 Section 5 base64url encoded string."]
    pub fn key_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_value", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the signed URL key."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeBackendServiceSignedUrlKeyTimeoutsElRef {
        ComputeBackendServiceSignedUrlKeyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct ComputeBackendServiceSignedUrlKeyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl ComputeBackendServiceSignedUrlKeyTimeoutsEl {
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
}

impl ToListMappable for ComputeBackendServiceSignedUrlKeyTimeoutsEl {
    type O = BlockAssignable<ComputeBackendServiceSignedUrlKeyTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeBackendServiceSignedUrlKeyTimeoutsEl {}

impl BuildComputeBackendServiceSignedUrlKeyTimeoutsEl {
    pub fn build(self) -> ComputeBackendServiceSignedUrlKeyTimeoutsEl {
        ComputeBackendServiceSignedUrlKeyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct ComputeBackendServiceSignedUrlKeyTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ComputeBackendServiceSignedUrlKeyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeBackendServiceSignedUrlKeyTimeoutsElRef {
        ComputeBackendServiceSignedUrlKeyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeBackendServiceSignedUrlKeyTimeoutsElRef {
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
}
