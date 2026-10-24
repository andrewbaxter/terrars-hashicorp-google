use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct PubsubSubscriptionData {
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
    ack_deadline_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_exactly_once_delivery: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_message_ordering: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_retention_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retain_acked_messages: std::option::Option<terrars::PrimField<bool>>,
    topic: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bigquery_config: std::option::Option<std::vec::Vec<PubsubSubscriptionBigqueryConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_storage_config: std::option::Option<std::vec::Vec<PubsubSubscriptionCloudStorageConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dead_letter_policy: std::option::Option<std::vec::Vec<PubsubSubscriptionDeadLetterPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expiration_policy: std::option::Option<std::vec::Vec<PubsubSubscriptionExpirationPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    push_config: std::option::Option<std::vec::Vec<PubsubSubscriptionPushConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_policy: std::option::Option<std::vec::Vec<PubsubSubscriptionRetryPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<PubsubSubscriptionTimeoutsEl>,
    dynamic: PubsubSubscriptionDynamic,
}

struct PubsubSubscription_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<PubsubSubscriptionData>,
}

#[derive(Clone)]
pub struct PubsubSubscription(std::rc::Rc<PubsubSubscription_>);

impl PubsubSubscription {
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

    #[doc =
        "Set the field `ack_deadline_seconds`.\nThis value is the maximum time after a subscriber receives a message\nbefore the subscriber should acknowledge the message. After message\ndelivery but before the ack deadline expires and before the message is\nacknowledged, it is an outstanding message and will not be delivered\nagain during that time (on a best-effort basis).\n\nFor pull subscriptions, this value is used as the initial value for\nthe ack deadline. To override this value for a given message, call\nsubscriptions.modifyAckDeadline with the corresponding ackId if using\npull. The minimum custom deadline you can specify is 10 seconds. The\nmaximum custom deadline you can specify is 600 seconds (10 minutes).\nIf this parameter is 0, a default value of 10 seconds is used.\n\nFor push delivery, this value is also used to set the request timeout\nfor the call to the push endpoint.\n\nIf the subscriber never acknowledges the message, the Pub/Sub system\nwill eventually redeliver the message."]
    pub fn set_ack_deadline_seconds(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ack_deadline_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_exactly_once_delivery`.\nIf 'true', Pub/Sub provides the following guarantees for the delivery\nof a message with a given value of messageId on this Subscriptions':\n\n- The message sent to a subscriber is guaranteed not to be resent before the message's acknowledgement deadline expires.\n\n- An acknowledged message will not be resent to a subscriber.\n\nNote that subscribers may still receive multiple copies of a message when 'enable_exactly_once_delivery'\nis true if the message was published multiple times by a publisher client. These copies are considered distinct by Pub/Sub and have distinct messageId values"]
    pub fn set_enable_exactly_once_delivery(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_exactly_once_delivery = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_message_ordering`.\nIf 'true', messages published with the same orderingKey in PubsubMessage will be delivered to\nthe subscribers in the order in which they are received by the Pub/Sub system. Otherwise, they\nmay be delivered in any order."]
    pub fn set_enable_message_ordering(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_message_ordering = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `filter`.\nThe subscription only delivers the messages that match the filter.\nPub/Sub automatically acknowledges the messages that don't match the filter. You can filter messages\nby their attributes. The maximum length of a filter is 256 bytes. After creating the subscription,\nyou can't modify the filter."]
    pub fn set_filter(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nA set of key/value label pairs to assign to this Subscription.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `message_retention_duration`.\nHow long to retain unacknowledged messages in the subscription's\nbacklog, from the moment a message is published. If\nretain_acked_messages is true, then this also configures the retention\nof acknowledged messages, and thus configures how far back in time a\nsubscriptions.seek can be done. Defaults to 7 days. Cannot be more\nthan 7 days ('\"604800s\"') or less than 10 minutes ('\"600s\"').\n\nA duration in seconds with up to nine fractional digits, terminated\nby 's'. Example: '\"600.5s\"'."]
    pub fn set_message_retention_duration(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().message_retention_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `retain_acked_messages`.\nIndicates whether to retain acknowledged messages. If 'true', then\nmessages are not expunged from the subscription's backlog, even if\nthey are acknowledged, until they fall out of the\nmessageRetentionDuration window."]
    pub fn set_retain_acked_messages(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().retain_acked_messages = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bigquery_config`.\n"]
    pub fn set_bigquery_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PubsubSubscriptionBigqueryConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().bigquery_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.bigquery_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cloud_storage_config`.\n"]
    pub fn set_cloud_storage_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PubsubSubscriptionCloudStorageConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cloud_storage_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cloud_storage_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dead_letter_policy`.\n"]
    pub fn set_dead_letter_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PubsubSubscriptionDeadLetterPolicyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dead_letter_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dead_letter_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `expiration_policy`.\n"]
    pub fn set_expiration_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PubsubSubscriptionExpirationPolicyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().expiration_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.expiration_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `push_config`.\n"]
    pub fn set_push_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PubsubSubscriptionPushConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().push_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.push_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `retry_policy`.\n"]
    pub fn set_retry_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PubsubSubscriptionRetryPolicyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().retry_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.retry_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<PubsubSubscriptionTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `ack_deadline_seconds` after provisioning.\nThis value is the maximum time after a subscriber receives a message\nbefore the subscriber should acknowledge the message. After message\ndelivery but before the ack deadline expires and before the message is\nacknowledged, it is an outstanding message and will not be delivered\nagain during that time (on a best-effort basis).\n\nFor pull subscriptions, this value is used as the initial value for\nthe ack deadline. To override this value for a given message, call\nsubscriptions.modifyAckDeadline with the corresponding ackId if using\npull. The minimum custom deadline you can specify is 10 seconds. The\nmaximum custom deadline you can specify is 600 seconds (10 minutes).\nIf this parameter is 0, a default value of 10 seconds is used.\n\nFor push delivery, this value is also used to set the request timeout\nfor the call to the push endpoint.\n\nIf the subscriber never acknowledges the message, the Pub/Sub system\nwill eventually redeliver the message."]
    pub fn ack_deadline_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ack_deadline_seconds", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_exactly_once_delivery` after provisioning.\nIf 'true', Pub/Sub provides the following guarantees for the delivery\nof a message with a given value of messageId on this Subscriptions':\n\n- The message sent to a subscriber is guaranteed not to be resent before the message's acknowledgement deadline expires.\n\n- An acknowledged message will not be resent to a subscriber.\n\nNote that subscribers may still receive multiple copies of a message when 'enable_exactly_once_delivery'\nis true if the message was published multiple times by a publisher client. These copies are considered distinct by Pub/Sub and have distinct messageId values"]
    pub fn enable_exactly_once_delivery(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_exactly_once_delivery", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_message_ordering` after provisioning.\nIf 'true', messages published with the same orderingKey in PubsubMessage will be delivered to\nthe subscribers in the order in which they are received by the Pub/Sub system. Otherwise, they\nmay be delivered in any order."]
    pub fn enable_message_ordering(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_message_ordering", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nThe subscription only delivers the messages that match the filter.\nPub/Sub automatically acknowledges the messages that don't match the filter. You can filter messages\nby their attributes. The maximum length of a filter is 256 bytes. After creating the subscription,\nyou can't modify the filter."]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to this Subscription.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `message_retention_duration` after provisioning.\nHow long to retain unacknowledged messages in the subscription's\nbacklog, from the moment a message is published. If\nretain_acked_messages is true, then this also configures the retention\nof acknowledged messages, and thus configures how far back in time a\nsubscriptions.seek can be done. Defaults to 7 days. Cannot be more\nthan 7 days ('\"604800s\"') or less than 10 minutes ('\"600s\"').\n\nA duration in seconds with up to nine fractional digits, terminated\nby 's'. Example: '\"600.5s\"'."]
    pub fn message_retention_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_retention_duration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the subscription."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `retain_acked_messages` after provisioning.\nIndicates whether to retain acknowledged messages. If 'true', then\nmessages are not expunged from the subscription's backlog, even if\nthey are acknowledged, until they fall out of the\nmessageRetentionDuration window."]
    pub fn retain_acked_messages(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retain_acked_messages", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `topic` after provisioning.\nA reference to a Topic resource."]
    pub fn topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bigquery_config` after provisioning.\n"]
    pub fn bigquery_config(&self) -> terrars::ListRef<PubsubSubscriptionBigqueryConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigquery_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloud_storage_config` after provisioning.\n"]
    pub fn cloud_storage_config(&self) -> terrars::ListRef<PubsubSubscriptionCloudStorageConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_storage_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dead_letter_policy` after provisioning.\n"]
    pub fn dead_letter_policy(&self) -> terrars::ListRef<PubsubSubscriptionDeadLetterPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dead_letter_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `expiration_policy` after provisioning.\n"]
    pub fn expiration_policy(&self) -> terrars::ListRef<PubsubSubscriptionExpirationPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expiration_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `push_config` after provisioning.\n"]
    pub fn push_config(&self) -> terrars::ListRef<PubsubSubscriptionPushConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.push_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> terrars::ListRef<PubsubSubscriptionRetryPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PubsubSubscriptionTimeoutsElRef {
        PubsubSubscriptionTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for PubsubSubscription {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for PubsubSubscription { }

impl terrars::ToListMappable for PubsubSubscription {
    type O = terrars::ListRef<PubsubSubscriptionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for PubsubSubscription_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_pubsub_subscription".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPubsubSubscription {
    pub tf_id: std::string::String,
    #[doc = "Name of the subscription."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "A reference to a Topic resource."]
    pub topic: terrars::PrimField<std::string::String>,
}

impl BuildPubsubSubscription {
    pub fn build(self, stack: &mut terrars::Stack) -> PubsubSubscription {
        let out = PubsubSubscription(std::rc::Rc::new(PubsubSubscription_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(PubsubSubscriptionData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
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
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PubsubSubscriptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubSubscriptionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PubsubSubscriptionRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ack_deadline_seconds` after provisioning.\nThis value is the maximum time after a subscriber receives a message\nbefore the subscriber should acknowledge the message. After message\ndelivery but before the ack deadline expires and before the message is\nacknowledged, it is an outstanding message and will not be delivered\nagain during that time (on a best-effort basis).\n\nFor pull subscriptions, this value is used as the initial value for\nthe ack deadline. To override this value for a given message, call\nsubscriptions.modifyAckDeadline with the corresponding ackId if using\npull. The minimum custom deadline you can specify is 10 seconds. The\nmaximum custom deadline you can specify is 600 seconds (10 minutes).\nIf this parameter is 0, a default value of 10 seconds is used.\n\nFor push delivery, this value is also used to set the request timeout\nfor the call to the push endpoint.\n\nIf the subscriber never acknowledges the message, the Pub/Sub system\nwill eventually redeliver the message."]
    pub fn ack_deadline_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ack_deadline_seconds", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_exactly_once_delivery` after provisioning.\nIf 'true', Pub/Sub provides the following guarantees for the delivery\nof a message with a given value of messageId on this Subscriptions':\n\n- The message sent to a subscriber is guaranteed not to be resent before the message's acknowledgement deadline expires.\n\n- An acknowledged message will not be resent to a subscriber.\n\nNote that subscribers may still receive multiple copies of a message when 'enable_exactly_once_delivery'\nis true if the message was published multiple times by a publisher client. These copies are considered distinct by Pub/Sub and have distinct messageId values"]
    pub fn enable_exactly_once_delivery(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_exactly_once_delivery", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_message_ordering` after provisioning.\nIf 'true', messages published with the same orderingKey in PubsubMessage will be delivered to\nthe subscribers in the order in which they are received by the Pub/Sub system. Otherwise, they\nmay be delivered in any order."]
    pub fn enable_message_ordering(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_message_ordering", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nThe subscription only delivers the messages that match the filter.\nPub/Sub automatically acknowledges the messages that don't match the filter. You can filter messages\nby their attributes. The maximum length of a filter is 256 bytes. After creating the subscription,\nyou can't modify the filter."]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to this Subscription.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `message_retention_duration` after provisioning.\nHow long to retain unacknowledged messages in the subscription's\nbacklog, from the moment a message is published. If\nretain_acked_messages is true, then this also configures the retention\nof acknowledged messages, and thus configures how far back in time a\nsubscriptions.seek can be done. Defaults to 7 days. Cannot be more\nthan 7 days ('\"604800s\"') or less than 10 minutes ('\"600s\"').\n\nA duration in seconds with up to nine fractional digits, terminated\nby 's'. Example: '\"600.5s\"'."]
    pub fn message_retention_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_retention_duration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the subscription."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `retain_acked_messages` after provisioning.\nIndicates whether to retain acknowledged messages. If 'true', then\nmessages are not expunged from the subscription's backlog, even if\nthey are acknowledged, until they fall out of the\nmessageRetentionDuration window."]
    pub fn retain_acked_messages(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retain_acked_messages", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `topic` after provisioning.\nA reference to a Topic resource."]
    pub fn topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bigquery_config` after provisioning.\n"]
    pub fn bigquery_config(&self) -> terrars::ListRef<PubsubSubscriptionBigqueryConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigquery_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloud_storage_config` after provisioning.\n"]
    pub fn cloud_storage_config(&self) -> terrars::ListRef<PubsubSubscriptionCloudStorageConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_storage_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dead_letter_policy` after provisioning.\n"]
    pub fn dead_letter_policy(&self) -> terrars::ListRef<PubsubSubscriptionDeadLetterPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dead_letter_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `expiration_policy` after provisioning.\n"]
    pub fn expiration_policy(&self) -> terrars::ListRef<PubsubSubscriptionExpirationPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expiration_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `push_config` after provisioning.\n"]
    pub fn push_config(&self) -> terrars::ListRef<PubsubSubscriptionPushConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.push_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> terrars::ListRef<PubsubSubscriptionRetryPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PubsubSubscriptionTimeoutsElRef {
        PubsubSubscriptionTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct PubsubSubscriptionBigqueryConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    drop_unknown_fields: std::option::Option<terrars::PrimField<bool>>,
    table: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_topic_schema: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    write_metadata: std::option::Option<terrars::PrimField<bool>>,
}

impl PubsubSubscriptionBigqueryConfigEl {
    #[doc =
        "Set the field `drop_unknown_fields`.\nWhen true and useTopicSchema is true, any fields that are a part of the topic schema that are not part of the BigQuery table schema are dropped when writing to BigQuery.\nOtherwise, the schemas must be kept in sync and any messages with extra fields are not written and remain in the subscription's backlog."]
    pub fn set_drop_unknown_fields(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.drop_unknown_fields = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `use_topic_schema`.\nWhen true, use the topic's schema as the columns to write to in BigQuery, if it exists."]
    pub fn set_use_topic_schema(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_topic_schema = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `write_metadata`.\nWhen true, write the subscription name, messageId, publishTime, attributes, and orderingKey to additional columns in the table.\nThe subscription name, messageId, and publishTime fields are put in their own columns while all other message properties (other than data) are written to a JSON object in the attributes column."]
    pub fn set_write_metadata(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.write_metadata = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PubsubSubscriptionBigqueryConfigEl {
    type O = terrars::BlockAssignable<PubsubSubscriptionBigqueryConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubSubscriptionBigqueryConfigEl {
    #[doc = "The name of the table to which to write data, of the form {projectId}:{datasetId}.{tableId}"]
    pub table: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubSubscriptionBigqueryConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PubsubSubscriptionBigqueryConfigElRef {
        PubsubSubscriptionBigqueryConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubSubscriptionBigqueryConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `drop_unknown_fields` after provisioning.\nWhen true and useTopicSchema is true, any fields that are a part of the topic schema that are not part of the BigQuery table schema are dropped when writing to BigQuery.\nOtherwise, the schemas must be kept in sync and any messages with extra fields are not written and remain in the subscription's backlog."]
    pub fn drop_unknown_fields(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.drop_unknown_fields", self.base))
    }

    #[doc =
        "Get a reference to the value of field `table` after provisioning.\nThe name of the table to which to write data, of the form {projectId}:{datasetId}.{tableId}"]
    pub fn table(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }

    #[doc =
        "Get a reference to the value of field `use_topic_schema` after provisioning.\nWhen true, use the topic's schema as the columns to write to in BigQuery, if it exists."]
    pub fn use_topic_schema(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_topic_schema", self.base))
    }

    #[doc =
        "Get a reference to the value of field `write_metadata` after provisioning.\nWhen true, write the subscription name, messageId, publishTime, attributes, and orderingKey to additional columns in the table.\nThe subscription name, messageId, and publishTime fields are put in their own columns while all other message properties (other than data) are written to a JSON object in the attributes column."]
    pub fn write_metadata(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write_metadata", self.base))
    }
}

#[derive(Serialize)]
pub struct PubsubSubscriptionCloudStorageConfigElAvroConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    write_metadata: std::option::Option<terrars::PrimField<bool>>,
}

impl PubsubSubscriptionCloudStorageConfigElAvroConfigEl {
    #[doc =
        "Set the field `write_metadata`.\nWhen true, write the subscription name, messageId, publishTime, attributes, and orderingKey as additional fields in the output."]
    pub fn set_write_metadata(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.write_metadata = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PubsubSubscriptionCloudStorageConfigElAvroConfigEl {
    type O = terrars::BlockAssignable<PubsubSubscriptionCloudStorageConfigElAvroConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubSubscriptionCloudStorageConfigElAvroConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PubsubSubscriptionCloudStorageConfigElAvroConfigElRef {
        PubsubSubscriptionCloudStorageConfigElAvroConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubSubscriptionCloudStorageConfigElAvroConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `write_metadata` after provisioning.\nWhen true, write the subscription name, messageId, publishTime, attributes, and orderingKey as additional fields in the output."]
    pub fn write_metadata(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write_metadata", self.base))
    }
}

#[derive(Serialize, Default)]
struct PubsubSubscriptionCloudStorageConfigElDynamic {
    avro_config: std::option::Option<terrars::DynamicBlock<PubsubSubscriptionCloudStorageConfigElAvroConfigEl>>,
}

#[derive(Serialize)]
pub struct PubsubSubscriptionCloudStorageConfigEl {
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filename_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filename_suffix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_bytes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    avro_config: std::option::Option<std::vec::Vec<PubsubSubscriptionCloudStorageConfigElAvroConfigEl>>,
    dynamic: PubsubSubscriptionCloudStorageConfigElDynamic,
}

impl PubsubSubscriptionCloudStorageConfigEl {
    #[doc = "Set the field `filename_prefix`.\nUser-provided prefix for Cloud Storage filename."]
    pub fn set_filename_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.filename_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filename_suffix`.\nUser-provided suffix for Cloud Storage filename. Must not end in \"/\"."]
    pub fn set_filename_suffix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.filename_suffix = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_bytes`.\nThe maximum bytes that can be written to a Cloud Storage file before a new file is created. Min 1 KB, max 10 GiB.\nThe maxBytes limit may be exceeded in cases where messages are larger than the limit."]
    pub fn set_max_bytes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_duration`.\nThe maximum duration that can elapse before a new Cloud Storage file is created. Min 1 minute, max 10 minutes, default 5 minutes.\nMay not exceed the subscription's acknowledgement deadline.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn set_max_duration(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `avro_config`.\n"]
    pub fn set_avro_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PubsubSubscriptionCloudStorageConfigElAvroConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.avro_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.avro_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PubsubSubscriptionCloudStorageConfigEl {
    type O = terrars::BlockAssignable<PubsubSubscriptionCloudStorageConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubSubscriptionCloudStorageConfigEl {
    #[doc =
        "User-provided name for the Cloud Storage bucket. The bucket must be created by the user. The bucket name must be without any prefix like \"gs://\"."]
    pub bucket: terrars::PrimField<std::string::String>,
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct PubsubSubscriptionCloudStorageConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubSubscriptionCloudStorageConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PubsubSubscriptionCloudStorageConfigElRef {
        PubsubSubscriptionCloudStorageConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubSubscriptionCloudStorageConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bucket` after provisioning.\nUser-provided name for the Cloud Storage bucket. The bucket must be created by the user. The bucket name must be without any prefix like \"gs://\"."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc =
        "Get a reference to the value of field `filename_prefix` after provisioning.\nUser-provided prefix for Cloud Storage filename."]
    pub fn filename_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filename_prefix", self.base))
    }

    #[doc =
        "Get a reference to the value of field `filename_suffix` after provisioning.\nUser-provided suffix for Cloud Storage filename. Must not end in \"/\"."]
    pub fn filename_suffix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filename_suffix", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_bytes` after provisioning.\nThe maximum bytes that can be written to a Cloud Storage file before a new file is created. Min 1 KB, max 10 GiB.\nThe maxBytes limit may be exceeded in cases where messages are larger than the limit."]
    pub fn max_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_bytes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_duration` after provisioning.\nThe maximum duration that can elapse before a new Cloud Storage file is created. Min 1 minute, max 10 minutes, default 5 minutes.\nMay not exceed the subscription's acknowledgement deadline.\nA duration in seconds with up to nine fractional digits, ending with 's'. Example: \"3.5s\"."]
    pub fn max_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_duration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nAn output-only field that indicates whether or not the subscription can receive messages."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `avro_config` after provisioning.\n"]
    pub fn avro_config(&self) -> terrars::ListRef<PubsubSubscriptionCloudStorageConfigElAvroConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.avro_config", self.base))
    }
}

#[derive(Serialize)]
pub struct PubsubSubscriptionDeadLetterPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dead_letter_topic: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_delivery_attempts: std::option::Option<terrars::PrimField<f64>>,
}

impl PubsubSubscriptionDeadLetterPolicyEl {
    #[doc =
        "Set the field `dead_letter_topic`.\nThe name of the topic to which dead letter messages should be published.\nFormat is 'projects/{project}/topics/{topic}'.\n\nThe Cloud Pub/Sub service account associated with the enclosing subscription's\nparent project (i.e.,\nservice-{project_number}@gcp-sa-pubsub.iam.gserviceaccount.com) must have\npermission to Publish() to this topic.\n\nThe operation will fail if the topic does not exist.\nUsers should ensure that there is a subscription attached to this topic\nsince messages published to a topic with no subscriptions are lost."]
    pub fn set_dead_letter_topic(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dead_letter_topic = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_delivery_attempts`.\nThe maximum number of delivery attempts for any message. The value must be\nbetween 5 and 100.\n\nThe number of delivery attempts is defined as 1 + (the sum of number of\nNACKs and number of times the acknowledgement deadline has been exceeded for the message).\n\nA NACK is any call to ModifyAckDeadline with a 0 deadline. Note that\nclient libraries may automatically extend ack_deadlines.\n\nThis field will be honored on a best effort basis.\n\nIf this parameter is 0, a default value of 5 is used."]
    pub fn set_max_delivery_attempts(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_delivery_attempts = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PubsubSubscriptionDeadLetterPolicyEl {
    type O = terrars::BlockAssignable<PubsubSubscriptionDeadLetterPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubSubscriptionDeadLetterPolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PubsubSubscriptionDeadLetterPolicyElRef {
        PubsubSubscriptionDeadLetterPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubSubscriptionDeadLetterPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dead_letter_topic` after provisioning.\nThe name of the topic to which dead letter messages should be published.\nFormat is 'projects/{project}/topics/{topic}'.\n\nThe Cloud Pub/Sub service account associated with the enclosing subscription's\nparent project (i.e.,\nservice-{project_number}@gcp-sa-pubsub.iam.gserviceaccount.com) must have\npermission to Publish() to this topic.\n\nThe operation will fail if the topic does not exist.\nUsers should ensure that there is a subscription attached to this topic\nsince messages published to a topic with no subscriptions are lost."]
    pub fn dead_letter_topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dead_letter_topic", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_delivery_attempts` after provisioning.\nThe maximum number of delivery attempts for any message. The value must be\nbetween 5 and 100.\n\nThe number of delivery attempts is defined as 1 + (the sum of number of\nNACKs and number of times the acknowledgement deadline has been exceeded for the message).\n\nA NACK is any call to ModifyAckDeadline with a 0 deadline. Note that\nclient libraries may automatically extend ack_deadlines.\n\nThis field will be honored on a best effort basis.\n\nIf this parameter is 0, a default value of 5 is used."]
    pub fn max_delivery_attempts(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_delivery_attempts", self.base))
    }
}

#[derive(Serialize)]
pub struct PubsubSubscriptionExpirationPolicyEl {
    ttl: terrars::PrimField<std::string::String>,
}

impl PubsubSubscriptionExpirationPolicyEl { }

impl terrars::ToListMappable for PubsubSubscriptionExpirationPolicyEl {
    type O = terrars::BlockAssignable<PubsubSubscriptionExpirationPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubSubscriptionExpirationPolicyEl {
    #[doc =
        "Specifies the \"time-to-live\" duration for an associated resource. The\nresource expires if it is not active for a period of ttl.\nIf ttl is set to \"\", the associated resource never expires.\nA duration in seconds with up to nine fractional digits, terminated by 's'.\nExample - \"3.5s\"."]
    pub ttl: terrars::PrimField<std::string::String>,
}

impl BuildPubsubSubscriptionExpirationPolicyEl {
    pub fn build(self) -> PubsubSubscriptionExpirationPolicyEl {
        PubsubSubscriptionExpirationPolicyEl { ttl: self.ttl }
    }
}

pub struct PubsubSubscriptionExpirationPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubSubscriptionExpirationPolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PubsubSubscriptionExpirationPolicyElRef {
        PubsubSubscriptionExpirationPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubSubscriptionExpirationPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ttl` after provisioning.\nSpecifies the \"time-to-live\" duration for an associated resource. The\nresource expires if it is not active for a period of ttl.\nIf ttl is set to \"\", the associated resource never expires.\nA duration in seconds with up to nine fractional digits, terminated by 's'.\nExample - \"3.5s\"."]
    pub fn ttl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.base))
    }
}

#[derive(Serialize)]
pub struct PubsubSubscriptionPushConfigElNoWrapperEl {
    write_metadata: terrars::PrimField<bool>,
}

impl PubsubSubscriptionPushConfigElNoWrapperEl { }

impl terrars::ToListMappable for PubsubSubscriptionPushConfigElNoWrapperEl {
    type O = terrars::BlockAssignable<PubsubSubscriptionPushConfigElNoWrapperEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubSubscriptionPushConfigElNoWrapperEl {
    #[doc =
        "When true, writes the Pub/Sub message metadata to\n'x-goog-pubsub-<KEY>:<VAL>' headers of the HTTP request. Writes the\nPub/Sub message attributes to '<KEY>:<VAL>' headers of the HTTP request."]
    pub write_metadata: terrars::PrimField<bool>,
}

impl BuildPubsubSubscriptionPushConfigElNoWrapperEl {
    pub fn build(self) -> PubsubSubscriptionPushConfigElNoWrapperEl {
        PubsubSubscriptionPushConfigElNoWrapperEl { write_metadata: self.write_metadata }
    }
}

pub struct PubsubSubscriptionPushConfigElNoWrapperElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubSubscriptionPushConfigElNoWrapperElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PubsubSubscriptionPushConfigElNoWrapperElRef {
        PubsubSubscriptionPushConfigElNoWrapperElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubSubscriptionPushConfigElNoWrapperElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `write_metadata` after provisioning.\nWhen true, writes the Pub/Sub message metadata to\n'x-goog-pubsub-<KEY>:<VAL>' headers of the HTTP request. Writes the\nPub/Sub message attributes to '<KEY>:<VAL>' headers of the HTTP request."]
    pub fn write_metadata(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write_metadata", self.base))
    }
}

#[derive(Serialize)]
pub struct PubsubSubscriptionPushConfigElOidcTokenEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audience: std::option::Option<terrars::PrimField<std::string::String>>,
    service_account_email: terrars::PrimField<std::string::String>,
}

impl PubsubSubscriptionPushConfigElOidcTokenEl {
    #[doc =
        "Set the field `audience`.\nAudience to be used when generating OIDC token. The audience claim\nidentifies the recipients that the JWT is intended for. The audience\nvalue is a single case-sensitive string. Having multiple values (array)\nfor the audience field is not supported. More info about the OIDC JWT\ntoken audience here: https://tools.ietf.org/html/rfc7519#section-4.1.3\nNote: if not specified, the Push endpoint URL will be used."]
    pub fn set_audience(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.audience = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PubsubSubscriptionPushConfigElOidcTokenEl {
    type O = terrars::BlockAssignable<PubsubSubscriptionPushConfigElOidcTokenEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubSubscriptionPushConfigElOidcTokenEl {
    #[doc =
        "Service account email to be used for generating the OIDC token.\nThe caller (for subscriptions.create, subscriptions.patch, and\nsubscriptions.modifyPushConfig RPCs) must have the\niam.serviceAccounts.actAs permission for the service account."]
    pub service_account_email: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubSubscriptionPushConfigElOidcTokenElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PubsubSubscriptionPushConfigElOidcTokenElRef {
        PubsubSubscriptionPushConfigElOidcTokenElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubSubscriptionPushConfigElOidcTokenElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `audience` after provisioning.\nAudience to be used when generating OIDC token. The audience claim\nidentifies the recipients that the JWT is intended for. The audience\nvalue is a single case-sensitive string. Having multiple values (array)\nfor the audience field is not supported. More info about the OIDC JWT\ntoken audience here: https://tools.ietf.org/html/rfc7519#section-4.1.3\nNote: if not specified, the Push endpoint URL will be used."]
    pub fn audience(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audience", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account_email` after provisioning.\nService account email to be used for generating the OIDC token.\nThe caller (for subscriptions.create, subscriptions.patch, and\nsubscriptions.modifyPushConfig RPCs) must have the\niam.serviceAccounts.actAs permission for the service account."]
    pub fn service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_email", self.base))
    }
}

#[derive(Serialize, Default)]
struct PubsubSubscriptionPushConfigElDynamic {
    no_wrapper: std::option::Option<terrars::DynamicBlock<PubsubSubscriptionPushConfigElNoWrapperEl>>,
    oidc_token: std::option::Option<terrars::DynamicBlock<PubsubSubscriptionPushConfigElOidcTokenEl>>,
}

#[derive(Serialize)]
pub struct PubsubSubscriptionPushConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attributes: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    push_endpoint: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_wrapper: std::option::Option<std::vec::Vec<PubsubSubscriptionPushConfigElNoWrapperEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oidc_token: std::option::Option<std::vec::Vec<PubsubSubscriptionPushConfigElOidcTokenEl>>,
    dynamic: PubsubSubscriptionPushConfigElDynamic,
}

impl PubsubSubscriptionPushConfigEl {
    #[doc =
        "Set the field `attributes`.\nEndpoint configuration attributes.\n\nEvery endpoint has a set of API supported attributes that can\nbe used to control different aspects of the message delivery.\n\nThe currently supported attribute is x-goog-version, which you\ncan use to change the format of the pushed message. This\nattribute indicates the version of the data expected by\nthe endpoint. This controls the shape of the pushed message\n(i.e., its fields and metadata). The endpoint version is\nbased on the version of the Pub/Sub API.\n\nIf not present during the subscriptions.create call,\nit will default to the version of the API used to make\nsuch call. If not present during a subscriptions.modifyPushConfig\ncall, its value will not be changed. subscriptions.get\ncalls will always return a valid version, even if the\nsubscription was created without this attribute.\n\nThe possible values for this attribute are:\n\n- v1beta1: uses the push format defined in the v1beta1 Pub/Sub API.\n- v1 or v1beta2: uses the push format defined in the v1 Pub/Sub API."]
    pub fn set_attributes(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.attributes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `no_wrapper`.\n"]
    pub fn set_no_wrapper(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PubsubSubscriptionPushConfigElNoWrapperEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.no_wrapper = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.no_wrapper = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oidc_token`.\n"]
    pub fn set_oidc_token(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PubsubSubscriptionPushConfigElOidcTokenEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oidc_token = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oidc_token = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PubsubSubscriptionPushConfigEl {
    type O = terrars::BlockAssignable<PubsubSubscriptionPushConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubSubscriptionPushConfigEl {
    #[doc =
        "A URL locating the endpoint to which messages should be pushed.\nFor example, a Webhook endpoint might use\n\"https://example.com/push\"."]
    pub push_endpoint: terrars::PrimField<std::string::String>,
}

impl BuildPubsubSubscriptionPushConfigEl {
    pub fn build(self) -> PubsubSubscriptionPushConfigEl {
        PubsubSubscriptionPushConfigEl {
            attributes: core::default::Default::default(),
            push_endpoint: self.push_endpoint,
            no_wrapper: core::default::Default::default(),
            oidc_token: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct PubsubSubscriptionPushConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubSubscriptionPushConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PubsubSubscriptionPushConfigElRef {
        PubsubSubscriptionPushConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubSubscriptionPushConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `attributes` after provisioning.\nEndpoint configuration attributes.\n\nEvery endpoint has a set of API supported attributes that can\nbe used to control different aspects of the message delivery.\n\nThe currently supported attribute is x-goog-version, which you\ncan use to change the format of the pushed message. This\nattribute indicates the version of the data expected by\nthe endpoint. This controls the shape of the pushed message\n(i.e., its fields and metadata). The endpoint version is\nbased on the version of the Pub/Sub API.\n\nIf not present during the subscriptions.create call,\nit will default to the version of the API used to make\nsuch call. If not present during a subscriptions.modifyPushConfig\ncall, its value will not be changed. subscriptions.get\ncalls will always return a valid version, even if the\nsubscription was created without this attribute.\n\nThe possible values for this attribute are:\n\n- v1beta1: uses the push format defined in the v1beta1 Pub/Sub API.\n- v1 or v1beta2: uses the push format defined in the v1 Pub/Sub API."]
    pub fn attributes(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.attributes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `push_endpoint` after provisioning.\nA URL locating the endpoint to which messages should be pushed.\nFor example, a Webhook endpoint might use\n\"https://example.com/push\"."]
    pub fn push_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.push_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `no_wrapper` after provisioning.\n"]
    pub fn no_wrapper(&self) -> terrars::ListRef<PubsubSubscriptionPushConfigElNoWrapperElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.no_wrapper", self.base))
    }

    #[doc = "Get a reference to the value of field `oidc_token` after provisioning.\n"]
    pub fn oidc_token(&self) -> terrars::ListRef<PubsubSubscriptionPushConfigElOidcTokenElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oidc_token", self.base))
    }
}

#[derive(Serialize)]
pub struct PubsubSubscriptionRetryPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_backoff: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum_backoff: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PubsubSubscriptionRetryPolicyEl {
    #[doc =
        "Set the field `maximum_backoff`.\nThe maximum delay between consecutive deliveries of a given message. Value should be between 0 and 600 seconds. Defaults to 600 seconds.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn set_maximum_backoff(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.maximum_backoff = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `minimum_backoff`.\nThe minimum delay between consecutive deliveries of a given message. Value should be between 0 and 600 seconds. Defaults to 10 seconds.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn set_minimum_backoff(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.minimum_backoff = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PubsubSubscriptionRetryPolicyEl {
    type O = terrars::BlockAssignable<PubsubSubscriptionRetryPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubSubscriptionRetryPolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PubsubSubscriptionRetryPolicyElRef {
        PubsubSubscriptionRetryPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubSubscriptionRetryPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `maximum_backoff` after provisioning.\nThe maximum delay between consecutive deliveries of a given message. Value should be between 0 and 600 seconds. Defaults to 600 seconds.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn maximum_backoff(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_backoff", self.base))
    }

    #[doc =
        "Get a reference to the value of field `minimum_backoff` after provisioning.\nThe minimum delay between consecutive deliveries of a given message. Value should be between 0 and 600 seconds. Defaults to 10 seconds.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
    pub fn minimum_backoff(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum_backoff", self.base))
    }
}

#[derive(Serialize)]
pub struct PubsubSubscriptionTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PubsubSubscriptionTimeoutsEl {
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

impl terrars::ToListMappable for PubsubSubscriptionTimeoutsEl {
    type O = terrars::BlockAssignable<PubsubSubscriptionTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubSubscriptionTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PubsubSubscriptionTimeoutsElRef {
        PubsubSubscriptionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubSubscriptionTimeoutsElRef {
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
struct PubsubSubscriptionDynamic {
    bigquery_config: std::option::Option<terrars::DynamicBlock<PubsubSubscriptionBigqueryConfigEl>>,
    cloud_storage_config: std::option::Option<terrars::DynamicBlock<PubsubSubscriptionCloudStorageConfigEl>>,
    dead_letter_policy: std::option::Option<terrars::DynamicBlock<PubsubSubscriptionDeadLetterPolicyEl>>,
    expiration_policy: std::option::Option<terrars::DynamicBlock<PubsubSubscriptionExpirationPolicyEl>>,
    push_config: std::option::Option<terrars::DynamicBlock<PubsubSubscriptionPushConfigEl>>,
    retry_policy: std::option::Option<terrars::DynamicBlock<PubsubSubscriptionRetryPolicyEl>>,
}
