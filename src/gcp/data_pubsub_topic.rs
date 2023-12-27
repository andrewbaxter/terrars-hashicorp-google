use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataPubsubTopicData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}

struct DataPubsubTopic_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataPubsubTopicData>,
}

#[derive(Clone)]
pub struct DataPubsubTopic(Rc<DataPubsubTopic_>);

impl DataPubsubTopic {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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

    #[doc= "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kms_key_name` after provisioning.\nThe resource name of the Cloud KMS CryptoKey to be used to protect access\nto messages published on this topic. Your project's PubSub service account\n('service-{{PROJECT_NUMBER}}@gcp-sa-pubsub.iam.gserviceaccount.com') must have\n'roles/cloudkms.cryptoKeyEncrypterDecrypter' to use this feature.\nThe expected format is 'projects/*/locations/*/keyRings/*/cryptoKeys/*'"]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to this Topic.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `message_retention_duration` after provisioning.\nIndicates the minimum duration to retain a message after it is published\nto the topic. If this field is set, messages published to the topic in\nthe last messageRetentionDuration are always available to subscribers.\nFor instance, it allows any attached subscription to seek to a timestamp\nthat is up to messageRetentionDuration in the past. If this field is not\nset, message retention is controlled by settings on individual subscriptions.\nThe rotation period has the format of a decimal number, followed by the\nletter 's' (seconds). Cannot be more than 31 days or less than 10 minutes."]
    pub fn message_retention_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message_retention_duration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `message_storage_policy` after provisioning.\nPolicy constraining the set of Google Cloud Platform regions where\nmessages published to the topic may be stored. If not present, then no\nconstraints are in effect."]
    pub fn message_storage_policy(&self) -> ListRef<DataPubsubTopicMessageStoragePolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.message_storage_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the topic."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `schema_settings` after provisioning.\nSettings for validating messages published against a schema."]
    pub fn schema_settings(&self) -> ListRef<DataPubsubTopicSchemaSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.schema_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }
}

impl Referable for DataPubsubTopic {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataPubsubTopic { }

impl ToListMappable for DataPubsubTopic {
    type O = ListRef<DataPubsubTopicRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataPubsubTopic_ {
    fn extract_datasource_type(&self) -> String {
        "google_pubsub_topic".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataPubsubTopic {
    pub tf_id: String,
    #[doc= "Name of the topic."]
    pub name: PrimField<String>,
}

impl BuildDataPubsubTopic {
    pub fn build(self, stack: &mut Stack) -> DataPubsubTopic {
        let out = DataPubsubTopic(Rc::new(DataPubsubTopic_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataPubsubTopicData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataPubsubTopicRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataPubsubTopicRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataPubsubTopicRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kms_key_name` after provisioning.\nThe resource name of the Cloud KMS CryptoKey to be used to protect access\nto messages published on this topic. Your project's PubSub service account\n('service-{{PROJECT_NUMBER}}@gcp-sa-pubsub.iam.gserviceaccount.com') must have\n'roles/cloudkms.cryptoKeyEncrypterDecrypter' to use this feature.\nThe expected format is 'projects/*/locations/*/keyRings/*/cryptoKeys/*'"]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to this Topic.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `message_retention_duration` after provisioning.\nIndicates the minimum duration to retain a message after it is published\nto the topic. If this field is set, messages published to the topic in\nthe last messageRetentionDuration are always available to subscribers.\nFor instance, it allows any attached subscription to seek to a timestamp\nthat is up to messageRetentionDuration in the past. If this field is not\nset, message retention is controlled by settings on individual subscriptions.\nThe rotation period has the format of a decimal number, followed by the\nletter 's' (seconds). Cannot be more than 31 days or less than 10 minutes."]
    pub fn message_retention_duration(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.message_retention_duration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `message_storage_policy` after provisioning.\nPolicy constraining the set of Google Cloud Platform regions where\nmessages published to the topic may be stored. If not present, then no\nconstraints are in effect."]
    pub fn message_storage_policy(&self) -> ListRef<DataPubsubTopicMessageStoragePolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.message_storage_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the topic."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `schema_settings` after provisioning.\nSettings for validating messages published against a schema."]
    pub fn schema_settings(&self) -> ListRef<DataPubsubTopicSchemaSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.schema_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataPubsubTopicMessageStoragePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_persistence_regions: Option<ListField<PrimField<String>>>,
}

impl DataPubsubTopicMessageStoragePolicyEl {
    #[doc= "Set the field `allowed_persistence_regions`.\n"]
    pub fn set_allowed_persistence_regions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allowed_persistence_regions = Some(v.into());
        self
    }
}

impl ToListMappable for DataPubsubTopicMessageStoragePolicyEl {
    type O = BlockAssignable<DataPubsubTopicMessageStoragePolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPubsubTopicMessageStoragePolicyEl {}

impl BuildDataPubsubTopicMessageStoragePolicyEl {
    pub fn build(self) -> DataPubsubTopicMessageStoragePolicyEl {
        DataPubsubTopicMessageStoragePolicyEl { allowed_persistence_regions: core::default::Default::default() }
    }
}

pub struct DataPubsubTopicMessageStoragePolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataPubsubTopicMessageStoragePolicyElRef {
    fn new(shared: StackShared, base: String) -> DataPubsubTopicMessageStoragePolicyElRef {
        DataPubsubTopicMessageStoragePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPubsubTopicMessageStoragePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `allowed_persistence_regions` after provisioning.\n"]
    pub fn allowed_persistence_regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.allowed_persistence_regions", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPubsubTopicSchemaSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema: Option<PrimField<String>>,
}

impl DataPubsubTopicSchemaSettingsEl {
    #[doc= "Set the field `encoding`.\n"]
    pub fn set_encoding(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encoding = Some(v.into());
        self
    }

    #[doc= "Set the field `schema`.\n"]
    pub fn set_schema(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.schema = Some(v.into());
        self
    }
}

impl ToListMappable for DataPubsubTopicSchemaSettingsEl {
    type O = BlockAssignable<DataPubsubTopicSchemaSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPubsubTopicSchemaSettingsEl {}

impl BuildDataPubsubTopicSchemaSettingsEl {
    pub fn build(self) -> DataPubsubTopicSchemaSettingsEl {
        DataPubsubTopicSchemaSettingsEl {
            encoding: core::default::Default::default(),
            schema: core::default::Default::default(),
        }
    }
}

pub struct DataPubsubTopicSchemaSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataPubsubTopicSchemaSettingsElRef {
    fn new(shared: StackShared, base: String) -> DataPubsubTopicSchemaSettingsElRef {
        DataPubsubTopicSchemaSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPubsubTopicSchemaSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `encoding` after provisioning.\n"]
    pub fn encoding(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encoding", self.base))
    }

    #[doc= "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }
}
