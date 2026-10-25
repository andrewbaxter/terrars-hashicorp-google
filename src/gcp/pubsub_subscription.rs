#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct PubsubSubscriptionData_ {
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
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    topic: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bigquery_config: std::option::Option<std::vec::Vec<PubsubSubscription_BigqueryConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bigtable_config: std::option::Option<std::vec::Vec<PubsubSubscription_BigtableConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_storage_config: std::option::Option<std::vec::Vec<PubsubSubscription_CloudStorageConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dead_letter_policy: std::option::Option<std::vec::Vec<PubsubSubscription_DeadLetterPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expiration_policy: std::option::Option<std::vec::Vec<PubsubSubscription_ExpirationPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_transforms: std::option::Option<std::vec::Vec<PubsubSubscription_MessageTransforms>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    push_config: std::option::Option<std::vec::Vec<PubsubSubscription_PushConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_policy: std::option::Option<std::vec::Vec<PubsubSubscription_RetryPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<PubsubSubscription_Timeouts>,
    dynamic: PubsubSubscriptionDynamic,
}

struct PubsubSubscription_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<PubsubSubscriptionData_>,
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
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
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
        "Set the field `message_retention_duration`.\nHow long to retain unacknowledged messages in the subscription's\nbacklog, from the moment a message is published. If\nretain_acked_messages is true, then this also configures the retention\nof acknowledged messages, and thus configures how far back in time a\nsubscriptions.seek can be done. Defaults to 7 days. Cannot be more\nthan 31 days ('\"2678400s\"') or less than 10 minutes ('\"600s\"').\n\nA duration in seconds with up to nine fractional digits, terminated\nby 's'. Example: '\"600.5s\"'."]
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

    #[doc =
        "Set the field `tags`.\nInput only. Resource manager tags to be bound to the subscription. Tag\nkeys and values have the same definition as resource manager tags. Keys\nmust be in the format tagKeys/{tag_key_id}, and values are in the format\ntagValues/456. The field is ignored when empty. The field is immutable and\ncauses resource replacement when mutated. This field is only set at create\ntime and modifying this field after creation will trigger recreation. To\napply tags to an existing resource, see the 'google_tags_tag_value'\nresource."]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bigquery_config`.\n"]
    pub fn set_bigquery_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PubsubSubscription_BigqueryConfig>>,
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

    #[doc = "Set the field `bigtable_config`.\n"]
    pub fn set_bigtable_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PubsubSubscription_BigtableConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().bigtable_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.bigtable_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cloud_storage_config`.\n"]
    pub fn set_cloud_storage_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PubsubSubscription_CloudStorageConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<PubsubSubscription_DeadLetterPolicy>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<PubsubSubscription_ExpirationPolicy>>,
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

    #[doc = "Set the field `message_transforms`.\n"]
    pub fn set_message_transforms(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PubsubSubscription_MessageTransforms>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().message_transforms = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.message_transforms = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `push_config`.\n"]
    pub fn set_push_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PubsubSubscription_PushConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<PubsubSubscription_RetryPolicy>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<PubsubSubscription_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `ack_deadline_seconds` after provisioning.\nThis value is the maximum time after a subscriber receives a message\nbefore the subscriber should acknowledge the message. After message\ndelivery but before the ack deadline expires and before the message is\nacknowledged, it is an outstanding message and will not be delivered\nagain during that time (on a best-effort basis).\n\nFor pull subscriptions, this value is used as the initial value for\nthe ack deadline. To override this value for a given message, call\nsubscriptions.modifyAckDeadline with the corresponding ackId if using\npull. The minimum custom deadline you can specify is 10 seconds. The\nmaximum custom deadline you can specify is 600 seconds (10 minutes).\nIf this parameter is 0, a default value of 10 seconds is used.\n\nFor push delivery, this value is also used to set the request timeout\nfor the call to the push endpoint.\n\nIf the subscriber never acknowledges the message, the Pub/Sub system\nwill eventually redeliver the message."]
    pub fn ack_deadline_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ack_deadline_seconds", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
        "Get a reference to the value of field `message_retention_duration` after provisioning.\nHow long to retain unacknowledged messages in the subscription's\nbacklog, from the moment a message is published. If\nretain_acked_messages is true, then this also configures the retention\nof acknowledged messages, and thus configures how far back in time a\nsubscriptions.seek can be done. Defaults to 7 days. Cannot be more\nthan 31 days ('\"2678400s\"') or less than 10 minutes ('\"600s\"').\n\nA duration in seconds with up to nine fractional digits, terminated\nby 's'. Example: '\"600.5s\"'."]
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
        "Get a reference to the value of field `tags` after provisioning.\nInput only. Resource manager tags to be bound to the subscription. Tag\nkeys and values have the same definition as resource manager tags. Keys\nmust be in the format tagKeys/{tag_key_id}, and values are in the format\ntagValues/456. The field is ignored when empty. The field is immutable and\ncauses resource replacement when mutated. This field is only set at create\ntime and modifying this field after creation will trigger recreation. To\napply tags to an existing resource, see the 'google_tags_tag_value'\nresource."]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `topic` after provisioning.\nA reference to a Topic resource, of the form projects/{project}/topics/{{name}}\n(as in the id property of a google_pubsub_topic), or just a topic name if\nthe topic is in the same project as the subscription."]
    pub fn topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bigquery_config` after provisioning.\n"]
    pub fn bigquery_config(&self) -> terrars::ListRef<PubsubSubscription_BigqueryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigquery_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bigtable_config` after provisioning.\n"]
    pub fn bigtable_config(&self) -> terrars::ListRef<PubsubSubscription_BigtableConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigtable_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloud_storage_config` after provisioning.\n"]
    pub fn cloud_storage_config(&self) -> terrars::ListRef<PubsubSubscription_CloudStorageConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_storage_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dead_letter_policy` after provisioning.\n"]
    pub fn dead_letter_policy(&self) -> terrars::ListRef<PubsubSubscription_DeadLetterPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dead_letter_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `expiration_policy` after provisioning.\n"]
    pub fn expiration_policy(&self) -> terrars::ListRef<PubsubSubscription_ExpirationPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expiration_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `message_transforms` after provisioning.\n"]
    pub fn message_transforms(&self) -> terrars::ListRef<PubsubSubscription_MessageTransformsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.message_transforms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `push_config` after provisioning.\n"]
    pub fn push_config(&self) -> terrars::ListRef<PubsubSubscription_PushConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.push_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> terrars::ListRef<PubsubSubscription_RetryPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PubsubSubscription_TimeoutsRef {
        PubsubSubscription_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
    #[doc =
        "A reference to a Topic resource, of the form projects/{project}/topics/{{name}}\n(as in the id property of a google_pubsub_topic), or just a topic name if\nthe topic is in the same project as the subscription."]
    pub topic: terrars::PrimField<std::string::String>,
}

