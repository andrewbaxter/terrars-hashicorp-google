#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataPubsubSubscriptionData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataPubsubSubscription_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataPubsubSubscriptionData_>,
}

#[derive(Clone)]
pub struct DataPubsubSubscription(std::rc::Rc<DataPubsubSubscription_>);

impl DataPubsubSubscription {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `ack_deadline_seconds` after provisioning.\nThis value is the maximum time after a subscriber receives a message\nbefore the subscriber should acknowledge the message. After message\ndelivery but before the ack deadline expires and before the message is\nacknowledged, it is an outstanding message and will not be delivered\nagain during that time (on a best-effort basis).\n\nFor pull subscriptions, this value is used as the initial value for\nthe ack deadline. To override this value for a given message, call\nsubscriptions.modifyAckDeadline with the corresponding ackId if using\npull. The minimum custom deadline you can specify is 10 seconds. The\nmaximum custom deadline you can specify is 600 seconds (10 minutes).\nIf this parameter is 0, a default value of 10 seconds is used.\n\nFor push delivery, this value is also used to set the request timeout\nfor the call to the push endpoint.\n\nIf the subscriber never acknowledges the message, the Pub/Sub system\nwill eventually redeliver the message."]
    pub fn ack_deadline_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ack_deadline_seconds", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bigquery_config` after provisioning.\nIf delivery to BigQuery is used with this subscription, this field is used to configure it.\nOnly one of pushConfig, bigqueryConfig, cloudStorageConfig, or bigtableConfig can be set.\nIf none of them are set, then the subscriber will pull and ack messages using API methods."]
    pub fn bigquery_config(&self) -> terrars::ListRef<DataPubsubSubscription_BigqueryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigquery_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bigtable_config` after provisioning.\nIf delivery to Bigtable is used with this subscription, this field is used to configure it.\nOnly one of pushConfig, bigqueryConfig, cloudStorageConfig, or bigtableConfig can be set.\nIf none of them are set, then the subscriber will pull and ack messages using API methods."]
    pub fn bigtable_config(&self) -> terrars::ListRef<DataPubsubSubscription_BigtableConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigtable_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cloud_storage_config` after provisioning.\nIf delivery to Cloud Storage is used with this subscription, this field is used to configure it.\nOnly one of pushConfig, bigqueryConfig, cloudStorageConfig, or bigtableConfig can be set.\nIf none of them are set, then the subscriber will pull and ack messages using API methods."]
    pub fn cloud_storage_config(&self) -> terrars::ListRef<DataPubsubSubscription_CloudStorageConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_storage_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dead_letter_policy` after provisioning.\nA policy that specifies the conditions for dead lettering messages in\nthis subscription. If dead_letter_policy is not set, dead lettering\nis disabled.\n\nThe Cloud Pub/Sub service account associated with this subscription's\nparent project (i.e.,\nservice-{project_number}@gcp-sa-pubsub.iam.gserviceaccount.com) must have\npermission to Acknowledge() messages on this subscription."]
    pub fn dead_letter_policy(&self) -> terrars::ListRef<DataPubsubSubscription_DeadLetterPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dead_letter_policy", self.extract_ref()))
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
        "Get a reference to the value of field `expiration_policy` after provisioning.\nA policy that specifies the conditions for this subscription's expiration.\nA subscription is considered active as long as any connected subscriber\nis successfully consuming messages from the subscription or is issuing\noperations on the subscription. If expirationPolicy is not set, a default\npolicy with ttl of 31 days will be used.  If it is set but ttl is \"\", the\nresource never expires.  The minimum allowed value for expirationPolicy.ttl\nis 1 day."]
    pub fn expiration_policy(&self) -> terrars::ListRef<DataPubsubSubscription_ExpirationPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expiration_policy", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `message_transforms` after provisioning.\nTransforms to be applied to messages published to the topic. Transforms are applied in the\norder specified."]
    pub fn message_transforms(&self) -> terrars::ListRef<DataPubsubSubscription_MessageTransformsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.message_transforms", self.extract_ref()))
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
        "Get a reference to the value of field `push_config` after provisioning.\nIf push delivery is used with this subscription, this field is used to\nconfigure it. An empty pushConfig signifies that the subscriber will\npull and ack messages using API methods."]
    pub fn push_config(&self) -> terrars::ListRef<DataPubsubSubscription_PushConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.push_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `retain_acked_messages` after provisioning.\nIndicates whether to retain acknowledged messages. If 'true', then\nmessages are not expunged from the subscription's backlog, even if\nthey are acknowledged, until they fall out of the\nmessageRetentionDuration window."]
    pub fn retain_acked_messages(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retain_acked_messages", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `retry_policy` after provisioning.\nA policy that specifies how Pub/Sub retries message delivery for this subscription.\n\nIf not set, the default retry policy is applied. This generally implies that messages will be retried as soon as possible for healthy subscribers.\nRetryPolicy will be triggered on NACKs or acknowledgement deadline exceeded events for a given message"]
    pub fn retry_policy(&self) -> terrars::ListRef<DataPubsubSubscription_RetryPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.extract_ref()))
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
}

impl terrars::Referable for DataPubsubSubscription {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataPubsubSubscription { }

impl terrars::ToListMappable for DataPubsubSubscription {
    type O = terrars::ListRef<DataPubsubSubscriptionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataPubsubSubscription_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_pubsub_subscription".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataPubsubSubscription {
    pub tf_id: std::string::String,
    #[doc = "Name of the subscription."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataPubsubSubscription {
    pub fn build(self, stack: &mut terrars::Stack) -> DataPubsubSubscription {
        let out = DataPubsubSubscription(std::rc::Rc::new(DataPubsubSubscription_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataPubsubSubscriptionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataPubsubSubscriptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPubsubSubscriptionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataPubsubSubscriptionRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `ack_deadline_seconds` after provisioning.\nThis value is the maximum time after a subscriber receives a message\nbefore the subscriber should acknowledge the message. After message\ndelivery but before the ack deadline expires and before the message is\nacknowledged, it is an outstanding message and will not be delivered\nagain during that time (on a best-effort basis).\n\nFor pull subscriptions, this value is used as the initial value for\nthe ack deadline. To override this value for a given message, call\nsubscriptions.modifyAckDeadline with the corresponding ackId if using\npull. The minimum custom deadline you can specify is 10 seconds. The\nmaximum custom deadline you can specify is 600 seconds (10 minutes).\nIf this parameter is 0, a default value of 10 seconds is used.\n\nFor push delivery, this value is also used to set the request timeout\nfor the call to the push endpoint.\n\nIf the subscriber never acknowledges the message, the Pub/Sub system\nwill eventually redeliver the message."]
    pub fn ack_deadline_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ack_deadline_seconds", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bigquery_config` after provisioning.\nIf delivery to BigQuery is used with this subscription, this field is used to configure it.\nOnly one of pushConfig, bigqueryConfig, cloudStorageConfig, or bigtableConfig can be set.\nIf none of them are set, then the subscriber will pull and ack messages using API methods."]
    pub fn bigquery_config(&self) -> terrars::ListRef<DataPubsubSubscription_BigqueryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigquery_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bigtable_config` after provisioning.\nIf delivery to Bigtable is used with this subscription, this field is used to configure it.\nOnly one of pushConfig, bigqueryConfig, cloudStorageConfig, or bigtableConfig can be set.\nIf none of them are set, then the subscriber will pull and ack messages using API methods."]
    pub fn bigtable_config(&self) -> terrars::ListRef<DataPubsubSubscription_BigtableConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigtable_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cloud_storage_config` after provisioning.\nIf delivery to Cloud Storage is used with this subscription, this field is used to configure it.\nOnly one of pushConfig, bigqueryConfig, cloudStorageConfig, or bigtableConfig can be set.\nIf none of them are set, then the subscriber will pull and ack messages using API methods."]
    pub fn cloud_storage_config(&self) -> terrars::ListRef<DataPubsubSubscription_CloudStorageConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_storage_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dead_letter_policy` after provisioning.\nA policy that specifies the conditions for dead lettering messages in\nthis subscription. If dead_letter_policy is not set, dead lettering\nis disabled.\n\nThe Cloud Pub/Sub service account associated with this subscription's\nparent project (i.e.,\nservice-{project_number}@gcp-sa-pubsub.iam.gserviceaccount.com) must have\npermission to Acknowledge() messages on this subscription."]
    pub fn dead_letter_policy(&self) -> terrars::ListRef<DataPubsubSubscription_DeadLetterPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dead_letter_policy", self.extract_ref()))
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
        "Get a reference to the value of field `expiration_policy` after provisioning.\nA policy that specifies the conditions for this subscription's expiration.\nA subscription is considered active as long as any connected subscriber\nis successfully consuming messages from the subscription or is issuing\noperations on the subscription. If expirationPolicy is not set, a default\npolicy with ttl of 31 days will be used.  If it is set but ttl is \"\", the\nresource never expires.  The minimum allowed value for expirationPolicy.ttl\nis 1 day."]
    pub fn expiration_policy(&self) -> terrars::ListRef<DataPubsubSubscription_ExpirationPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expiration_policy", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `message_transforms` after provisioning.\nTransforms to be applied to messages published to the topic. Transforms are applied in the\norder specified."]
    pub fn message_transforms(&self) -> terrars::ListRef<DataPubsubSubscription_MessageTransformsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.message_transforms", self.extract_ref()))
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
        "Get a reference to the value of field `push_config` after provisioning.\nIf push delivery is used with this subscription, this field is used to\nconfigure it. An empty pushConfig signifies that the subscriber will\npull and ack messages using API methods."]
    pub fn push_config(&self) -> terrars::ListRef<DataPubsubSubscription_PushConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.push_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `retain_acked_messages` after provisioning.\nIndicates whether to retain acknowledged messages. If 'true', then\nmessages are not expunged from the subscription's backlog, even if\nthey are acknowledged, until they fall out of the\nmessageRetentionDuration window."]
    pub fn retain_acked_messages(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retain_acked_messages", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `retry_policy` after provisioning.\nA policy that specifies how Pub/Sub retries message delivery for this subscription.\n\nIf not set, the default retry policy is applied. This generally implies that messages will be retried as soon as possible for healthy subscribers.\nRetryPolicy will be triggered on NACKs or acknowledgement deadline exceeded events for a given message"]
    pub fn retry_policy(&self) -> terrars::ListRef<DataPubsubSubscription_RetryPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.extract_ref()))
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
}

#[derive(Serialize)]
pub struct DataPubsubSubscription_BigqueryConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    drop_unknown_fields: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_table_schema: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_topic_schema: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    write_metadata: std::option::Option<terrars::PrimField<bool>>,
}

impl DataPubsubSubscription_BigqueryConfig {
    #[doc = "Set the field `drop_unknown_fields`.\n"]
    pub fn set_drop_unknown_fields(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.drop_unknown_fields = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_account_email`.\n"]
    pub fn set_service_account_email(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_account_email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `table`.\n"]
    pub fn set_table(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.table = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_table_schema`.\n"]
    pub fn set_use_table_schema(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_table_schema = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_topic_schema`.\n"]
    pub fn set_use_topic_schema(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_topic_schema = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `write_metadata`.\n"]
    pub fn set_write_metadata(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.write_metadata = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPubsubSubscription_BigqueryConfig {
    type O = terrars::BlockAssignable<DataPubsubSubscription_BigqueryConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPubsubSubscription_BigqueryConfig {}

impl BuildDataPubsubSubscription_BigqueryConfig {
    pub fn build(self) -> DataPubsubSubscription_BigqueryConfig {
        DataPubsubSubscription_BigqueryConfig {
            drop_unknown_fields: core::default::Default::default(),
            service_account_email: core::default::Default::default(),
            table: core::default::Default::default(),
            use_table_schema: core::default::Default::default(),
            use_topic_schema: core::default::Default::default(),
            write_metadata: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPubsubSubscription_BigqueryConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPubsubSubscription_BigqueryConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPubsubSubscription_BigqueryConfigRef {
        DataPubsubSubscription_BigqueryConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPubsubSubscription_BigqueryConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `drop_unknown_fields` after provisioning.\n"]
    pub fn drop_unknown_fields(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.drop_unknown_fields", self.base))
    }

    #[doc = "Get a reference to the value of field `service_account_email` after provisioning.\n"]
    pub fn service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_email", self.base))
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\n"]
    pub fn table(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }

    #[doc = "Get a reference to the value of field `use_table_schema` after provisioning.\n"]
    pub fn use_table_schema(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_table_schema", self.base))
    }

    #[doc = "Get a reference to the value of field `use_topic_schema` after provisioning.\n"]
    pub fn use_topic_schema(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_topic_schema", self.base))
    }

    #[doc = "Get a reference to the value of field `write_metadata` after provisioning.\n"]
    pub fn write_metadata(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write_metadata", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPubsubSubscription_BigtableConfig_ColumnFamilyMapping_DelimitedKey {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delimiter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_fields: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataPubsubSubscription_BigtableConfig_ColumnFamilyMapping_DelimitedKey {
    #[doc = "Set the field `delimiter`.\n"]
    pub fn set_delimiter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delimiter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_fields`.\n"]
    pub fn set_key_fields(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.key_fields = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPubsubSubscription_BigtableConfig_ColumnFamilyMapping_DelimitedKey {
    type O = terrars::BlockAssignable<DataPubsubSubscription_BigtableConfig_ColumnFamilyMapping_DelimitedKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPubsubSubscription_BigtableConfig_ColumnFamilyMapping_DelimitedKey {}

impl BuildDataPubsubSubscription_BigtableConfig_ColumnFamilyMapping_DelimitedKey {
    pub fn build(self) -> DataPubsubSubscription_BigtableConfig_ColumnFamilyMapping_DelimitedKey {
        DataPubsubSubscription_BigtableConfig_ColumnFamilyMapping_DelimitedKey {
            delimiter: core::default::Default::default(),
            key_fields: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPubsubSubscription_BigtableConfig_ColumnFamilyMapping_DelimitedKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPubsubSubscription_BigtableConfig_ColumnFamilyMapping_DelimitedKeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPubsubSubscription_BigtableConfig_ColumnFamilyMapping_DelimitedKeyRef {
        DataPubsubSubscription_BigtableConfig_ColumnFamilyMapping_DelimitedKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPubsubSubscription_BigtableConfig_ColumnFamilyMapping_DelimitedKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delimiter` after provisioning.\n"]
    pub fn delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delimiter", self.base))
    }

    #[doc = "Get a reference to the value of field `key_fields` after provisioning.\n"]
    pub fn key_fields(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.key_fields", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPubsubSubscription_BigtableConfig_ColumnFamilyMapping_RowKeySchema {}

impl DataPubsubSubscription_BigtableConfig_ColumnFamilyMapping_RowKeySchema { }

impl terrars::ToListMappable for DataPubsubSubscription_BigtableConfig_ColumnFamilyMapping_RowKeySchema {
    type O = terrars::BlockAssignable<DataPubsubSubscription_BigtableConfig_ColumnFamilyMapping_RowKeySchema>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPubsubSubscription_BigtableConfig_ColumnFamilyMapping_RowKeySchema {}

impl BuildDataPubsubSubscription_BigtableConfig_ColumnFamilyMapping_RowKeySchema {
    pub fn build(self) -> DataPubsubSubscription_BigtableConfig_ColumnFamilyMapping_RowKeySchema {
        DataPubsubSubscription_BigtableConfig_ColumnFamilyMapping_RowKeySchema {}
    }
}

#[allow(dead_code)]
pub struct DataPubsubSubscription_BigtableConfig_ColumnFamilyMapping_RowKeySchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPubsubSubscription_BigtableConfig_ColumnFamilyMapping_RowKeySchemaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPubsubSubscription_BigtableConfig_ColumnFamilyMapping_RowKeySchemaRef {
        DataPubsubSubscription_BigtableConfig_ColumnFamilyMapping_RowKeySchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPubsubSubscription_BigtableConfig_ColumnFamilyMapping_RowKeySchemaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataPubsubSubscription_BigtableConfig_ColumnFamilyMapping {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delimited_key: std
    ::option
    ::Option<terrars::ListField<DataPubsubSubscription_BigtableConfig_ColumnFamilyMapping_DelimitedKey>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    row_key_schema: std
    ::option
    ::Option<terrars::ListField<DataPubsubSubscription_BigtableConfig_ColumnFamilyMapping_RowKeySchema>>,
}

impl DataPubsubSubscription_BigtableConfig_ColumnFamilyMapping {
    #[doc = "Set the field `delimited_key`.\n"]
    pub fn set_delimited_key(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataPubsubSubscription_BigtableConfig_ColumnFamilyMapping_DelimitedKey>,
                    >,
    ) -> Self {
        self.delimited_key = std::option::Option::Some(v.into());
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
                        terrars::ListField<DataPubsubSubscription_BigtableConfig_ColumnFamilyMapping_RowKeySchema>,
                    >,
    ) -> Self {
        self.row_key_schema = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPubsubSubscription_BigtableConfig_ColumnFamilyMapping {
    type O = terrars::BlockAssignable<DataPubsubSubscription_BigtableConfig_ColumnFamilyMapping>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPubsubSubscription_BigtableConfig_ColumnFamilyMapping {}

impl BuildDataPubsubSubscription_BigtableConfig_ColumnFamilyMapping {
    pub fn build(self) -> DataPubsubSubscription_BigtableConfig_ColumnFamilyMapping {
        DataPubsubSubscription_BigtableConfig_ColumnFamilyMapping {
            delimited_key: core::default::Default::default(),
            row_key_schema: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPubsubSubscription_BigtableConfig_ColumnFamilyMappingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPubsubSubscription_BigtableConfig_ColumnFamilyMappingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPubsubSubscription_BigtableConfig_ColumnFamilyMappingRef {
        DataPubsubSubscription_BigtableConfig_ColumnFamilyMappingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPubsubSubscription_BigtableConfig_ColumnFamilyMappingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delimited_key` after provisioning.\n"]
    pub fn delimited_key(
        &self,
    ) -> terrars::ListRef<DataPubsubSubscription_BigtableConfig_ColumnFamilyMapping_DelimitedKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.delimited_key", self.base))
    }

    #[doc = "Get a reference to the value of field `row_key_schema` after provisioning.\n"]
    pub fn row_key_schema(
        &self,
    ) -> terrars::ListRef<DataPubsubSubscription_BigtableConfig_ColumnFamilyMapping_RowKeySchemaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.row_key_schema", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPubsubSubscription_BigtableConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    app_profile_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column_family_mapping: std
    ::option
    ::Option<terrars::ListField<DataPubsubSubscription_BigtableConfig_ColumnFamilyMapping>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    write_metadata: std::option::Option<terrars::PrimField<bool>>,
}

impl DataPubsubSubscription_BigtableConfig {
    #[doc = "Set the field `app_profile_id`.\n"]
    pub fn set_app_profile_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.app_profile_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `column_family_mapping`.\n"]
    pub fn set_column_family_mapping(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataPubsubSubscription_BigtableConfig_ColumnFamilyMapping>>,
    ) -> Self {
        self.column_family_mapping = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_account_email`.\n"]
    pub fn set_service_account_email(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_account_email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `table`.\n"]
    pub fn set_table(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.table = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `write_metadata`.\n"]
    pub fn set_write_metadata(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.write_metadata = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPubsubSubscription_BigtableConfig {
    type O = terrars::BlockAssignable<DataPubsubSubscription_BigtableConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPubsubSubscription_BigtableConfig {}

impl BuildDataPubsubSubscription_BigtableConfig {
    pub fn build(self) -> DataPubsubSubscription_BigtableConfig {
        DataPubsubSubscription_BigtableConfig {
            app_profile_id: core::default::Default::default(),
            column_family_mapping: core::default::Default::default(),
            service_account_email: core::default::Default::default(),
            state: core::default::Default::default(),
            table: core::default::Default::default(),
            write_metadata: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPubsubSubscription_BigtableConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPubsubSubscription_BigtableConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPubsubSubscription_BigtableConfigRef {
        DataPubsubSubscription_BigtableConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPubsubSubscription_BigtableConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app_profile_id` after provisioning.\n"]
    pub fn app_profile_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_profile_id", self.base))
    }

    #[doc = "Get a reference to the value of field `column_family_mapping` after provisioning.\n"]
    pub fn column_family_mapping(
        &self,
    ) -> terrars::ListRef<DataPubsubSubscription_BigtableConfig_ColumnFamilyMappingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.column_family_mapping", self.base))
    }

    #[doc = "Get a reference to the value of field `service_account_email` after provisioning.\n"]
    pub fn service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_email", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\n"]
    pub fn table(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }

    #[doc = "Get a reference to the value of field `write_metadata` after provisioning.\n"]
    pub fn write_metadata(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write_metadata", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPubsubSubscription_CloudStorageConfig_AvroConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_topic_schema: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    write_metadata: std::option::Option<terrars::PrimField<bool>>,
}

impl DataPubsubSubscription_CloudStorageConfig_AvroConfig {
    #[doc = "Set the field `use_topic_schema`.\n"]
    pub fn set_use_topic_schema(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_topic_schema = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `write_metadata`.\n"]
    pub fn set_write_metadata(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.write_metadata = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPubsubSubscription_CloudStorageConfig_AvroConfig {
    type O = terrars::BlockAssignable<DataPubsubSubscription_CloudStorageConfig_AvroConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPubsubSubscription_CloudStorageConfig_AvroConfig {}

impl BuildDataPubsubSubscription_CloudStorageConfig_AvroConfig {
    pub fn build(self) -> DataPubsubSubscription_CloudStorageConfig_AvroConfig {
        DataPubsubSubscription_CloudStorageConfig_AvroConfig {
            use_topic_schema: core::default::Default::default(),
            write_metadata: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPubsubSubscription_CloudStorageConfig_AvroConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPubsubSubscription_CloudStorageConfig_AvroConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPubsubSubscription_CloudStorageConfig_AvroConfigRef {
        DataPubsubSubscription_CloudStorageConfig_AvroConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPubsubSubscription_CloudStorageConfig_AvroConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `use_topic_schema` after provisioning.\n"]
    pub fn use_topic_schema(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_topic_schema", self.base))
    }

    #[doc = "Get a reference to the value of field `write_metadata` after provisioning.\n"]
    pub fn write_metadata(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write_metadata", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPubsubSubscription_CloudStorageConfig_TextConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataPubsubSubscription_CloudStorageConfig_TextConfig {
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPubsubSubscription_CloudStorageConfig_TextConfig {
    type O = terrars::BlockAssignable<DataPubsubSubscription_CloudStorageConfig_TextConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPubsubSubscription_CloudStorageConfig_TextConfig {}

impl BuildDataPubsubSubscription_CloudStorageConfig_TextConfig {
    pub fn build(self) -> DataPubsubSubscription_CloudStorageConfig_TextConfig {
        DataPubsubSubscription_CloudStorageConfig_TextConfig { state: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataPubsubSubscription_CloudStorageConfig_TextConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPubsubSubscription_CloudStorageConfig_TextConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPubsubSubscription_CloudStorageConfig_TextConfigRef {
        DataPubsubSubscription_CloudStorageConfig_TextConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPubsubSubscription_CloudStorageConfig_TextConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPubsubSubscription_CloudStorageConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    avro_config: std::option::Option<terrars::ListField<DataPubsubSubscription_CloudStorageConfig_AvroConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
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
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_config: std::option::Option<terrars::ListField<DataPubsubSubscription_CloudStorageConfig_TextConfig>>,
}

impl DataPubsubSubscription_CloudStorageConfig {
    #[doc = "Set the field `avro_config`.\n"]
    pub fn set_avro_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataPubsubSubscription_CloudStorageConfig_AvroConfig>>,
    ) -> Self {
        self.avro_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filename_datetime_format`.\n"]
    pub fn set_filename_datetime_format(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.filename_datetime_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filename_prefix`.\n"]
    pub fn set_filename_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.filename_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filename_suffix`.\n"]
    pub fn set_filename_suffix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.filename_suffix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_bytes`.\n"]
    pub fn set_max_bytes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_duration`.\n"]
    pub fn set_max_duration(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_messages`.\n"]
    pub fn set_max_messages(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_messages = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_account_email`.\n"]
    pub fn set_service_account_email(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_account_email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `text_config`.\n"]
    pub fn set_text_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataPubsubSubscription_CloudStorageConfig_TextConfig>>,
    ) -> Self {
        self.text_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPubsubSubscription_CloudStorageConfig {
    type O = terrars::BlockAssignable<DataPubsubSubscription_CloudStorageConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPubsubSubscription_CloudStorageConfig {}

impl BuildDataPubsubSubscription_CloudStorageConfig {
    pub fn build(self) -> DataPubsubSubscription_CloudStorageConfig {
        DataPubsubSubscription_CloudStorageConfig {
            avro_config: core::default::Default::default(),
            bucket: core::default::Default::default(),
            filename_datetime_format: core::default::Default::default(),
            filename_prefix: core::default::Default::default(),
            filename_suffix: core::default::Default::default(),
            max_bytes: core::default::Default::default(),
            max_duration: core::default::Default::default(),
            max_messages: core::default::Default::default(),
            service_account_email: core::default::Default::default(),
            state: core::default::Default::default(),
            text_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPubsubSubscription_CloudStorageConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPubsubSubscription_CloudStorageConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPubsubSubscription_CloudStorageConfigRef {
        DataPubsubSubscription_CloudStorageConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPubsubSubscription_CloudStorageConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `avro_config` after provisioning.\n"]
    pub fn avro_config(&self) -> terrars::ListRef<DataPubsubSubscription_CloudStorageConfig_AvroConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.avro_config", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `filename_datetime_format` after provisioning.\n"]
    pub fn filename_datetime_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filename_datetime_format", self.base))
    }

    #[doc = "Get a reference to the value of field `filename_prefix` after provisioning.\n"]
    pub fn filename_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filename_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `filename_suffix` after provisioning.\n"]
    pub fn filename_suffix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filename_suffix", self.base))
    }

    #[doc = "Get a reference to the value of field `max_bytes` after provisioning.\n"]
    pub fn max_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_bytes", self.base))
    }

    #[doc = "Get a reference to the value of field `max_duration` after provisioning.\n"]
    pub fn max_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `max_messages` after provisioning.\n"]
    pub fn max_messages(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_messages", self.base))
    }

    #[doc = "Get a reference to the value of field `service_account_email` after provisioning.\n"]
    pub fn service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_email", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `text_config` after provisioning.\n"]
    pub fn text_config(&self) -> terrars::ListRef<DataPubsubSubscription_CloudStorageConfig_TextConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPubsubSubscription_DeadLetterPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dead_letter_topic: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_delivery_attempts: std::option::Option<terrars::PrimField<f64>>,
}

impl DataPubsubSubscription_DeadLetterPolicy {
    #[doc = "Set the field `dead_letter_topic`.\n"]
    pub fn set_dead_letter_topic(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dead_letter_topic = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_delivery_attempts`.\n"]
    pub fn set_max_delivery_attempts(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_delivery_attempts = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPubsubSubscription_DeadLetterPolicy {
    type O = terrars::BlockAssignable<DataPubsubSubscription_DeadLetterPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPubsubSubscription_DeadLetterPolicy {}

impl BuildDataPubsubSubscription_DeadLetterPolicy {
    pub fn build(self) -> DataPubsubSubscription_DeadLetterPolicy {
        DataPubsubSubscription_DeadLetterPolicy {
            dead_letter_topic: core::default::Default::default(),
            max_delivery_attempts: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPubsubSubscription_DeadLetterPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPubsubSubscription_DeadLetterPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPubsubSubscription_DeadLetterPolicyRef {
        DataPubsubSubscription_DeadLetterPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPubsubSubscription_DeadLetterPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dead_letter_topic` after provisioning.\n"]
    pub fn dead_letter_topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dead_letter_topic", self.base))
    }

    #[doc = "Get a reference to the value of field `max_delivery_attempts` after provisioning.\n"]
    pub fn max_delivery_attempts(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_delivery_attempts", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPubsubSubscription_ExpirationPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ttl: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataPubsubSubscription_ExpirationPolicy {
    #[doc = "Set the field `ttl`.\n"]
    pub fn set_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ttl = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPubsubSubscription_ExpirationPolicy {
    type O = terrars::BlockAssignable<DataPubsubSubscription_ExpirationPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPubsubSubscription_ExpirationPolicy {}

impl BuildDataPubsubSubscription_ExpirationPolicy {
    pub fn build(self) -> DataPubsubSubscription_ExpirationPolicy {
        DataPubsubSubscription_ExpirationPolicy { ttl: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataPubsubSubscription_ExpirationPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPubsubSubscription_ExpirationPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPubsubSubscription_ExpirationPolicyRef {
        DataPubsubSubscription_ExpirationPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPubsubSubscription_ExpirationPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPubsubSubscription_MessageTransforms_AiInference_UnstructuredInference {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataPubsubSubscription_MessageTransforms_AiInference_UnstructuredInference {
    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.parameters = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPubsubSubscription_MessageTransforms_AiInference_UnstructuredInference {
    type O = terrars::BlockAssignable<DataPubsubSubscription_MessageTransforms_AiInference_UnstructuredInference>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPubsubSubscription_MessageTransforms_AiInference_UnstructuredInference {}

impl BuildDataPubsubSubscription_MessageTransforms_AiInference_UnstructuredInference {
    pub fn build(self) -> DataPubsubSubscription_MessageTransforms_AiInference_UnstructuredInference {
        DataPubsubSubscription_MessageTransforms_AiInference_UnstructuredInference {
            parameters: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPubsubSubscription_MessageTransforms_AiInference_UnstructuredInferenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPubsubSubscription_MessageTransforms_AiInference_UnstructuredInferenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPubsubSubscription_MessageTransforms_AiInference_UnstructuredInferenceRef {
        DataPubsubSubscription_MessageTransforms_AiInference_UnstructuredInferenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPubsubSubscription_MessageTransforms_AiInference_UnstructuredInferenceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPubsubSubscription_MessageTransforms_AiInference {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unstructured_inference: std
    ::option
    ::Option<terrars::ListField<DataPubsubSubscription_MessageTransforms_AiInference_UnstructuredInference>>,
}

impl DataPubsubSubscription_MessageTransforms_AiInference {
    #[doc = "Set the field `endpoint`.\n"]
    pub fn set_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_account_email`.\n"]
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
                        terrars::ListField<DataPubsubSubscription_MessageTransforms_AiInference_UnstructuredInference>,
                    >,
    ) -> Self {
        self.unstructured_inference = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPubsubSubscription_MessageTransforms_AiInference {
    type O = terrars::BlockAssignable<DataPubsubSubscription_MessageTransforms_AiInference>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPubsubSubscription_MessageTransforms_AiInference {}

impl BuildDataPubsubSubscription_MessageTransforms_AiInference {
    pub fn build(self) -> DataPubsubSubscription_MessageTransforms_AiInference {
        DataPubsubSubscription_MessageTransforms_AiInference {
            endpoint: core::default::Default::default(),
            service_account_email: core::default::Default::default(),
            unstructured_inference: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPubsubSubscription_MessageTransforms_AiInferenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPubsubSubscription_MessageTransforms_AiInferenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPubsubSubscription_MessageTransforms_AiInferenceRef {
        DataPubsubSubscription_MessageTransforms_AiInferenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPubsubSubscription_MessageTransforms_AiInferenceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `service_account_email` after provisioning.\n"]
    pub fn service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_email", self.base))
    }

    #[doc = "Get a reference to the value of field `unstructured_inference` after provisioning.\n"]
    pub fn unstructured_inference(
        &self,
    ) -> terrars::ListRef<DataPubsubSubscription_MessageTransforms_AiInference_UnstructuredInferenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.unstructured_inference", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPubsubSubscription_MessageTransforms_JavascriptUdf {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    function_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataPubsubSubscription_MessageTransforms_JavascriptUdf {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `function_name`.\n"]
    pub fn set_function_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.function_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPubsubSubscription_MessageTransforms_JavascriptUdf {
    type O = terrars::BlockAssignable<DataPubsubSubscription_MessageTransforms_JavascriptUdf>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPubsubSubscription_MessageTransforms_JavascriptUdf {}

impl BuildDataPubsubSubscription_MessageTransforms_JavascriptUdf {
    pub fn build(self) -> DataPubsubSubscription_MessageTransforms_JavascriptUdf {
        DataPubsubSubscription_MessageTransforms_JavascriptUdf {
            code: core::default::Default::default(),
            function_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPubsubSubscription_MessageTransforms_JavascriptUdfRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPubsubSubscription_MessageTransforms_JavascriptUdfRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPubsubSubscription_MessageTransforms_JavascriptUdfRef {
        DataPubsubSubscription_MessageTransforms_JavascriptUdfRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPubsubSubscription_MessageTransforms_JavascriptUdfRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `function_name` after provisioning.\n"]
    pub fn function_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPubsubSubscription_MessageTransforms {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ai_inference: std::option::Option<terrars::ListField<DataPubsubSubscription_MessageTransforms_AiInference>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    javascript_udf: std::option::Option<terrars::ListField<DataPubsubSubscription_MessageTransforms_JavascriptUdf>>,
}

impl DataPubsubSubscription_MessageTransforms {
    #[doc = "Set the field `ai_inference`.\n"]
    pub fn set_ai_inference(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataPubsubSubscription_MessageTransforms_AiInference>>,
    ) -> Self {
        self.ai_inference = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `javascript_udf`.\n"]
    pub fn set_javascript_udf(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataPubsubSubscription_MessageTransforms_JavascriptUdf>>,
    ) -> Self {
        self.javascript_udf = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPubsubSubscription_MessageTransforms {
    type O = terrars::BlockAssignable<DataPubsubSubscription_MessageTransforms>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPubsubSubscription_MessageTransforms {}

impl BuildDataPubsubSubscription_MessageTransforms {
    pub fn build(self) -> DataPubsubSubscription_MessageTransforms {
        DataPubsubSubscription_MessageTransforms {
            ai_inference: core::default::Default::default(),
            disabled: core::default::Default::default(),
            javascript_udf: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPubsubSubscription_MessageTransformsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPubsubSubscription_MessageTransformsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPubsubSubscription_MessageTransformsRef {
        DataPubsubSubscription_MessageTransformsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPubsubSubscription_MessageTransformsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ai_inference` after provisioning.\n"]
    pub fn ai_inference(&self) -> terrars::ListRef<DataPubsubSubscription_MessageTransforms_AiInferenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ai_inference", self.base))
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `javascript_udf` after provisioning.\n"]
    pub fn javascript_udf(&self) -> terrars::ListRef<DataPubsubSubscription_MessageTransforms_JavascriptUdfRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.javascript_udf", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPubsubSubscription_PushConfig_NoWrapper {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    write_metadata: std::option::Option<terrars::PrimField<bool>>,
}

impl DataPubsubSubscription_PushConfig_NoWrapper {
    #[doc = "Set the field `write_metadata`.\n"]
    pub fn set_write_metadata(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.write_metadata = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPubsubSubscription_PushConfig_NoWrapper {
    type O = terrars::BlockAssignable<DataPubsubSubscription_PushConfig_NoWrapper>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPubsubSubscription_PushConfig_NoWrapper {}

impl BuildDataPubsubSubscription_PushConfig_NoWrapper {
    pub fn build(self) -> DataPubsubSubscription_PushConfig_NoWrapper {
        DataPubsubSubscription_PushConfig_NoWrapper { write_metadata: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataPubsubSubscription_PushConfig_NoWrapperRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPubsubSubscription_PushConfig_NoWrapperRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPubsubSubscription_PushConfig_NoWrapperRef {
        DataPubsubSubscription_PushConfig_NoWrapperRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPubsubSubscription_PushConfig_NoWrapperRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `write_metadata` after provisioning.\n"]
    pub fn write_metadata(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write_metadata", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPubsubSubscription_PushConfig_OidcToken {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audience: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_email: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataPubsubSubscription_PushConfig_OidcToken {
    #[doc = "Set the field `audience`.\n"]
    pub fn set_audience(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.audience = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_account_email`.\n"]
    pub fn set_service_account_email(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_account_email = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPubsubSubscription_PushConfig_OidcToken {
    type O = terrars::BlockAssignable<DataPubsubSubscription_PushConfig_OidcToken>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPubsubSubscription_PushConfig_OidcToken {}

impl BuildDataPubsubSubscription_PushConfig_OidcToken {
    pub fn build(self) -> DataPubsubSubscription_PushConfig_OidcToken {
        DataPubsubSubscription_PushConfig_OidcToken {
            audience: core::default::Default::default(),
            service_account_email: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPubsubSubscription_PushConfig_OidcTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPubsubSubscription_PushConfig_OidcTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPubsubSubscription_PushConfig_OidcTokenRef {
        DataPubsubSubscription_PushConfig_OidcTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPubsubSubscription_PushConfig_OidcTokenRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audience` after provisioning.\n"]
    pub fn audience(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audience", self.base))
    }

    #[doc = "Get a reference to the value of field `service_account_email` after provisioning.\n"]
    pub fn service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_email", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPubsubSubscription_PushConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attributes: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_wrapper: std::option::Option<terrars::ListField<DataPubsubSubscription_PushConfig_NoWrapper>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oidc_token: std::option::Option<terrars::ListField<DataPubsubSubscription_PushConfig_OidcToken>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    push_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataPubsubSubscription_PushConfig {
    #[doc = "Set the field `attributes`.\n"]
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
        v: impl std::convert::Into<terrars::ListField<DataPubsubSubscription_PushConfig_NoWrapper>>,
    ) -> Self {
        self.no_wrapper = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oidc_token`.\n"]
    pub fn set_oidc_token(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataPubsubSubscription_PushConfig_OidcToken>>,
    ) -> Self {
        self.oidc_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `push_endpoint`.\n"]
    pub fn set_push_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.push_endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPubsubSubscription_PushConfig {
    type O = terrars::BlockAssignable<DataPubsubSubscription_PushConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPubsubSubscription_PushConfig {}

impl BuildDataPubsubSubscription_PushConfig {
    pub fn build(self) -> DataPubsubSubscription_PushConfig {
        DataPubsubSubscription_PushConfig {
            attributes: core::default::Default::default(),
            no_wrapper: core::default::Default::default(),
            oidc_token: core::default::Default::default(),
            push_endpoint: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPubsubSubscription_PushConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPubsubSubscription_PushConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPubsubSubscription_PushConfigRef {
        DataPubsubSubscription_PushConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPubsubSubscription_PushConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attributes` after provisioning.\n"]
    pub fn attributes(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.attributes", self.base))
    }

    #[doc = "Get a reference to the value of field `no_wrapper` after provisioning.\n"]
    pub fn no_wrapper(&self) -> terrars::ListRef<DataPubsubSubscription_PushConfig_NoWrapperRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.no_wrapper", self.base))
    }

    #[doc = "Get a reference to the value of field `oidc_token` after provisioning.\n"]
    pub fn oidc_token(&self) -> terrars::ListRef<DataPubsubSubscription_PushConfig_OidcTokenRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oidc_token", self.base))
    }

    #[doc = "Get a reference to the value of field `push_endpoint` after provisioning.\n"]
    pub fn push_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.push_endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPubsubSubscription_RetryPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_backoff: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum_backoff: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataPubsubSubscription_RetryPolicy {
    #[doc = "Set the field `maximum_backoff`.\n"]
    pub fn set_maximum_backoff(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.maximum_backoff = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minimum_backoff`.\n"]
    pub fn set_minimum_backoff(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.minimum_backoff = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPubsubSubscription_RetryPolicy {
    type O = terrars::BlockAssignable<DataPubsubSubscription_RetryPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPubsubSubscription_RetryPolicy {}

impl BuildDataPubsubSubscription_RetryPolicy {
    pub fn build(self) -> DataPubsubSubscription_RetryPolicy {
        DataPubsubSubscription_RetryPolicy {
            maximum_backoff: core::default::Default::default(),
            minimum_backoff: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPubsubSubscription_RetryPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPubsubSubscription_RetryPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPubsubSubscription_RetryPolicyRef {
        DataPubsubSubscription_RetryPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPubsubSubscription_RetryPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `maximum_backoff` after provisioning.\n"]
    pub fn maximum_backoff(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_backoff", self.base))
    }

    #[doc = "Get a reference to the value of field `minimum_backoff` after provisioning.\n"]
    pub fn minimum_backoff(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum_backoff", self.base))
    }
}
