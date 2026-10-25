#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct SecretManagerSecretData_ {
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
    annotations: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expire_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    secret_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ttl: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version_aliases: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version_destroy_ttl: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replication: std::option::Option<std::vec::Vec<SecretManagerSecret_Replication>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rotation: std::option::Option<std::vec::Vec<SecretManagerSecret_Rotation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<SecretManagerSecret_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topics: std::option::Option<std::vec::Vec<SecretManagerSecret_Topics>>,
    dynamic: SecretManagerSecretDynamic,
}

struct SecretManagerSecret_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SecretManagerSecretData_>,
}

#[derive(Clone)]
pub struct SecretManagerSecret(std::rc::Rc<SecretManagerSecret_>);

impl SecretManagerSecret {
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
        "Set the field `annotations`.\nCustom metadata about the secret.\n\nAnnotations are distinct from various forms of labels. Annotations exist to allow\nclient tools to store their own state information without requiring a database.\n\nAnnotation keys must be between 1 and 63 characters long, have a UTF-8 encoding of\nmaximum 128 bytes, begin and end with an alphanumeric character ([a-z0-9A-Z]), and\nmay have dashes (-), underscores (_), dots (.), and alphanumerics in between these\nsymbols.\n\nThe total size of annotation keys and values must be less than 16KiB.\n\nAn object containing a list of \"key\": value pairs. Example:\n{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().annotations = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_protection`.\nWhether Terraform will be prevented from destroying the secret. Defaults to false.\nWhen the field is set to true in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the secret will fail."]
    pub fn set_deletion_protection(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `expire_time`.\nTimestamp in UTC when the Secret is scheduled to expire. This is always provided on output, regardless of what was sent on input.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\".\nOnly one of 'expire_time' or 'ttl' can be provided."]
    pub fn set_expire_time(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().expire_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nThe labels assigned to this Secret.\n\nLabel keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes,\nand must conform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}][\\p{Ll}\\p{Lo}\\p{N}_-]{0,62}\n\nLabel values must be between 0 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes,\nand must conform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63}\n\nNo more than 64 labels can be assigned to a given resource.\n\nAn object containing a list of \"key\": value pairs. Example:\n{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `secret_type`.\nThis defines the type of the secret. Enforces certain structural requirements on the SecretVersions.\nFor secret of type UNSPECIFIED, the SecretVersions can be of any type."]
    pub fn set_secret_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().secret_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tags`.\nA map of resource manager tags.\nResource manager tag keys and values have the same definition as resource manager tags.\nKeys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/{tag_value_id}."]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ttl`.\nThe TTL for the Secret.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\".\nOnly one of 'ttl' or 'expire_time' can be provided."]
    pub fn set_ttl(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `version_aliases`.\nMapping from version alias to version name.\n\nA version alias is a string with a maximum length of 63 characters and can contain\nuppercase and lowercase letters, numerals, and the hyphen (-) and underscore ('_')\ncharacters. An alias string must start with a letter and cannot be the string\n'latest' or 'NEW'. No more than 50 aliases can be assigned to a given secret.\n\nAn object containing a list of \"key\": value pairs. Example:\n{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn set_version_aliases(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().version_aliases = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `version_destroy_ttl`.\nSecret Version TTL after destruction request.\nThis is a part of the delayed delete feature on Secret Version.\nFor secret with versionDestroyTtl>0, version destruction doesn't happen immediately\non calling destroy instead the version goes to a disabled state and\nthe actual destruction happens after this TTL expires."]
    pub fn set_version_destroy_ttl(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().version_destroy_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replication`.\n"]
    pub fn set_replication(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecretManagerSecret_Replication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().replication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.replication = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rotation`.\n"]
    pub fn set_rotation(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecretManagerSecret_Rotation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rotation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rotation = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<SecretManagerSecret_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `topics`.\n"]
    pub fn set_topics(self, v: impl std::convert::Into<terrars::BlockAssignable<SecretManagerSecret_Topics>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().topics = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.topics = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nCustom metadata about the secret.\n\nAnnotations are distinct from various forms of labels. Annotations exist to allow\nclient tools to store their own state information without requiring a database.\n\nAnnotation keys must be between 1 and 63 characters long, have a UTF-8 encoding of\nmaximum 128 bytes, begin and end with an alphanumeric character ([a-z0-9A-Z]), and\nmay have dashes (-), underscores (_), dots (.), and alphanumerics in between these\nsymbols.\n\nThe total size of annotation keys and values must be less than 16KiB.\n\nAn object containing a list of \"key\": value pairs. Example:\n{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time at which the Secret was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the secret. Defaults to false.\nWhen the field is set to true in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the secret will fail."]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expire_time` after provisioning.\nTimestamp in UTC when the Secret is scheduled to expire. This is always provided on output, regardless of what was sent on input.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\".\nOnly one of 'expire_time' or 'ttl' can be provided."]
    pub fn expire_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expire_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe labels assigned to this Secret.\n\nLabel keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes,\nand must conform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}][\\p{Ll}\\p{Lo}\\p{N}_-]{0,62}\n\nLabel values must be between 0 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes,\nand must conform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63}\n\nNo more than 64 labels can be assigned to a given resource.\n\nAn object containing a list of \"key\": value pairs. Example:\n{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Secret. Format:\n'projects/{{project}}/secrets/{{secret_id}}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `secret_id` after provisioning.\nThis must be unique within the project."]
    pub fn secret_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `secret_type` after provisioning.\nThis defines the type of the secret. Enforces certain structural requirements on the SecretVersions.\nFor secret of type UNSPECIFIED, the SecretVersions can be of any type."]
    pub fn secret_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nA map of resource manager tags.\nResource manager tag keys and values have the same definition as resource manager tags.\nKeys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/{tag_value_id}."]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ttl` after provisioning.\nThe TTL for the Secret.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\".\nOnly one of 'ttl' or 'expire_time' can be provided."]
    pub fn ttl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version_aliases` after provisioning.\nMapping from version alias to version name.\n\nA version alias is a string with a maximum length of 63 characters and can contain\nuppercase and lowercase letters, numerals, and the hyphen (-) and underscore ('_')\ncharacters. An alias string must start with a letter and cannot be the string\n'latest' or 'NEW'. No more than 50 aliases can be assigned to a given secret.\n\nAn object containing a list of \"key\": value pairs. Example:\n{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn version_aliases(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.version_aliases", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version_destroy_ttl` after provisioning.\nSecret Version TTL after destruction request.\nThis is a part of the delayed delete feature on Secret Version.\nFor secret with versionDestroyTtl>0, version destruction doesn't happen immediately\non calling destroy instead the version goes to a disabled state and\nthe actual destruction happens after this TTL expires."]
    pub fn version_destroy_ttl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_destroy_ttl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication` after provisioning.\n"]
    pub fn replication(&self) -> terrars::ListRef<SecretManagerSecret_ReplicationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replication", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rotation` after provisioning.\n"]
    pub fn rotation(&self) -> terrars::ListRef<SecretManagerSecret_RotationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rotation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SecretManagerSecret_TimeoutsRef {
        SecretManagerSecret_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `topics` after provisioning.\n"]
    pub fn topics(&self) -> terrars::ListRef<SecretManagerSecret_TopicsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.topics", self.extract_ref()))
    }
}