impl BuildPubsubSubscription {
    pub fn build(self, stack: &mut terrars::Stack) -> PubsubSubscription {
        let out = PubsubSubscription(std::rc::Rc::new(PubsubSubscription_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(PubsubSubscriptionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                ack_deadline_seconds: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                enable_exactly_once_delivery: core::default::Default::default(),
                enable_message_ordering: core::default::Default::default(),
                filter: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                message_retention_duration: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                retain_acked_messages: core::default::Default::default(),
                tags: core::default::Default::default(),
                topic: self.topic,
                bigquery_config: core::default::Default::default(),
                bigtable_config: core::default::Default::default(),
                cloud_storage_config: core::default::Default::default(),
                dead_letter_policy: core::default::Default::default(),
                expiration_policy: core::default::Default::default(),
                message_transforms: core::default::Default::default(),
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
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
        "Get a reference to the value of field `message_retention_duration` after provisioning.\nHow long to retain unacknowledged messages in the subscription's\nbacklog, from the moment a message is published. If\nretain_acked_messages is true, then this also configures the retention\nof acknowledged messages, and thus configures how far back in time a\nsubscriptions.seek can be done. Defaults to 7 days. Cannot be more\nthan 31 days ('\"2678400s\"') or less than 10 minutes ('\"600s\"').\n\nA duration in seconds with up to nine fractional digits, terminated\nby 's'. Example: '\"600.5s\"'."]
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
        "Get a reference to the value of field `tags` after provisioning.\nInput only. Resource manager tags to be bound to the subscription. Tag\nkeys and values have the same definition as resource manager tags. Keys\nmust be in the format tagKeys/{tag_key_id}, and values are in the format\ntagValues/456. The field is ignored when empty. The field is immutable and\ncauses resource replacement when mutated. This field is only set at create\ntime and modifying this field after creation will trigger recreation. To\napply tags to an existing resource, see the 'google_tags_tag_value'\nresource."]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `topic` after provisioning.\nA reference to a Topic resource, of the form projects/{project}/topics/{{name}}\n(as in the id property of a google_pubsub_topic), or just a topic name if\nthe topic is in the same project as the subscription."]
    pub fn topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bigquery_config` after provisioning.\n"]
    pub fn bigquery_config(&self) -> terrars::ListRef<PubsubSubscription_BigqueryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigquery_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bigtable_config` after provisioning.\n"]
    pub fn bigtable_config(&self) -> terrars::ListRef<PubsubSubscription_BigtableConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigtable_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloud_storage_config` after provisioning.\n"]
    pub fn cloud_storage_config(&self) -> terrars::ListRef<PubsubSubscription_CloudStorageConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_storage_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dead_letter_policy` after provisioning.\n"]
    pub fn dead_letter_policy(&self) -> terrars::ListRef<PubsubSubscription_DeadLetterPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dead_letter_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `expiration_policy` after provisioning.\n"]
    pub fn expiration_policy(&self) -> terrars::ListRef<PubsubSubscription_ExpirationPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expiration_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `message_transforms` after provisioning.\n"]
    pub fn message_transforms(&self) -> terrars::ListRef<PubsubSubscription_MessageTransformsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.message_transforms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `push_config` after provisioning.\n"]
    pub fn push_config(&self) -> terrars::ListRef<PubsubSubscription_PushConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.push_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> terrars::ListRef<PubsubSubscription_RetryPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PubsubSubscription_TimeoutsRef {
        PubsubSubscription_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct PubsubSubscriptionDynamic {
    bigquery_config: std::option::Option<terrars::DynamicBlock<PubsubSubscription_BigqueryConfig>>,
    bigtable_config: std::option::Option<terrars::DynamicBlock<PubsubSubscription_BigtableConfig>>,
    cloud_storage_config: std::option::Option<terrars::DynamicBlock<PubsubSubscription_CloudStorageConfig>>,
    dead_letter_policy: std::option::Option<terrars::DynamicBlock<PubsubSubscription_DeadLetterPolicy>>,
    expiration_policy: std::option::Option<terrars::DynamicBlock<PubsubSubscription_ExpirationPolicy>>,
    message_transforms: std::option::Option<terrars::DynamicBlock<PubsubSubscription_MessageTransforms>>,
    push_config: std::option::Option<terrars::DynamicBlock<PubsubSubscription_PushConfig>>,
    retry_policy: std::option::Option<terrars::DynamicBlock<PubsubSubscription_RetryPolicy>>,
}

#[derive(Serialize)]
pub struct PubsubSubscription_BigqueryConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    drop_unknown_fields: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_email: std::option::Option<terrars::PrimField<std::string::String>>,
    table: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_table_schema: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_topic_schema: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    write_metadata: std::option::Option<terrars::PrimField<bool>>,
}

impl PubsubSubscription_BigqueryConfig {
    #[doc =
        "Set the field `drop_unknown_fields`.\nWhen true and use_topic_schema or use_table_schema is true, any fields that are a part of the topic schema or message schema that\nare not part of the BigQuery table schema are dropped when writing to BigQuery. Otherwise, the schemas must be kept in sync\nand any messages with extra fields are not written and remain in the subscription's backlog."]
    pub fn set_drop_unknown_fields(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.drop_unknown_fields = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_account_email`.\nThe service account to use to write to BigQuery. If not specified, the Pub/Sub\n[service agent](https://cloud.google.com/iam/docs/service-agents),\nservice-{project_number}@gcp-sa-pubsub.iam.gserviceaccount.com, is used."]
    pub fn set_service_account_email(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_account_email = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `use_table_schema`.\nWhen true, use the BigQuery table's schema as the columns to write to in BigQuery. Messages\nmust be published in JSON format. Only one of use_topic_schema and use_table_schema can be set."]
    pub fn set_use_table_schema(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_table_schema = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `use_topic_schema`.\nWhen true, use the topic's schema as the columns to write to in BigQuery, if it exists.\nOnly one of use_topic_schema and use_table_schema can be set."]
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

impl terrars::ToListMappable for PubsubSubscription_BigqueryConfig {
    type O = terrars::BlockAssignable<PubsubSubscription_BigqueryConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubSubscription_BigqueryConfig {
    #[doc = "The name of the table to which to write data, of the form {projectId}.{datasetId}.{tableId}"]
    pub table: terrars::PrimField<std::string::String>,
}

impl BuildPubsubSubscription_BigqueryConfig {
    pub fn build(self) -> PubsubSubscription_BigqueryConfig {
        PubsubSubscription_BigqueryConfig {
            drop_unknown_fields: core::default::Default::default(),
            service_account_email: core::default::Default::default(),
            table: self.table,
            use_table_schema: core::default::Default::default(),
            use_topic_schema: core::default::Default::default(),
            write_metadata: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PubsubSubscription_BigqueryConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubSubscription_BigqueryConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PubsubSubscription_BigqueryConfigRef {
        PubsubSubscription_BigqueryConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubSubscription_BigqueryConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `drop_unknown_fields` after provisioning.\nWhen true and use_topic_schema or use_table_schema is true, any fields that are a part of the topic schema or message schema that\nare not part of the BigQuery table schema are dropped when writing to BigQuery. Otherwise, the schemas must be kept in sync\nand any messages with extra fields are not written and remain in the subscription's backlog."]
    pub fn drop_unknown_fields(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.drop_unknown_fields", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account_email` after provisioning.\nThe service account to use to write to BigQuery. If not specified, the Pub/Sub\n[service agent](https://cloud.google.com/iam/docs/service-agents),\nservice-{project_number}@gcp-sa-pubsub.iam.gserviceaccount.com, is used."]
    pub fn service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_email", self.base))
    }

    #[doc =
        "Get a reference to the value of field `table` after provisioning.\nThe name of the table to which to write data, of the form {projectId}.{datasetId}.{tableId}"]
    pub fn table(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }

    #[doc =
        "Get a reference to the value of field `use_table_schema` after provisioning.\nWhen true, use the BigQuery table's schema as the columns to write to in BigQuery. Messages\nmust be published in JSON format. Only one of use_topic_schema and use_table_schema can be set."]
    pub fn use_table_schema(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_table_schema", self.base))
    }

    #[doc =
        "Get a reference to the value of field `use_topic_schema` after provisioning.\nWhen true, use the topic's schema as the columns to write to in BigQuery, if it exists.\nOnly one of use_topic_schema and use_table_schema can be set."]
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
pub struct PubsubSubscription_BigtableConfig_ColumnFamilyMapping_DelimitedKey {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delimiter: std::option::Option<terrars::PrimField<std::string::String>>,
    key_fields: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl PubsubSubscription_BigtableConfig_ColumnFamilyMapping_DelimitedKey {
    #[doc =
        "Set the field `delimiter`.\nThe byte sequence used to delimit concatenated key fields, as a base64-encoded string\n(for example, 'Iw==' for '#'). Must be specified if multiple key fields are used. The\ndelimiter must contain at least 1 character and at most 50 characters."]
    pub fn set_delimiter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delimiter = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PubsubSubscription_BigtableConfig_ColumnFamilyMapping_DelimitedKey {
    type O = terrars::BlockAssignable<PubsubSubscription_BigtableConfig_ColumnFamilyMapping_DelimitedKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubSubscription_BigtableConfig_ColumnFamilyMapping_DelimitedKey {
    #[doc =
        "The key fields to construct the row key from. The fields must be present in the message\nas top-level fields; JSON path expressions will not traverse into nested objects."]
    pub key_fields: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildPubsubSubscription_BigtableConfig_ColumnFamilyMapping_DelimitedKey {
    pub fn build(self) -> PubsubSubscription_BigtableConfig_ColumnFamilyMapping_DelimitedKey {
        PubsubSubscription_BigtableConfig_ColumnFamilyMapping_DelimitedKey {
            delimiter: core::default::Default::default(),
            key_fields: self.key_fields,
        }
    }
}

#[allow(dead_code)]
pub struct PubsubSubscription_BigtableConfig_ColumnFamilyMapping_DelimitedKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubSubscription_BigtableConfig_ColumnFamilyMapping_DelimitedKeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PubsubSubscription_BigtableConfig_ColumnFamilyMapping_DelimitedKeyRef {
        PubsubSubscription_BigtableConfig_ColumnFamilyMapping_DelimitedKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubSubscription_BigtableConfig_ColumnFamilyMapping_DelimitedKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `delimiter` after provisioning.\nThe byte sequence used to delimit concatenated key fields, as a base64-encoded string\n(for example, 'Iw==' for '#'). Must be specified if multiple key fields are used. The\ndelimiter must contain at least 1 character and at most 50 characters."]
    pub fn delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delimiter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key_fields` after provisioning.\nThe key fields to construct the row key from. The fields must be present in the message\nas top-level fields; JSON path expressions will not traverse into nested objects."]
    pub fn key_fields(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.key_fields", self.base))
    }
}

#[derive(Serialize)]
pub struct PubsubSubscription_BigtableConfig_ColumnFamilyMapping_RowKeySchema {}

impl PubsubSubscription_BigtableConfig_ColumnFamilyMapping_RowKeySchema { }

impl terrars::ToListMappable for PubsubSubscription_BigtableConfig_ColumnFamilyMapping_RowKeySchema {
    type O = terrars::BlockAssignable<PubsubSubscription_BigtableConfig_ColumnFamilyMapping_RowKeySchema>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubSubscription_BigtableConfig_ColumnFamilyMapping_RowKeySchema {}

impl BuildPubsubSubscription_BigtableConfig_ColumnFamilyMapping_RowKeySchema {
    pub fn build(self) -> PubsubSubscription_BigtableConfig_ColumnFamilyMapping_RowKeySchema {
        PubsubSubscription_BigtableConfig_ColumnFamilyMapping_RowKeySchema {}
    }
}

#[allow(dead_code)]
pub struct PubsubSubscription_BigtableConfig_ColumnFamilyMapping_RowKeySchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubSubscription_BigtableConfig_ColumnFamilyMapping_RowKeySchemaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PubsubSubscription_BigtableConfig_ColumnFamilyMapping_RowKeySchemaRef {
        PubsubSubscription_BigtableConfig_ColumnFamilyMapping_RowKeySchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubSubscription_BigtableConfig_ColumnFamilyMapping_RowKeySchemaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct PubsubSubscription_BigtableConfig_ColumnFamilyMappingDynamic {
    delimited_key: std
    ::option
    ::Option<terrars::DynamicBlock<PubsubSubscription_BigtableConfig_ColumnFamilyMapping_DelimitedKey>>,
    row_key_schema: std
    ::option
    ::Option<terrars::DynamicBlock<PubsubSubscription_BigtableConfig_ColumnFamilyMapping_RowKeySchema>>,
}

#[derive(Serialize)]
pub struct PubsubSubscription_BigtableConfig_ColumnFamilyMapping {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delimited_key: std::option::Option<std::vec::Vec<PubsubSubscription_BigtableConfig_ColumnFamilyMapping_DelimitedKey>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    row_key_schema: std
    ::option
    ::Option<std::vec::Vec<PubsubSubscription_BigtableConfig_ColumnFamilyMapping_RowKeySchema>>,
    dynamic: PubsubSubscription_BigtableConfig_ColumnFamilyMappingDynamic,
}

impl PubsubSubscription_BigtableConfig_ColumnFamilyMapping {
    #[doc = "Set the field `delimited_key`.\n"]
    pub fn set_delimited_key(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<PubsubSubscription_BigtableConfig_ColumnFamilyMapping_DelimitedKey>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.delimited_key = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.delimited_key = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `row_key_schema`.\n"]
    pub fn set_row_key_schema(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<PubsubSubscription_BigtableConfig_ColumnFamilyMapping_RowKeySchema>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.row_key_schema = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.row_key_schema = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PubsubSubscription_BigtableConfig_ColumnFamilyMapping {
    type O = terrars::BlockAssignable<PubsubSubscription_BigtableConfig_ColumnFamilyMapping>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubSubscription_BigtableConfig_ColumnFamilyMapping {}

impl BuildPubsubSubscription_BigtableConfig_ColumnFamilyMapping {
    pub fn build(self) -> PubsubSubscription_BigtableConfig_ColumnFamilyMapping {
        PubsubSubscription_BigtableConfig_ColumnFamilyMapping {
            delimited_key: core::default::Default::default(),
            row_key_schema: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PubsubSubscription_BigtableConfig_ColumnFamilyMappingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubSubscription_BigtableConfig_ColumnFamilyMappingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PubsubSubscription_BigtableConfig_ColumnFamilyMappingRef {
        PubsubSubscription_BigtableConfig_ColumnFamilyMappingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubSubscription_BigtableConfig_ColumnFamilyMappingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delimited_key` after provisioning.\n"]
    pub fn delimited_key(
        &self,
    ) -> terrars::ListRef<PubsubSubscription_BigtableConfig_ColumnFamilyMapping_DelimitedKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delimited_key", self.base))
    }

    #[doc = "Get a reference to the value of field `row_key_schema` after provisioning.\n"]
    pub fn row_key_schema(
        &self,
    ) -> terrars::ListRef<PubsubSubscription_BigtableConfig_ColumnFamilyMapping_RowKeySchemaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.row_key_schema", self.base))
    }
}

#[derive(Serialize, Default)]
struct PubsubSubscription_BigtableConfigDynamic {
    column_family_mapping: std
    ::option
    ::Option<terrars::DynamicBlock<PubsubSubscription_BigtableConfig_ColumnFamilyMapping>>,
}

#[derive(Serialize)]
pub struct PubsubSubscription_BigtableConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    app_profile_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_email: std::option::Option<terrars::PrimField<std::string::String>>,
    table: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    write_metadata: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column_family_mapping: std::option::Option<std::vec::Vec<PubsubSubscription_BigtableConfig_ColumnFamilyMapping>>,
    dynamic: PubsubSubscription_BigtableConfigDynamic,
}

