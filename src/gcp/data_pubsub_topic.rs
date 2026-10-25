#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataPubsubTopicData_ {
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

struct DataPubsubTopic_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataPubsubTopicData_>,
}

#[derive(Clone)]
pub struct DataPubsubTopic(std::rc::Rc<DataPubsubTopic_>);

impl DataPubsubTopic {
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
        "Get a reference to the value of field `ingestion_data_source_settings` after provisioning.\nSettings for ingestion from a data source into this topic."]
    pub fn ingestion_data_source_settings(&self) -> terrars::ListRef<DataPubsubTopic_IngestionDataSourceSettingsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.ingestion_data_source_settings", self.extract_ref()),
        )
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

    #[doc =
        "Get a reference to the value of field `message_storage_policy` after provisioning.\nPolicy constraining the set of Google Cloud Platform regions where\nmessages published to the topic may be stored. If not present, then no\nconstraints are in effect."]
    pub fn message_storage_policy(&self) -> terrars::ListRef<DataPubsubTopic_MessageStoragePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.message_storage_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `message_transforms` after provisioning.\nTransforms to be applied to messages published to the topic. Transforms are applied in the\norder specified."]
    pub fn message_transforms(&self) -> terrars::ListRef<DataPubsubTopic_MessageTransformsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.message_transforms", self.extract_ref()))
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
        "Get a reference to the value of field `schema_settings` after provisioning.\nSettings for validating messages published against a schema."]
    pub fn schema_settings(&self) -> terrars::ListRef<DataPubsubTopic_SchemaSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema_settings", self.extract_ref()))
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
}

