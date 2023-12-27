use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct AccessContextManagerGcpUserAccessBindingData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    access_levels: ListField<PrimField<String>>,
    group_key: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    organization_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<AccessContextManagerGcpUserAccessBindingTimeoutsEl>,
}

struct AccessContextManagerGcpUserAccessBinding_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AccessContextManagerGcpUserAccessBindingData>,
}

#[derive(Clone)]
pub struct AccessContextManagerGcpUserAccessBinding(Rc<AccessContextManagerGcpUserAccessBinding_>);

impl AccessContextManagerGcpUserAccessBinding {
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

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<AccessContextManagerGcpUserAccessBindingTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `access_levels` after provisioning.\nRequired. Access level that a user must have to be granted access. Only one access level is supported, not multiple. This repeated field must have exactly one element. Example: \"accessPolicies/9522/accessLevels/device_trusted\""]
    pub fn access_levels(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.access_levels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group_key` after provisioning.\nRequired. Immutable. Google Group id whose members are subject to this binding's restrictions. See \"id\" in the G Suite Directory API's Groups resource. If a group's email address/alias is changed, this resource will continue to point at the changed group. This field does not accept group email addresses or aliases. Example: \"01d520gv4vjcrht\""]
    pub fn group_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nImmutable. Assigned by the server during creation. The last segment has an arbitrary length and has only URI unreserved characters (as defined by RFC 3986 Section 2.3). Should not be specified by the client during creation. Example: \"organizations/256/gcpUserAccessBindings/b3-BhcX_Ud5N\""]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `organization_id` after provisioning.\nRequired. ID of the parent organization."]
    pub fn organization_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.organization_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AccessContextManagerGcpUserAccessBindingTimeoutsElRef {
        AccessContextManagerGcpUserAccessBindingTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for AccessContextManagerGcpUserAccessBinding {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for AccessContextManagerGcpUserAccessBinding { }

impl ToListMappable for AccessContextManagerGcpUserAccessBinding {
    type O = ListRef<AccessContextManagerGcpUserAccessBindingRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for AccessContextManagerGcpUserAccessBinding_ {
    fn extract_resource_type(&self) -> String {
        "google_access_context_manager_gcp_user_access_binding".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAccessContextManagerGcpUserAccessBinding {
    pub tf_id: String,
    #[doc= "Required. Access level that a user must have to be granted access. Only one access level is supported, not multiple. This repeated field must have exactly one element. Example: \"accessPolicies/9522/accessLevels/device_trusted\""]
    pub access_levels: ListField<PrimField<String>>,
    #[doc= "Required. Immutable. Google Group id whose members are subject to this binding's restrictions. See \"id\" in the G Suite Directory API's Groups resource. If a group's email address/alias is changed, this resource will continue to point at the changed group. This field does not accept group email addresses or aliases. Example: \"01d520gv4vjcrht\""]
    pub group_key: PrimField<String>,
    #[doc= "Required. ID of the parent organization."]
    pub organization_id: PrimField<String>,
}

impl BuildAccessContextManagerGcpUserAccessBinding {
    pub fn build(self, stack: &mut Stack) -> AccessContextManagerGcpUserAccessBinding {
        let out = AccessContextManagerGcpUserAccessBinding(Rc::new(AccessContextManagerGcpUserAccessBinding_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AccessContextManagerGcpUserAccessBindingData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                access_levels: self.access_levels,
                group_key: self.group_key,
                id: core::default::Default::default(),
                organization_id: self.organization_id,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AccessContextManagerGcpUserAccessBindingRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessContextManagerGcpUserAccessBindingRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AccessContextManagerGcpUserAccessBindingRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `access_levels` after provisioning.\nRequired. Access level that a user must have to be granted access. Only one access level is supported, not multiple. This repeated field must have exactly one element. Example: \"accessPolicies/9522/accessLevels/device_trusted\""]
    pub fn access_levels(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.access_levels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `group_key` after provisioning.\nRequired. Immutable. Google Group id whose members are subject to this binding's restrictions. See \"id\" in the G Suite Directory API's Groups resource. If a group's email address/alias is changed, this resource will continue to point at the changed group. This field does not accept group email addresses or aliases. Example: \"01d520gv4vjcrht\""]
    pub fn group_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.group_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nImmutable. Assigned by the server during creation. The last segment has an arbitrary length and has only URI unreserved characters (as defined by RFC 3986 Section 2.3). Should not be specified by the client during creation. Example: \"organizations/256/gcpUserAccessBindings/b3-BhcX_Ud5N\""]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `organization_id` after provisioning.\nRequired. ID of the parent organization."]
    pub fn organization_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.organization_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AccessContextManagerGcpUserAccessBindingTimeoutsElRef {
        AccessContextManagerGcpUserAccessBindingTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct AccessContextManagerGcpUserAccessBindingTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl AccessContextManagerGcpUserAccessBindingTimeoutsEl {
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

impl ToListMappable for AccessContextManagerGcpUserAccessBindingTimeoutsEl {
    type O = BlockAssignable<AccessContextManagerGcpUserAccessBindingTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAccessContextManagerGcpUserAccessBindingTimeoutsEl {}

impl BuildAccessContextManagerGcpUserAccessBindingTimeoutsEl {
    pub fn build(self) -> AccessContextManagerGcpUserAccessBindingTimeoutsEl {
        AccessContextManagerGcpUserAccessBindingTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct AccessContextManagerGcpUserAccessBindingTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AccessContextManagerGcpUserAccessBindingTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> AccessContextManagerGcpUserAccessBindingTimeoutsElRef {
        AccessContextManagerGcpUserAccessBindingTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AccessContextManagerGcpUserAccessBindingTimeoutsElRef {
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
