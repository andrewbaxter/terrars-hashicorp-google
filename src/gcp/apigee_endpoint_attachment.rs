use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ApigeeEndpointAttachmentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    endpoint_attachment_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    org_id: PrimField<String>,
    service_attachment: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ApigeeEndpointAttachmentTimeoutsEl>,
}

struct ApigeeEndpointAttachment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApigeeEndpointAttachmentData>,
}

#[derive(Clone)]
pub struct ApigeeEndpointAttachment(Rc<ApigeeEndpointAttachment_>);

impl ApigeeEndpointAttachment {
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
    pub fn set_timeouts(self, v: impl Into<ApigeeEndpointAttachmentTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `connection_state` after provisioning.\nState of the endpoint attachment connection to the service attachment."]
    pub fn connection_state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connection_state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `endpoint_attachment_id` after provisioning.\nID of the endpoint attachment."]
    pub fn endpoint_attachment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_attachment_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `host` after provisioning.\nHost that can be used in either HTTP Target Endpoint directly, or as the host in Target Server."]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nLocation of the endpoint attachment."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the Endpoint Attachment in the following format:\norganizations/{organization}/endpointAttachments/{endpointAttachment}."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `org_id` after provisioning.\nThe Apigee Organization associated with the Apigee instance,\nin the format 'organizations/{{org_name}}'."]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.org_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service_attachment` after provisioning.\nFormat: projects/*/regions/*/serviceAttachments/*"]
    pub fn service_attachment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_attachment", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeEndpointAttachmentTimeoutsElRef {
        ApigeeEndpointAttachmentTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for ApigeeEndpointAttachment {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ApigeeEndpointAttachment { }

impl ToListMappable for ApigeeEndpointAttachment {
    type O = ListRef<ApigeeEndpointAttachmentRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ApigeeEndpointAttachment_ {
    fn extract_resource_type(&self) -> String {
        "google_apigee_endpoint_attachment".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApigeeEndpointAttachment {
    pub tf_id: String,
    #[doc= "ID of the endpoint attachment."]
    pub endpoint_attachment_id: PrimField<String>,
    #[doc= "Location of the endpoint attachment."]
    pub location: PrimField<String>,
    #[doc= "The Apigee Organization associated with the Apigee instance,\nin the format 'organizations/{{org_name}}'."]
    pub org_id: PrimField<String>,
    #[doc= "Format: projects/*/regions/*/serviceAttachments/*"]
    pub service_attachment: PrimField<String>,
}

impl BuildApigeeEndpointAttachment {
    pub fn build(self, stack: &mut Stack) -> ApigeeEndpointAttachment {
        let out = ApigeeEndpointAttachment(Rc::new(ApigeeEndpointAttachment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApigeeEndpointAttachmentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                endpoint_attachment_id: self.endpoint_attachment_id,
                id: core::default::Default::default(),
                location: self.location,
                org_id: self.org_id,
                service_attachment: self.service_attachment,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApigeeEndpointAttachmentRef {
    shared: StackShared,
    base: String,
}

impl Ref for ApigeeEndpointAttachmentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ApigeeEndpointAttachmentRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `connection_state` after provisioning.\nState of the endpoint attachment connection to the service attachment."]
    pub fn connection_state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.connection_state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `endpoint_attachment_id` after provisioning.\nID of the endpoint attachment."]
    pub fn endpoint_attachment_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_attachment_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `host` after provisioning.\nHost that can be used in either HTTP Target Endpoint directly, or as the host in Target Server."]
    pub fn host(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.host", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nLocation of the endpoint attachment."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the Endpoint Attachment in the following format:\norganizations/{organization}/endpointAttachments/{endpointAttachment}."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `org_id` after provisioning.\nThe Apigee Organization associated with the Apigee instance,\nin the format 'organizations/{{org_name}}'."]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.org_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `service_attachment` after provisioning.\nFormat: projects/*/regions/*/serviceAttachments/*"]
    pub fn service_attachment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_attachment", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeEndpointAttachmentTimeoutsElRef {
        ApigeeEndpointAttachmentTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ApigeeEndpointAttachmentTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl ApigeeEndpointAttachmentTimeoutsEl {
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

impl ToListMappable for ApigeeEndpointAttachmentTimeoutsEl {
    type O = BlockAssignable<ApigeeEndpointAttachmentTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeEndpointAttachmentTimeoutsEl {}

impl BuildApigeeEndpointAttachmentTimeoutsEl {
    pub fn build(self) -> ApigeeEndpointAttachmentTimeoutsEl {
        ApigeeEndpointAttachmentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct ApigeeEndpointAttachmentTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ApigeeEndpointAttachmentTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ApigeeEndpointAttachmentTimeoutsElRef {
        ApigeeEndpointAttachmentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeEndpointAttachmentTimeoutsElRef {
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
