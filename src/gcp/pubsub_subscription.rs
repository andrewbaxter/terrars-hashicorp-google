use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct PubsubSubscriptionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ack_deadline_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_exactly_once_delivery: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_message_ordering: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_retention_duration: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retain_acked_messages: Option<PrimField<bool>>,
    topic: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bigquery_config: Option<Vec<PubsubSubscriptionBigqueryConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloud_storage_config: Option<Vec<PubsubSubscriptionCloudStorageConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dead_letter_policy: Option<Vec<PubsubSubscriptionDeadLetterPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expiration_policy: Option<Vec<PubsubSubscriptionExpirationPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    push_config: Option<Vec<PubsubSubscriptionPushConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_policy: Option<Vec<PubsubSubscriptionRetryPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<PubsubSubscriptionTimeoutsEl>,
    dynamic: PubsubSubscriptionDynamic,
}

struct PubsubSubscription_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<PubsubSubscriptionData>,
}

#[derive(Clone)]
pub struct PubsubSubscription(Rc<PubsubSubscription_>);

impl PubsubSubscription {
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

    #[doc= "Set the field `ack_deadline_seconds`.\nThis value is the maximum time after a subscriber receives a message\nbefore the subscriber should acknowledge the message. After message\ndelivery but before the ack deadline expires and before the message is\nacknowledged, it is an outstanding message and will not be delivered\nagain during that time (on a best-effort basis).\n\nFor pull subscriptions, this value is used as the initial value for\nthe ack deadline. To override this value for a given message, call\nsubscriptions.modifyAckDeadline with the corresponding ackId if using\npull. The minimum custom deadline you can specify is 10 seconds. The\nmaximum custom deadline you can specify is 600 seconds (10 minutes).\nIf this parameter is 0, a default value of 10 seconds is used.\n\nFor push delivery, this value is also used to set the request timeout\nfor the call to the push endpoint.\n\nIf the subscriber never acknowledges the message, the Pub/Sub system\nwill eventually redeliver the message."]
    pub fn set_ack_deadline_seconds(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ack_deadline_seconds = Some(v.into());
        self
    }

    #[doc= "Set the field `enable_exactly_once_delivery`.\nIf 'true', Pub/Sub provides the following guarantees for the delivery\nof a message with a given value of messageId on this Subscriptions':\n\n- The message sent to a subscriber is guaranteed not to be resent before the message's acknowledgement deadline expires.\n\n- An acknowledged message will not be resent to a subscriber.\n\nNote that subscribers may still receive multiple copies of a message when 'enable_exactly_once_delivery'\nis true if the message was published multiple times by a publisher client. These copies are considered distinct by Pub/Sub and have distinct messageId values"]
    pub fn set_enable_exactly_once_delivery(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_exactly_once_delivery = Some(v.into());
        self
    }

    #[doc= "Set the field `enable_message_ordering`.\nIf 'true', messages published with the same orderingKey in PubsubMessage will be delivered to\nthe subscribers in the order in which they are received by the Pub/Sub system. Otherwise, they\nmay be delivered in any order."]
    pub fn set_enable_message_ordering(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_message_ordering = Some(v.into());
        self
    }

    #[doc= "Set the field `filter`.\nThe subscription only delivers the messages that match the filter.\nPub/Sub automatically acknowledges the messages that don't match the filter. You can filter messages\nby their attributes. The maximum length of a filter is 256 bytes. After creating the subscription,\nyou can't modify the filter."]
    pub fn set_filter(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().filter = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `labels`.\nA set of key/value label pairs to assign to this Subscription.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }

