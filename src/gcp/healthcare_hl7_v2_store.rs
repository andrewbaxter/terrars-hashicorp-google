use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct HealthcareHl7V2StoreData {
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
    dataset: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification_config: std::option::Option<std::vec::Vec<HealthcareHl7V2StoreNotificationConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification_configs: std::option::Option<std::vec::Vec<HealthcareHl7V2StoreNotificationConfigsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parser_config: std::option::Option<std::vec::Vec<HealthcareHl7V2StoreParserConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<HealthcareHl7V2StoreTimeoutsEl>,
    dynamic: HealthcareHl7V2StoreDynamic,
}

struct HealthcareHl7V2Store_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<HealthcareHl7V2StoreData>,
}

#[derive(Clone)]
pub struct HealthcareHl7V2Store(std::rc::Rc<HealthcareHl7V2Store_>);

impl HealthcareHl7V2Store {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nUser-supplied key-value pairs used to organize HL7v2 stores.\n\nLabel keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must\nconform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}][\\p{Ll}\\p{Lo}\\p{N}_-]{0,62}\n\nLabel values are optional, must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128\nbytes, and must conform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63}\n\nNo more than 64 labels can be associated with a given store.\n\nAn object containing a list of \"key\": value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `notification_config`.\n"]
    pub fn set_notification_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<HealthcareHl7V2StoreNotificationConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().notification_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.notification_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `notification_configs`.\n"]
    pub fn set_notification_configs(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<HealthcareHl7V2StoreNotificationConfigsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().notification_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.notification_configs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `parser_config`.\n"]
    pub fn set_parser_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<HealthcareHl7V2StoreParserConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().parser_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.parser_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<HealthcareHl7V2StoreTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `dataset` after provisioning.\nIdentifies the dataset addressed by this request. Must be in the format\n'projects/{project}/locations/{location}/datasets/{dataset}'"]
    pub fn dataset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUser-supplied key-value pairs used to organize HL7v2 stores.\n\nLabel keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must\nconform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}][\\p{Ll}\\p{Lo}\\p{N}_-]{0,62}\n\nLabel values are optional, must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128\nbytes, and must conform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63}\n\nNo more than 64 labels can be associated with a given store.\n\nAn object containing a list of \"key\": value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name for the Hl7V2Store.\n\n** Changing this property may recreate the Hl7v2 store (removing all data) **"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `self_link` after provisioning.\nThe fully qualified name of this dataset"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_config` after provisioning.\n"]
    pub fn notification_config(&self) -> terrars::ListRef<HealthcareHl7V2StoreNotificationConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_configs` after provisioning.\n"]
    pub fn notification_configs(&self) -> terrars::ListRef<HealthcareHl7V2StoreNotificationConfigsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_configs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parser_config` after provisioning.\n"]
    pub fn parser_config(&self) -> terrars::ListRef<HealthcareHl7V2StoreParserConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parser_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> HealthcareHl7V2StoreTimeoutsElRef {
        HealthcareHl7V2StoreTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for HealthcareHl7V2Store {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for HealthcareHl7V2Store { }

impl terrars::ToListMappable for HealthcareHl7V2Store {
    type O = terrars::ListRef<HealthcareHl7V2StoreRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for HealthcareHl7V2Store_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_healthcare_hl7_v2_store".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildHealthcareHl7V2Store {
    pub tf_id: std::string::String,
    #[doc =
        "Identifies the dataset addressed by this request. Must be in the format\n'projects/{project}/locations/{location}/datasets/{dataset}'"]
    pub dataset: terrars::PrimField<std::string::String>,
    #[doc =
        "The resource name for the Hl7V2Store.\n\n** Changing this property may recreate the Hl7v2 store (removing all data) **"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildHealthcareHl7V2Store {
    pub fn build(self, stack: &mut terrars::Stack) -> HealthcareHl7V2Store {
        let out = HealthcareHl7V2Store(std::rc::Rc::new(HealthcareHl7V2Store_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(HealthcareHl7V2StoreData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                dataset: self.dataset,
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: self.name,
                notification_config: core::default::Default::default(),
                notification_configs: core::default::Default::default(),
                parser_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct HealthcareHl7V2StoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HealthcareHl7V2StoreRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl HealthcareHl7V2StoreRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dataset` after provisioning.\nIdentifies the dataset addressed by this request. Must be in the format\n'projects/{project}/locations/{location}/datasets/{dataset}'"]
    pub fn dataset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUser-supplied key-value pairs used to organize HL7v2 stores.\n\nLabel keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must\nconform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}][\\p{Ll}\\p{Lo}\\p{N}_-]{0,62}\n\nLabel values are optional, must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128\nbytes, and must conform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63}\n\nNo more than 64 labels can be associated with a given store.\n\nAn object containing a list of \"key\": value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name for the Hl7V2Store.\n\n** Changing this property may recreate the Hl7v2 store (removing all data) **"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `self_link` after provisioning.\nThe fully qualified name of this dataset"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_config` after provisioning.\n"]
    pub fn notification_config(&self) -> terrars::ListRef<HealthcareHl7V2StoreNotificationConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_configs` after provisioning.\n"]
    pub fn notification_configs(&self) -> terrars::ListRef<HealthcareHl7V2StoreNotificationConfigsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_configs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parser_config` after provisioning.\n"]
    pub fn parser_config(&self) -> terrars::ListRef<HealthcareHl7V2StoreParserConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parser_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> HealthcareHl7V2StoreTimeoutsElRef {
        HealthcareHl7V2StoreTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct HealthcareHl7V2StoreNotificationConfigEl {
    pubsub_topic: terrars::PrimField<std::string::String>,
}

impl HealthcareHl7V2StoreNotificationConfigEl { }

impl terrars::ToListMappable for HealthcareHl7V2StoreNotificationConfigEl {
    type O = terrars::BlockAssignable<HealthcareHl7V2StoreNotificationConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHealthcareHl7V2StoreNotificationConfigEl {
    #[doc =
        "The Cloud Pub/Sub topic that notifications of changes are published on. Supplied by the client.\nPubsubMessage.Data will contain the resource name. PubsubMessage.MessageId is the ID of this message.\nIt is guaranteed to be unique within the topic. PubsubMessage.PublishTime is the time at which the message\nwas published. Notifications are only sent if the topic is non-empty. Topic names must be scoped to a\nproject. service-PROJECT_NUMBER@gcp-sa-healthcare.iam.gserviceaccount.com must have publisher permissions on the given\nCloud Pub/Sub topic. Not having adequate permissions will cause the calls that send notifications to fail."]
    pub pubsub_topic: terrars::PrimField<std::string::String>,
}

impl BuildHealthcareHl7V2StoreNotificationConfigEl {
    pub fn build(self) -> HealthcareHl7V2StoreNotificationConfigEl {
        HealthcareHl7V2StoreNotificationConfigEl { pubsub_topic: self.pubsub_topic }
    }
}

pub struct HealthcareHl7V2StoreNotificationConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HealthcareHl7V2StoreNotificationConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> HealthcareHl7V2StoreNotificationConfigElRef {
        HealthcareHl7V2StoreNotificationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HealthcareHl7V2StoreNotificationConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `pubsub_topic` after provisioning.\nThe Cloud Pub/Sub topic that notifications of changes are published on. Supplied by the client.\nPubsubMessage.Data will contain the resource name. PubsubMessage.MessageId is the ID of this message.\nIt is guaranteed to be unique within the topic. PubsubMessage.PublishTime is the time at which the message\nwas published. Notifications are only sent if the topic is non-empty. Topic names must be scoped to a\nproject. service-PROJECT_NUMBER@gcp-sa-healthcare.iam.gserviceaccount.com must have publisher permissions on the given\nCloud Pub/Sub topic. Not having adequate permissions will cause the calls that send notifications to fail."]
    pub fn pubsub_topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pubsub_topic", self.base))
    }
}

#[derive(Serialize)]
pub struct HealthcareHl7V2StoreNotificationConfigsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<terrars::PrimField<std::string::String>>,
    pubsub_topic: terrars::PrimField<std::string::String>,
}

impl HealthcareHl7V2StoreNotificationConfigsEl {
    #[doc =
        "Set the field `filter`.\nRestricts notifications sent for messages matching a filter. If this is empty, all messages\nare matched. Syntax: https://cloud.google.com/appengine/docs/standard/python/search/query_strings\n\nFields/functions available for filtering are:\n\n* messageType, from the MSH-9.1 field. For example, NOT messageType = \"ADT\".\n* send_date or sendDate, the YYYY-MM-DD date the message was sent in the dataset's timeZone, from the MSH-7 segment. For example, send_date < \"2017-01-02\".\n* sendTime, the timestamp when the message was sent, using the RFC3339 time format for comparisons, from the MSH-7 segment. For example, sendTime < \"2017-01-02T00:00:00-05:00\".\n* sendFacility, the care center that the message came from, from the MSH-4 segment. For example, sendFacility = \"ABC\".\n* PatientId(value, type), which matches if the message lists a patient having an ID of the given value and type in the PID-2, PID-3, or PID-4 segments. For example, PatientId(\"123456\", \"MRN\").\n* labels.x, a string value of the label with key x as set using the Message.labels map. For example, labels.\"priority\"=\"high\". The operator :* can be used to assert the existence of a label. For example, labels.\"priority\":*."]
    pub fn set_filter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.filter = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for HealthcareHl7V2StoreNotificationConfigsEl {
    type O = terrars::BlockAssignable<HealthcareHl7V2StoreNotificationConfigsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHealthcareHl7V2StoreNotificationConfigsEl {
    #[doc =
        "The Cloud Pub/Sub topic that notifications of changes are published on. Supplied by the client.\nPubsubMessage.Data will contain the resource name. PubsubMessage.MessageId is the ID of this message.\nIt is guaranteed to be unique within the topic. PubsubMessage.PublishTime is the time at which the message\nwas published. Notifications are only sent if the topic is non-empty. Topic names must be scoped to a\nproject. service-PROJECT_NUMBER@gcp-sa-healthcare.iam.gserviceaccount.com must have publisher permissions on the given\nCloud Pub/Sub topic. Not having adequate permissions will cause the calls that send notifications to fail.\n\nIf a notification cannot be published to Cloud Pub/Sub, errors will be logged to Stackdriver"]
    pub pubsub_topic: terrars::PrimField<std::string::String>,
}

impl BuildHealthcareHl7V2StoreNotificationConfigsEl {
    pub fn build(self) -> HealthcareHl7V2StoreNotificationConfigsEl {
        HealthcareHl7V2StoreNotificationConfigsEl {
            filter: core::default::Default::default(),
            pubsub_topic: self.pubsub_topic,
        }
    }
}

pub struct HealthcareHl7V2StoreNotificationConfigsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HealthcareHl7V2StoreNotificationConfigsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> HealthcareHl7V2StoreNotificationConfigsElRef {
        HealthcareHl7V2StoreNotificationConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HealthcareHl7V2StoreNotificationConfigsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nRestricts notifications sent for messages matching a filter. If this is empty, all messages\nare matched. Syntax: https://cloud.google.com/appengine/docs/standard/python/search/query_strings\n\nFields/functions available for filtering are:\n\n* messageType, from the MSH-9.1 field. For example, NOT messageType = \"ADT\".\n* send_date or sendDate, the YYYY-MM-DD date the message was sent in the dataset's timeZone, from the MSH-7 segment. For example, send_date < \"2017-01-02\".\n* sendTime, the timestamp when the message was sent, using the RFC3339 time format for comparisons, from the MSH-7 segment. For example, sendTime < \"2017-01-02T00:00:00-05:00\".\n* sendFacility, the care center that the message came from, from the MSH-4 segment. For example, sendFacility = \"ABC\".\n* PatientId(value, type), which matches if the message lists a patient having an ID of the given value and type in the PID-2, PID-3, or PID-4 segments. For example, PatientId(\"123456\", \"MRN\").\n* labels.x, a string value of the label with key x as set using the Message.labels map. For example, labels.\"priority\"=\"high\". The operator :* can be used to assert the existence of a label. For example, labels.\"priority\":*."]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pubsub_topic` after provisioning.\nThe Cloud Pub/Sub topic that notifications of changes are published on. Supplied by the client.\nPubsubMessage.Data will contain the resource name. PubsubMessage.MessageId is the ID of this message.\nIt is guaranteed to be unique within the topic. PubsubMessage.PublishTime is the time at which the message\nwas published. Notifications are only sent if the topic is non-empty. Topic names must be scoped to a\nproject. service-PROJECT_NUMBER@gcp-sa-healthcare.iam.gserviceaccount.com must have publisher permissions on the given\nCloud Pub/Sub topic. Not having adequate permissions will cause the calls that send notifications to fail.\n\nIf a notification cannot be published to Cloud Pub/Sub, errors will be logged to Stackdriver"]
    pub fn pubsub_topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pubsub_topic", self.base))
    }
}

#[derive(Serialize)]
pub struct HealthcareHl7V2StoreParserConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow_null_header: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    segment_terminator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl HealthcareHl7V2StoreParserConfigEl {
    #[doc = "Set the field `allow_null_header`.\nDetermines whether messages with no header are allowed."]
    pub fn set_allow_null_header(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.allow_null_header = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `schema`.\nJSON encoded string for schemas used to parse messages in this\nstore if schematized parsing is desired."]
    pub fn set_schema(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.schema = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `segment_terminator`.\nByte(s) to be used as the segment terminator. If this is unset, '\\r' will be used as segment terminator.\n\nA base64-encoded string."]
    pub fn set_segment_terminator(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.segment_terminator = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `version`.\nThe version of the unschematized parser to be used when a custom 'schema' is not set. Default value: \"V1\" Possible values: [\"V1\", \"V2\", \"V3\"]"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for HealthcareHl7V2StoreParserConfigEl {
    type O = terrars::BlockAssignable<HealthcareHl7V2StoreParserConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHealthcareHl7V2StoreParserConfigEl {}

impl BuildHealthcareHl7V2StoreParserConfigEl {
    pub fn build(self) -> HealthcareHl7V2StoreParserConfigEl {
        HealthcareHl7V2StoreParserConfigEl {
            allow_null_header: core::default::Default::default(),
            schema: core::default::Default::default(),
            segment_terminator: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct HealthcareHl7V2StoreParserConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HealthcareHl7V2StoreParserConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> HealthcareHl7V2StoreParserConfigElRef {
        HealthcareHl7V2StoreParserConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HealthcareHl7V2StoreParserConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_null_header` after provisioning.\nDetermines whether messages with no header are allowed."]
    pub fn allow_null_header(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_null_header", self.base))
    }

    #[doc =
        "Get a reference to the value of field `schema` after provisioning.\nJSON encoded string for schemas used to parse messages in this\nstore if schematized parsing is desired."]
    pub fn schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }

    #[doc =
        "Get a reference to the value of field `segment_terminator` after provisioning.\nByte(s) to be used as the segment terminator. If this is unset, '\\r' will be used as segment terminator.\n\nA base64-encoded string."]
    pub fn segment_terminator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.segment_terminator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nThe version of the unschematized parser to be used when a custom 'schema' is not set. Default value: \"V1\" Possible values: [\"V1\", \"V2\", \"V3\"]"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct HealthcareHl7V2StoreTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl HealthcareHl7V2StoreTimeoutsEl {
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

impl terrars::ToListMappable for HealthcareHl7V2StoreTimeoutsEl {
    type O = terrars::BlockAssignable<HealthcareHl7V2StoreTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHealthcareHl7V2StoreTimeoutsEl {}

impl BuildHealthcareHl7V2StoreTimeoutsEl {
    pub fn build(self) -> HealthcareHl7V2StoreTimeoutsEl {
        HealthcareHl7V2StoreTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct HealthcareHl7V2StoreTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HealthcareHl7V2StoreTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> HealthcareHl7V2StoreTimeoutsElRef {
        HealthcareHl7V2StoreTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HealthcareHl7V2StoreTimeoutsElRef {
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
struct HealthcareHl7V2StoreDynamic {
    notification_config: std::option::Option<terrars::DynamicBlock<HealthcareHl7V2StoreNotificationConfigEl>>,
    notification_configs: std::option::Option<terrars::DynamicBlock<HealthcareHl7V2StoreNotificationConfigsEl>>,
    parser_config: std::option::Option<terrars::DynamicBlock<HealthcareHl7V2StoreParserConfigEl>>,
}