impl terrars::Referable for SecretManagerSecret {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SecretManagerSecret { }

impl terrars::ToListMappable for SecretManagerSecret {
    type O = terrars::ListRef<SecretManagerSecretRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SecretManagerSecret_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_secret_manager_secret".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSecretManagerSecret {
    pub tf_id: std::string::String,
    #[doc = "This must be unique within the project."]
    pub secret_id: terrars::PrimField<std::string::String>,
}

impl BuildSecretManagerSecret {
    pub fn build(self, stack: &mut terrars::Stack) -> SecretManagerSecret {
        let out = SecretManagerSecret(std::rc::Rc::new(SecretManagerSecret_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SecretManagerSecretData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                annotations: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                expire_time: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                project: core::default::Default::default(),
                secret_id: self.secret_id,
                secret_type: core::default::Default::default(),
                tags: core::default::Default::default(),
                ttl: core::default::Default::default(),
                version_aliases: core::default::Default::default(),
                version_destroy_ttl: core::default::Default::default(),
                replication: core::default::Default::default(),
                rotation: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                topics: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SecretManagerSecretRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecretManagerSecretRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SecretManagerSecretRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nCustom metadata about the secret.\n\nAnnotations are distinct from various forms of labels. Annotations exist to allow\nclient tools to store their own state information without requiring a database.\n\nAnnotation keys must be between 1 and 63 characters long, have a UTF-8 encoding of\nmaximum 128 bytes, begin and end with an alphanumeric character ([a-z0-9A-Z]), and\nmay have dashes (-), underscores (_), dots (.), and alphanumerics in between these\nsymbols.\n\nThe total size of annotation keys and values must be less than 16KiB.\n\nAn object containing a list of \"key\": value pairs. Example:\n{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time at which the Secret was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the secret. Defaults to false.\nWhen the field is set to true in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the secret will fail."]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `expire_time` after provisioning.\nTimestamp in UTC when the Secret is scheduled to expire. This is always provided on output, regardless of what was sent on input.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\".\nOnly one of 'expire_time' or 'ttl' can be provided."]
    pub fn expire_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expire_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe labels assigned to this Secret.\n\nLabel keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes,\nand must conform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}][\\p{Ll}\\p{Lo}\\p{N}_-]{0,62}\n\nLabel values must be between 0 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes,\nand must conform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63}\n\nNo more than 64 labels can be assigned to a given resource.\n\nAn object containing a list of \"key\": value pairs. Example:\n{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Secret. Format:\n'projects/{{project}}/secrets/{{secret_id}}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `secret_id` after provisioning.\nThis must be unique within the project."]
    pub fn secret_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `secret_type` after provisioning.\nThis defines the type of the secret. Enforces certain structural requirements on the SecretVersions.\nFor secret of type UNSPECIFIED, the SecretVersions can be of any type."]
    pub fn secret_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nA map of resource manager tags.\nResource manager tag keys and values have the same definition as resource manager tags.\nKeys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/{tag_value_id}."]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ttl` after provisioning.\nThe TTL for the Secret.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\".\nOnly one of 'ttl' or 'expire_time' can be provided."]
    pub fn ttl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version_aliases` after provisioning.\nMapping from version alias to version name.\n\nA version alias is a string with a maximum length of 63 characters and can contain\nuppercase and lowercase letters, numerals, and the hyphen (-) and underscore ('_')\ncharacters. An alias string must start with a letter and cannot be the string\n'latest' or 'NEW'. No more than 50 aliases can be assigned to a given secret.\n\nAn object containing a list of \"key\": value pairs. Example:\n{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn version_aliases(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.version_aliases", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version_destroy_ttl` after provisioning.\nSecret Version TTL after destruction request.\nThis is a part of the delayed delete feature on Secret Version.\nFor secret with versionDestroyTtl>0, version destruction doesn't happen immediately\non calling destroy instead the version goes to a disabled state and\nthe actual destruction happens after this TTL expires."]
    pub fn version_destroy_ttl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_destroy_ttl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication` after provisioning.\n"]
    pub fn replication(&self) -> terrars::ListRef<SecretManagerSecret_ReplicationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replication", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rotation` after provisioning.\n"]
    pub fn rotation(&self) -> terrars::ListRef<SecretManagerSecret_RotationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rotation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SecretManagerSecret_TimeoutsRef {
        SecretManagerSecret_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `topics` after provisioning.\n"]
    pub fn topics(&self) -> terrars::ListRef<SecretManagerSecret_TopicsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.topics", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SecretManagerSecretDynamic {
    replication: std::option::Option<terrars::DynamicBlock<SecretManagerSecret_Replication>>,
    rotation: std::option::Option<terrars::DynamicBlock<SecretManagerSecret_Rotation>>,
    topics: std::option::Option<terrars::DynamicBlock<SecretManagerSecret_Topics>>,
}