    #[doc= "Set the field `message_retention_duration`.\nHow long to retain unacknowledged messages in the subscription's\nbacklog, from the moment a message is published. If\nretain_acked_messages is true, then this also configures the retention\nof acknowledged messages, and thus configures how far back in time a\nsubscriptions.seek can be done. Defaults to 7 days. Cannot be more\nthan 7 days ('\"604800s\"') or less than 10 minutes ('\"600s\"').\n\nA duration in seconds with up to nine fractional digits, terminated\nby 's'. Example: '\"600.5s\"'."]
    pub fn set_message_retention_duration(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().message_retention_duration = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `retain_acked_messages`.\nIndicates whether to retain acknowledged messages. If 'true', then\nmessages are not expunged from the subscription's backlog, even if\nthey are acknowledged, until they fall out of the\nmessageRetentionDuration window."]
    pub fn set_retain_acked_messages(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().retain_acked_messages = Some(v.into());
        self
    }

    #[doc= "Set the field `bigquery_config`.\n"]
    pub fn set_bigquery_config(self, v: impl Into<BlockAssignable<PubsubSubscriptionBigqueryConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().bigquery_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.bigquery_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `cloud_storage_config`.\n"]
    pub fn set_cloud_storage_config(
        self,
        v: impl Into<BlockAssignable<PubsubSubscriptionCloudStorageConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cloud_storage_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cloud_storage_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `dead_letter_policy`.\n"]
    pub fn set_dead_letter_policy(self, v: impl Into<BlockAssignable<PubsubSubscriptionDeadLetterPolicyEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dead_letter_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dead_letter_policy = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `expiration_policy`.\n"]
    pub fn set_expiration_policy(self, v: impl Into<BlockAssignable<PubsubSubscriptionExpirationPolicyEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().expiration_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.expiration_policy = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `push_config`.\n"]
    pub fn set_push_config(self, v: impl Into<BlockAssignable<PubsubSubscriptionPushConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().push_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.push_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `retry_policy`.\n"]
    pub fn set_retry_policy(self, v: impl Into<BlockAssignable<PubsubSubscriptionRetryPolicyEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().retry_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.retry_policy = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<PubsubSubscriptionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `ack_deadline_seconds` after provisioning.\nThis value is the maximum time after a subscriber receives a message\nbefore the subscriber should acknowledge the message. After message\ndelivery but before the ack deadline expires and before the message is\nacknowledged, it is an outstanding message and will not be delivered\nagain during that time (on a best-effort basis).\n\nFor pull subscriptions, this value is used as the initial value for\nthe ack deadline. To override this value for a given message, call\nsubscriptions.modifyAckDeadline with the corresponding ackId if using\npull. The minimum custom deadline you can specify is 10 seconds. The\nmaximum custom deadline you can specify is 600 seconds (10 minutes).\nIf this parameter is 0, a default value of 10 seconds is used.\n\nFor push delivery, this value is also used to set the request timeout\nfor the call to the push endpoint.\n\nIf the subscriber never acknowledges the message, the Pub/Sub system\nwill eventually redeliver the message."]
    pub fn ack_deadline_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ack_deadline_seconds", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_exactly_once_delivery` after provisioning.\nIf 'true', Pub/Sub provides the following guarantees for the delivery\nof a message with a given value of messageId on this Subscriptions':\n\n- The message sent to a subscriber is guaranteed not to be resent before the message's acknowledgement deadline expires.\n\n- An acknowledged message will not be resent to a subscriber.\n\nNote that subscribers may still receive multiple copies of a message when 'enable_exactly_once_delivery'\nis true if the message was published multiple times by a publisher client. These copies are considered distinct by Pub/Sub and have distinct messageId values"]
    pub fn enable_exactly_once_delivery(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_exactly_once_delivery", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_message_ordering` after provisioning.\nIf 'true', messages published with the same orderingKey in PubsubMessage will be delivered to\nthe subscribers in the order in which they are received by the Pub/Sub system. Otherwise, they\nmay be delivered in any order."]
    pub fn enable_message_ordering(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_message_ordering", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `filter` after provisioning.\nThe subscription only delivers the messages that match the filter.\nPub/Sub automatically acknowledges the messages that don't match the filter. You can filter messages\nby their attributes. The maximum length of a filter is 256 bytes. After creating the subscription,\nyou can't modify the filter."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to this Subscription.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `message_retention_duration` after provisioning.\nHow long to retain unacknowledged messages in the subscription's\nbacklog, from the moment a message is published. If\nretain_acked_messages is true, then this also configures the retention\nof acknowledged messages, and thus configures how far back in time a\nsubscriptions.seek can be done. Defaults to 7 days. Cannot be more\nthan 7 days ('\"604800s\"') or less than 10 minutes ('\"600s\"').\n\nA duration in seconds with up to nine fractional digits, terminated\nby 's'. Example: '\"600.5s\"'."]
    pub fn message_retention_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message_retention_duration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the subscription."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `retain_acked_messages` after provisioning.\nIndicates whether to retain acknowledged messages. If 'true', then\nmessages are not expunged from the subscription's backlog, even if\nthey are acknowledged, until they fall out of the\nmessageRetentionDuration window."]
    pub fn retain_acked_messages(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.retain_acked_messages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `topic` after provisioning.\nA reference to a Topic resource."]
    pub fn topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bigquery_config` after provisioning.\n"]
    pub fn bigquery_config(&self) -> ListRef<PubsubSubscriptionBigqueryConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.bigquery_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cloud_storage_config` after provisioning.\n"]
    pub fn cloud_storage_config(&self) -> ListRef<PubsubSubscriptionCloudStorageConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cloud_storage_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dead_letter_policy` after provisioning.\n"]
    pub fn dead_letter_policy(&self) -> ListRef<PubsubSubscriptionDeadLetterPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dead_letter_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `expiration_policy` after provisioning.\n"]
    pub fn expiration_policy(&self) -> ListRef<PubsubSubscriptionExpirationPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.expiration_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `push_config` after provisioning.\n"]
    pub fn push_config(&self) -> ListRef<PubsubSubscriptionPushConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.push_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> ListRef<PubsubSubscriptionRetryPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PubsubSubscriptionTimeoutsElRef {
        PubsubSubscriptionTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for PubsubSubscription {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for PubsubSubscription { }

impl ToListMappable for PubsubSubscription {
    type O = ListRef<PubsubSubscriptionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for PubsubSubscription_ {
    fn extract_resource_type(&self) -> String {
        "google_pubsub_subscription".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPubsubSubscription {
    pub tf_id: String,
    #[doc= "Name of the subscription."]
    pub name: PrimField<String>,
    #[doc= "A reference to a Topic resource."]
    pub topic: PrimField<String>,
}

impl BuildPubsubSubscription {
    pub fn build(self, stack: &mut Stack) -> PubsubSubscription {
        let out = PubsubSubscription(Rc::new(PubsubSubscription_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(PubsubSubscriptionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                ack_deadline_seconds: core::default::Default::default(),
                enable_exactly_once_delivery: core::default::Default::default(),
                enable_message_ordering: core::default::Default::default(),
                filter: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                message_retention_duration: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                retain_acked_messages: core::default::Default::default(),
                topic: self.topic,
                bigquery_config: core::default::Default::default(),
                cloud_storage_config: core::default::Default::default(),
                dead_letter_policy: core::default::Default::default(),
                expiration_policy: core::default::Default::default(),
                push_config: core::default::Default::default(),
                retry_policy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PubsubSubscriptionRef {
    shared: StackShared,
    base: String,
}

impl Ref for PubsubSubscriptionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PubsubSubscriptionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `ack_deadline_seconds` after provisioning.\nThis value is the maximum time after a subscriber receives a message\nbefore the subscriber should acknowledge the message. After message\ndelivery but before the ack deadline expires and before the message is\nacknowledged, it is an outstanding message and will not be delivered\nagain during that time (on a best-effort basis).\n\nFor pull subscriptions, this value is used as the initial value for\nthe ack deadline. To override this value for a given message, call\nsubscriptions.modifyAckDeadline with the corresponding ackId if using\npull. The minimum custom deadline you can specify is 10 seconds. The\nmaximum custom deadline you can specify is 600 seconds (10 minutes).\nIf this parameter is 0, a default value of 10 seconds is used.\n\nFor push delivery, this value is also used to set the request timeout\nfor the call to the push endpoint.\n\nIf the subscriber never acknowledges the message, the Pub/Sub system\nwill eventually redeliver the message."]
    pub fn ack_deadline_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ack_deadline_seconds", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_exactly_once_delivery` after provisioning.\nIf 'true', Pub/Sub provides the following guarantees for the delivery\nof a message with a given value of messageId on this Subscriptions':\n\n- The message sent to a subscriber is guaranteed not to be resent before the message's acknowledgement deadline expires.\n\n- An acknowledged message will not be resent to a subscriber.\n\nNote that subscribers may still receive multiple copies of a message when 'enable_exactly_once_delivery'\nis true if the message was published multiple times by a publisher client. These copies are considered distinct by Pub/Sub and have distinct messageId values"]
    pub fn enable_exactly_once_delivery(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_exactly_once_delivery", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `enable_message_ordering` after provisioning.\nIf 'true', messages published with the same orderingKey in PubsubMessage will be delivered to\nthe subscribers in the order in which they are received by the Pub/Sub system. Otherwise, they\nmay be delivered in any order."]
    pub fn enable_message_ordering(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable_message_ordering", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `filter` after provisioning.\nThe subscription only delivers the messages that match the filter.\nPub/Sub automatically acknowledges the messages that don't match the filter. You can filter messages\nby their attributes. The maximum length of a filter is 256 bytes. After creating the subscription,\nyou can't modify the filter."]
    pub fn filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to this Subscription.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `message_retention_duration` after provisioning.\nHow long to retain unacknowledged messages in the subscription's\nbacklog, from the moment a message is published. If\nretain_acked_messages is true, then this also configures the retention\nof acknowledged messages, and thus configures how far back in time a\nsubscriptions.seek can be done. Defaults to 7 days. Cannot be more\nthan 7 days ('\"604800s\"') or less than 10 minutes ('\"600s\"').\n\nA duration in seconds with up to nine fractional digits, terminated\nby 's'. Example: '\"600.5s\"'."]
    pub fn message_retention_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message_retention_duration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the subscription."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `retain_acked_messages` after provisioning.\nIndicates whether to retain acknowledged messages. If 'true', then\nmessages are not expunged from the subscription's backlog, even if\nthey are acknowledged, until they fall out of the\nmessageRetentionDuration window."]
    pub fn retain_acked_messages(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.retain_acked_messages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `topic` after provisioning.\nA reference to a Topic resource."]
    pub fn topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.topic", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bigquery_config` after provisioning.\n"]
    pub fn bigquery_config(&self) -> ListRef<PubsubSubscriptionBigqueryConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.bigquery_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cloud_storage_config` after provisioning.\n"]
    pub fn cloud_storage_config(&self) -> ListRef<PubsubSubscriptionCloudStorageConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cloud_storage_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dead_letter_policy` after provisioning.\n"]
    pub fn dead_letter_policy(&self) -> ListRef<PubsubSubscriptionDeadLetterPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dead_letter_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `expiration_policy` after provisioning.\n"]
    pub fn expiration_policy(&self) -> ListRef<PubsubSubscriptionExpirationPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.expiration_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `push_config` after provisioning.\n"]
    pub fn push_config(&self) -> ListRef<PubsubSubscriptionPushConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.push_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> ListRef<PubsubSubscriptionRetryPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PubsubSubscriptionTimeoutsElRef {
        PubsubSubscriptionTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct PubsubSubscriptionBigqueryConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    drop_unknown_fields: Option<PrimField<bool>>,
    table: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_topic_schema: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    write_metadata: Option<PrimField<bool>>,
}

impl PubsubSubscriptionBigqueryConfigEl {
    #[doc= "Set the field `drop_unknown_fields`.\nWhen true and useTopicSchema is true, any fields that are a part of the topic schema that are not part of the BigQuery table schema are dropped when writing to BigQuery.\nOtherwise, the schemas must be kept in sync and any messages with extra fields are not written and remain in the subscription's backlog."]
    pub fn set_drop_unknown_fields(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.drop_unknown_fields = Some(v.into());
        self
    }

    #[doc= "Set the field `use_topic_schema`.\nWhen true, use the topic's schema as the columns to write to in BigQuery, if it exists."]
    pub fn set_use_topic_schema(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.use_topic_schema = Some(v.into());
        self
    }

    #[doc= "Set the field `write_metadata`.\nWhen true, write the subscription name, messageId, publishTime, attributes, and orderingKey to additional columns in the table.\nThe subscription name, messageId, and publishTime fields are put in their own columns while all other message properties (other than data) are written to a JSON object in the attributes column."]
    pub fn set_write_metadata(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.write_metadata = Some(v.into());
        self
    }
}

impl ToListMappable for PubsubSubscriptionBigqueryConfigEl {
    type O = BlockAssignable<PubsubSubscriptionBigqueryConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubSubscriptionBigqueryConfigEl {
    #[doc= "The name of the table to which to write data, of the form {projectId}:{datasetId}.{tableId}"]
    pub table: PrimField<String>,
}

impl BuildPubsubSubscriptionBigqueryConfigEl {
    pub fn build(self) -> PubsubSubscriptionBigqueryConfigEl {
        PubsubSubscriptionBigqueryConfigEl {
            drop_unknown_fields: core::default::Default::default(),
            table: self.table,
            use_topic_schema: core::default::Default::default(),
            write_metadata: core::default::Default::default(),
        }
    }
}

pub struct PubsubSubscriptionBigqueryConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for PubsubSubscriptionBigqueryConfigElRef {
    fn new(shared: StackShared, base: String) -> PubsubSubscriptionBigqueryConfigElRef {
        PubsubSubscriptionBigqueryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubSubscriptionBigqueryConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `drop_unknown_fields` after provisioning.\nWhen true and useTopicSchema is true, any fields that are a part of the topic schema that are not part of the BigQuery table schema are dropped when writing to BigQuery.\nOtherwise, the schemas must be kept in sync and any messages with extra fields are not written and remain in the subscription's backlog."]
    pub fn drop_unknown_fields(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.drop_unknown_fields", self.base))
    }

    #[doc= "Get a reference to the value of field `table` after provisioning.\nThe name of the table to which to write data, of the form {projectId}:{datasetId}.{tableId}"]
    pub fn table(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }

    #[doc= "Get a reference to the value of field `use_topic_schema` after provisioning.\nWhen true, use the topic's schema as the columns to write to in BigQuery, if it exists."]
    pub fn use_topic_schema(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_topic_schema", self.base))
    }

    #[doc= "Get a reference to the value of field `write_metadata` after provisioning.\nWhen true, write the subscription name, messageId, publishTime, attributes, and orderingKey to additional columns in the table.\nThe subscription name, messageId, and publishTime fields are put in their own columns while all other message properties (other than data) are written to a JSON object in the attributes column."]
    pub fn write_metadata(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.write_metadata", self.base))
    }
}

#[derive(Serialize)]
pub struct PubsubSubscriptionCloudStorageConfigElAvroConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    write_metadata: Option<PrimField<bool>>,
}

impl PubsubSubscriptionCloudStorageConfigElAvroConfigEl {
    #[doc= "Set the field `write_metadata`.\nWhen true, write the subscription name, messageId, publishTime, attributes, and orderingKey as additional fields in the output."]
    pub fn set_write_metadata(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.write_metadata = Some(v.into());
        self
    }
}

impl ToListMappable for PubsubSubscriptionCloudStorageConfigElAvroConfigEl {
    type O = BlockAssignable<PubsubSubscriptionCloudStorageConfigElAvroConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubSubscriptionCloudStorageConfigElAvroConfigEl {}

impl BuildPubsubSubscriptionCloudStorageConfigElAvroConfigEl {
    pub fn build(self) -> PubsubSubscriptionCloudStorageConfigElAvroConfigEl {
        PubsubSubscriptionCloudStorageConfigElAvroConfigEl { write_metadata: core::default::Default::default() }
    }
}

pub struct PubsubSubscriptionCloudStorageConfigElAvroConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for PubsubSubscriptionCloudStorageConfigElAvroConfigElRef {
    fn new(shared: StackShared, base: String) -> PubsubSubscriptionCloudStorageConfigElAvroConfigElRef {
        PubsubSubscriptionCloudStorageConfigElAvroConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubSubscriptionCloudStorageConfigElAvroConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `write_metadata` after provisioning.\nWhen true, write the subscription name, messageId, publishTime, attributes, and orderingKey as additional fields in the output."]
    pub fn write_metadata(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.write_metadata", self.base))
    }
}

#[derive(Serialize, Default)]
struct PubsubSubscriptionCloudStorageConfigElDynamic {
    avro_config: Option<DynamicBlock<PubsubSubscriptionCloudStorageConfigElAvroConfigEl>>,
}

#[derive(Serialize)]
pub struct PubsubSubscriptionCloudStorageConfigEl {
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filename_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filename_suffix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_bytes: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_duration: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avro_config: Option<Vec<PubsubSubscriptionCloudStorageConfigElAvroConfigEl>>,
    dynamic: PubsubSubscriptionCloudStorageConfigElDynamic,
}

impl PubsubSubscriptionCloudStorageConfigEl {
    #[doc= "Set the field `filename_prefix`.\nUser-provided prefix for Cloud Storage filename."]
    pub fn set_filename_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filename_prefix = Some(v.into());
        self
    }

    #[doc= "Set the field `filename_suffix`.\nUser-provided suffix for Cloud Storage filename. Must not end in \"/\"."]
    pub fn set_filename_suffix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filename_suffix = Some(v.into());
        self
    }

    #[doc= "Set the field `max_bytes`.\nThe maximum bytes that can be written to a Cloud Storage file before a new file is created. Min 1 KB, max 10 GiB.\nThe maxBytes limit may be exceeded in cases where messages are larger than the limit."]
    pub fn set_max_bytes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_bytes = Some(v.into());
        self
    }

    #[doc= "Set the field `max_duration`.\nThe maximum duration that can elapse before a new Cloud Storage file is created. Min 1 minute, max 10 minutes, default 5 minutes.\nMay not exceed the subscription's acknowledgement deadline.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn set_max_duration(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_duration = Some(v.into());
        self
    }

    #[doc= "Set the field `avro_config`.\n"]
    pub fn set_avro_config(
        mut self,
        v: impl Into<BlockAssignable<PubsubSubscriptionCloudStorageConfigElAvroConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.avro_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.avro_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for PubsubSubscriptionCloudStorageConfigEl {
    type O = BlockAssignable<PubsubSubscriptionCloudStorageConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubSubscriptionCloudStorageConfigEl {
    #[doc= "User-provided name for the Cloud Storage bucket. The bucket must be created by the user. The bucket name must be without any prefix like \"gs://\"."]
    pub bucket: PrimField<String>,
}

impl BuildPubsubSubscriptionCloudStorageConfigEl {
    pub fn build(self) -> PubsubSubscriptionCloudStorageConfigEl {
        PubsubSubscriptionCloudStorageConfigEl {
            bucket: self.bucket,
            filename_prefix: core::default::Default::default(),
            filename_suffix: core::default::Default::default(),
            max_bytes: core::default::Default::default(),
            max_duration: core::default::Default::default(),
            avro_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct PubsubSubscriptionCloudStorageConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for PubsubSubscriptionCloudStorageConfigElRef {
    fn new(shared: StackShared, base: String) -> PubsubSubscriptionCloudStorageConfigElRef {
        PubsubSubscriptionCloudStorageConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubSubscriptionCloudStorageConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `bucket` after provisioning.\nUser-provided name for the Cloud Storage bucket. The bucket must be created by the user. The bucket name must be without any prefix like \"gs://\"."]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc= "Get a reference to the value of field `filename_prefix` after provisioning.\nUser-provided prefix for Cloud Storage filename."]
    pub fn filename_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filename_prefix", self.base))
    }

    #[doc= "Get a reference to the value of field `filename_suffix` after provisioning.\nUser-provided suffix for Cloud Storage filename. Must not end in \"/\"."]
    pub fn filename_suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filename_suffix", self.base))
    }

    #[doc= "Get a reference to the value of field `max_bytes` after provisioning.\nThe maximum bytes that can be written to a Cloud Storage file before a new file is created. Min 1 KB, max 10 GiB.\nThe maxBytes limit may be exceeded in cases where messages are larger than the limit."]
    pub fn max_bytes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_bytes", self.base))
    }

    #[doc= "Get a reference to the value of field `max_duration` after provisioning.\nThe maximum duration that can elapse before a new Cloud Storage file is created. Min 1 minute, max 10 minutes, default 5 minutes.\nMay not exceed the subscription's acknowledgement deadline.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn max_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_duration", self.base))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\nAn output-only field that indicates whether or not the subscription can receive messages."]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc= "Get a reference to the value of field `avro_config` after provisioning.\n"]
    pub fn avro_config(&self) -> ListRef<PubsubSubscriptionCloudStorageConfigElAvroConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.avro_config", self.base))
    }
}

#[derive(Serialize)]
pub struct PubsubSubscriptionDeadLetterPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dead_letter_topic: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_delivery_attempts: Option<PrimField<f64>>,
}

impl PubsubSubscriptionDeadLetterPolicyEl {
    #[doc= "Set the field `dead_letter_topic`.\nThe name of the topic to which dead letter messages should be published.\nFormat is 'projects/{project}/topics/{topic}'.\n\nThe Cloud Pub/Sub service account associated with the enclosing subscription's\nparent project (i.e.,\nservice-{project_number}@gcp-sa-pubsub.iam.gserviceaccount.com) must have\npermission to Publish() to this topic.\n\nThe operation will fail if the topic does not exist.\nUsers should ensure that there is a subscription attached to this topic\nsince messages published to a topic with no subscriptions are lost."]
    pub fn set_dead_letter_topic(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.dead_letter_topic = Some(v.into());
        self
    }

    #[doc= "Set the field `max_delivery_attempts`.\nThe maximum number of delivery attempts for any message. The value must be\nbetween 5 and 100.\n\nThe number of delivery attempts is defined as 1 + (the sum of number of\nNACKs and number of times the acknowledgement deadline has been exceeded for the message).\n\nA NACK is any call to ModifyAckDeadline with a 0 deadline. Note that\nclient libraries may automatically extend ack_deadlines.\n\nThis field will be honored on a best effort basis.\n\nIf this parameter is 0, a default value of 5 is used."]
    pub fn set_max_delivery_attempts(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_delivery_attempts = Some(v.into());
        self
    }
}

impl ToListMappable for PubsubSubscriptionDeadLetterPolicyEl {
    type O = BlockAssignable<PubsubSubscriptionDeadLetterPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubSubscriptionDeadLetterPolicyEl {}

impl BuildPubsubSubscriptionDeadLetterPolicyEl {
    pub fn build(self) -> PubsubSubscriptionDeadLetterPolicyEl {
        PubsubSubscriptionDeadLetterPolicyEl {
            dead_letter_topic: core::default::Default::default(),
            max_delivery_attempts: core::default::Default::default(),
        }
    }
}

pub struct PubsubSubscriptionDeadLetterPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for PubsubSubscriptionDeadLetterPolicyElRef {
    fn new(shared: StackShared, base: String) -> PubsubSubscriptionDeadLetterPolicyElRef {
        PubsubSubscriptionDeadLetterPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubSubscriptionDeadLetterPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `dead_letter_topic` after provisioning.\nThe name of the topic to which dead letter messages should be published.\nFormat is 'projects/{project}/topics/{topic}'.\n\nThe Cloud Pub/Sub service account associated with the enclosing subscription's\nparent project (i.e.,\nservice-{project_number}@gcp-sa-pubsub.iam.gserviceaccount.com) must have\npermission to Publish() to this topic.\n\nThe operation will fail if the topic does not exist.\nUsers should ensure that there is a subscription attached to this topic\nsince messages published to a topic with no subscriptions are lost."]
    pub fn dead_letter_topic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dead_letter_topic", self.base))
    }

    #[doc= "Get a reference to the value of field `max_delivery_attempts` after provisioning.\nThe maximum number of delivery attempts for any message. The value must be\nbetween 5 and 100.\n\nThe number of delivery attempts is defined as 1 + (the sum of number of\nNACKs and number of times the acknowledgement deadline has been exceeded for the message).\n\nA NACK is any call to ModifyAckDeadline with a 0 deadline. Note that\nclient libraries may automatically extend ack_deadlines.\n\nThis field will be honored on a best effort basis.\n\nIf this parameter is 0, a default value of 5 is used."]
    pub fn max_delivery_attempts(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_delivery_attempts", self.base))
    }
}

#[derive(Serialize)]
pub struct PubsubSubscriptionExpirationPolicyEl {
    ttl: PrimField<String>,
}

impl PubsubSubscriptionExpirationPolicyEl { }

impl ToListMappable for PubsubSubscriptionExpirationPolicyEl {
    type O = BlockAssignable<PubsubSubscriptionExpirationPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubSubscriptionExpirationPolicyEl {
    #[doc= "Specifies the \"time-to-live\" duration for an associated resource. The\nresource expires if it is not active for a period of ttl.\nIf ttl is set to \"\", the associated resource never expires.\nA duration in seconds with up to nine fractional digits, terminated by 's'.\nExample - \"3.5s\"."]
    pub ttl: PrimField<String>,
}

impl BuildPubsubSubscriptionExpirationPolicyEl {
    pub fn build(self) -> PubsubSubscriptionExpirationPolicyEl {
        PubsubSubscriptionExpirationPolicyEl { ttl: self.ttl }
    }
}

pub struct PubsubSubscriptionExpirationPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for PubsubSubscriptionExpirationPolicyElRef {
    fn new(shared: StackShared, base: String) -> PubsubSubscriptionExpirationPolicyElRef {
        PubsubSubscriptionExpirationPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubSubscriptionExpirationPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `ttl` after provisioning.\nSpecifies the \"time-to-live\" duration for an associated resource. The\nresource expires if it is not active for a period of ttl.\nIf ttl is set to \"\", the associated resource never expires.\nA duration in seconds with up to nine fractional digits, terminated by 's'.\nExample - \"3.5s\"."]
    pub fn ttl(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.base))
    }
}

#[derive(Serialize)]
pub struct PubsubSubscriptionPushConfigElNoWrapperEl {
    write_metadata: PrimField<bool>,
}

impl PubsubSubscriptionPushConfigElNoWrapperEl { }

impl ToListMappable for PubsubSubscriptionPushConfigElNoWrapperEl {
    type O = BlockAssignable<PubsubSubscriptionPushConfigElNoWrapperEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubSubscriptionPushConfigElNoWrapperEl {
    #[doc= "When true, writes the Pub/Sub message metadata to\n'x-goog-pubsub-<KEY>:<VAL>' headers of the HTTP request. Writes the\nPub/Sub message attributes to '<KEY>:<VAL>' headers of the HTTP request."]
    pub write_metadata: PrimField<bool>,
}

impl BuildPubsubSubscriptionPushConfigElNoWrapperEl {
    pub fn build(self) -> PubsubSubscriptionPushConfigElNoWrapperEl {
        PubsubSubscriptionPushConfigElNoWrapperEl { write_metadata: self.write_metadata }
    }
}

pub struct PubsubSubscriptionPushConfigElNoWrapperElRef {
    shared: StackShared,
    base: String,
}

impl Ref for PubsubSubscriptionPushConfigElNoWrapperElRef {
    fn new(shared: StackShared, base: String) -> PubsubSubscriptionPushConfigElNoWrapperElRef {
        PubsubSubscriptionPushConfigElNoWrapperElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubSubscriptionPushConfigElNoWrapperElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `write_metadata` after provisioning.\nWhen true, writes the Pub/Sub message metadata to\n'x-goog-pubsub-<KEY>:<VAL>' headers of the HTTP request. Writes the\nPub/Sub message attributes to '<KEY>:<VAL>' headers of the HTTP request."]
    pub fn write_metadata(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.write_metadata", self.base))
    }
}

#[derive(Serialize)]
pub struct PubsubSubscriptionPushConfigElOidcTokenEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    audience: Option<PrimField<String>>,
    service_account_email: PrimField<String>,
}

impl PubsubSubscriptionPushConfigElOidcTokenEl {
    #[doc= "Set the field `audience`.\nAudience to be used when generating OIDC token. The audience claim\nidentifies the recipients that the JWT is intended for. The audience\nvalue is a single case-sensitive string. Having multiple values (array)\nfor the audience field is not supported. More info about the OIDC JWT\ntoken audience here: https://tools.ietf.org/html/rfc7519#section-4.1.3\nNote: if not specified, the Push endpoint URL will be used."]
    pub fn set_audience(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.audience = Some(v.into());
        self
    }
}

impl ToListMappable for PubsubSubscriptionPushConfigElOidcTokenEl {
    type O = BlockAssignable<PubsubSubscriptionPushConfigElOidcTokenEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubSubscriptionPushConfigElOidcTokenEl {
    #[doc= "Service account email to be used for generating the OIDC token.\nThe caller (for subscriptions.create, subscriptions.patch, and\nsubscriptions.modifyPushConfig RPCs) must have the\niam.serviceAccounts.actAs permission for the service account."]
    pub service_account_email: PrimField<String>,
}

impl BuildPubsubSubscriptionPushConfigElOidcTokenEl {
    pub fn build(self) -> PubsubSubscriptionPushConfigElOidcTokenEl {
        PubsubSubscriptionPushConfigElOidcTokenEl {
            audience: core::default::Default::default(),
            service_account_email: self.service_account_email,
        }
    }
}

pub struct PubsubSubscriptionPushConfigElOidcTokenElRef {
    shared: StackShared,
    base: String,
}

impl Ref for PubsubSubscriptionPushConfigElOidcTokenElRef {
    fn new(shared: StackShared, base: String) -> PubsubSubscriptionPushConfigElOidcTokenElRef {
        PubsubSubscriptionPushConfigElOidcTokenElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubSubscriptionPushConfigElOidcTokenElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `audience` after provisioning.\nAudience to be used when generating OIDC token. The audience claim\nidentifies the recipients that the JWT is intended for. The audience\nvalue is a single case-sensitive string. Having multiple values (array)\nfor the audience field is not supported. More info about the OIDC JWT\ntoken audience here: https://tools.ietf.org/html/rfc7519#section-4.1.3\nNote: if not specified, the Push endpoint URL will be used."]
    pub fn audience(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.audience", self.base))
    }

    #[doc= "Get a reference to the value of field `service_account_email` after provisioning.\nService account email to be used for generating the OIDC token.\nThe caller (for subscriptions.create, subscriptions.patch, and\nsubscriptions.modifyPushConfig RPCs) must have the\niam.serviceAccounts.actAs permission for the service account."]
    pub fn service_account_email(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_account_email", self.base))
    }
}

#[derive(Serialize, Default)]
struct PubsubSubscriptionPushConfigElDynamic {
    no_wrapper: Option<DynamicBlock<PubsubSubscriptionPushConfigElNoWrapperEl>>,
    oidc_token: Option<DynamicBlock<PubsubSubscriptionPushConfigElOidcTokenEl>>,
}

#[derive(Serialize)]
pub struct PubsubSubscriptionPushConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<RecField<PrimField<String>>>,
    push_endpoint: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_wrapper: Option<Vec<PubsubSubscriptionPushConfigElNoWrapperEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oidc_token: Option<Vec<PubsubSubscriptionPushConfigElOidcTokenEl>>,
    dynamic: PubsubSubscriptionPushConfigElDynamic,
}

impl PubsubSubscriptionPushConfigEl {
    #[doc= "Set the field `attributes`.\nEndpoint configuration attributes.\n\nEvery endpoint has a set of API supported attributes that can\nbe used to control different aspects of the message delivery.\n\nThe currently supported attribute is x-goog-version, which you\ncan use to change the format of the pushed message. This\nattribute indicates the version of the data expected by\nthe endpoint. This controls the shape of the pushed message\n(i.e., its fields and metadata). The endpoint version is\nbased on the version of the Pub/Sub API.\n\nIf not present during the subscriptions.create call,\nit will default to the version of the API used to make\nsuch call. If not present during a subscriptions.modifyPushConfig\ncall, its value will not be changed. subscriptions.get\ncalls will always return a valid version, even if the\nsubscription was created without this attribute.\n\nThe possible values for this attribute are:\n\n- v1beta1: uses the push format defined in the v1beta1 Pub/Sub API.\n- v1 or v1beta2: uses the push format defined in the v1 Pub/Sub API."]
    pub fn set_attributes(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.attributes = Some(v.into());
        self
    }

    #[doc= "Set the field `no_wrapper`.\n"]
    pub fn set_no_wrapper(mut self, v: impl Into<BlockAssignable<PubsubSubscriptionPushConfigElNoWrapperEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.no_wrapper = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.no_wrapper = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `oidc_token`.\n"]
    pub fn set_oidc_token(mut self, v: impl Into<BlockAssignable<PubsubSubscriptionPushConfigElOidcTokenEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.oidc_token = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.oidc_token = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for PubsubSubscriptionPushConfigEl {
    type O = BlockAssignable<PubsubSubscriptionPushConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubSubscriptionPushConfigEl {
    #[doc= "A URL locating the endpoint to which messages should be pushed.\nFor example, a Webhook endpoint might use\n\"https://example.com/push\"."]
    pub push_endpoint: PrimField<String>,
}

impl BuildPubsubSubscriptionPushConfigEl {
    pub fn build(self) -> PubsubSubscriptionPushConfigEl {
        PubsubSubscriptionPushConfigEl {
            attributes: core::default::Default::default(),
            push_endpoint: self.push_endpoint,
            no_wrapper: core::default::Default::default(),
            oidc_token: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct PubsubSubscriptionPushConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for PubsubSubscriptionPushConfigElRef {
    fn new(shared: StackShared, base: String) -> PubsubSubscriptionPushConfigElRef {
        PubsubSubscriptionPushConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubSubscriptionPushConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `attributes` after provisioning.\nEndpoint configuration attributes.\n\nEvery endpoint has a set of API supported attributes that can\nbe used to control different aspects of the message delivery.\n\nThe currently supported attribute is x-goog-version, which you\ncan use to change the format of the pushed message. This\nattribute indicates the version of the data expected by\nthe endpoint. This controls the shape of the pushed message\n(i.e., its fields and metadata). The endpoint version is\nbased on the version of the Pub/Sub API.\n\nIf not present during the subscriptions.create call,\nit will default to the version of the API used to make\nsuch call. If not present during a subscriptions.modifyPushConfig\ncall, its value will not be changed. subscriptions.get\ncalls will always return a valid version, even if the\nsubscription was created without this attribute.\n\nThe possible values for this attribute are:\n\n- v1beta1: uses the push format defined in the v1beta1 Pub/Sub API.\n- v1 or v1beta2: uses the push format defined in the v1 Pub/Sub API."]
    pub fn attributes(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.attributes", self.base))
    }

    #[doc= "Get a reference to the value of field `push_endpoint` after provisioning.\nA URL locating the endpoint to which messages should be pushed.\nFor example, a Webhook endpoint might use\n\"https://example.com/push\"."]
    pub fn push_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.push_endpoint", self.base))
    }

    #[doc= "Get a reference to the value of field `no_wrapper` after provisioning.\n"]
    pub fn no_wrapper(&self) -> ListRef<PubsubSubscriptionPushConfigElNoWrapperElRef> {
        ListRef::new(self.shared().clone(), format!("{}.no_wrapper", self.base))
    }

    #[doc= "Get a reference to the value of field `oidc_token` after provisioning.\n"]
    pub fn oidc_token(&self) -> ListRef<PubsubSubscriptionPushConfigElOidcTokenElRef> {
        ListRef::new(self.shared().clone(), format!("{}.oidc_token", self.base))
    }
}

#[derive(Serialize)]
pub struct PubsubSubscriptionRetryPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_backoff: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_backoff: Option<PrimField<String>>,
}

impl PubsubSubscriptionRetryPolicyEl {
    #[doc= "Set the field `maximum_backoff`.\nThe maximum delay between consecutive deliveries of a given message. Value should be between 0 and 600 seconds. Defaults to 600 seconds.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn set_maximum_backoff(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.maximum_backoff = Some(v.into());
        self
    }

    #[doc= "Set the field `minimum_backoff`.\nThe minimum delay between consecutive deliveries of a given message. Value should be between 0 and 600 seconds. Defaults to 10 seconds.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn set_minimum_backoff(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.minimum_backoff = Some(v.into());
        self
    }
}

impl ToListMappable for PubsubSubscriptionRetryPolicyEl {
    type O = BlockAssignable<PubsubSubscriptionRetryPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubSubscriptionRetryPolicyEl {}

impl BuildPubsubSubscriptionRetryPolicyEl {
    pub fn build(self) -> PubsubSubscriptionRetryPolicyEl {
        PubsubSubscriptionRetryPolicyEl {
            maximum_backoff: core::default::Default::default(),
            minimum_backoff: core::default::Default::default(),
        }
    }
}

pub struct PubsubSubscriptionRetryPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for PubsubSubscriptionRetryPolicyElRef {
    fn new(shared: StackShared, base: String) -> PubsubSubscriptionRetryPolicyElRef {
        PubsubSubscriptionRetryPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubSubscriptionRetryPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `maximum_backoff` after provisioning.\nThe maximum delay between consecutive deliveries of a given message. Value should be between 0 and 600 seconds. Defaults to 600 seconds.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn maximum_backoff(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum_backoff", self.base))
    }

    #[doc= "Get a reference to the value of field `minimum_backoff` after provisioning.\nThe minimum delay between consecutive deliveries of a given message. Value should be between 0 and 600 seconds. Defaults to 10 seconds.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn minimum_backoff(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.minimum_backoff", self.base))
    }
}

#[derive(Serialize)]
pub struct PubsubSubscriptionTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl PubsubSubscriptionTimeoutsEl {
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

impl ToListMappable for PubsubSubscriptionTimeoutsEl {
    type O = BlockAssignable<PubsubSubscriptionTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubSubscriptionTimeoutsEl {}

impl BuildPubsubSubscriptionTimeoutsEl {
    pub fn build(self) -> PubsubSubscriptionTimeoutsEl {
        PubsubSubscriptionTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct PubsubSubscriptionTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for PubsubSubscriptionTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> PubsubSubscriptionTimeoutsElRef {
        PubsubSubscriptionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubSubscriptionTimeoutsElRef {
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
struct PubsubSubscriptionDynamic {
    bigquery_config: Option<DynamicBlock<PubsubSubscriptionBigqueryConfigEl>>,
    cloud_storage_config: Option<DynamicBlock<PubsubSubscriptionCloudStorageConfigEl>>,
    dead_letter_policy: Option<DynamicBlock<PubsubSubscriptionDeadLetterPolicyEl>>,
    expiration_policy: Option<DynamicBlock<PubsubSubscriptionExpirationPolicyEl>>,
    push_config: Option<DynamicBlock<PubsubSubscriptionPushConfigEl>>,
    retry_policy: Option<DynamicBlock<PubsubSubscriptionRetryPolicyEl>>,
}