impl PubsubSubscription_BigtableConfig {
    #[doc =
        "Set the field `app_profile_id`.\nThe app profile to use for the Bigtable writes. If not specified, the \"default\" application\nprofile will be used. The app profile must use single-cluster routing."]
    pub fn set_app_profile_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.app_profile_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_account_email`.\nThe service account to use to write to Bigtable. The subscription creator or updater that\nspecifies this field must have 'iam.serviceAccounts.actAs' permission on the service account.\nIf not specified, the Pub/Sub [service agent](https://cloud.google.com/iam/docs/service-agents),\nservice-{project_number}@gcp-sa-pubsub.iam.gserviceaccount.com, is used."]
    pub fn set_service_account_email(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_account_email = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `write_metadata`.\nWhen true, write the subscription name, messageId, publishTime, attributes, and orderingKey to\nadditional columns in the table under the 'pubsub_metadata' column family. The subscription name,\nmessageId, and publishTime fields are put in their own columns while all other message properties\n(other than data) are written to a JSON object in the attributes column."]
    pub fn set_write_metadata(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.write_metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `column_family_mapping`.\n"]
    pub fn set_column_family_mapping(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PubsubSubscription_BigtableConfig_ColumnFamilyMapping>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.column_family_mapping = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.column_family_mapping = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PubsubSubscription_BigtableConfig {
    type O = terrars::BlockAssignable<PubsubSubscription_BigtableConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubSubscription_BigtableConfig {
    #[doc =
        "The unique name of the table to write messages to, of the form\n'projects/{project}/instances/{instance}/tables/{table}'."]
    pub table: terrars::PrimField<std::string::String>,
}

impl BuildPubsubSubscription_BigtableConfig {
    pub fn build(self) -> PubsubSubscription_BigtableConfig {
        PubsubSubscription_BigtableConfig {
            app_profile_id: core::default::Default::default(),
            service_account_email: core::default::Default::default(),
            table: self.table,
            write_metadata: core::default::Default::default(),
            column_family_mapping: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PubsubSubscription_BigtableConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubSubscription_BigtableConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PubsubSubscription_BigtableConfigRef {
        PubsubSubscription_BigtableConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubSubscription_BigtableConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `app_profile_id` after provisioning.\nThe app profile to use for the Bigtable writes. If not specified, the \"default\" application\nprofile will be used. The app profile must use single-cluster routing."]
    pub fn app_profile_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_profile_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account_email` after provisioning.\nThe service account to use to write to Bigtable. The subscription creator or updater that\nspecifies this field must have 'iam.serviceAccounts.actAs' permission on the service account.\nIf not specified, the Pub/Sub [service agent](https://cloud.google.com/iam/docs/service-agents),\nservice-{project_number}@gcp-sa-pubsub.iam.gserviceaccount.com, is used."]
    pub fn service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_email", self.base))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nAn output-only field that indicates whether or not the subscription can receive messages."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc =
        "Get a reference to the value of field `table` after provisioning.\nThe unique name of the table to write messages to, of the form\n'projects/{project}/instances/{instance}/tables/{table}'."]
    pub fn table(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }

    #[doc =
        "Get a reference to the value of field `write_metadata` after provisioning.\nWhen true, write the subscription name, messageId, publishTime, attributes, and orderingKey to\nadditional columns in the table under the 'pubsub_metadata' column family. The subscription name,\nmessageId, and publishTime fields are put in their own columns while all other message properties\n(other than data) are written to a JSON object in the attributes column."]
    pub fn write_metadata(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write_metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `column_family_mapping` after provisioning.\n"]
    pub fn column_family_mapping(&self) -> terrars::ListRef<PubsubSubscription_BigtableConfig_ColumnFamilyMappingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.column_family_mapping", self.base))
    }
}

