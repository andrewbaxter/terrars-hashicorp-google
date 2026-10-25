#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct PubsubTopicData_ {
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
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_retention_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingestion_data_source_settings: std::option::Option<std::vec::Vec<PubsubTopic_IngestionDataSourceSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_storage_policy: std::option::Option<std::vec::Vec<PubsubTopic_MessageStoragePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_transforms: std::option::Option<std::vec::Vec<PubsubTopic_MessageTransforms>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_settings: std::option::Option<std::vec::Vec<PubsubTopic_SchemaSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<PubsubTopic_Timeouts>,
    dynamic: PubsubTopicDynamic,
}

struct PubsubTopic_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<PubsubTopicData_>,
}

#[derive(Clone)]
pub struct PubsubTopic(std::rc::Rc<PubsubTopic_>);

impl PubsubTopic {
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
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kms_key_name`.\nThe resource name of the Cloud KMS CryptoKey to be used to protect access\nto messages published on this topic. Your project's PubSub service account\n('service-{{PROJECT_NUMBER}}@gcp-sa-pubsub.iam.gserviceaccount.com') must have\n'roles/cloudkms.cryptoKeyEncrypterDecrypter' to use this feature.\nThe expected format is 'projects/*/locations/*/keyRings/*/cryptoKeys/*'"]
    pub fn set_kms_key_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nA set of key/value label pairs to assign to this Topic.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `message_retention_duration`.\nIndicates the minimum duration to retain a message after it is published\nto the topic. If this field is set, messages published to the topic in\nthe last messageRetentionDuration are always available to subscribers.\nFor instance, it allows any attached subscription to seek to a timestamp\nthat is up to messageRetentionDuration in the past. If this field is not\nset, message retention is controlled by settings on individual subscriptions.\nThe rotation period has the format of a decimal number, followed by the\nletter 's' (seconds). Cannot be more than 31 days or less than 10 minutes."]
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
        "Set the field `tags`.\nInput only. Resource manager tags to be bound to the topic. Tag keys and\nvalues have the same definition as resource manager tags. Keys must be in\nthe format tagKeys/{tag_key_id}, and values are in the format\ntagValues/456. The field is ignored when empty. The field is immutable and\ncauses resource replacement when mutated. This field is only set at create\ntime and modifying this field after creation will trigger recreation. To\napply tags to an existing resource, see the 'google_tags_tag_value'\nresource."]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ingestion_data_source_settings`.\n"]
    pub fn set_ingestion_data_source_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PubsubTopic_IngestionDataSourceSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ingestion_data_source_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ingestion_data_source_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `message_storage_policy`.\n"]
    pub fn set_message_storage_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PubsubTopic_MessageStoragePolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().message_storage_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.message_storage_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `message_transforms`.\n"]
    pub fn set_message_transforms(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PubsubTopic_MessageTransforms>>,
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

    #[doc = "Set the field `schema_settings`.\n"]
    pub fn set_schema_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<PubsubTopic_SchemaSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schema_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schema_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<PubsubTopic_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nThe resource name of the Cloud KMS CryptoKey to be used to protect access\nto messages published on this topic. Your project's PubSub service account\n('service-{{PROJECT_NUMBER}}@gcp-sa-pubsub.iam.gserviceaccount.com') must have\n'roles/cloudkms.cryptoKeyEncrypterDecrypter' to use this feature.\nThe expected format is 'projects/*/locations/*/keyRings/*/cryptoKeys/*'"]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to this Topic.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `message_retention_duration` after provisioning.\nIndicates the minimum duration to retain a message after it is published\nto the topic. If this field is set, messages published to the topic in\nthe last messageRetentionDuration are always available to subscribers.\nFor instance, it allows any attached subscription to seek to a timestamp\nthat is up to messageRetentionDuration in the past. If this field is not\nset, message retention is controlled by settings on individual subscriptions.\nThe rotation period has the format of a decimal number, followed by the\nletter 's' (seconds). Cannot be more than 31 days or less than 10 minutes."]
    pub fn message_retention_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_retention_duration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the topic."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nInput only. Resource manager tags to be bound to the topic. Tag keys and\nvalues have the same definition as resource manager tags. Keys must be in\nthe format tagKeys/{tag_key_id}, and values are in the format\ntagValues/456. The field is ignored when empty. The field is immutable and\ncauses resource replacement when mutated. This field is only set at create\ntime and modifying this field after creation will trigger recreation. To\napply tags to an existing resource, see the 'google_tags_tag_value'\nresource."]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ingestion_data_source_settings` after provisioning.\n"]
    pub fn ingestion_data_source_settings(&self) -> terrars::ListRef<PubsubTopic_IngestionDataSourceSettingsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.ingestion_data_source_settings", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `message_storage_policy` after provisioning.\n"]
    pub fn message_storage_policy(&self) -> terrars::ListRef<PubsubTopic_MessageStoragePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.message_storage_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `message_transforms` after provisioning.\n"]
    pub fn message_transforms(&self) -> terrars::ListRef<PubsubTopic_MessageTransformsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.message_transforms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schema_settings` after provisioning.\n"]
    pub fn schema_settings(&self) -> terrars::ListRef<PubsubTopic_SchemaSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PubsubTopic_TimeoutsRef {
        PubsubTopic_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for PubsubTopic {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for PubsubTopic { }

impl terrars::ToListMappable for PubsubTopic {
    type O = terrars::ListRef<PubsubTopicRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for PubsubTopic_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_pubsub_topic".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildPubsubTopic {
    pub tf_id: std::string::String,
    #[doc = "Name of the topic."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildPubsubTopic {
    pub fn build(self, stack: &mut terrars::Stack) -> PubsubTopic {
        let out = PubsubTopic(std::rc::Rc::new(PubsubTopic_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(PubsubTopicData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_name: core::default::Default::default(),
                labels: core::default::Default::default(),
                message_retention_duration: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                tags: core::default::Default::default(),
                ingestion_data_source_settings: core::default::Default::default(),
                message_storage_policy: core::default::Default::default(),
                message_transforms: core::default::Default::default(),
                schema_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct PubsubTopicRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubTopicRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl PubsubTopicRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nThe resource name of the Cloud KMS CryptoKey to be used to protect access\nto messages published on this topic. Your project's PubSub service account\n('service-{{PROJECT_NUMBER}}@gcp-sa-pubsub.iam.gserviceaccount.com') must have\n'roles/cloudkms.cryptoKeyEncrypterDecrypter' to use this feature.\nThe expected format is 'projects/*/locations/*/keyRings/*/cryptoKeys/*'"]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs to assign to this Topic.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `message_retention_duration` after provisioning.\nIndicates the minimum duration to retain a message after it is published\nto the topic. If this field is set, messages published to the topic in\nthe last messageRetentionDuration are always available to subscribers.\nFor instance, it allows any attached subscription to seek to a timestamp\nthat is up to messageRetentionDuration in the past. If this field is not\nset, message retention is controlled by settings on individual subscriptions.\nThe rotation period has the format of a decimal number, followed by the\nletter 's' (seconds). Cannot be more than 31 days or less than 10 minutes."]
    pub fn message_retention_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_retention_duration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the topic."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nInput only. Resource manager tags to be bound to the topic. Tag keys and\nvalues have the same definition as resource manager tags. Keys must be in\nthe format tagKeys/{tag_key_id}, and values are in the format\ntagValues/456. The field is ignored when empty. The field is immutable and\ncauses resource replacement when mutated. This field is only set at create\ntime and modifying this field after creation will trigger recreation. To\napply tags to an existing resource, see the 'google_tags_tag_value'\nresource."]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ingestion_data_source_settings` after provisioning.\n"]
    pub fn ingestion_data_source_settings(&self) -> terrars::ListRef<PubsubTopic_IngestionDataSourceSettingsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.ingestion_data_source_settings", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `message_storage_policy` after provisioning.\n"]
    pub fn message_storage_policy(&self) -> terrars::ListRef<PubsubTopic_MessageStoragePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.message_storage_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `message_transforms` after provisioning.\n"]
    pub fn message_transforms(&self) -> terrars::ListRef<PubsubTopic_MessageTransformsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.message_transforms", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schema_settings` after provisioning.\n"]
    pub fn schema_settings(&self) -> terrars::ListRef<PubsubTopic_SchemaSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> PubsubTopic_TimeoutsRef {
        PubsubTopic_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct PubsubTopicDynamic {
    ingestion_data_source_settings: std
    ::option
    ::Option<terrars::DynamicBlock<PubsubTopic_IngestionDataSourceSettings>>,
    message_storage_policy: std::option::Option<terrars::DynamicBlock<PubsubTopic_MessageStoragePolicy>>,
    message_transforms: std::option::Option<terrars::DynamicBlock<PubsubTopic_MessageTransforms>>,
    schema_settings: std::option::Option<terrars::DynamicBlock<PubsubTopic_SchemaSettings>>,
}

#[derive(Serialize)]
pub struct PubsubTopic_IngestionDataSourceSettings_AwsKinesis {
    aws_role_arn: terrars::PrimField<std::string::String>,
    consumer_arn: terrars::PrimField<std::string::String>,
    gcp_service_account: terrars::PrimField<std::string::String>,
    stream_arn: terrars::PrimField<std::string::String>,
}

impl PubsubTopic_IngestionDataSourceSettings_AwsKinesis { }

impl terrars::ToListMappable for PubsubTopic_IngestionDataSourceSettings_AwsKinesis {
    type O = terrars::BlockAssignable<PubsubTopic_IngestionDataSourceSettings_AwsKinesis>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubTopic_IngestionDataSourceSettings_AwsKinesis {
    #[doc =
        "AWS role ARN to be used for Federated Identity authentication with\nKinesis. Check the Pub/Sub docs for how to set up this role and the\nrequired permissions that need to be attached to it."]
    pub aws_role_arn: terrars::PrimField<std::string::String>,
    #[doc =
        "The Kinesis consumer ARN to used for ingestion in\nEnhanced Fan-Out mode. The consumer must be already\ncreated and ready to be used."]
    pub consumer_arn: terrars::PrimField<std::string::String>,
    #[doc =
        "The GCP service account to be used for Federated Identity authentication\nwith Kinesis (via a 'AssumeRoleWithWebIdentity' call for the provided\nrole). The 'awsRoleArn' must be set up with 'accounts.google.com:sub'\nequals to this service account number."]
    pub gcp_service_account: terrars::PrimField<std::string::String>,
    #[doc = "The Kinesis stream ARN to ingest data from."]
    pub stream_arn: terrars::PrimField<std::string::String>,
}

impl BuildPubsubTopic_IngestionDataSourceSettings_AwsKinesis {
    pub fn build(self) -> PubsubTopic_IngestionDataSourceSettings_AwsKinesis {
        PubsubTopic_IngestionDataSourceSettings_AwsKinesis {
            aws_role_arn: self.aws_role_arn,
            consumer_arn: self.consumer_arn,
            gcp_service_account: self.gcp_service_account,
            stream_arn: self.stream_arn,
        }
    }
}

#[allow(dead_code)]
pub struct PubsubTopic_IngestionDataSourceSettings_AwsKinesisRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubTopic_IngestionDataSourceSettings_AwsKinesisRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PubsubTopic_IngestionDataSourceSettings_AwsKinesisRef {
        PubsubTopic_IngestionDataSourceSettings_AwsKinesisRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubTopic_IngestionDataSourceSettings_AwsKinesisRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `aws_role_arn` after provisioning.\nAWS role ARN to be used for Federated Identity authentication with\nKinesis. Check the Pub/Sub docs for how to set up this role and the\nrequired permissions that need to be attached to it."]
    pub fn aws_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_role_arn", self.base))
    }

    #[doc =
        "Get a reference to the value of field `consumer_arn` after provisioning.\nThe Kinesis consumer ARN to used for ingestion in\nEnhanced Fan-Out mode. The consumer must be already\ncreated and ready to be used."]
    pub fn consumer_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_arn", self.base))
    }

    #[doc =
        "Get a reference to the value of field `gcp_service_account` after provisioning.\nThe GCP service account to be used for Federated Identity authentication\nwith Kinesis (via a 'AssumeRoleWithWebIdentity' call for the provided\nrole). The 'awsRoleArn' must be set up with 'accounts.google.com:sub'\nequals to this service account number."]
    pub fn gcp_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_service_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `stream_arn` after provisioning.\nThe Kinesis stream ARN to ingest data from."]
    pub fn stream_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stream_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct PubsubTopic_IngestionDataSourceSettings_AwsMsk {
    aws_role_arn: terrars::PrimField<std::string::String>,
    cluster_arn: terrars::PrimField<std::string::String>,
    gcp_service_account: terrars::PrimField<std::string::String>,
    topic: terrars::PrimField<std::string::String>,
}

impl PubsubTopic_IngestionDataSourceSettings_AwsMsk { }

impl terrars::ToListMappable for PubsubTopic_IngestionDataSourceSettings_AwsMsk {
    type O = terrars::BlockAssignable<PubsubTopic_IngestionDataSourceSettings_AwsMsk>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubTopic_IngestionDataSourceSettings_AwsMsk {
    #[doc =
        "AWS role ARN to be used for Federated Identity authentication with\nMSK. Check the Pub/Sub docs for how to set up this role and the\nrequired permissions that need to be attached to it."]
    pub aws_role_arn: terrars::PrimField<std::string::String>,
    #[doc = "ARN that uniquely identifies the MSK cluster."]
    pub cluster_arn: terrars::PrimField<std::string::String>,
    #[doc =
        "The GCP service account to be used for Federated Identity authentication\nwith MSK (via a 'AssumeRoleWithWebIdentity' call for the provided\nrole). The 'awsRoleArn' must be set up with 'accounts.google.com:sub'\nequals to this service account number."]
    pub gcp_service_account: terrars::PrimField<std::string::String>,
    #[doc = "The name of the MSK topic that Pub/Sub will import from."]
    pub topic: terrars::PrimField<std::string::String>,
}

impl BuildPubsubTopic_IngestionDataSourceSettings_AwsMsk {
    pub fn build(self) -> PubsubTopic_IngestionDataSourceSettings_AwsMsk {
        PubsubTopic_IngestionDataSourceSettings_AwsMsk {
            aws_role_arn: self.aws_role_arn,
            cluster_arn: self.cluster_arn,
            gcp_service_account: self.gcp_service_account,
            topic: self.topic,
        }
    }
}

#[allow(dead_code)]
pub struct PubsubTopic_IngestionDataSourceSettings_AwsMskRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubTopic_IngestionDataSourceSettings_AwsMskRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PubsubTopic_IngestionDataSourceSettings_AwsMskRef {
        PubsubTopic_IngestionDataSourceSettings_AwsMskRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubTopic_IngestionDataSourceSettings_AwsMskRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `aws_role_arn` after provisioning.\nAWS role ARN to be used for Federated Identity authentication with\nMSK. Check the Pub/Sub docs for how to set up this role and the\nrequired permissions that need to be attached to it."]
    pub fn aws_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_role_arn", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cluster_arn` after provisioning.\nARN that uniquely identifies the MSK cluster."]
    pub fn cluster_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_arn", self.base))
    }

    #[doc =
        "Get a reference to the value of field `gcp_service_account` after provisioning.\nThe GCP service account to be used for Federated Identity authentication\nwith MSK (via a 'AssumeRoleWithWebIdentity' call for the provided\nrole). The 'awsRoleArn' must be set up with 'accounts.google.com:sub'\nequals to this service account number."]
    pub fn gcp_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_service_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `topic` after provisioning.\nThe name of the MSK topic that Pub/Sub will import from."]
    pub fn topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}

#[derive(Serialize)]
pub struct PubsubTopic_IngestionDataSourceSettings_AzureEventHubs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_hub: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subscription_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tenant_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PubsubTopic_IngestionDataSourceSettings_AzureEventHubs {
    #[doc = "Set the field `client_id`.\nThe Azure event hub client ID to use for ingestion."]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `event_hub`.\nThe Azure event hub to ingest data from."]
    pub fn set_event_hub(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.event_hub = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `gcp_service_account`.\nThe GCP service account to be used for Federated Identity authentication\nwith Azure (via a 'AssumeRoleWithWebIdentity' call for the provided\nrole)."]
    pub fn set_gcp_service_account(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.gcp_service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `namespace`.\nThe Azure event hub namespace to ingest data from."]
    pub fn set_namespace(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.namespace = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_group`.\nThe name of the resource group within an Azure subscription."]
    pub fn set_resource_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subscription_id`.\nThe Azure event hub subscription ID to use for ingestion."]
    pub fn set_subscription_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subscription_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tenant_id`.\nThe Azure event hub tenant ID to use for ingestion."]
    pub fn set_tenant_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tenant_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PubsubTopic_IngestionDataSourceSettings_AzureEventHubs {
    type O = terrars::BlockAssignable<PubsubTopic_IngestionDataSourceSettings_AzureEventHubs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubTopic_IngestionDataSourceSettings_AzureEventHubs {}

impl BuildPubsubTopic_IngestionDataSourceSettings_AzureEventHubs {
    pub fn build(self) -> PubsubTopic_IngestionDataSourceSettings_AzureEventHubs {
        PubsubTopic_IngestionDataSourceSettings_AzureEventHubs {
            client_id: core::default::Default::default(),
            event_hub: core::default::Default::default(),
            gcp_service_account: core::default::Default::default(),
            namespace: core::default::Default::default(),
            resource_group: core::default::Default::default(),
            subscription_id: core::default::Default::default(),
            tenant_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PubsubTopic_IngestionDataSourceSettings_AzureEventHubsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubTopic_IngestionDataSourceSettings_AzureEventHubsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PubsubTopic_IngestionDataSourceSettings_AzureEventHubsRef {
        PubsubTopic_IngestionDataSourceSettings_AzureEventHubsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubTopic_IngestionDataSourceSettings_AzureEventHubsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `client_id` after provisioning.\nThe Azure event hub client ID to use for ingestion."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `event_hub` after provisioning.\nThe Azure event hub to ingest data from."]
    pub fn event_hub(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_hub", self.base))
    }

    #[doc =
        "Get a reference to the value of field `gcp_service_account` after provisioning.\nThe GCP service account to be used for Federated Identity authentication\nwith Azure (via a 'AssumeRoleWithWebIdentity' call for the provided\nrole)."]
    pub fn gcp_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_service_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `namespace` after provisioning.\nThe Azure event hub namespace to ingest data from."]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource_group` after provisioning.\nThe name of the resource group within an Azure subscription."]
    pub fn resource_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_group", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subscription_id` after provisioning.\nThe Azure event hub subscription ID to use for ingestion."]
    pub fn subscription_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscription_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tenant_id` after provisioning.\nThe Azure event hub tenant ID to use for ingestion."]
    pub fn tenant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenant_id", self.base))
    }
}

#[derive(Serialize)]
pub struct PubsubTopic_IngestionDataSourceSettings_CloudStorage_AvroFormat {}

impl PubsubTopic_IngestionDataSourceSettings_CloudStorage_AvroFormat { }

impl terrars::ToListMappable for PubsubTopic_IngestionDataSourceSettings_CloudStorage_AvroFormat {
    type O = terrars::BlockAssignable<PubsubTopic_IngestionDataSourceSettings_CloudStorage_AvroFormat>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubTopic_IngestionDataSourceSettings_CloudStorage_AvroFormat {}

impl BuildPubsubTopic_IngestionDataSourceSettings_CloudStorage_AvroFormat {
    pub fn build(self) -> PubsubTopic_IngestionDataSourceSettings_CloudStorage_AvroFormat {
        PubsubTopic_IngestionDataSourceSettings_CloudStorage_AvroFormat {}
    }
}

#[allow(dead_code)]
pub struct PubsubTopic_IngestionDataSourceSettings_CloudStorage_AvroFormatRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubTopic_IngestionDataSourceSettings_CloudStorage_AvroFormatRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PubsubTopic_IngestionDataSourceSettings_CloudStorage_AvroFormatRef {
        PubsubTopic_IngestionDataSourceSettings_CloudStorage_AvroFormatRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubTopic_IngestionDataSourceSettings_CloudStorage_AvroFormatRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct PubsubTopic_IngestionDataSourceSettings_CloudStorage_PubsubAvroFormat {}

impl PubsubTopic_IngestionDataSourceSettings_CloudStorage_PubsubAvroFormat { }

impl terrars::ToListMappable for PubsubTopic_IngestionDataSourceSettings_CloudStorage_PubsubAvroFormat {
    type O = terrars::BlockAssignable<PubsubTopic_IngestionDataSourceSettings_CloudStorage_PubsubAvroFormat>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubTopic_IngestionDataSourceSettings_CloudStorage_PubsubAvroFormat {}

impl BuildPubsubTopic_IngestionDataSourceSettings_CloudStorage_PubsubAvroFormat {
    pub fn build(self) -> PubsubTopic_IngestionDataSourceSettings_CloudStorage_PubsubAvroFormat {
        PubsubTopic_IngestionDataSourceSettings_CloudStorage_PubsubAvroFormat {}
    }
}

#[allow(dead_code)]
pub struct PubsubTopic_IngestionDataSourceSettings_CloudStorage_PubsubAvroFormatRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubTopic_IngestionDataSourceSettings_CloudStorage_PubsubAvroFormatRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PubsubTopic_IngestionDataSourceSettings_CloudStorage_PubsubAvroFormatRef {
        PubsubTopic_IngestionDataSourceSettings_CloudStorage_PubsubAvroFormatRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubTopic_IngestionDataSourceSettings_CloudStorage_PubsubAvroFormatRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct PubsubTopic_IngestionDataSourceSettings_CloudStorage_TextFormat {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delimiter: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PubsubTopic_IngestionDataSourceSettings_CloudStorage_TextFormat {
    #[doc =
        "Set the field `delimiter`.\nThe delimiter to use when using the 'text' format. Each line of text as\nspecified by the delimiter will be set to the 'data' field of a Pub/Sub\nmessage. When unset, '\\n' is used."]
    pub fn set_delimiter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delimiter = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PubsubTopic_IngestionDataSourceSettings_CloudStorage_TextFormat {
    type O = terrars::BlockAssignable<PubsubTopic_IngestionDataSourceSettings_CloudStorage_TextFormat>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubTopic_IngestionDataSourceSettings_CloudStorage_TextFormat {}

impl BuildPubsubTopic_IngestionDataSourceSettings_CloudStorage_TextFormat {
    pub fn build(self) -> PubsubTopic_IngestionDataSourceSettings_CloudStorage_TextFormat {
        PubsubTopic_IngestionDataSourceSettings_CloudStorage_TextFormat {
            delimiter: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PubsubTopic_IngestionDataSourceSettings_CloudStorage_TextFormatRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubTopic_IngestionDataSourceSettings_CloudStorage_TextFormatRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PubsubTopic_IngestionDataSourceSettings_CloudStorage_TextFormatRef {
        PubsubTopic_IngestionDataSourceSettings_CloudStorage_TextFormatRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubTopic_IngestionDataSourceSettings_CloudStorage_TextFormatRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `delimiter` after provisioning.\nThe delimiter to use when using the 'text' format. Each line of text as\nspecified by the delimiter will be set to the 'data' field of a Pub/Sub\nmessage. When unset, '\\n' is used."]
    pub fn delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delimiter", self.base))
    }
}

#[derive(Serialize, Default)]
struct PubsubTopic_IngestionDataSourceSettings_CloudStorageDynamic {
    avro_format: std
    ::option
    ::Option<terrars::DynamicBlock<PubsubTopic_IngestionDataSourceSettings_CloudStorage_AvroFormat>>,
    pubsub_avro_format: std
    ::option
    ::Option<terrars::DynamicBlock<PubsubTopic_IngestionDataSourceSettings_CloudStorage_PubsubAvroFormat>>,
    text_format: std
    ::option
    ::Option<terrars::DynamicBlock<PubsubTopic_IngestionDataSourceSettings_CloudStorage_TextFormat>>,
}

#[derive(Serialize)]
pub struct PubsubTopic_IngestionDataSourceSettings_CloudStorage {
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_glob: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum_object_create_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    avro_format: std::option::Option<std::vec::Vec<PubsubTopic_IngestionDataSourceSettings_CloudStorage_AvroFormat>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pubsub_avro_format: std
    ::option
    ::Option<std::vec::Vec<PubsubTopic_IngestionDataSourceSettings_CloudStorage_PubsubAvroFormat>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_format: std::option::Option<std::vec::Vec<PubsubTopic_IngestionDataSourceSettings_CloudStorage_TextFormat>>,
    dynamic: PubsubTopic_IngestionDataSourceSettings_CloudStorageDynamic,
}

impl PubsubTopic_IngestionDataSourceSettings_CloudStorage {
    #[doc =
        "Set the field `match_glob`.\nGlob pattern used to match objects that will be ingested. If unset, all\nobjects will be ingested. See the supported patterns:\nhttps://cloud.google.com/storage/docs/json_api/v1/objects/list#list-objects-and-prefixes-using-glob"]
    pub fn set_match_glob(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.match_glob = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `minimum_object_create_time`.\nThe timestamp set in RFC3339 text format. If set, only objects with a\nlarger or equal timestamp will be ingested. Unset by default, meaning\nall objects will be ingested."]
    pub fn set_minimum_object_create_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.minimum_object_create_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `avro_format`.\n"]
    pub fn set_avro_format(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<PubsubTopic_IngestionDataSourceSettings_CloudStorage_AvroFormat>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.avro_format = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.avro_format = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pubsub_avro_format`.\n"]
    pub fn set_pubsub_avro_format(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<PubsubTopic_IngestionDataSourceSettings_CloudStorage_PubsubAvroFormat>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pubsub_avro_format = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pubsub_avro_format = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `text_format`.\n"]
    pub fn set_text_format(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<PubsubTopic_IngestionDataSourceSettings_CloudStorage_TextFormat>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.text_format = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.text_format = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PubsubTopic_IngestionDataSourceSettings_CloudStorage {
    type O = terrars::BlockAssignable<PubsubTopic_IngestionDataSourceSettings_CloudStorage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubTopic_IngestionDataSourceSettings_CloudStorage {
    #[doc =
        "Cloud Storage bucket. The bucket name must be without any\nprefix like \"gs://\". See the bucket naming requirements:\nhttps://cloud.google.com/storage/docs/buckets#naming."]
    pub bucket: terrars::PrimField<std::string::String>,
}

impl BuildPubsubTopic_IngestionDataSourceSettings_CloudStorage {
    pub fn build(self) -> PubsubTopic_IngestionDataSourceSettings_CloudStorage {
        PubsubTopic_IngestionDataSourceSettings_CloudStorage {
            bucket: self.bucket,
            match_glob: core::default::Default::default(),
            minimum_object_create_time: core::default::Default::default(),
            avro_format: core::default::Default::default(),
            pubsub_avro_format: core::default::Default::default(),
            text_format: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PubsubTopic_IngestionDataSourceSettings_CloudStorageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubTopic_IngestionDataSourceSettings_CloudStorageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PubsubTopic_IngestionDataSourceSettings_CloudStorageRef {
        PubsubTopic_IngestionDataSourceSettings_CloudStorageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubTopic_IngestionDataSourceSettings_CloudStorageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bucket` after provisioning.\nCloud Storage bucket. The bucket name must be without any\nprefix like \"gs://\". See the bucket naming requirements:\nhttps://cloud.google.com/storage/docs/buckets#naming."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc =
        "Get a reference to the value of field `match_glob` after provisioning.\nGlob pattern used to match objects that will be ingested. If unset, all\nobjects will be ingested. See the supported patterns:\nhttps://cloud.google.com/storage/docs/json_api/v1/objects/list#list-objects-and-prefixes-using-glob"]
    pub fn match_glob(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.match_glob", self.base))
    }

    #[doc =
        "Get a reference to the value of field `minimum_object_create_time` after provisioning.\nThe timestamp set in RFC3339 text format. If set, only objects with a\nlarger or equal timestamp will be ingested. Unset by default, meaning\nall objects will be ingested."]
    pub fn minimum_object_create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum_object_create_time", self.base))
    }

    #[doc = "Get a reference to the value of field `avro_format` after provisioning.\n"]
    pub fn avro_format(&self) -> terrars::ListRef<PubsubTopic_IngestionDataSourceSettings_CloudStorage_AvroFormatRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.avro_format", self.base))
    }

    #[doc = "Get a reference to the value of field `pubsub_avro_format` after provisioning.\n"]
    pub fn pubsub_avro_format(
        &self,
    ) -> terrars::ListRef<PubsubTopic_IngestionDataSourceSettings_CloudStorage_PubsubAvroFormatRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pubsub_avro_format", self.base))
    }

    #[doc = "Get a reference to the value of field `text_format` after provisioning.\n"]
    pub fn text_format(&self) -> terrars::ListRef<PubsubTopic_IngestionDataSourceSettings_CloudStorage_TextFormatRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text_format", self.base))
    }
}

#[derive(Serialize)]
pub struct PubsubTopic_IngestionDataSourceSettings_ConfluentCloud {
    bootstrap_server: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_id: std::option::Option<terrars::PrimField<std::string::String>>,
    gcp_service_account: terrars::PrimField<std::string::String>,
    identity_pool_id: terrars::PrimField<std::string::String>,
    topic: terrars::PrimField<std::string::String>,
}

impl PubsubTopic_IngestionDataSourceSettings_ConfluentCloud {
    #[doc = "Set the field `cluster_id`.\nThe Confluent Cloud cluster ID."]
    pub fn set_cluster_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PubsubTopic_IngestionDataSourceSettings_ConfluentCloud {
    type O = terrars::BlockAssignable<PubsubTopic_IngestionDataSourceSettings_ConfluentCloud>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubTopic_IngestionDataSourceSettings_ConfluentCloud {
    #[doc = "The Confluent Cloud bootstrap server. The format is url:port."]
    pub bootstrap_server: terrars::PrimField<std::string::String>,
    #[doc = "The GCP service account to be used for Federated Identity authentication\nwith Confluent Cloud."]
    pub gcp_service_account: terrars::PrimField<std::string::String>,
    #[doc = "Identity pool ID to be used for Federated Identity authentication with Confluent Cloud."]
    pub identity_pool_id: terrars::PrimField<std::string::String>,
    #[doc = "Name of the Confluent Cloud topic that Pub/Sub will import from."]
    pub topic: terrars::PrimField<std::string::String>,
}

impl BuildPubsubTopic_IngestionDataSourceSettings_ConfluentCloud {
    pub fn build(self) -> PubsubTopic_IngestionDataSourceSettings_ConfluentCloud {
        PubsubTopic_IngestionDataSourceSettings_ConfluentCloud {
            bootstrap_server: self.bootstrap_server,
            cluster_id: core::default::Default::default(),
            gcp_service_account: self.gcp_service_account,
            identity_pool_id: self.identity_pool_id,
            topic: self.topic,
        }
    }
}

#[allow(dead_code)]
pub struct PubsubTopic_IngestionDataSourceSettings_ConfluentCloudRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubTopic_IngestionDataSourceSettings_ConfluentCloudRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PubsubTopic_IngestionDataSourceSettings_ConfluentCloudRef {
        PubsubTopic_IngestionDataSourceSettings_ConfluentCloudRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubTopic_IngestionDataSourceSettings_ConfluentCloudRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bootstrap_server` after provisioning.\nThe Confluent Cloud bootstrap server. The format is url:port."]
    pub fn bootstrap_server(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_server", self.base))
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\nThe Confluent Cloud cluster ID."]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `gcp_service_account` after provisioning.\nThe GCP service account to be used for Federated Identity authentication\nwith Confluent Cloud."]
    pub fn gcp_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_service_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `identity_pool_id` after provisioning.\nIdentity pool ID to be used for Federated Identity authentication with Confluent Cloud."]
    pub fn identity_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_pool_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `topic` after provisioning.\nName of the Confluent Cloud topic that Pub/Sub will import from."]
    pub fn topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}

#[derive(Serialize)]
pub struct PubsubTopic_IngestionDataSourceSettings_PlatformLogsSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    severity: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PubsubTopic_IngestionDataSourceSettings_PlatformLogsSettings {
    #[doc =
        "Set the field `severity`.\nThe minimum severity level of Platform Logs that will be written. If unspecified,\nno Platform Logs will be written. Default value: \"SEVERITY_UNSPECIFIED\" Possible values: [\"SEVERITY_UNSPECIFIED\", \"DISABLED\", \"DEBUG\", \"INFO\", \"WARNING\", \"ERROR\"]"]
    pub fn set_severity(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.severity = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PubsubTopic_IngestionDataSourceSettings_PlatformLogsSettings {
    type O = terrars::BlockAssignable<PubsubTopic_IngestionDataSourceSettings_PlatformLogsSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubTopic_IngestionDataSourceSettings_PlatformLogsSettings {}

impl BuildPubsubTopic_IngestionDataSourceSettings_PlatformLogsSettings {
    pub fn build(self) -> PubsubTopic_IngestionDataSourceSettings_PlatformLogsSettings {
        PubsubTopic_IngestionDataSourceSettings_PlatformLogsSettings { severity: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct PubsubTopic_IngestionDataSourceSettings_PlatformLogsSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubTopic_IngestionDataSourceSettings_PlatformLogsSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PubsubTopic_IngestionDataSourceSettings_PlatformLogsSettingsRef {
        PubsubTopic_IngestionDataSourceSettings_PlatformLogsSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubTopic_IngestionDataSourceSettings_PlatformLogsSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `severity` after provisioning.\nThe minimum severity level of Platform Logs that will be written. If unspecified,\nno Platform Logs will be written. Default value: \"SEVERITY_UNSPECIFIED\" Possible values: [\"SEVERITY_UNSPECIFIED\", \"DISABLED\", \"DEBUG\", \"INFO\", \"WARNING\", \"ERROR\"]"]
    pub fn severity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.severity", self.base))
    }
}

#[derive(Serialize, Default)]
struct PubsubTopic_IngestionDataSourceSettingsDynamic {
    aws_kinesis: std::option::Option<terrars::DynamicBlock<PubsubTopic_IngestionDataSourceSettings_AwsKinesis>>,
    aws_msk: std::option::Option<terrars::DynamicBlock<PubsubTopic_IngestionDataSourceSettings_AwsMsk>>,
    azure_event_hubs: std
    ::option
    ::Option<terrars::DynamicBlock<PubsubTopic_IngestionDataSourceSettings_AzureEventHubs>>,
    cloud_storage: std::option::Option<terrars::DynamicBlock<PubsubTopic_IngestionDataSourceSettings_CloudStorage>>,
    confluent_cloud: std
    ::option
    ::Option<terrars::DynamicBlock<PubsubTopic_IngestionDataSourceSettings_ConfluentCloud>>,
    platform_logs_settings: std
    ::option
    ::Option<terrars::DynamicBlock<PubsubTopic_IngestionDataSourceSettings_PlatformLogsSettings>>,
}

#[derive(Serialize)]
pub struct PubsubTopic_IngestionDataSourceSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_kinesis: std::option::Option<std::vec::Vec<PubsubTopic_IngestionDataSourceSettings_AwsKinesis>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_msk: std::option::Option<std::vec::Vec<PubsubTopic_IngestionDataSourceSettings_AwsMsk>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    azure_event_hubs: std::option::Option<std::vec::Vec<PubsubTopic_IngestionDataSourceSettings_AzureEventHubs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_storage: std::option::Option<std::vec::Vec<PubsubTopic_IngestionDataSourceSettings_CloudStorage>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confluent_cloud: std::option::Option<std::vec::Vec<PubsubTopic_IngestionDataSourceSettings_ConfluentCloud>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    platform_logs_settings: std
    ::option
    ::Option<std::vec::Vec<PubsubTopic_IngestionDataSourceSettings_PlatformLogsSettings>>,
    dynamic: PubsubTopic_IngestionDataSourceSettingsDynamic,
}

impl PubsubTopic_IngestionDataSourceSettings {
    #[doc = "Set the field `aws_kinesis`.\n"]
    pub fn set_aws_kinesis(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PubsubTopic_IngestionDataSourceSettings_AwsKinesis>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_kinesis = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_kinesis = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `aws_msk`.\n"]
    pub fn set_aws_msk(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PubsubTopic_IngestionDataSourceSettings_AwsMsk>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_msk = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_msk = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `azure_event_hubs`.\n"]
    pub fn set_azure_event_hubs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PubsubTopic_IngestionDataSourceSettings_AzureEventHubs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.azure_event_hubs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.azure_event_hubs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cloud_storage`.\n"]
    pub fn set_cloud_storage(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PubsubTopic_IngestionDataSourceSettings_CloudStorage>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloud_storage = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_storage = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `confluent_cloud`.\n"]
    pub fn set_confluent_cloud(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PubsubTopic_IngestionDataSourceSettings_ConfluentCloud>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.confluent_cloud = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.confluent_cloud = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `platform_logs_settings`.\n"]
    pub fn set_platform_logs_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<PubsubTopic_IngestionDataSourceSettings_PlatformLogsSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.platform_logs_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.platform_logs_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for PubsubTopic_IngestionDataSourceSettings {
    type O = terrars::BlockAssignable<PubsubTopic_IngestionDataSourceSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubTopic_IngestionDataSourceSettings {}

impl BuildPubsubTopic_IngestionDataSourceSettings {
    pub fn build(self) -> PubsubTopic_IngestionDataSourceSettings {
        PubsubTopic_IngestionDataSourceSettings {
            aws_kinesis: core::default::Default::default(),
            aws_msk: core::default::Default::default(),
            azure_event_hubs: core::default::Default::default(),
            cloud_storage: core::default::Default::default(),
            confluent_cloud: core::default::Default::default(),
            platform_logs_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PubsubTopic_IngestionDataSourceSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubTopic_IngestionDataSourceSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PubsubTopic_IngestionDataSourceSettingsRef {
        PubsubTopic_IngestionDataSourceSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubTopic_IngestionDataSourceSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aws_kinesis` after provisioning.\n"]
    pub fn aws_kinesis(&self) -> terrars::ListRef<PubsubTopic_IngestionDataSourceSettings_AwsKinesisRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_kinesis", self.base))
    }

    #[doc = "Get a reference to the value of field `aws_msk` after provisioning.\n"]
    pub fn aws_msk(&self) -> terrars::ListRef<PubsubTopic_IngestionDataSourceSettings_AwsMskRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_msk", self.base))
    }

    #[doc = "Get a reference to the value of field `azure_event_hubs` after provisioning.\n"]
    pub fn azure_event_hubs(&self) -> terrars::ListRef<PubsubTopic_IngestionDataSourceSettings_AzureEventHubsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.azure_event_hubs", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_storage` after provisioning.\n"]
    pub fn cloud_storage(&self) -> terrars::ListRef<PubsubTopic_IngestionDataSourceSettings_CloudStorageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_storage", self.base))
    }

    #[doc = "Get a reference to the value of field `confluent_cloud` after provisioning.\n"]
    pub fn confluent_cloud(&self) -> terrars::ListRef<PubsubTopic_IngestionDataSourceSettings_ConfluentCloudRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.confluent_cloud", self.base))
    }

    #[doc = "Get a reference to the value of field `platform_logs_settings` after provisioning.\n"]
    pub fn platform_logs_settings(
        &self,
    ) -> terrars::ListRef<PubsubTopic_IngestionDataSourceSettings_PlatformLogsSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.platform_logs_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct PubsubTopic_MessageStoragePolicy {
    allowed_persistence_regions: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce_in_transit: std::option::Option<terrars::PrimField<bool>>,
}

impl PubsubTopic_MessageStoragePolicy {
    #[doc =
        "Set the field `enforce_in_transit`.\nIf true, 'allowedPersistenceRegions' is also used to enforce in-transit\nguarantees for messages. That is, Pub/Sub will fail topics.publish\noperations on this topic and subscribe operations on any subscription\nattached to this topic in any region that is not in 'allowedPersistenceRegions'."]
    pub fn set_enforce_in_transit(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enforce_in_transit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PubsubTopic_MessageStoragePolicy {
    type O = terrars::BlockAssignable<PubsubTopic_MessageStoragePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubTopic_MessageStoragePolicy {
    #[doc =
        "A list of IDs of GCP regions where messages that are published to\nthe topic may be persisted in storage. Messages published by\npublishers running in non-allowed GCP regions (or running outside\nof GCP altogether) will be routed for storage in one of the\nallowed regions. An empty list means that no regions are allowed,\nand is not a valid configuration."]
    pub allowed_persistence_regions: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildPubsubTopic_MessageStoragePolicy {
    pub fn build(self) -> PubsubTopic_MessageStoragePolicy {
        PubsubTopic_MessageStoragePolicy {
            allowed_persistence_regions: self.allowed_persistence_regions,
            enforce_in_transit: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PubsubTopic_MessageStoragePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubTopic_MessageStoragePolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PubsubTopic_MessageStoragePolicyRef {
        PubsubTopic_MessageStoragePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubTopic_MessageStoragePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_persistence_regions` after provisioning.\nA list of IDs of GCP regions where messages that are published to\nthe topic may be persisted in storage. Messages published by\npublishers running in non-allowed GCP regions (or running outside\nof GCP altogether) will be routed for storage in one of the\nallowed regions. An empty list means that no regions are allowed,\nand is not a valid configuration."]
    pub fn allowed_persistence_regions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_persistence_regions", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enforce_in_transit` after provisioning.\nIf true, 'allowedPersistenceRegions' is also used to enforce in-transit\nguarantees for messages. That is, Pub/Sub will fail topics.publish\noperations on this topic and subscribe operations on any subscription\nattached to this topic in any region that is not in 'allowedPersistenceRegions'."]
    pub fn enforce_in_transit(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforce_in_transit", self.base))
    }
}

#[derive(Serialize)]
pub struct PubsubTopic_MessageTransforms_AiInference_UnstructuredInference {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl PubsubTopic_MessageTransforms_AiInference_UnstructuredInference {
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

impl terrars::ToListMappable for PubsubTopic_MessageTransforms_AiInference_UnstructuredInference {
    type O = terrars::BlockAssignable<PubsubTopic_MessageTransforms_AiInference_UnstructuredInference>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubTopic_MessageTransforms_AiInference_UnstructuredInference {}

impl BuildPubsubTopic_MessageTransforms_AiInference_UnstructuredInference {
    pub fn build(self) -> PubsubTopic_MessageTransforms_AiInference_UnstructuredInference {
        PubsubTopic_MessageTransforms_AiInference_UnstructuredInference {
            parameters: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PubsubTopic_MessageTransforms_AiInference_UnstructuredInferenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubTopic_MessageTransforms_AiInference_UnstructuredInferenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PubsubTopic_MessageTransforms_AiInference_UnstructuredInferenceRef {
        PubsubTopic_MessageTransforms_AiInference_UnstructuredInferenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubTopic_MessageTransforms_AiInference_UnstructuredInferenceRef {
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
struct PubsubTopic_MessageTransforms_AiInferenceDynamic {
    unstructured_inference: std
    ::option
    ::Option<terrars::DynamicBlock<PubsubTopic_MessageTransforms_AiInference_UnstructuredInference>>,
}

#[derive(Serialize)]
pub struct PubsubTopic_MessageTransforms_AiInference {
    endpoint: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unstructured_inference: std
    ::option
    ::Option<std::vec::Vec<PubsubTopic_MessageTransforms_AiInference_UnstructuredInference>>,
    dynamic: PubsubTopic_MessageTransforms_AiInferenceDynamic,
}

impl PubsubTopic_MessageTransforms_AiInference {
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
                    ::Into<terrars::BlockAssignable<PubsubTopic_MessageTransforms_AiInference_UnstructuredInference>>,
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

impl terrars::ToListMappable for PubsubTopic_MessageTransforms_AiInference {
    type O = terrars::BlockAssignable<PubsubTopic_MessageTransforms_AiInference>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubTopic_MessageTransforms_AiInference {
    #[doc =
        "The endpoint to a Vertex AI model of the form\n'projects/{project}/locations/{location}/endpoints/{endpoint}' or\n'projects/{project}/locations/{location}/publishers/{publisher}/models/{model}'.\nVertex AI API requests will be sent to this endpoint."]
    pub endpoint: terrars::PrimField<std::string::String>,
}

impl BuildPubsubTopic_MessageTransforms_AiInference {
    pub fn build(self) -> PubsubTopic_MessageTransforms_AiInference {
        PubsubTopic_MessageTransforms_AiInference {
            endpoint: self.endpoint,
            service_account_email: core::default::Default::default(),
            unstructured_inference: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PubsubTopic_MessageTransforms_AiInferenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubTopic_MessageTransforms_AiInferenceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PubsubTopic_MessageTransforms_AiInferenceRef {
        PubsubTopic_MessageTransforms_AiInferenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubTopic_MessageTransforms_AiInferenceRef {
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
    ) -> terrars::ListRef<PubsubTopic_MessageTransforms_AiInference_UnstructuredInferenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.unstructured_inference", self.base))
    }
}

#[derive(Serialize)]
pub struct PubsubTopic_MessageTransforms_JavascriptUdf {
    code: terrars::PrimField<std::string::String>,
    function_name: terrars::PrimField<std::string::String>,
}

impl PubsubTopic_MessageTransforms_JavascriptUdf { }

impl terrars::ToListMappable for PubsubTopic_MessageTransforms_JavascriptUdf {
    type O = terrars::BlockAssignable<PubsubTopic_MessageTransforms_JavascriptUdf>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubTopic_MessageTransforms_JavascriptUdf {
    #[doc =
        "JavaScript code that contains a function 'function_name' with the\nfollowing signature:\n'''\n  /**\n  * Transforms a Pub/Sub message.\n  *\n  * @return {(Object<string, (string | Object<string, string>)>|null)} - To\n  * filter a message, return 'null'. To transform a message return a map\n  * with the following keys:\n  *   - (required) 'data' : {string}\n  *   - (optional) 'attributes' : {Object<string, string>}\n  * Returning empty 'attributes' will remove all attributes from the\n  * message.\n  *\n  * @param  {(Object<string, (string | Object<string, string>)>} Pub/Sub\n  * message. Keys:\n  *   - (required) 'data' : {string}\n  *   - (required) 'attributes' : {Object<string, string>}\n  *\n  * @param  {Object<string, any>} metadata - Pub/Sub message metadata.\n  * Keys:\n  *   - (required) 'message_id'  : {string}\n  *   - (optional) 'publish_time': {string} YYYY-MM-DDTHH:MM:SSZ format\n  *   - (optional) 'ordering_key': {string}\n  */\n  function <function_name>(message, metadata) {\n  }\n'''"]
    pub code: terrars::PrimField<std::string::String>,
    #[doc = "Name of the JavaScript function that should be applied to Pub/Sub messages."]
    pub function_name: terrars::PrimField<std::string::String>,
}

impl BuildPubsubTopic_MessageTransforms_JavascriptUdf {
    pub fn build(self) -> PubsubTopic_MessageTransforms_JavascriptUdf {
        PubsubTopic_MessageTransforms_JavascriptUdf {
            code: self.code,
            function_name: self.function_name,
        }
    }
}

#[allow(dead_code)]
pub struct PubsubTopic_MessageTransforms_JavascriptUdfRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubTopic_MessageTransforms_JavascriptUdfRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> PubsubTopic_MessageTransforms_JavascriptUdfRef {
        PubsubTopic_MessageTransforms_JavascriptUdfRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubTopic_MessageTransforms_JavascriptUdfRef {
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
struct PubsubTopic_MessageTransformsDynamic {
    ai_inference: std::option::Option<terrars::DynamicBlock<PubsubTopic_MessageTransforms_AiInference>>,
    javascript_udf: std::option::Option<terrars::DynamicBlock<PubsubTopic_MessageTransforms_JavascriptUdf>>,
}

#[derive(Serialize)]
pub struct PubsubTopic_MessageTransforms {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ai_inference: std::option::Option<std::vec::Vec<PubsubTopic_MessageTransforms_AiInference>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    javascript_udf: std::option::Option<std::vec::Vec<PubsubTopic_MessageTransforms_JavascriptUdf>>,
    dynamic: PubsubTopic_MessageTransformsDynamic,
}

impl PubsubTopic_MessageTransforms {
    #[doc =
        "Set the field `disabled`.\nControls whether or not to use this transform. If not set or 'false',\nthe transform will be applied to messages. Default: 'true'."]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ai_inference`.\n"]
    pub fn set_ai_inference(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<PubsubTopic_MessageTransforms_AiInference>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<PubsubTopic_MessageTransforms_JavascriptUdf>>,
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

impl terrars::ToListMappable for PubsubTopic_MessageTransforms {
    type O = terrars::BlockAssignable<PubsubTopic_MessageTransforms>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubTopic_MessageTransforms {}

impl BuildPubsubTopic_MessageTransforms {
    pub fn build(self) -> PubsubTopic_MessageTransforms {
        PubsubTopic_MessageTransforms {
            disabled: core::default::Default::default(),
            ai_inference: core::default::Default::default(),
            javascript_udf: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PubsubTopic_MessageTransformsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubTopic_MessageTransformsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PubsubTopic_MessageTransformsRef {
        PubsubTopic_MessageTransformsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubTopic_MessageTransformsRef {
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
    pub fn ai_inference(&self) -> terrars::ListRef<PubsubTopic_MessageTransforms_AiInferenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ai_inference", self.base))
    }

    #[doc = "Get a reference to the value of field `javascript_udf` after provisioning.\n"]
    pub fn javascript_udf(&self) -> terrars::ListRef<PubsubTopic_MessageTransforms_JavascriptUdfRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.javascript_udf", self.base))
    }
}

#[derive(Serialize)]
pub struct PubsubTopic_SchemaSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encoding: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    first_revision_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_revision_id: std::option::Option<terrars::PrimField<std::string::String>>,
    schema: terrars::PrimField<std::string::String>,
}

impl PubsubTopic_SchemaSettings {
    #[doc =
        "Set the field `encoding`.\nThe encoding of messages validated against schema. Default value: \"ENCODING_UNSPECIFIED\" Possible values: [\"ENCODING_UNSPECIFIED\", \"JSON\", \"BINARY\"]"]
    pub fn set_encoding(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encoding = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `first_revision_id`.\nThe minimum (inclusive) revision allowed for validating messages. If empty or not present, allow any revision to be validated against last_revision or any revision created before."]
    pub fn set_first_revision_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.first_revision_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `last_revision_id`.\nThe maximum (inclusive) revision allowed for validating messages. If empty or not present, allow any revision to be validated against first_revision or any revision created after."]
    pub fn set_last_revision_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.last_revision_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for PubsubTopic_SchemaSettings {
    type O = terrars::BlockAssignable<PubsubTopic_SchemaSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubTopic_SchemaSettings {
    #[doc =
        "The name of the schema that messages published should be\nvalidated against. Format is projects/{project}/schemas/{schema}.\nThe value of this field will be _deleted-schema_\nif the schema has been deleted."]
    pub schema: terrars::PrimField<std::string::String>,
}

impl BuildPubsubTopic_SchemaSettings {
    pub fn build(self) -> PubsubTopic_SchemaSettings {
        PubsubTopic_SchemaSettings {
            encoding: core::default::Default::default(),
            first_revision_id: core::default::Default::default(),
            last_revision_id: core::default::Default::default(),
            schema: self.schema,
        }
    }
}

#[allow(dead_code)]
pub struct PubsubTopic_SchemaSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubTopic_SchemaSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PubsubTopic_SchemaSettingsRef {
        PubsubTopic_SchemaSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubTopic_SchemaSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `encoding` after provisioning.\nThe encoding of messages validated against schema. Default value: \"ENCODING_UNSPECIFIED\" Possible values: [\"ENCODING_UNSPECIFIED\", \"JSON\", \"BINARY\"]"]
    pub fn encoding(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encoding", self.base))
    }

    #[doc =
        "Get a reference to the value of field `first_revision_id` after provisioning.\nThe minimum (inclusive) revision allowed for validating messages. If empty or not present, allow any revision to be validated against last_revision or any revision created before."]
    pub fn first_revision_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.first_revision_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `last_revision_id` after provisioning.\nThe maximum (inclusive) revision allowed for validating messages. If empty or not present, allow any revision to be validated against first_revision or any revision created after."]
    pub fn last_revision_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_revision_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `schema` after provisioning.\nThe name of the schema that messages published should be\nvalidated against. Format is projects/{project}/schemas/{schema}.\nThe value of this field will be _deleted-schema_\nif the schema has been deleted."]
    pub fn schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }
}

#[derive(Serialize)]
pub struct PubsubTopic_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl PubsubTopic_Timeouts {
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

impl terrars::ToListMappable for PubsubTopic_Timeouts {
    type O = terrars::BlockAssignable<PubsubTopic_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildPubsubTopic_Timeouts {}

impl BuildPubsubTopic_Timeouts {
    pub fn build(self) -> PubsubTopic_Timeouts {
        PubsubTopic_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct PubsubTopic_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for PubsubTopic_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> PubsubTopic_TimeoutsRef {
        PubsubTopic_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl PubsubTopic_TimeoutsRef {
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
