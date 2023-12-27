use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct PubsubTopicData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_retention_duration: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_storage_policy: Option<Vec<PubsubTopicMessageStoragePolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schema_settings: Option<Vec<PubsubTopicSchemaSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<PubsubTopicTimeoutsEl>,
    dynamic: PubsubTopicDynamic,
}

struct PubsubTopic_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<PubsubTopicData>,
}

#[derive(Clone)]
pub struct PubsubTopic(Rc<PubsubTopic_>);

impl PubsubTopic {
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

    #[doc= "Set the field `kms_key_name`.\nThe resource name of the Cloud KMS CryptoKey to be used to protect access\nto messages published on this topic. Your project's PubSub service account\n('service-{{PROJECT_NUMBER}}@gcp-sa-pubsub.iam.gserviceaccount.com') must have\n'roles/cloudkms.cryptoKeyEncrypterDecrypter' to use this feature.\nThe expected format is 'projects/*/locations/*/keyRings/*/cryptoKeys/*'"]
    pub fn set_kms_key_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().kms_key_name = Some(v.into());
        self
    }

    #[doc= "Set the field `labels`.\nA set of key/value label pairs to assign to this Topic.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }

    #[doc= "Set the field `message_retention_duration`.\nIndicates the minimum duration to retain a message after it is published\nto the topic. If this field is set, messages published to the topic in\nthe last messageRetentionDuration are always available to subscribers.\nFor instance, it allows any attached subscription to seek to a timestamp\nthat is up to messageRetentionDuration in the past. If this field is not\nset, message retention is controlled by settings on individual subscriptions.\nThe rotation period has the format of a decimal number, followed by the\nletter 's' (seconds). Cannot be more than 31 days or less than 10 minutes."]
    pub fn set_message_retention_duration(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().message_retention_duration = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `message_storage_policy`.\n"]
    pub fn set_message_storage_policy(self, v: impl Into<BlockAssignable<PubsubTopicMessageStoragePolicyEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().message_storage_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.message_storage_policy = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `schema_settings`.\n"]
    pub fn set_schema_settings(self, v: impl Into<BlockAssignable<PubsubTopicSchemaSettingsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schema_settings = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schema_settings = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<PubsubTopicTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
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

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the topic."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `message_storage_policy` after provisioning.\n"]
    pub fn message_storage_policy(&self) -> ListRef<PubsubTopicMessageStoragePolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.message_storage_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `schema_settings` after provisioning.\n"]
    pub fn schema_settings(&self) -> ListRef<PubsubTopicSchemaSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.schema_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PubsubTopicTimeoutsElRef {
        PubsubTopicTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for PubsubTopic {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for PubsubTopic { }

impl ToListMappable for PubsubTopic {
    type O = ListRef<PubsubTopicRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for PubsubTopic_ {
    fn extract_resource_type(&self) -> String {
        "google_pubsub_topic".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPubsubTopic {
    pub tf_id: String,
    #[doc= "Name of the topic."]
    pub name: PrimField<String>,
}

impl BuildPubsubTopic {
    pub fn build(self, stack: &mut Stack) -> PubsubTopic {
        let out = PubsubTopic(Rc::new(PubsubTopic_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(PubsubTopicData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                kms_key_name: core::default::Default::default(),
                labels: core::default::Default::default(),
                message_retention_duration: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                message_storage_policy: core::default::Default::default(),
                schema_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PubsubTopicRef {
    shared: StackShared,
    base: String,
}

impl Ref for PubsubTopicRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PubsubTopicRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
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

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the topic."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `message_storage_policy` after provisioning.\n"]
    pub fn message_storage_policy(&self) -> ListRef<PubsubTopicMessageStoragePolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.message_storage_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `schema_settings` after provisioning.\n"]
    pub fn schema_settings(&self) -> ListRef<PubsubTopicSchemaSettingsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.schema_settings", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PubsubTopicTimeoutsElRef {
        PubsubTopicTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct PubsubTopicMessageStoragePolicyEl {
    allowed_persistence_regions: ListField<PrimField<String>>,
}

impl PubsubTopicMessageStoragePolicyEl { }

impl ToListMappable for PubsubTopicMessageStoragePolicyEl {
    type O = BlockAssignable<PubsubTopicMessageStoragePolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubTopicMessageStoragePolicyEl {
    #[doc= "A list of IDs of GCP regions where messages that are published to\nthe topic may be persisted in storage. Messages published by\npublishers running in non-allowed GCP regions (or running outside\nof GCP altogether) will be routed for storage in one of the\nallowed regions. An empty list means that no regions are allowed,\nand is not a valid configuration."]
    pub allowed_persistence_regions: ListField<PrimField<String>>,
}

impl BuildPubsubTopicMessageStoragePolicyEl {
    pub fn build(self) -> PubsubTopicMessageStoragePolicyEl {
        PubsubTopicMessageStoragePolicyEl { allowed_persistence_regions: self.allowed_persistence_regions }
    }
}

pub struct PubsubTopicMessageStoragePolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for PubsubTopicMessageStoragePolicyElRef {
    fn new(shared: StackShared, base: String) -> PubsubTopicMessageStoragePolicyElRef {
        PubsubTopicMessageStoragePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubTopicMessageStoragePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `allowed_persistence_regions` after provisioning.\nA list of IDs of GCP regions where messages that are published to\nthe topic may be persisted in storage. Messages published by\npublishers running in non-allowed GCP regions (or running outside\nof GCP altogether) will be routed for storage in one of the\nallowed regions. An empty list means that no regions are allowed,\nand is not a valid configuration."]
    pub fn allowed_persistence_regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.allowed_persistence_regions", self.base))
    }
}

#[derive(Serialize)]
pub struct PubsubTopicSchemaSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding: Option<PrimField<String>>,
    schema: PrimField<String>,
}

impl PubsubTopicSchemaSettingsEl {
    #[doc= "Set the field `encoding`.\nThe encoding of messages validated against schema. Default value: \"ENCODING_UNSPECIFIED\" Possible values: [\"ENCODING_UNSPECIFIED\", \"JSON\", \"BINARY\"]"]
    pub fn set_encoding(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encoding = Some(v.into());
        self
    }
}

impl ToListMappable for PubsubTopicSchemaSettingsEl {
    type O = BlockAssignable<PubsubTopicSchemaSettingsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubTopicSchemaSettingsEl {
    #[doc= "The name of the schema that messages published should be\nvalidated against. Format is projects/{project}/schemas/{schema}.\nThe value of this field will be _deleted-schema_\nif the schema has been deleted."]
    pub schema: PrimField<String>,
}

impl BuildPubsubTopicSchemaSettingsEl {
    pub fn build(self) -> PubsubTopicSchemaSettingsEl {
        PubsubTopicSchemaSettingsEl {
            encoding: core::default::Default::default(),
            schema: self.schema,
        }
    }
}

pub struct PubsubTopicSchemaSettingsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for PubsubTopicSchemaSettingsElRef {
    fn new(shared: StackShared, base: String) -> PubsubTopicSchemaSettingsElRef {
        PubsubTopicSchemaSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubTopicSchemaSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `encoding` after provisioning.\nThe encoding of messages validated against schema. Default value: \"ENCODING_UNSPECIFIED\" Possible values: [\"ENCODING_UNSPECIFIED\", \"JSON\", \"BINARY\"]"]
    pub fn encoding(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encoding", self.base))
    }

    #[doc= "Get a reference to the value of field `schema` after provisioning.\nThe name of the schema that messages published should be\nvalidated against. Format is projects/{project}/schemas/{schema}.\nThe value of this field will be _deleted-schema_\nif the schema has been deleted."]
    pub fn schema(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }
}

#[derive(Serialize)]
pub struct PubsubTopicTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl PubsubTopicTimeoutsEl {
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

impl ToListMappable for PubsubTopicTimeoutsEl {
    type O = BlockAssignable<PubsubTopicTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubTopicTimeoutsEl {}

impl BuildPubsubTopicTimeoutsEl {
    pub fn build(self) -> PubsubTopicTimeoutsEl {
        PubsubTopicTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct PubsubTopicTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for PubsubTopicTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> PubsubTopicTimeoutsElRef {
        PubsubTopicTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubTopicTimeoutsElRef {
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
struct PubsubTopicDynamic {
    message_storage_policy: Option<DynamicBlock<PubsubTopicMessageStoragePolicyEl>>,
    schema_settings: Option<DynamicBlock<PubsubTopicSchemaSettingsEl>>,
}