#[derive(Serialize)]
pub struct PubsubSubscription_CloudStorageConfig_AvroConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_topic_schema: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    write_metadata: std::option::Option<terrars::PrimField<bool>>,
}

impl PubsubSubscription_CloudStorageConfig_AvroConfig {
    #[doc =
        "Set the field `use_topic_schema`.\nWhen true, the output Cloud Storage file will be serialized using the topic schema, if it exists."]
    pub fn set_use_topic_schema(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_topic_schema = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `write_metadata`.\nWhen true, write the subscription name, messageId, publishTime, attributes, and orderingKey as additional fields in the output."]
    pub fn set_write_metadata(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.write_metadata = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PubsubSubscription_CloudStorageConfig_AvroConfig {
    type O = terrars::BlockAssignable<PubsubSubscription_CloudStorageConfig_AvroConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubSubscription_CloudStorageConfig_AvroConfig {}

impl BuildPubsubSubscription_CloudStorageConfig_AvroConfig {
    pub fn build(self) -> PubsubSubscription_CloudStorageConfig_AvroConfig {
        PubsubSubscription_CloudStorageConfig_AvroConfig {
            use_topic_schema: core::default::Default::default(),
            write_metadata: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PubsubSubscription_CloudStorageConfig_AvroConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubSubscription_CloudStorageConfig_AvroConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PubsubSubscription_CloudStorageConfig_AvroConfigRef {
        PubsubSubscription_CloudStorageConfig_AvroConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubSubscription_CloudStorageConfig_AvroConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `use_topic_schema` after provisioning.\nWhen true, the output Cloud Storage file will be serialized using the topic schema, if it exists."]
    pub fn use_topic_schema(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_topic_schema", self.base))
    }

    #[doc =
        "Get a reference to the value of field `write_metadata` after provisioning.\nWhen true, write the subscription name, messageId, publishTime, attributes, and orderingKey as additional fields in the output."]
    pub fn write_metadata(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write_metadata", self.base))
    }
}

#[derive(Serialize)]
pub struct PubsubSubscription_CloudStorageConfig_TextConfig {}

impl PubsubSubscription_CloudStorageConfig_TextConfig { }

impl terrars::ToListMappable for PubsubSubscription_CloudStorageConfig_TextConfig {
    type O = terrars::BlockAssignable<PubsubSubscription_CloudStorageConfig_TextConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubSubscription_CloudStorageConfig_TextConfig {}

impl BuildPubsubSubscription_CloudStorageConfig_TextConfig {
    pub fn build(self) -> PubsubSubscription_CloudStorageConfig_TextConfig {
        PubsubSubscription_CloudStorageConfig_TextConfig {}
    }
}

#[allow(dead_code)]
pub struct PubsubSubscription_CloudStorageConfig_TextConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubSubscription_CloudStorageConfig_TextConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PubsubSubscription_CloudStorageConfig_TextConfigRef {
        PubsubSubscription_CloudStorageConfig_TextConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubSubscription_CloudStorageConfig_TextConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. Placeholder to allow the empty text_config block."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize, Default)]
struct PubsubSubscription_CloudStorageConfigDynamic {
    avro_config: std::option::Option<terrars::DynamicBlock<PubsubSubscription_CloudStorageConfig_AvroConfig>>,
    text_config: std::option::Option<terrars::DynamicBlock<PubsubSubscription_CloudStorageConfig_TextConfig>>,
}

#[derive(Serialize)]
pub struct PubsubSubscription_CloudStorageConfig {
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filename_datetime_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filename_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filename_suffix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_bytes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_messages: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    avro_config: std::option::Option<std::vec::Vec<PubsubSubscription_CloudStorageConfig_AvroConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_config: std::option::Option<std::vec::Vec<PubsubSubscription_CloudStorageConfig_TextConfig>>,
    dynamic: PubsubSubscription_CloudStorageConfigDynamic,
}

impl PubsubSubscription_CloudStorageConfig {
    #[doc =
        "Set the field `filename_datetime_format`.\nUser-provided format string specifying how to represent datetimes in Cloud Storage filenames."]
    pub fn set_filename_datetime_format(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.filename_datetime_format = std::option::Option::Some(v.into());
        self
    }

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

    #[doc =
        "Set the field `max_messages`.\nThe maximum messages that can be written to a Cloud Storage file before a new file is created. Min 1000 messages."]
    pub fn set_max_messages(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_messages = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_account_email`.\nThe service account to use to write to Cloud Storage. If not specified, the Pub/Sub\n[service agent](https://cloud.google.com/iam/docs/service-agents),\nservice-{project_number}@gcp-sa-pubsub.iam.gserviceaccount.com, is used."]
    pub fn set_service_account_email(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_account_email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `avro_config`.\n"]
    pub fn set_avro_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PubsubSubscription_CloudStorageConfig_AvroConfig>>,
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

    #[doc = "Set the field `text_config`.\n"]
    pub fn set_text_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PubsubSubscription_CloudStorageConfig_TextConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.text_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.text_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PubsubSubscription_CloudStorageConfig {
    type O = terrars::BlockAssignable<PubsubSubscription_CloudStorageConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubSubscription_CloudStorageConfig {
    #[doc =
        "User-provided name for the Cloud Storage bucket. The bucket must be created by the user. The bucket name must be without any prefix like \"gs://\"."]
    pub bucket: terrars::PrimField<std::string::String>,
}

impl BuildPubsubSubscription_CloudStorageConfig {
    pub fn build(self) -> PubsubSubscription_CloudStorageConfig {
        PubsubSubscription_CloudStorageConfig {
            bucket: self.bucket,
            filename_datetime_format: core::default::Default::default(),
            filename_prefix: core::default::Default::default(),
            filename_suffix: core::default::Default::default(),
            max_bytes: core::default::Default::default(),
            max_duration: core::default::Default::default(),
            max_messages: core::default::Default::default(),
            service_account_email: core::default::Default::default(),
            avro_config: core::default::Default::default(),
            text_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PubsubSubscription_CloudStorageConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubSubscription_CloudStorageConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PubsubSubscription_CloudStorageConfigRef {
        PubsubSubscription_CloudStorageConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubSubscription_CloudStorageConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bucket` after provisioning.\nUser-provided name for the Cloud Storage bucket. The bucket must be created by the user. The bucket name must be without any prefix like \"gs://\"."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc =
        "Get a reference to the value of field `filename_datetime_format` after provisioning.\nUser-provided format string specifying how to represent datetimes in Cloud Storage filenames."]
    pub fn filename_datetime_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filename_datetime_format", self.base))
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
        "Get a reference to the value of field `max_messages` after provisioning.\nThe maximum messages that can be written to a Cloud Storage file before a new file is created. Min 1000 messages."]
    pub fn max_messages(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_messages", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account_email` after provisioning.\nThe service account to use to write to Cloud Storage. If not specified, the Pub/Sub\n[service agent](https://cloud.google.com/iam/docs/service-agents),\nservice-{project_number}@gcp-sa-pubsub.iam.gserviceaccount.com, is used."]
    pub fn service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_email", self.base))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nAn output-only field that indicates whether or not the subscription can receive messages."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `avro_config` after provisioning.\n"]
    pub fn avro_config(&self) -> terrars::ListRef<PubsubSubscription_CloudStorageConfig_AvroConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.avro_config", self.base))
    }

    #[doc = "Get a reference to the value of field `text_config` after provisioning.\n"]
    pub fn text_config(&self) -> terrars::ListRef<PubsubSubscription_CloudStorageConfig_TextConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text_config", self.base))
    }
}

#[derive(Serialize)]
pub struct PubsubSubscription_DeadLetterPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dead_letter_topic: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_delivery_attempts: std::option::Option<terrars::PrimField<f64>>,
}

impl PubsubSubscription_DeadLetterPolicy {
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

impl terrars::ToListMappable for PubsubSubscription_DeadLetterPolicy {
    type O = terrars::BlockAssignable<PubsubSubscription_DeadLetterPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubSubscription_DeadLetterPolicy {}

impl BuildPubsubSubscription_DeadLetterPolicy {
    pub fn build(self) -> PubsubSubscription_DeadLetterPolicy {
        PubsubSubscription_DeadLetterPolicy {
            dead_letter_topic: core::default::Default::default(),
            max_delivery_attempts: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PubsubSubscription_DeadLetterPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubSubscription_DeadLetterPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PubsubSubscription_DeadLetterPolicyRef {
        PubsubSubscription_DeadLetterPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubSubscription_DeadLetterPolicyRef {
    #[allow(dead_code)]
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
pub struct PubsubSubscription_ExpirationPolicy {
    ttl: terrars::PrimField<std::string::String>,
}

impl PubsubSubscription_ExpirationPolicy { }

impl terrars::ToListMappable for PubsubSubscription_ExpirationPolicy {
    type O = terrars::BlockAssignable<PubsubSubscription_ExpirationPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubSubscription_ExpirationPolicy {
    #[doc =
        "Specifies the \"time-to-live\" duration for an associated resource. The\nresource expires if it is not active for a period of ttl.\nIf ttl is set to \"\", the associated resource never expires.\nA duration in seconds with up to nine fractional digits, terminated by 's'.\nExample - \"3.5s\"."]
    pub ttl: terrars::PrimField<std::string::String>,
}

impl BuildPubsubSubscription_ExpirationPolicy {
    pub fn build(self) -> PubsubSubscription_ExpirationPolicy {
        PubsubSubscription_ExpirationPolicy { ttl: self.ttl }
    }
}

#[allow(dead_code)]
pub struct PubsubSubscription_ExpirationPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubSubscription_ExpirationPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PubsubSubscription_ExpirationPolicyRef {
        PubsubSubscription_ExpirationPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubSubscription_ExpirationPolicyRef {
    #[allow(dead_code)]
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
pub struct PubsubSubscription_MessageTransforms_AiInference_UnstructuredInference {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl PubsubSubscription_MessageTransforms_AiInference_UnstructuredInference {
    #[doc =
        "Set the field `parameters`.\nA parameters object to be included in each inference request.\nThe parameters object is combined with the data field of the Pub/Sub\nmessage to form the inference request."]
    pub fn set_parameters(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.parameters = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PubsubSubscription_MessageTransforms_AiInference_UnstructuredInference {
    type O = terrars::BlockAssignable<PubsubSubscription_MessageTransforms_AiInference_UnstructuredInference>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubSubscription_MessageTransforms_AiInference_UnstructuredInference {}

impl BuildPubsubSubscription_MessageTransforms_AiInference_UnstructuredInference {
    pub fn build(self) -> PubsubSubscription_MessageTransforms_AiInference_UnstructuredInference {
        PubsubSubscription_MessageTransforms_AiInference_UnstructuredInference {
            parameters: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PubsubSubscription_MessageTransforms_AiInference_UnstructuredInferenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubSubscription_MessageTransforms_AiInference_UnstructuredInferenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PubsubSubscription_MessageTransforms_AiInference_UnstructuredInferenceRef {
        PubsubSubscription_MessageTransforms_AiInference_UnstructuredInferenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubSubscription_MessageTransforms_AiInference_UnstructuredInferenceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `parameters` after provisioning.\nA parameters object to be included in each inference request.\nThe parameters object is combined with the data field of the Pub/Sub\nmessage to form the inference request."]
    pub fn parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }
}

#[derive(Serialize, Default)]
struct PubsubSubscription_MessageTransforms_AiInferenceDynamic {
    unstructured_inference: std
    ::option
    ::Option<terrars::DynamicBlock<PubsubSubscription_MessageTransforms_AiInference_UnstructuredInference>>,
}

#[derive(Serialize)]
pub struct PubsubSubscription_MessageTransforms_AiInference {
    endpoint: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unstructured_inference: std
    ::option
    ::Option<std::vec::Vec<PubsubSubscription_MessageTransforms_AiInference_UnstructuredInference>>,
    dynamic: PubsubSubscription_MessageTransforms_AiInferenceDynamic,
}

impl PubsubSubscription_MessageTransforms_AiInference {
    #[doc =
        "Set the field `service_account_email`.\nThe service account to use to make prediction requests against\nendpoints."]
    pub fn set_service_account_email(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_account_email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unstructured_inference`.\n"]
    pub fn set_unstructured_inference(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<PubsubSubscription_MessageTransforms_AiInference_UnstructuredInference>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.unstructured_inference = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.unstructured_inference = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PubsubSubscription_MessageTransforms_AiInference {
    type O = terrars::BlockAssignable<PubsubSubscription_MessageTransforms_AiInference>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubSubscription_MessageTransforms_AiInference {
    #[doc =
        "The endpoint to a Vertex AI model of the form\n'projects/{project}/locations/{location}/endpoints/{endpoint}' or\n'projects/{project}/locations/{location}/publishers/{publisher}/models/{model}'.\nVertex AI API requests will be sent to this endpoint."]
    pub endpoint: terrars::PrimField<std::string::String>,
}

impl BuildPubsubSubscription_MessageTransforms_AiInference {
    pub fn build(self) -> PubsubSubscription_MessageTransforms_AiInference {
        PubsubSubscription_MessageTransforms_AiInference {
            endpoint: self.endpoint,
            service_account_email: core::default::Default::default(),
            unstructured_inference: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PubsubSubscription_MessageTransforms_AiInferenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubSubscription_MessageTransforms_AiInferenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PubsubSubscription_MessageTransforms_AiInferenceRef {
        PubsubSubscription_MessageTransforms_AiInferenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubSubscription_MessageTransforms_AiInferenceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nThe endpoint to a Vertex AI model of the form\n'projects/{project}/locations/{location}/endpoints/{endpoint}' or\n'projects/{project}/locations/{location}/publishers/{publisher}/models/{model}'.\nVertex AI API requests will be sent to this endpoint."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account_email` after provisioning.\nThe service account to use to make prediction requests against\nendpoints."]
    pub fn service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_email", self.base))
    }

    #[doc = "Get a reference to the value of field `unstructured_inference` after provisioning.\n"]
    pub fn unstructured_inference(
        &self,
    ) -> terrars::ListRef<PubsubSubscription_MessageTransforms_AiInference_UnstructuredInferenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.unstructured_inference", self.base))
    }
}

#[derive(Serialize)]
pub struct PubsubSubscription_MessageTransforms_JavascriptUdf {
    code: terrars::PrimField<std::string::String>,
    function_name: terrars::PrimField<std::string::String>,
}

impl PubsubSubscription_MessageTransforms_JavascriptUdf { }

impl terrars::ToListMappable for PubsubSubscription_MessageTransforms_JavascriptUdf {
    type O = terrars::BlockAssignable<PubsubSubscription_MessageTransforms_JavascriptUdf>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubSubscription_MessageTransforms_JavascriptUdf {
    #[doc =
        "JavaScript code that contains a function 'function_name' with the\nfollowing signature:\n'''\n  /**\n  * Transforms a Pub/Sub message.\n  *\n  * @return {(Object<string, (string | Object<string, string>)>|null)} - To\n  * filter a message, return 'null'. To transform a message return a map\n  * with the following keys:\n  *   - (required) 'data' : {string}\n  *   - (optional) 'attributes' : {Object<string, string>}\n  * Returning empty 'attributes' will remove all attributes from the\n  * message.\n  *\n  * @param  {(Object<string, (string | Object<string, string>)>} Pub/Sub\n  * message. Keys:\n  *   - (required) 'data' : {string}\n  *   - (required) 'attributes' : {Object<string, string>}\n  *\n  * @param  {Object<string, any>} metadata - Pub/Sub message metadata.\n  * Keys:\n  *   - (required) 'message_id'  : {string}\n  *   - (optional) 'publish_time': {string} YYYY-MM-DDTHH:MM:SSZ format\n  *   - (optional) 'ordering_key': {string}\n  */\n  function <function_name>(message, metadata) {\n  }\n'''"]
    pub code: terrars::PrimField<std::string::String>,
    #[doc = "Name of the JavaScript function that should be applied to Pub/Sub messages."]
    pub function_name: terrars::PrimField<std::string::String>,
}

impl BuildPubsubSubscription_MessageTransforms_JavascriptUdf {
    pub fn build(self) -> PubsubSubscription_MessageTransforms_JavascriptUdf {
        PubsubSubscription_MessageTransforms_JavascriptUdf {
            code: self.code,
            function_name: self.function_name,
        }
    }
}

#[allow(dead_code)]
pub struct PubsubSubscription_MessageTransforms_JavascriptUdfRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubSubscription_MessageTransforms_JavascriptUdfRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PubsubSubscription_MessageTransforms_JavascriptUdfRef {
        PubsubSubscription_MessageTransforms_JavascriptUdfRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubSubscription_MessageTransforms_JavascriptUdfRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `code` after provisioning.\nJavaScript code that contains a function 'function_name' with the\nfollowing signature:\n'''\n  /**\n  * Transforms a Pub/Sub message.\n  *\n  * @return {(Object<string, (string | Object<string, string>)>|null)} - To\n  * filter a message, return 'null'. To transform a message return a map\n  * with the following keys:\n  *   - (required) 'data' : {string}\n  *   - (optional) 'attributes' : {Object<string, string>}\n  * Returning empty 'attributes' will remove all attributes from the\n  * message.\n  *\n  * @param  {(Object<string, (string | Object<string, string>)>} Pub/Sub\n  * message. Keys:\n  *   - (required) 'data' : {string}\n  *   - (required) 'attributes' : {Object<string, string>}\n  *\n  * @param  {Object<string, any>} metadata - Pub/Sub message metadata.\n  * Keys:\n  *   - (required) 'message_id'  : {string}\n  *   - (optional) 'publish_time': {string} YYYY-MM-DDTHH:MM:SSZ format\n  *   - (optional) 'ordering_key': {string}\n  */\n  function <function_name>(message, metadata) {\n  }\n'''"]
    pub fn code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `function_name` after provisioning.\nName of the JavaScript function that should be applied to Pub/Sub messages."]
    pub fn function_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct PubsubSubscription_MessageTransformsDynamic {
    ai_inference: std::option::Option<terrars::DynamicBlock<PubsubSubscription_MessageTransforms_AiInference>>,
    javascript_udf: std::option::Option<terrars::DynamicBlock<PubsubSubscription_MessageTransforms_JavascriptUdf>>,
}

#[derive(Serialize)]
pub struct PubsubSubscription_MessageTransforms {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ai_inference: std::option::Option<std::vec::Vec<PubsubSubscription_MessageTransforms_AiInference>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    javascript_udf: std::option::Option<std::vec::Vec<PubsubSubscription_MessageTransforms_JavascriptUdf>>,
    dynamic: PubsubSubscription_MessageTransformsDynamic,
}

impl PubsubSubscription_MessageTransforms {
    #[doc =
        "Set the field `disabled`.\nControls whether or not to use this transform. If not set or 'false',\nthe transform will be applied to messages. Default: 'true'."]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ai_inference`.\n"]
    pub fn set_ai_inference(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PubsubSubscription_MessageTransforms_AiInference>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ai_inference = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ai_inference = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `javascript_udf`.\n"]
    pub fn set_javascript_udf(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PubsubSubscription_MessageTransforms_JavascriptUdf>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.javascript_udf = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.javascript_udf = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PubsubSubscription_MessageTransforms {
    type O = terrars::BlockAssignable<PubsubSubscription_MessageTransforms>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubSubscription_MessageTransforms {}

impl BuildPubsubSubscription_MessageTransforms {
    pub fn build(self) -> PubsubSubscription_MessageTransforms {
        PubsubSubscription_MessageTransforms {
            disabled: core::default::Default::default(),
            ai_inference: core::default::Default::default(),
            javascript_udf: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PubsubSubscription_MessageTransformsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubSubscription_MessageTransformsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PubsubSubscription_MessageTransformsRef {
        PubsubSubscription_MessageTransformsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubSubscription_MessageTransformsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nControls whether or not to use this transform. If not set or 'false',\nthe transform will be applied to messages. Default: 'true'."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `ai_inference` after provisioning.\n"]
    pub fn ai_inference(&self) -> terrars::ListRef<PubsubSubscription_MessageTransforms_AiInferenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ai_inference", self.base))
    }

    #[doc = "Get a reference to the value of field `javascript_udf` after provisioning.\n"]
    pub fn javascript_udf(&self) -> terrars::ListRef<PubsubSubscription_MessageTransforms_JavascriptUdfRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.javascript_udf", self.base))
    }
}

#[derive(Serialize)]
pub struct PubsubSubscription_PushConfig_NoWrapper {
    write_metadata: terrars::PrimField<bool>,
}

impl PubsubSubscription_PushConfig_NoWrapper { }

impl terrars::ToListMappable for PubsubSubscription_PushConfig_NoWrapper {
    type O = terrars::BlockAssignable<PubsubSubscription_PushConfig_NoWrapper>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubSubscription_PushConfig_NoWrapper {
    #[doc =
        "When true, writes the Pub/Sub message metadata to\n'x-goog-pubsub-<KEY>:<VAL>' headers of the HTTP request. Writes the\nPub/Sub message attributes to '<KEY>:<VAL>' headers of the HTTP request."]
    pub write_metadata: terrars::PrimField<bool>,
}

impl BuildPubsubSubscription_PushConfig_NoWrapper {
    pub fn build(self) -> PubsubSubscription_PushConfig_NoWrapper {
        PubsubSubscription_PushConfig_NoWrapper { write_metadata: self.write_metadata }
    }
}

#[allow(dead_code)]
pub struct PubsubSubscription_PushConfig_NoWrapperRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubSubscription_PushConfig_NoWrapperRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PubsubSubscription_PushConfig_NoWrapperRef {
        PubsubSubscription_PushConfig_NoWrapperRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubSubscription_PushConfig_NoWrapperRef {
    #[allow(dead_code)]
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
pub struct PubsubSubscription_PushConfig_OidcToken {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audience: std::option::Option<terrars::PrimField<std::string::String>>,
    service_account_email: terrars::PrimField<std::string::String>,
}

impl PubsubSubscription_PushConfig_OidcToken {
    #[doc =
        "Set the field `audience`.\nAudience to be used when generating OIDC token. The audience claim\nidentifies the recipients that the JWT is intended for. The audience\nvalue is a single case-sensitive string. Having multiple values (array)\nfor the audience field is not supported. More info about the OIDC JWT\ntoken audience here: https://tools.ietf.org/html/rfc7519#section-4.1.3\nNote: if not specified, the Push endpoint URL will be used."]
    pub fn set_audience(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.audience = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PubsubSubscription_PushConfig_OidcToken {
    type O = terrars::BlockAssignable<PubsubSubscription_PushConfig_OidcToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubSubscription_PushConfig_OidcToken {
    #[doc =
        "Service account email to be used for generating the OIDC token.\nThe caller (for subscriptions.create, subscriptions.patch, and\nsubscriptions.modifyPushConfig RPCs) must have the\niam.serviceAccounts.actAs permission for the service account."]
    pub service_account_email: terrars::PrimField<std::string::String>,
}

impl BuildPubsubSubscription_PushConfig_OidcToken {
    pub fn build(self) -> PubsubSubscription_PushConfig_OidcToken {
        PubsubSubscription_PushConfig_OidcToken {
            audience: core::default::Default::default(),
            service_account_email: self.service_account_email,
        }
    }
}

#[allow(dead_code)]
pub struct PubsubSubscription_PushConfig_OidcTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubSubscription_PushConfig_OidcTokenRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PubsubSubscription_PushConfig_OidcTokenRef {
        PubsubSubscription_PushConfig_OidcTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubSubscription_PushConfig_OidcTokenRef {
    #[allow(dead_code)]
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
struct PubsubSubscription_PushConfigDynamic {
    no_wrapper: std::option::Option<terrars::DynamicBlock<PubsubSubscription_PushConfig_NoWrapper>>,
    oidc_token: std::option::Option<terrars::DynamicBlock<PubsubSubscription_PushConfig_OidcToken>>,
}

#[derive(Serialize)]
pub struct PubsubSubscription_PushConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attributes: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    push_endpoint: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_wrapper: std::option::Option<std::vec::Vec<PubsubSubscription_PushConfig_NoWrapper>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oidc_token: std::option::Option<std::vec::Vec<PubsubSubscription_PushConfig_OidcToken>>,
    dynamic: PubsubSubscription_PushConfigDynamic,
}

impl PubsubSubscription_PushConfig {
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
        v: impl std::convert::Into<terrars::BlockAssignable<PubsubSubscription_PushConfig_NoWrapper>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<PubsubSubscription_PushConfig_OidcToken>>,
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

impl terrars::ToListMappable for PubsubSubscription_PushConfig {
    type O = terrars::BlockAssignable<PubsubSubscription_PushConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubSubscription_PushConfig {
    #[doc =
        "A URL locating the endpoint to which messages should be pushed.\nFor example, a Webhook endpoint might use\n\"https://example.com/push\"."]
    pub push_endpoint: terrars::PrimField<std::string::String>,
}

impl BuildPubsubSubscription_PushConfig {
    pub fn build(self) -> PubsubSubscription_PushConfig {
        PubsubSubscription_PushConfig {
            attributes: core::default::Default::default(),
            push_endpoint: self.push_endpoint,
            no_wrapper: core::default::Default::default(),
            oidc_token: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PubsubSubscription_PushConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubSubscription_PushConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PubsubSubscription_PushConfigRef {
        PubsubSubscription_PushConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubSubscription_PushConfigRef {
    #[allow(dead_code)]
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
    pub fn no_wrapper(&self) -> terrars::ListRef<PubsubSubscription_PushConfig_NoWrapperRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.no_wrapper", self.base))
    }

    #[doc = "Get a reference to the value of field `oidc_token` after provisioning.\n"]
    pub fn oidc_token(&self) -> terrars::ListRef<PubsubSubscription_PushConfig_OidcTokenRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oidc_token", self.base))
    }
}

#[derive(Serialize)]
pub struct PubsubSubscription_RetryPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_backoff: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum_backoff: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PubsubSubscription_RetryPolicy {
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

impl terrars::ToListMappable for PubsubSubscription_RetryPolicy {
    type O = terrars::BlockAssignable<PubsubSubscription_RetryPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubSubscription_RetryPolicy {}

impl BuildPubsubSubscription_RetryPolicy {
    pub fn build(self) -> PubsubSubscription_RetryPolicy {
        PubsubSubscription_RetryPolicy {
            maximum_backoff: core::default::Default::default(),
            minimum_backoff: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PubsubSubscription_RetryPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubSubscription_RetryPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PubsubSubscription_RetryPolicyRef {
        PubsubSubscription_RetryPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubSubscription_RetryPolicyRef {
    #[allow(dead_code)]
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
pub struct PubsubSubscription_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PubsubSubscription_Timeouts {
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

impl terrars::ToListMappable for PubsubSubscription_Timeouts {
    type O = terrars::BlockAssignable<PubsubSubscription_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubSubscription_Timeouts {}

impl BuildPubsubSubscription_Timeouts {
    pub fn build(self) -> PubsubSubscription_Timeouts {
        PubsubSubscription_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PubsubSubscription_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubSubscription_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PubsubSubscription_TimeoutsRef {
        PubsubSubscription_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubSubscription_TimeoutsRef {
    #[allow(dead_code)]
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