#[derive(Serialize)]
pub struct SecretManagerSecret_Replication_Auto_CustomerManagedEncryption {
    kms_key_name: terrars::PrimField<std::string::String>,
}

impl SecretManagerSecret_Replication_Auto_CustomerManagedEncryption { }

impl terrars::ToListMappable for SecretManagerSecret_Replication_Auto_CustomerManagedEncryption {
    type O = terrars::BlockAssignable<SecretManagerSecret_Replication_Auto_CustomerManagedEncryption>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecretManagerSecret_Replication_Auto_CustomerManagedEncryption {
    #[doc = "The resource name of the Cloud KMS CryptoKey used to encrypt secret payloads."]
    pub kms_key_name: terrars::PrimField<std::string::String>,
}

impl BuildSecretManagerSecret_Replication_Auto_CustomerManagedEncryption {
    pub fn build(self) -> SecretManagerSecret_Replication_Auto_CustomerManagedEncryption {
        SecretManagerSecret_Replication_Auto_CustomerManagedEncryption { kms_key_name: self.kms_key_name }
    }
}

#[allow(dead_code)]
pub struct SecretManagerSecret_Replication_Auto_CustomerManagedEncryptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecretManagerSecret_Replication_Auto_CustomerManagedEncryptionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecretManagerSecret_Replication_Auto_CustomerManagedEncryptionRef {
        SecretManagerSecret_Replication_Auto_CustomerManagedEncryptionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecretManagerSecret_Replication_Auto_CustomerManagedEncryptionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nThe resource name of the Cloud KMS CryptoKey used to encrypt secret payloads."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecretManagerSecret_Replication_AutoDynamic {
    customer_managed_encryption: std
    ::option
    ::Option<terrars::DynamicBlock<SecretManagerSecret_Replication_Auto_CustomerManagedEncryption>>,
}

#[derive(Serialize)]
pub struct SecretManagerSecret_Replication_Auto {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_managed_encryption: std
    ::option
    ::Option<std::vec::Vec<SecretManagerSecret_Replication_Auto_CustomerManagedEncryption>>,
    dynamic: SecretManagerSecret_Replication_AutoDynamic,
}

impl SecretManagerSecret_Replication_Auto {
    #[doc = "Set the field `customer_managed_encryption`.\n"]
    pub fn set_customer_managed_encryption(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecretManagerSecret_Replication_Auto_CustomerManagedEncryption>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.customer_managed_encryption = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.customer_managed_encryption = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecretManagerSecret_Replication_Auto {
    type O = terrars::BlockAssignable<SecretManagerSecret_Replication_Auto>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecretManagerSecret_Replication_Auto {}

impl BuildSecretManagerSecret_Replication_Auto {
    pub fn build(self) -> SecretManagerSecret_Replication_Auto {
        SecretManagerSecret_Replication_Auto {
            customer_managed_encryption: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecretManagerSecret_Replication_AutoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecretManagerSecret_Replication_AutoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecretManagerSecret_Replication_AutoRef {
        SecretManagerSecret_Replication_AutoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecretManagerSecret_Replication_AutoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `customer_managed_encryption` after provisioning.\n"]
    pub fn customer_managed_encryption(
        &self,
    ) -> terrars::ListRef<SecretManagerSecret_Replication_Auto_CustomerManagedEncryptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customer_managed_encryption", self.base))
    }
}

#[derive(Serialize)]
pub struct SecretManagerSecret_Replication_UserManaged_Replicas_CustomerManagedEncryption {
    kms_key_name: terrars::PrimField<std::string::String>,
}

impl SecretManagerSecret_Replication_UserManaged_Replicas_CustomerManagedEncryption { }

impl terrars::ToListMappable for SecretManagerSecret_Replication_UserManaged_Replicas_CustomerManagedEncryption {
    type O =
        terrars::BlockAssignable<SecretManagerSecret_Replication_UserManaged_Replicas_CustomerManagedEncryption>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecretManagerSecret_Replication_UserManaged_Replicas_CustomerManagedEncryption {
    #[doc = "Describes the Cloud KMS encryption key that will be used to protect destination secret."]
    pub kms_key_name: terrars::PrimField<std::string::String>,
}

impl BuildSecretManagerSecret_Replication_UserManaged_Replicas_CustomerManagedEncryption {
    pub fn build(self) -> SecretManagerSecret_Replication_UserManaged_Replicas_CustomerManagedEncryption {
        SecretManagerSecret_Replication_UserManaged_Replicas_CustomerManagedEncryption {
            kms_key_name: self.kms_key_name,
        }
    }
}

#[allow(dead_code)]
pub struct SecretManagerSecret_Replication_UserManaged_Replicas_CustomerManagedEncryptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecretManagerSecret_Replication_UserManaged_Replicas_CustomerManagedEncryptionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecretManagerSecret_Replication_UserManaged_Replicas_CustomerManagedEncryptionRef {
        SecretManagerSecret_Replication_UserManaged_Replicas_CustomerManagedEncryptionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecretManagerSecret_Replication_UserManaged_Replicas_CustomerManagedEncryptionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nDescribes the Cloud KMS encryption key that will be used to protect destination secret."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecretManagerSecret_Replication_UserManaged_ReplicasDynamic {
    customer_managed_encryption: std
    ::option
    ::Option<terrars::DynamicBlock<SecretManagerSecret_Replication_UserManaged_Replicas_CustomerManagedEncryption>>,
}

#[derive(Serialize)]
pub struct SecretManagerSecret_Replication_UserManaged_Replicas {
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_managed_encryption: std
    ::option
    ::Option<std::vec::Vec<SecretManagerSecret_Replication_UserManaged_Replicas_CustomerManagedEncryption>>,
    dynamic: SecretManagerSecret_Replication_UserManaged_ReplicasDynamic,
}

impl SecretManagerSecret_Replication_UserManaged_Replicas {
    #[doc = "Set the field `customer_managed_encryption`.\n"]
    pub fn set_customer_managed_encryption(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SecretManagerSecret_Replication_UserManaged_Replicas_CustomerManagedEncryption,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.customer_managed_encryption = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.customer_managed_encryption = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecretManagerSecret_Replication_UserManaged_Replicas {
    type O = terrars::BlockAssignable<SecretManagerSecret_Replication_UserManaged_Replicas>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecretManagerSecret_Replication_UserManaged_Replicas {
    #[doc = "The canonical IDs of the location to replicate data. For example: \"us-east1\"."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildSecretManagerSecret_Replication_UserManaged_Replicas {
    pub fn build(self) -> SecretManagerSecret_Replication_UserManaged_Replicas {
        SecretManagerSecret_Replication_UserManaged_Replicas {
            location: self.location,
            customer_managed_encryption: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecretManagerSecret_Replication_UserManaged_ReplicasRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecretManagerSecret_Replication_UserManaged_ReplicasRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecretManagerSecret_Replication_UserManaged_ReplicasRef {
        SecretManagerSecret_Replication_UserManaged_ReplicasRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecretManagerSecret_Replication_UserManaged_ReplicasRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe canonical IDs of the location to replicate data. For example: \"us-east1\"."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc = "Get a reference to the value of field `customer_managed_encryption` after provisioning.\n"]
    pub fn customer_managed_encryption(
        &self,
    ) -> terrars::ListRef<SecretManagerSecret_Replication_UserManaged_Replicas_CustomerManagedEncryptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customer_managed_encryption", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecretManagerSecret_Replication_UserManagedDynamic {
    replicas: std::option::Option<terrars::DynamicBlock<SecretManagerSecret_Replication_UserManaged_Replicas>>,
}

#[derive(Serialize)]
pub struct SecretManagerSecret_Replication_UserManaged {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replicas: std::option::Option<std::vec::Vec<SecretManagerSecret_Replication_UserManaged_Replicas>>,
    dynamic: SecretManagerSecret_Replication_UserManagedDynamic,
}

impl SecretManagerSecret_Replication_UserManaged {
    #[doc = "Set the field `replicas`.\n"]
    pub fn set_replicas(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecretManagerSecret_Replication_UserManaged_Replicas>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.replicas = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.replicas = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecretManagerSecret_Replication_UserManaged {
    type O = terrars::BlockAssignable<SecretManagerSecret_Replication_UserManaged>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecretManagerSecret_Replication_UserManaged {}

impl BuildSecretManagerSecret_Replication_UserManaged {
    pub fn build(self) -> SecretManagerSecret_Replication_UserManaged {
        SecretManagerSecret_Replication_UserManaged {
            replicas: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecretManagerSecret_Replication_UserManagedRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecretManagerSecret_Replication_UserManagedRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecretManagerSecret_Replication_UserManagedRef {
        SecretManagerSecret_Replication_UserManagedRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecretManagerSecret_Replication_UserManagedRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `replicas` after provisioning.\n"]
    pub fn replicas(&self) -> terrars::ListRef<SecretManagerSecret_Replication_UserManaged_ReplicasRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replicas", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecretManagerSecret_ReplicationDynamic {
    auto: std::option::Option<terrars::DynamicBlock<SecretManagerSecret_Replication_Auto>>,
    user_managed: std::option::Option<terrars::DynamicBlock<SecretManagerSecret_Replication_UserManaged>>,
}

#[derive(Serialize)]
pub struct SecretManagerSecret_Replication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto: std::option::Option<std::vec::Vec<SecretManagerSecret_Replication_Auto>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_managed: std::option::Option<std::vec::Vec<SecretManagerSecret_Replication_UserManaged>>,
    dynamic: SecretManagerSecret_ReplicationDynamic,
}

impl SecretManagerSecret_Replication {
    #[doc = "Set the field `auto`.\n"]
    pub fn set_auto(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecretManagerSecret_Replication_Auto>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.auto = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.auto = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `user_managed`.\n"]
    pub fn set_user_managed(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecretManagerSecret_Replication_UserManaged>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.user_managed = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.user_managed = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecretManagerSecret_Replication {
    type O = terrars::BlockAssignable<SecretManagerSecret_Replication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecretManagerSecret_Replication {}

impl BuildSecretManagerSecret_Replication {
    pub fn build(self) -> SecretManagerSecret_Replication {
        SecretManagerSecret_Replication {
            auto: core::default::Default::default(),
            user_managed: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecretManagerSecret_ReplicationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecretManagerSecret_ReplicationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecretManagerSecret_ReplicationRef {
        SecretManagerSecret_ReplicationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecretManagerSecret_ReplicationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto` after provisioning.\n"]
    pub fn auto(&self) -> terrars::ListRef<SecretManagerSecret_Replication_AutoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto", self.base))
    }

    #[doc = "Get a reference to the value of field `user_managed` after provisioning.\n"]
    pub fn user_managed(&self) -> terrars::ListRef<SecretManagerSecret_Replication_UserManagedRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_managed", self.base))
    }
}

#[derive(Serialize)]
pub struct SecretManagerSecret_Rotation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_rotation_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rotation_period: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecretManagerSecret_Rotation {
    #[doc =
        "Set the field `next_rotation_time`.\nTimestamp in UTC at which the Secret is scheduled to rotate.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn set_next_rotation_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.next_rotation_time = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `rotation_period`.\nThe Duration between rotation notifications. Must be in seconds and at least 3600s (1h) and at most 3153600000s (100 years).\nIf rotationPeriod is set, 'next_rotation_time' must be set. 'next_rotation_time' will be advanced by this period when the service automatically sends rotation notifications."]
    pub fn set_rotation_period(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rotation_period = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecretManagerSecret_Rotation {
    type O = terrars::BlockAssignable<SecretManagerSecret_Rotation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecretManagerSecret_Rotation {}

impl BuildSecretManagerSecret_Rotation {
    pub fn build(self) -> SecretManagerSecret_Rotation {
        SecretManagerSecret_Rotation {
            next_rotation_time: core::default::Default::default(),
            rotation_period: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecretManagerSecret_RotationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecretManagerSecret_RotationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecretManagerSecret_RotationRef {
        SecretManagerSecret_RotationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecretManagerSecret_RotationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `next_rotation_time` after provisioning.\nTimestamp in UTC at which the Secret is scheduled to rotate.\nA timestamp in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits. Examples: \"2014-10-02T15:01:23Z\" and \"2014-10-02T15:01:23.045123456Z\"."]
    pub fn next_rotation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_rotation_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rotation_period` after provisioning.\nThe Duration between rotation notifications. Must be in seconds and at least 3600s (1h) and at most 3153600000s (100 years).\nIf rotationPeriod is set, 'next_rotation_time' must be set. 'next_rotation_time' will be advanced by this period when the service automatically sends rotation notifications."]
    pub fn rotation_period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rotation_period", self.base))
    }
}

#[derive(Serialize)]
pub struct SecretManagerSecret_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecretManagerSecret_Timeouts {
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

impl terrars::ToListMappable for SecretManagerSecret_Timeouts {
    type O = terrars::BlockAssignable<SecretManagerSecret_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecretManagerSecret_Timeouts {}

impl BuildSecretManagerSecret_Timeouts {
    pub fn build(self) -> SecretManagerSecret_Timeouts {
        SecretManagerSecret_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecretManagerSecret_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecretManagerSecret_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecretManagerSecret_TimeoutsRef {
        SecretManagerSecret_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecretManagerSecret_TimeoutsRef {
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

#[derive(Serialize)]
pub struct SecretManagerSecret_Topics {
    name: terrars::PrimField<std::string::String>,
}

impl SecretManagerSecret_Topics { }

impl terrars::ToListMappable for SecretManagerSecret_Topics {
    type O = terrars::BlockAssignable<SecretManagerSecret_Topics>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecretManagerSecret_Topics {
    #[doc =
        "The resource name of the Pub/Sub topic that will be published to, in the following format: projects/*/topics/*.\nFor publication to succeed, the Secret Manager Service Agent service account must have pubsub.publisher permissions on the topic."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildSecretManagerSecret_Topics {
    pub fn build(self) -> SecretManagerSecret_Topics {
        SecretManagerSecret_Topics { name: self.name }
    }
}

#[allow(dead_code)]
pub struct SecretManagerSecret_TopicsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecretManagerSecret_TopicsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecretManagerSecret_TopicsRef {
        SecretManagerSecret_TopicsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecretManagerSecret_TopicsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Pub/Sub topic that will be published to, in the following format: projects/*/topics/*.\nFor publication to succeed, the Secret Manager Service Agent service account must have pubsub.publisher permissions on the topic."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
