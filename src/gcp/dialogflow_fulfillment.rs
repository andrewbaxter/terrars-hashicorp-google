use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DialogflowFulfillmentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    features: Option<Vec<DialogflowFulfillmentFeaturesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generic_web_service: Option<Vec<DialogflowFulfillmentGenericWebServiceEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DialogflowFulfillmentTimeoutsEl>,
    dynamic: DialogflowFulfillmentDynamic,
}

struct DialogflowFulfillment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DialogflowFulfillmentData>,
}

#[derive(Clone)]
pub struct DialogflowFulfillment(Rc<DialogflowFulfillment_>);

impl DialogflowFulfillment {
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

    #[doc= "Set the field `enabled`.\nWhether fulfillment is enabled."]
    pub fn set_enabled(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = Some(v.into());
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

    #[doc= "Set the field `features`.\n"]
    pub fn set_features(self, v: impl Into<BlockAssignable<DialogflowFulfillmentFeaturesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().features = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.features = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `generic_web_service`.\n"]
    pub fn set_generic_web_service(
        self,
        v: impl Into<BlockAssignable<DialogflowFulfillmentGenericWebServiceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().generic_web_service = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.generic_web_service = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DialogflowFulfillmentTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the fulfillment, unique within the agent."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nWhether fulfillment is enabled."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the fulfillment.\nFormat: projects/<Project ID>/agent/fulfillment - projects/<Project ID>/locations/<Location ID>/agent/fulfillment"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `features` after provisioning.\n"]
    pub fn features(&self) -> ListRef<DialogflowFulfillmentFeaturesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.features", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `generic_web_service` after provisioning.\n"]
    pub fn generic_web_service(&self) -> ListRef<DialogflowFulfillmentGenericWebServiceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.generic_web_service", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowFulfillmentTimeoutsElRef {
        DialogflowFulfillmentTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for DialogflowFulfillment {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for DialogflowFulfillment { }

impl ToListMappable for DialogflowFulfillment {
    type O = ListRef<DialogflowFulfillmentRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DialogflowFulfillment_ {
    fn extract_resource_type(&self) -> String {
        "google_dialogflow_fulfillment".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDialogflowFulfillment {
    pub tf_id: String,
    #[doc= "The human-readable name of the fulfillment, unique within the agent."]
    pub display_name: PrimField<String>,
}

impl BuildDialogflowFulfillment {
    pub fn build(self, stack: &mut Stack) -> DialogflowFulfillment {
        let out = DialogflowFulfillment(Rc::new(DialogflowFulfillment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DialogflowFulfillmentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                display_name: self.display_name,
                enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                features: core::default::Default::default(),
                generic_web_service: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DialogflowFulfillmentRef {
    shared: StackShared,
    base: String,
}

impl Ref for DialogflowFulfillmentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DialogflowFulfillmentRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the fulfillment, unique within the agent."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\nWhether fulfillment is enabled."]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the fulfillment.\nFormat: projects/<Project ID>/agent/fulfillment - projects/<Project ID>/locations/<Location ID>/agent/fulfillment"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `features` after provisioning.\n"]
    pub fn features(&self) -> ListRef<DialogflowFulfillmentFeaturesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.features", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `generic_web_service` after provisioning.\n"]
    pub fn generic_web_service(&self) -> ListRef<DialogflowFulfillmentGenericWebServiceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.generic_web_service", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowFulfillmentTimeoutsElRef {
        DialogflowFulfillmentTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DialogflowFulfillmentFeaturesEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl DialogflowFulfillmentFeaturesEl { }

impl ToListMappable for DialogflowFulfillmentFeaturesEl {
    type O = BlockAssignable<DialogflowFulfillmentFeaturesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowFulfillmentFeaturesEl {
    #[doc= "The type of the feature that enabled for fulfillment.\n* SMALLTALK: Fulfillment is enabled for SmallTalk. Possible values: [\"SMALLTALK\"]"]
    pub type_: PrimField<String>,
}

impl BuildDialogflowFulfillmentFeaturesEl {
    pub fn build(self) -> DialogflowFulfillmentFeaturesEl {
        DialogflowFulfillmentFeaturesEl { type_: self.type_ }
    }
}

pub struct DialogflowFulfillmentFeaturesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DialogflowFulfillmentFeaturesElRef {
    fn new(shared: StackShared, base: String) -> DialogflowFulfillmentFeaturesElRef {
        DialogflowFulfillmentFeaturesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowFulfillmentFeaturesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nThe type of the feature that enabled for fulfillment.\n* SMALLTALK: Fulfillment is enabled for SmallTalk. Possible values: [\"SMALLTALK\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowFulfillmentGenericWebServiceEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_headers: Option<RecField<PrimField<String>>>,
    uri: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<PrimField<String>>,
}

impl DialogflowFulfillmentGenericWebServiceEl {
    #[doc= "Set the field `password`.\nThe password for HTTP Basic authentication."]
    pub fn set_password(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.password = Some(v.into());
        self
    }

    #[doc= "Set the field `request_headers`.\nThe HTTP request headers to send together with fulfillment requests."]
    pub fn set_request_headers(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.request_headers = Some(v.into());
        self
    }

    #[doc= "Set the field `username`.\nThe user name for HTTP Basic authentication."]
    pub fn set_username(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.username = Some(v.into());
        self
    }
}

impl ToListMappable for DialogflowFulfillmentGenericWebServiceEl {
    type O = BlockAssignable<DialogflowFulfillmentGenericWebServiceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowFulfillmentGenericWebServiceEl {
    #[doc= "The fulfillment URI for receiving POST requests. It must use https protocol."]
    pub uri: PrimField<String>,
}

impl BuildDialogflowFulfillmentGenericWebServiceEl {
    pub fn build(self) -> DialogflowFulfillmentGenericWebServiceEl {
        DialogflowFulfillmentGenericWebServiceEl {
            password: core::default::Default::default(),
            request_headers: core::default::Default::default(),
            uri: self.uri,
            username: core::default::Default::default(),
        }
    }
}

pub struct DialogflowFulfillmentGenericWebServiceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DialogflowFulfillmentGenericWebServiceElRef {
    fn new(shared: StackShared, base: String) -> DialogflowFulfillmentGenericWebServiceElRef {
        DialogflowFulfillmentGenericWebServiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowFulfillmentGenericWebServiceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `password` after provisioning.\nThe password for HTTP Basic authentication."]
    pub fn password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc= "Get a reference to the value of field `request_headers` after provisioning.\nThe HTTP request headers to send together with fulfillment requests."]
    pub fn request_headers(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.request_headers", self.base))
    }

    #[doc= "Get a reference to the value of field `uri` after provisioning.\nThe fulfillment URI for receiving POST requests. It must use https protocol."]
    pub fn uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }

    #[doc= "Get a reference to the value of field `username` after provisioning.\nThe user name for HTTP Basic authentication."]
    pub fn username(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowFulfillmentTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl DialogflowFulfillmentTimeoutsEl {
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

impl ToListMappable for DialogflowFulfillmentTimeoutsEl {
    type O = BlockAssignable<DialogflowFulfillmentTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowFulfillmentTimeoutsEl {}

impl BuildDialogflowFulfillmentTimeoutsEl {
    pub fn build(self) -> DialogflowFulfillmentTimeoutsEl {
        DialogflowFulfillmentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct DialogflowFulfillmentTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DialogflowFulfillmentTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DialogflowFulfillmentTimeoutsElRef {
        DialogflowFulfillmentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowFulfillmentTimeoutsElRef {
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

#[derive(Serialize, Default)]
struct DialogflowFulfillmentDynamic {
    features: Option<DynamicBlock<DialogflowFulfillmentFeaturesEl>>,
    generic_web_service: Option<DynamicBlock<DialogflowFulfillmentGenericWebServiceEl>>,
}
