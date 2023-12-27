use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct PubsubLiteSubscriptionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    topic: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delivery_config: Option<Vec<PubsubLiteSubscriptionDeliveryConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<PubsubLiteSubscriptionTimeoutsEl>,
    dynamic: PubsubLiteSubscriptionDynamic,
}

struct PubsubLiteSubscription_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<PubsubLiteSubscriptionData>,
}

#[derive(Clone)]
pub struct PubsubLiteSubscription(Rc<PubsubLiteSubscription_>);

impl PubsubLiteSubscription {
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

    #[doc= "Set the field `region`.\nThe region of the pubsub lite topic."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc= "Set the field `zone`.\nThe zone of the pubsub lite topic."]
    pub fn set_zone(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().zone = Some(v.into());
        self
    }

    #[doc= "Set the field `delivery_config`.\n"]
    pub fn set_delivery_config(self, v: impl Into<BlockAssignable<PubsubLiteSubscriptionDeliveryConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().delivery_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.delivery_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<PubsubLiteSubscriptionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the subscription."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nThe region of the pubsub lite topic."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `topic` after provisioning.\nA reference to a Topic resource."]
    pub fn topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone` after provisioning.\nThe zone of the pubsub lite topic."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `delivery_config` after provisioning.\n"]
    pub fn delivery_config(&self) -> ListRef<PubsubLiteSubscriptionDeliveryConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.delivery_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PubsubLiteSubscriptionTimeoutsElRef {
        PubsubLiteSubscriptionTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for PubsubLiteSubscription {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for PubsubLiteSubscription { }

impl ToListMappable for PubsubLiteSubscription {
    type O = ListRef<PubsubLiteSubscriptionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for PubsubLiteSubscription_ {
    fn extract_resource_type(&self) -> String {
        "google_pubsub_lite_subscription".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPubsubLiteSubscription {
    pub tf_id: String,
    #[doc= "Name of the subscription."]
    pub name: PrimField<String>,
    #[doc= "A reference to a Topic resource."]
    pub topic: PrimField<String>,
}

impl BuildPubsubLiteSubscription {
    pub fn build(self, stack: &mut Stack) -> PubsubLiteSubscription {
        let out = PubsubLiteSubscription(Rc::new(PubsubLiteSubscription_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(PubsubLiteSubscriptionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                topic: self.topic,
                zone: core::default::Default::default(),
                delivery_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PubsubLiteSubscriptionRef {
    shared: StackShared,
    base: String,
}

impl Ref for PubsubLiteSubscriptionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PubsubLiteSubscriptionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the subscription."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\nThe region of the pubsub lite topic."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `topic` after provisioning.\nA reference to a Topic resource."]
    pub fn topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `zone` after provisioning.\nThe zone of the pubsub lite topic."]
    pub fn zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.zone", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `delivery_config` after provisioning.\n"]
    pub fn delivery_config(&self) -> ListRef<PubsubLiteSubscriptionDeliveryConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.delivery_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PubsubLiteSubscriptionTimeoutsElRef {
        PubsubLiteSubscriptionTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct PubsubLiteSubscriptionDeliveryConfigEl {
    delivery_requirement: PrimField<String>,
}

impl PubsubLiteSubscriptionDeliveryConfigEl { }

impl ToListMappable for PubsubLiteSubscriptionDeliveryConfigEl {
    type O = BlockAssignable<PubsubLiteSubscriptionDeliveryConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubLiteSubscriptionDeliveryConfigEl {
    #[doc= "When this subscription should send messages to subscribers relative to messages persistence in storage. Possible values: [\"DELIVER_IMMEDIATELY\", \"DELIVER_AFTER_STORED\", \"DELIVERY_REQUIREMENT_UNSPECIFIED\"]"]
    pub delivery_requirement: PrimField<String>,
}

impl BuildPubsubLiteSubscriptionDeliveryConfigEl {
    pub fn build(self) -> PubsubLiteSubscriptionDeliveryConfigEl {
        PubsubLiteSubscriptionDeliveryConfigEl { delivery_requirement: self.delivery_requirement }
    }
}

pub struct PubsubLiteSubscriptionDeliveryConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for PubsubLiteSubscriptionDeliveryConfigElRef {
    fn new(shared: StackShared, base: String) -> PubsubLiteSubscriptionDeliveryConfigElRef {
        PubsubLiteSubscriptionDeliveryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubLiteSubscriptionDeliveryConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `delivery_requirement` after provisioning.\nWhen this subscription should send messages to subscribers relative to messages persistence in storage. Possible values: [\"DELIVER_IMMEDIATELY\", \"DELIVER_AFTER_STORED\", \"DELIVERY_REQUIREMENT_UNSPECIFIED\"]"]
    pub fn delivery_requirement(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delivery_requirement", self.base))
    }
}

#[derive(Serialize)]
pub struct PubsubLiteSubscriptionTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl PubsubLiteSubscriptionTimeoutsEl {
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

impl ToListMappable for PubsubLiteSubscriptionTimeoutsEl {
    type O = BlockAssignable<PubsubLiteSubscriptionTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubLiteSubscriptionTimeoutsEl {}

impl BuildPubsubLiteSubscriptionTimeoutsEl {
    pub fn build(self) -> PubsubLiteSubscriptionTimeoutsEl {
        PubsubLiteSubscriptionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct PubsubLiteSubscriptionTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for PubsubLiteSubscriptionTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> PubsubLiteSubscriptionTimeoutsElRef {
        PubsubLiteSubscriptionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubLiteSubscriptionTimeoutsElRef {
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
struct PubsubLiteSubscriptionDynamic {
    delivery_config: Option<DynamicBlock<PubsubLiteSubscriptionDeliveryConfigEl>>,
}