impl terrars::Referable for DataPubsubTopic {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataPubsubTopic { }

impl terrars::ToListMappable for DataPubsubTopic {
    type O = terrars::ListRef<DataPubsubTopicRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataPubsubTopic_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_pubsub_topic".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataPubsubTopic {
    pub tf_id: std::string::String,
    #[doc = "Name of the topic."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataPubsubTopic {
    pub fn build(self, stack: &mut terrars::Stack) -> DataPubsubTopic {
        let out = DataPubsubTopic(std::rc::Rc::new(DataPubsubTopic_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataPubsubTopicData_ {
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

pub struct DataPubsubTopicRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPubsubTopicRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataPubsubTopicRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
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
        "Get a reference to the value of field `ingestion_data_source_settings` after provisioning.\nSettings for ingestion from a data source into this topic."]
    pub fn ingestion_data_source_settings(&self) -> terrars::ListRef<DataPubsubTopic_IngestionDataSourceSettingsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.ingestion_data_source_settings", self.extract_ref()),
        )
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

    #[doc =
        "Get a reference to the value of field `message_storage_policy` after provisioning.\nPolicy constraining the set of Google Cloud Platform regions where\nmessages published to the topic may be stored. If not present, then no\nconstraints are in effect."]
    pub fn message_storage_policy(&self) -> terrars::ListRef<DataPubsubTopic_MessageStoragePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.message_storage_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `message_transforms` after provisioning.\nTransforms to be applied to messages published to the topic. Transforms are applied in the\norder specified."]
    pub fn message_transforms(&self) -> terrars::ListRef<DataPubsubTopic_MessageTransformsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.message_transforms", self.extract_ref()))
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
        "Get a reference to the value of field `schema_settings` after provisioning.\nSettings for validating messages published against a schema."]
    pub fn schema_settings(&self) -> terrars::ListRef<DataPubsubTopic_SchemaSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema_settings", self.extract_ref()))
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
}

#[derive(Serialize)]
pub struct DataPubsubTopic_IngestionDataSourceSettings_AwsKinesis {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consumer_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stream_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataPubsubTopic_IngestionDataSourceSettings_AwsKinesis {
    #[doc = "Set the field `aws_role_arn`.\n"]
    pub fn set_aws_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.aws_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `consumer_arn`.\n"]
    pub fn set_consumer_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.consumer_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gcp_service_account`.\n"]
    pub fn set_gcp_service_account(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.gcp_service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stream_arn`.\n"]
    pub fn set_stream_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.stream_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPubsubTopic_IngestionDataSourceSettings_AwsKinesis {
    type O = terrars::BlockAssignable<DataPubsubTopic_IngestionDataSourceSettings_AwsKinesis>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPubsubTopic_IngestionDataSourceSettings_AwsKinesis {}

impl BuildDataPubsubTopic_IngestionDataSourceSettings_AwsKinesis {
    pub fn build(self) -> DataPubsubTopic_IngestionDataSourceSettings_AwsKinesis {
        DataPubsubTopic_IngestionDataSourceSettings_AwsKinesis {
            aws_role_arn: core::default::Default::default(),
            consumer_arn: core::default::Default::default(),
            gcp_service_account: core::default::Default::default(),
            stream_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPubsubTopic_IngestionDataSourceSettings_AwsKinesisRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPubsubTopic_IngestionDataSourceSettings_AwsKinesisRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPubsubTopic_IngestionDataSourceSettings_AwsKinesisRef {
        DataPubsubTopic_IngestionDataSourceSettings_AwsKinesisRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPubsubTopic_IngestionDataSourceSettings_AwsKinesisRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aws_role_arn` after provisioning.\n"]
    pub fn aws_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `consumer_arn` after provisioning.\n"]
    pub fn consumer_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `gcp_service_account` after provisioning.\n"]
    pub fn gcp_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_service_account", self.base))
    }

    #[doc = "Get a reference to the value of field `stream_arn` after provisioning.\n"]
    pub fn stream_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stream_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPubsubTopic_IngestionDataSourceSettings_AwsMsk {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topic: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataPubsubTopic_IngestionDataSourceSettings_AwsMsk {
    #[doc = "Set the field `aws_role_arn`.\n"]
    pub fn set_aws_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.aws_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_arn`.\n"]
    pub fn set_cluster_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gcp_service_account`.\n"]
    pub fn set_gcp_service_account(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.gcp_service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `topic`.\n"]
    pub fn set_topic(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.topic = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPubsubTopic_IngestionDataSourceSettings_AwsMsk {
    type O = terrars::BlockAssignable<DataPubsubTopic_IngestionDataSourceSettings_AwsMsk>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPubsubTopic_IngestionDataSourceSettings_AwsMsk {}

impl BuildDataPubsubTopic_IngestionDataSourceSettings_AwsMsk {
    pub fn build(self) -> DataPubsubTopic_IngestionDataSourceSettings_AwsMsk {
        DataPubsubTopic_IngestionDataSourceSettings_AwsMsk {
            aws_role_arn: core::default::Default::default(),
            cluster_arn: core::default::Default::default(),
            gcp_service_account: core::default::Default::default(),
            topic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPubsubTopic_IngestionDataSourceSettings_AwsMskRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPubsubTopic_IngestionDataSourceSettings_AwsMskRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPubsubTopic_IngestionDataSourceSettings_AwsMskRef {
        DataPubsubTopic_IngestionDataSourceSettings_AwsMskRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPubsubTopic_IngestionDataSourceSettings_AwsMskRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aws_role_arn` after provisioning.\n"]
    pub fn aws_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `cluster_arn` after provisioning.\n"]
    pub fn cluster_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `gcp_service_account` after provisioning.\n"]
    pub fn gcp_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_service_account", self.base))
    }

    #[doc = "Get a reference to the value of field `topic` after provisioning.\n"]
    pub fn topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPubsubTopic_IngestionDataSourceSettings_AzureEventHubs {
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

impl DataPubsubTopic_IngestionDataSourceSettings_AzureEventHubs {
    #[doc = "Set the field `client_id`.\n"]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `event_hub`.\n"]
    pub fn set_event_hub(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.event_hub = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gcp_service_account`.\n"]
    pub fn set_gcp_service_account(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.gcp_service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `namespace`.\n"]
    pub fn set_namespace(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.namespace = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_group`.\n"]
    pub fn set_resource_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subscription_id`.\n"]
    pub fn set_subscription_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subscription_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tenant_id`.\n"]
    pub fn set_tenant_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tenant_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPubsubTopic_IngestionDataSourceSettings_AzureEventHubs {
    type O = terrars::BlockAssignable<DataPubsubTopic_IngestionDataSourceSettings_AzureEventHubs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPubsubTopic_IngestionDataSourceSettings_AzureEventHubs {}

impl BuildDataPubsubTopic_IngestionDataSourceSettings_AzureEventHubs {
    pub fn build(self) -> DataPubsubTopic_IngestionDataSourceSettings_AzureEventHubs {
        DataPubsubTopic_IngestionDataSourceSettings_AzureEventHubs {
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
pub struct DataPubsubTopic_IngestionDataSourceSettings_AzureEventHubsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPubsubTopic_IngestionDataSourceSettings_AzureEventHubsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPubsubTopic_IngestionDataSourceSettings_AzureEventHubsRef {
        DataPubsubTopic_IngestionDataSourceSettings_AzureEventHubsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPubsubTopic_IngestionDataSourceSettings_AzureEventHubsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `event_hub` after provisioning.\n"]
    pub fn event_hub(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_hub", self.base))
    }

    #[doc = "Get a reference to the value of field `gcp_service_account` after provisioning.\n"]
    pub fn gcp_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_service_account", self.base))
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_group` after provisioning.\n"]
    pub fn resource_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_group", self.base))
    }

    #[doc = "Get a reference to the value of field `subscription_id` after provisioning.\n"]
    pub fn subscription_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscription_id", self.base))
    }

    #[doc = "Get a reference to the value of field `tenant_id` after provisioning.\n"]
    pub fn tenant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenant_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_AvroFormat {}

impl DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_AvroFormat { }

impl terrars::ToListMappable for DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_AvroFormat {
    type O = terrars::BlockAssignable<DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_AvroFormat>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPubsubTopic_IngestionDataSourceSettings_CloudStorage_AvroFormat {}

impl BuildDataPubsubTopic_IngestionDataSourceSettings_CloudStorage_AvroFormat {
    pub fn build(self) -> DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_AvroFormat {
        DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_AvroFormat {}
    }
}

#[allow(dead_code)]
pub struct DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_AvroFormatRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_AvroFormatRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_AvroFormatRef {
        DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_AvroFormatRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_AvroFormatRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_PubsubAvroFormat {}

impl DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_PubsubAvroFormat { }

impl terrars::ToListMappable for DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_PubsubAvroFormat {
    type O = terrars::BlockAssignable<DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_PubsubAvroFormat>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPubsubTopic_IngestionDataSourceSettings_CloudStorage_PubsubAvroFormat {}

impl BuildDataPubsubTopic_IngestionDataSourceSettings_CloudStorage_PubsubAvroFormat {
    pub fn build(self) -> DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_PubsubAvroFormat {
        DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_PubsubAvroFormat {}
    }
}

#[allow(dead_code)]
pub struct DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_PubsubAvroFormatRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_PubsubAvroFormatRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_PubsubAvroFormatRef {
        DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_PubsubAvroFormatRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_PubsubAvroFormatRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_TextFormat {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delimiter: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_TextFormat {
    #[doc = "Set the field `delimiter`.\n"]
    pub fn set_delimiter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delimiter = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_TextFormat {
    type O = terrars::BlockAssignable<DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_TextFormat>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPubsubTopic_IngestionDataSourceSettings_CloudStorage_TextFormat {}

impl BuildDataPubsubTopic_IngestionDataSourceSettings_CloudStorage_TextFormat {
    pub fn build(self) -> DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_TextFormat {
        DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_TextFormat {
            delimiter: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_TextFormatRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_TextFormatRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_TextFormatRef {
        DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_TextFormatRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_TextFormatRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delimiter` after provisioning.\n"]
    pub fn delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delimiter", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPubsubTopic_IngestionDataSourceSettings_CloudStorage {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    avro_format: std
    ::option
    ::Option<terrars::ListField<DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_AvroFormat>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_glob: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum_object_create_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pubsub_avro_format: std
    ::option
    ::Option<terrars::ListField<DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_PubsubAvroFormat>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_format: std
    ::option
    ::Option<terrars::ListField<DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_TextFormat>>,
}

impl DataPubsubTopic_IngestionDataSourceSettings_CloudStorage {
    #[doc = "Set the field `avro_format`.\n"]
    pub fn set_avro_format(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_AvroFormat>>,
    ) -> Self {
        self.avro_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_glob`.\n"]
    pub fn set_match_glob(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.match_glob = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minimum_object_create_time`.\n"]
    pub fn set_minimum_object_create_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.minimum_object_create_time = std::option::Option::Some(v.into());
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
                        terrars::ListField<DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_PubsubAvroFormat>,
                    >,
    ) -> Self {
        self.pubsub_avro_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `text_format`.\n"]
    pub fn set_text_format(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_TextFormat>>,
    ) -> Self {
        self.text_format = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPubsubTopic_IngestionDataSourceSettings_CloudStorage {
    type O = terrars::BlockAssignable<DataPubsubTopic_IngestionDataSourceSettings_CloudStorage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPubsubTopic_IngestionDataSourceSettings_CloudStorage {}

impl BuildDataPubsubTopic_IngestionDataSourceSettings_CloudStorage {
    pub fn build(self) -> DataPubsubTopic_IngestionDataSourceSettings_CloudStorage {
        DataPubsubTopic_IngestionDataSourceSettings_CloudStorage {
            avro_format: core::default::Default::default(),
            bucket: core::default::Default::default(),
            match_glob: core::default::Default::default(),
            minimum_object_create_time: core::default::Default::default(),
            pubsub_avro_format: core::default::Default::default(),
            text_format: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPubsubTopic_IngestionDataSourceSettings_CloudStorageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPubsubTopic_IngestionDataSourceSettings_CloudStorageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPubsubTopic_IngestionDataSourceSettings_CloudStorageRef {
        DataPubsubTopic_IngestionDataSourceSettings_CloudStorageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPubsubTopic_IngestionDataSourceSettings_CloudStorageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `avro_format` after provisioning.\n"]
    pub fn avro_format(
        &self,
    ) -> terrars::ListRef<DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_AvroFormatRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.avro_format", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `match_glob` after provisioning.\n"]
    pub fn match_glob(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.match_glob", self.base))
    }

    #[doc = "Get a reference to the value of field `minimum_object_create_time` after provisioning.\n"]
    pub fn minimum_object_create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum_object_create_time", self.base))
    }

    #[doc = "Get a reference to the value of field `pubsub_avro_format` after provisioning.\n"]
    pub fn pubsub_avro_format(
        &self,
    ) -> terrars::ListRef<DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_PubsubAvroFormatRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pubsub_avro_format", self.base))
    }

    #[doc = "Get a reference to the value of field `text_format` after provisioning.\n"]
    pub fn text_format(
        &self,
    ) -> terrars::ListRef<DataPubsubTopic_IngestionDataSourceSettings_CloudStorage_TextFormatRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.text_format", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPubsubTopic_IngestionDataSourceSettings_ConfluentCloud {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bootstrap_server: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_pool_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topic: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataPubsubTopic_IngestionDataSourceSettings_ConfluentCloud {
    #[doc = "Set the field `bootstrap_server`.\n"]
    pub fn set_bootstrap_server(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bootstrap_server = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_id`.\n"]
    pub fn set_cluster_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gcp_service_account`.\n"]
    pub fn set_gcp_service_account(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.gcp_service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `identity_pool_id`.\n"]
    pub fn set_identity_pool_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.identity_pool_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `topic`.\n"]
    pub fn set_topic(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.topic = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPubsubTopic_IngestionDataSourceSettings_ConfluentCloud {
    type O = terrars::BlockAssignable<DataPubsubTopic_IngestionDataSourceSettings_ConfluentCloud>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPubsubTopic_IngestionDataSourceSettings_ConfluentCloud {}

impl BuildDataPubsubTopic_IngestionDataSourceSettings_ConfluentCloud {
    pub fn build(self) -> DataPubsubTopic_IngestionDataSourceSettings_ConfluentCloud {
        DataPubsubTopic_IngestionDataSourceSettings_ConfluentCloud {
            bootstrap_server: core::default::Default::default(),
            cluster_id: core::default::Default::default(),
            gcp_service_account: core::default::Default::default(),
            identity_pool_id: core::default::Default::default(),
            topic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPubsubTopic_IngestionDataSourceSettings_ConfluentCloudRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPubsubTopic_IngestionDataSourceSettings_ConfluentCloudRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPubsubTopic_IngestionDataSourceSettings_ConfluentCloudRef {
        DataPubsubTopic_IngestionDataSourceSettings_ConfluentCloudRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPubsubTopic_IngestionDataSourceSettings_ConfluentCloudRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bootstrap_server` after provisioning.\n"]
    pub fn bootstrap_server(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bootstrap_server", self.base))
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.base))
    }

    #[doc = "Get a reference to the value of field `gcp_service_account` after provisioning.\n"]
    pub fn gcp_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_service_account", self.base))
    }

    #[doc = "Get a reference to the value of field `identity_pool_id` after provisioning.\n"]
    pub fn identity_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_pool_id", self.base))
    }

    #[doc = "Get a reference to the value of field `topic` after provisioning.\n"]
    pub fn topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPubsubTopic_IngestionDataSourceSettings_PlatformLogsSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    severity: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataPubsubTopic_IngestionDataSourceSettings_PlatformLogsSettings {
    #[doc = "Set the field `severity`.\n"]
    pub fn set_severity(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.severity = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPubsubTopic_IngestionDataSourceSettings_PlatformLogsSettings {
    type O = terrars::BlockAssignable<DataPubsubTopic_IngestionDataSourceSettings_PlatformLogsSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPubsubTopic_IngestionDataSourceSettings_PlatformLogsSettings {}

impl BuildDataPubsubTopic_IngestionDataSourceSettings_PlatformLogsSettings {
    pub fn build(self) -> DataPubsubTopic_IngestionDataSourceSettings_PlatformLogsSettings {
        DataPubsubTopic_IngestionDataSourceSettings_PlatformLogsSettings {
            severity: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPubsubTopic_IngestionDataSourceSettings_PlatformLogsSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPubsubTopic_IngestionDataSourceSettings_PlatformLogsSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPubsubTopic_IngestionDataSourceSettings_PlatformLogsSettingsRef {
        DataPubsubTopic_IngestionDataSourceSettings_PlatformLogsSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPubsubTopic_IngestionDataSourceSettings_PlatformLogsSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `severity` after provisioning.\n"]
    pub fn severity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.severity", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPubsubTopic_IngestionDataSourceSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_kinesis: std::option::Option<terrars::ListField<DataPubsubTopic_IngestionDataSourceSettings_AwsKinesis>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_msk: std::option::Option<terrars::ListField<DataPubsubTopic_IngestionDataSourceSettings_AwsMsk>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    azure_event_hubs: std::option::Option<terrars::ListField<DataPubsubTopic_IngestionDataSourceSettings_AzureEventHubs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_storage: std::option::Option<terrars::ListField<DataPubsubTopic_IngestionDataSourceSettings_CloudStorage>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confluent_cloud: std::option::Option<terrars::ListField<DataPubsubTopic_IngestionDataSourceSettings_ConfluentCloud>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    platform_logs_settings: std
    ::option
    ::Option<terrars::ListField<DataPubsubTopic_IngestionDataSourceSettings_PlatformLogsSettings>>,
}

impl DataPubsubTopic_IngestionDataSourceSettings {
    #[doc = "Set the field `aws_kinesis`.\n"]
    pub fn set_aws_kinesis(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataPubsubTopic_IngestionDataSourceSettings_AwsKinesis>>,
    ) -> Self {
        self.aws_kinesis = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `aws_msk`.\n"]
    pub fn set_aws_msk(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataPubsubTopic_IngestionDataSourceSettings_AwsMsk>>,
    ) -> Self {
        self.aws_msk = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `azure_event_hubs`.\n"]
    pub fn set_azure_event_hubs(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataPubsubTopic_IngestionDataSourceSettings_AzureEventHubs>>,
    ) -> Self {
        self.azure_event_hubs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloud_storage`.\n"]
    pub fn set_cloud_storage(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataPubsubTopic_IngestionDataSourceSettings_CloudStorage>>,
    ) -> Self {
        self.cloud_storage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `confluent_cloud`.\n"]
    pub fn set_confluent_cloud(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataPubsubTopic_IngestionDataSourceSettings_ConfluentCloud>>,
    ) -> Self {
        self.confluent_cloud = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `platform_logs_settings`.\n"]
    pub fn set_platform_logs_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataPubsubTopic_IngestionDataSourceSettings_PlatformLogsSettings>>,
    ) -> Self {
        self.platform_logs_settings = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPubsubTopic_IngestionDataSourceSettings {
    type O = terrars::BlockAssignable<DataPubsubTopic_IngestionDataSourceSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPubsubTopic_IngestionDataSourceSettings {}

impl BuildDataPubsubTopic_IngestionDataSourceSettings {
    pub fn build(self) -> DataPubsubTopic_IngestionDataSourceSettings {
        DataPubsubTopic_IngestionDataSourceSettings {
            aws_kinesis: core::default::Default::default(),
            aws_msk: core::default::Default::default(),
            azure_event_hubs: core::default::Default::default(),
            cloud_storage: core::default::Default::default(),
            confluent_cloud: core::default::Default::default(),
            platform_logs_settings: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPubsubTopic_IngestionDataSourceSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPubsubTopic_IngestionDataSourceSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPubsubTopic_IngestionDataSourceSettingsRef {
        DataPubsubTopic_IngestionDataSourceSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPubsubTopic_IngestionDataSourceSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aws_kinesis` after provisioning.\n"]
    pub fn aws_kinesis(&self) -> terrars::ListRef<DataPubsubTopic_IngestionDataSourceSettings_AwsKinesisRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_kinesis", self.base))
    }

    #[doc = "Get a reference to the value of field `aws_msk` after provisioning.\n"]
    pub fn aws_msk(&self) -> terrars::ListRef<DataPubsubTopic_IngestionDataSourceSettings_AwsMskRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_msk", self.base))
    }

    #[doc = "Get a reference to the value of field `azure_event_hubs` after provisioning.\n"]
    pub fn azure_event_hubs(&self) -> terrars::ListRef<DataPubsubTopic_IngestionDataSourceSettings_AzureEventHubsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.azure_event_hubs", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_storage` after provisioning.\n"]
    pub fn cloud_storage(&self) -> terrars::ListRef<DataPubsubTopic_IngestionDataSourceSettings_CloudStorageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_storage", self.base))
    }

    #[doc = "Get a reference to the value of field `confluent_cloud` after provisioning.\n"]
    pub fn confluent_cloud(&self) -> terrars::ListRef<DataPubsubTopic_IngestionDataSourceSettings_ConfluentCloudRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.confluent_cloud", self.base))
    }

    #[doc = "Get a reference to the value of field `platform_logs_settings` after provisioning.\n"]
    pub fn platform_logs_settings(
        &self,
    ) -> terrars::ListRef<DataPubsubTopic_IngestionDataSourceSettings_PlatformLogsSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.platform_logs_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPubsubTopic_MessageStoragePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_persistence_regions: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enforce_in_transit: std::option::Option<terrars::PrimField<bool>>,
}

impl DataPubsubTopic_MessageStoragePolicy {
    #[doc = "Set the field `allowed_persistence_regions`.\n"]
    pub fn set_allowed_persistence_regions(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_persistence_regions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enforce_in_transit`.\n"]
    pub fn set_enforce_in_transit(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enforce_in_transit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPubsubTopic_MessageStoragePolicy {
    type O = terrars::BlockAssignable<DataPubsubTopic_MessageStoragePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPubsubTopic_MessageStoragePolicy {}

impl BuildDataPubsubTopic_MessageStoragePolicy {
    pub fn build(self) -> DataPubsubTopic_MessageStoragePolicy {
        DataPubsubTopic_MessageStoragePolicy {
            allowed_persistence_regions: core::default::Default::default(),
            enforce_in_transit: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPubsubTopic_MessageStoragePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPubsubTopic_MessageStoragePolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPubsubTopic_MessageStoragePolicyRef {
        DataPubsubTopic_MessageStoragePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPubsubTopic_MessageStoragePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_persistence_regions` after provisioning.\n"]
    pub fn allowed_persistence_regions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_persistence_regions", self.base))
    }

    #[doc = "Get a reference to the value of field `enforce_in_transit` after provisioning.\n"]
    pub fn enforce_in_transit(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforce_in_transit", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPubsubTopic_MessageTransforms_AiInference_UnstructuredInference {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataPubsubTopic_MessageTransforms_AiInference_UnstructuredInference {
    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.parameters = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPubsubTopic_MessageTransforms_AiInference_UnstructuredInference {
    type O = terrars::BlockAssignable<DataPubsubTopic_MessageTransforms_AiInference_UnstructuredInference>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPubsubTopic_MessageTransforms_AiInference_UnstructuredInference {}

impl BuildDataPubsubTopic_MessageTransforms_AiInference_UnstructuredInference {
    pub fn build(self) -> DataPubsubTopic_MessageTransforms_AiInference_UnstructuredInference {
        DataPubsubTopic_MessageTransforms_AiInference_UnstructuredInference {
            parameters: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPubsubTopic_MessageTransforms_AiInference_UnstructuredInferenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPubsubTopic_MessageTransforms_AiInference_UnstructuredInferenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPubsubTopic_MessageTransforms_AiInference_UnstructuredInferenceRef {
        DataPubsubTopic_MessageTransforms_AiInference_UnstructuredInferenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPubsubTopic_MessageTransforms_AiInference_UnstructuredInferenceRef {
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
pub struct DataPubsubTopic_MessageTransforms_AiInference {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unstructured_inference: std
    ::option
    ::Option<terrars::ListField<DataPubsubTopic_MessageTransforms_AiInference_UnstructuredInference>>,
}

impl DataPubsubTopic_MessageTransforms_AiInference {
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
                    ::Into<terrars::ListField<DataPubsubTopic_MessageTransforms_AiInference_UnstructuredInference>>,
    ) -> Self {
        self.unstructured_inference = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPubsubTopic_MessageTransforms_AiInference {
    type O = terrars::BlockAssignable<DataPubsubTopic_MessageTransforms_AiInference>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPubsubTopic_MessageTransforms_AiInference {}

impl BuildDataPubsubTopic_MessageTransforms_AiInference {
    pub fn build(self) -> DataPubsubTopic_MessageTransforms_AiInference {
        DataPubsubTopic_MessageTransforms_AiInference {
            endpoint: core::default::Default::default(),
            service_account_email: core::default::Default::default(),
            unstructured_inference: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPubsubTopic_MessageTransforms_AiInferenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPubsubTopic_MessageTransforms_AiInferenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPubsubTopic_MessageTransforms_AiInferenceRef {
        DataPubsubTopic_MessageTransforms_AiInferenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPubsubTopic_MessageTransforms_AiInferenceRef {
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
    ) -> terrars::ListRef<DataPubsubTopic_MessageTransforms_AiInference_UnstructuredInferenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.unstructured_inference", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPubsubTopic_MessageTransforms_JavascriptUdf {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    function_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataPubsubTopic_MessageTransforms_JavascriptUdf {
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

impl terrars::ToListMappable for DataPubsubTopic_MessageTransforms_JavascriptUdf {
    type O = terrars::BlockAssignable<DataPubsubTopic_MessageTransforms_JavascriptUdf>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPubsubTopic_MessageTransforms_JavascriptUdf {}

impl BuildDataPubsubTopic_MessageTransforms_JavascriptUdf {
    pub fn build(self) -> DataPubsubTopic_MessageTransforms_JavascriptUdf {
        DataPubsubTopic_MessageTransforms_JavascriptUdf {
            code: core::default::Default::default(),
            function_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPubsubTopic_MessageTransforms_JavascriptUdfRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPubsubTopic_MessageTransforms_JavascriptUdfRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataPubsubTopic_MessageTransforms_JavascriptUdfRef {
        DataPubsubTopic_MessageTransforms_JavascriptUdfRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPubsubTopic_MessageTransforms_JavascriptUdfRef {
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
pub struct DataPubsubTopic_MessageTransforms {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ai_inference: std::option::Option<terrars::ListField<DataPubsubTopic_MessageTransforms_AiInference>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    javascript_udf: std::option::Option<terrars::ListField<DataPubsubTopic_MessageTransforms_JavascriptUdf>>,
}

impl DataPubsubTopic_MessageTransforms {
    #[doc = "Set the field `ai_inference`.\n"]
    pub fn set_ai_inference(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataPubsubTopic_MessageTransforms_AiInference>>,
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
        v: impl std::convert::Into<terrars::ListField<DataPubsubTopic_MessageTransforms_JavascriptUdf>>,
    ) -> Self {
        self.javascript_udf = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPubsubTopic_MessageTransforms {
    type O = terrars::BlockAssignable<DataPubsubTopic_MessageTransforms>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPubsubTopic_MessageTransforms {}

impl BuildDataPubsubTopic_MessageTransforms {
    pub fn build(self) -> DataPubsubTopic_MessageTransforms {
        DataPubsubTopic_MessageTransforms {
            ai_inference: core::default::Default::default(),
            disabled: core::default::Default::default(),
            javascript_udf: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPubsubTopic_MessageTransformsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPubsubTopic_MessageTransformsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPubsubTopic_MessageTransformsRef {
        DataPubsubTopic_MessageTransformsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPubsubTopic_MessageTransformsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ai_inference` after provisioning.\n"]
    pub fn ai_inference(&self) -> terrars::ListRef<DataPubsubTopic_MessageTransforms_AiInferenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ai_inference", self.base))
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `javascript_udf` after provisioning.\n"]
    pub fn javascript_udf(&self) -> terrars::ListRef<DataPubsubTopic_MessageTransforms_JavascriptUdfRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.javascript_udf", self.base))
    }
}

#[derive(Serialize)]
pub struct DataPubsubTopic_SchemaSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encoding: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    first_revision_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_revision_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataPubsubTopic_SchemaSettings {
    #[doc = "Set the field `encoding`.\n"]
    pub fn set_encoding(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encoding = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `first_revision_id`.\n"]
    pub fn set_first_revision_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.first_revision_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_revision_id`.\n"]
    pub fn set_last_revision_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.last_revision_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schema`.\n"]
    pub fn set_schema(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.schema = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataPubsubTopic_SchemaSettings {
    type O = terrars::BlockAssignable<DataPubsubTopic_SchemaSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataPubsubTopic_SchemaSettings {}

impl BuildDataPubsubTopic_SchemaSettings {
    pub fn build(self) -> DataPubsubTopic_SchemaSettings {
        DataPubsubTopic_SchemaSettings {
            encoding: core::default::Default::default(),
            first_revision_id: core::default::Default::default(),
            last_revision_id: core::default::Default::default(),
            schema: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataPubsubTopic_SchemaSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataPubsubTopic_SchemaSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataPubsubTopic_SchemaSettingsRef {
        DataPubsubTopic_SchemaSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataPubsubTopic_SchemaSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `encoding` after provisioning.\n"]
    pub fn encoding(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encoding", self.base))
    }

    #[doc = "Get a reference to the value of field `first_revision_id` after provisioning.\n"]
    pub fn first_revision_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.first_revision_id", self.base))
    }

    #[doc = "Get a reference to the value of field `last_revision_id` after provisioning.\n"]
    pub fn last_revision_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_revision_id", self.base))
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }
}
