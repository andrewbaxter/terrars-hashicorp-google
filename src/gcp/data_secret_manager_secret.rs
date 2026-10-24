use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataSecretManagerSecretData {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    secret_id: terrars::PrimField<std::string::String>,
}

struct DataSecretManagerSecret_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataSecretManagerSecretData>,
}

#[derive(Clone)]
pub struct DataSecretManagerSecret(std::rc::Rc<DataSecretManagerSecret_>);

impl DataSecretManagerSecret {
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
        "Get a reference to the value of field `replication` after provisioning.\nThe replication policy of the secret data attached to the Secret. It cannot be changed\nafter the Secret has been created."]
    pub fn replication(&self) -> terrars::ListRef<DataSecretManagerSecretReplicationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replication", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rotation` after provisioning.\nThe rotation time and period for a Secret. At 'next_rotation_time', Secret Manager will send a Pub/Sub notification to the topics configured on the Secret. 'topics' must be set to configure rotation."]
    pub fn rotation(&self) -> terrars::ListRef<DataSecretManagerSecretRotationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rotation", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `secret_id` after provisioning.\nThis must be unique within the project."]
    pub fn secret_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `topics` after provisioning.\nA list of up to 10 Pub/Sub topics to which messages are published when control plane operations are called on the secret or its versions."]
    pub fn topics(&self) -> terrars::ListRef<DataSecretManagerSecretTopicsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.topics", self.extract_ref()))
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
}

impl terrars::Referable for DataSecretManagerSecret {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataSecretManagerSecret { }

impl terrars::ToListMappable for DataSecretManagerSecret {
    type O = terrars::ListRef<DataSecretManagerSecretRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataSecretManagerSecret_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_secret_manager_secret".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSecretManagerSecret {
    pub tf_id: std::string::String,
    #[doc = "This must be unique within the project."]
    pub secret_id: terrars::PrimField<std::string::String>,
}

impl BuildDataSecretManagerSecret {
    pub fn build(self, stack: &mut terrars::Stack) -> DataSecretManagerSecret {
        let out = DataSecretManagerSecret(std::rc::Rc::new(DataSecretManagerSecret_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataSecretManagerSecretData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                secret_id: self.secret_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSecretManagerSecretRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSecretManagerSecretRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataSecretManagerSecretRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
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
        "Get a reference to the value of field `replication` after provisioning.\nThe replication policy of the secret data attached to the Secret. It cannot be changed\nafter the Secret has been created."]
    pub fn replication(&self) -> terrars::ListRef<DataSecretManagerSecretReplicationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replication", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rotation` after provisioning.\nThe rotation time and period for a Secret. At 'next_rotation_time', Secret Manager will send a Pub/Sub notification to the topics configured on the Secret. 'topics' must be set to configure rotation."]
    pub fn rotation(&self) -> terrars::ListRef<DataSecretManagerSecretRotationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rotation", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `secret_id` after provisioning.\nThis must be unique within the project."]
    pub fn secret_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `topics` after provisioning.\nA list of up to 10 Pub/Sub topics to which messages are published when control plane operations are called on the secret or its versions."]
    pub fn topics(&self) -> terrars::ListRef<DataSecretManagerSecretTopicsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.topics", self.extract_ref()))
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
}

#[derive(Serialize)]
pub struct DataSecretManagerSecretReplicationElAutoElCustomerManagedEncryptionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSecretManagerSecretReplicationElAutoElCustomerManagedEncryptionEl {
    #[doc = "Set the field `kms_key_name`.\n"]
    pub fn set_kms_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSecretManagerSecretReplicationElAutoElCustomerManagedEncryptionEl {
    type O = terrars::BlockAssignable<DataSecretManagerSecretReplicationElAutoElCustomerManagedEncryptionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSecretManagerSecretReplicationElAutoElCustomerManagedEncryptionEl {}

impl BuildDataSecretManagerSecretReplicationElAutoElCustomerManagedEncryptionEl {
    pub fn build(self) -> DataSecretManagerSecretReplicationElAutoElCustomerManagedEncryptionEl {
        DataSecretManagerSecretReplicationElAutoElCustomerManagedEncryptionEl {
            kms_key_name: core::default::Default::default(),
        }
    }
}

pub struct DataSecretManagerSecretReplicationElAutoElCustomerManagedEncryptionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSecretManagerSecretReplicationElAutoElCustomerManagedEncryptionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSecretManagerSecretReplicationElAutoElCustomerManagedEncryptionElRef {
        DataSecretManagerSecretReplicationElAutoElCustomerManagedEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSecretManagerSecretReplicationElAutoElCustomerManagedEncryptionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\n"]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSecretManagerSecretReplicationElAutoEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_managed_encryption: std
    ::option
    ::Option<terrars::ListField<DataSecretManagerSecretReplicationElAutoElCustomerManagedEncryptionEl>>,
}

impl DataSecretManagerSecretReplicationElAutoEl {
    #[doc = "Set the field `customer_managed_encryption`.\n"]
    pub fn set_customer_managed_encryption(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataSecretManagerSecretReplicationElAutoElCustomerManagedEncryptionEl>>,
    ) -> Self {
        self.customer_managed_encryption = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSecretManagerSecretReplicationElAutoEl {
    type O = terrars::BlockAssignable<DataSecretManagerSecretReplicationElAutoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSecretManagerSecretReplicationElAutoEl {}

impl BuildDataSecretManagerSecretReplicationElAutoEl {
    pub fn build(self) -> DataSecretManagerSecretReplicationElAutoEl {
        DataSecretManagerSecretReplicationElAutoEl { customer_managed_encryption: core::default::Default::default() }
    }
}

pub struct DataSecretManagerSecretReplicationElAutoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSecretManagerSecretReplicationElAutoElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSecretManagerSecretReplicationElAutoElRef {
        DataSecretManagerSecretReplicationElAutoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSecretManagerSecretReplicationElAutoElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `customer_managed_encryption` after provisioning.\n"]
    pub fn customer_managed_encryption(
        &self,
    ) -> terrars::ListRef<DataSecretManagerSecretReplicationElAutoElCustomerManagedEncryptionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customer_managed_encryption", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl {
    #[doc = "Set the field `kms_key_name`.\n"]
    pub fn set_kms_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataSecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl {
    type O =
        terrars
        ::BlockAssignable<DataSecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl {}

impl BuildDataSecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl {
    pub fn build(self) -> DataSecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl {
        DataSecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl {
            kms_key_name: core::default::Default::default(),
        }
    }
}

pub struct DataSecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionElRef {
        DataSecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\n"]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSecretManagerSecretReplicationElUserManagedElReplicasEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_managed_encryption: std
    ::option
    ::Option<
        terrars::ListField<DataSecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSecretManagerSecretReplicationElUserManagedElReplicasEl {
    #[doc = "Set the field `customer_managed_encryption`.\n"]
    pub fn set_customer_managed_encryption(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataSecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl,
                        >,
                    >,
    ) -> Self {
        self.customer_managed_encryption = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSecretManagerSecretReplicationElUserManagedElReplicasEl {
    type O = terrars::BlockAssignable<DataSecretManagerSecretReplicationElUserManagedElReplicasEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSecretManagerSecretReplicationElUserManagedElReplicasEl {}

impl BuildDataSecretManagerSecretReplicationElUserManagedElReplicasEl {
    pub fn build(self) -> DataSecretManagerSecretReplicationElUserManagedElReplicasEl {
        DataSecretManagerSecretReplicationElUserManagedElReplicasEl {
            customer_managed_encryption: core::default::Default::default(),
            location: core::default::Default::default(),
        }
    }
}

pub struct DataSecretManagerSecretReplicationElUserManagedElReplicasElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSecretManagerSecretReplicationElUserManagedElReplicasElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSecretManagerSecretReplicationElUserManagedElReplicasElRef {
        DataSecretManagerSecretReplicationElUserManagedElReplicasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSecretManagerSecretReplicationElUserManagedElReplicasElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `customer_managed_encryption` after provisioning.\n"]
    pub fn customer_managed_encryption(
        &self,
    ) -> terrars::ListRef<DataSecretManagerSecretReplicationElUserManagedElReplicasElCustomerManagedEncryptionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customer_managed_encryption", self.base))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSecretManagerSecretReplicationElUserManagedEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replicas: std::option::Option<terrars::ListField<DataSecretManagerSecretReplicationElUserManagedElReplicasEl>>,
}

impl DataSecretManagerSecretReplicationElUserManagedEl {
    #[doc = "Set the field `replicas`.\n"]
    pub fn set_replicas(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSecretManagerSecretReplicationElUserManagedElReplicasEl>>,
    ) -> Self {
        self.replicas = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSecretManagerSecretReplicationElUserManagedEl {
    type O = terrars::BlockAssignable<DataSecretManagerSecretReplicationElUserManagedEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSecretManagerSecretReplicationElUserManagedEl {}

impl BuildDataSecretManagerSecretReplicationElUserManagedEl {
    pub fn build(self) -> DataSecretManagerSecretReplicationElUserManagedEl {
        DataSecretManagerSecretReplicationElUserManagedEl { replicas: core::default::Default::default() }
    }
}

pub struct DataSecretManagerSecretReplicationElUserManagedElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSecretManagerSecretReplicationElUserManagedElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSecretManagerSecretReplicationElUserManagedElRef {
        DataSecretManagerSecretReplicationElUserManagedElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSecretManagerSecretReplicationElUserManagedElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `replicas` after provisioning.\n"]
    pub fn replicas(&self) -> terrars::ListRef<DataSecretManagerSecretReplicationElUserManagedElReplicasElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replicas", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSecretManagerSecretReplicationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto: std::option::Option<terrars::ListField<DataSecretManagerSecretReplicationElAutoEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_managed: std::option::Option<terrars::ListField<DataSecretManagerSecretReplicationElUserManagedEl>>,
}

impl DataSecretManagerSecretReplicationEl {
    #[doc = "Set the field `auto`.\n"]
    pub fn set_auto(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSecretManagerSecretReplicationElAutoEl>>,
    ) -> Self {
        self.auto = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_managed`.\n"]
    pub fn set_user_managed(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSecretManagerSecretReplicationElUserManagedEl>>,
    ) -> Self {
        self.user_managed = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSecretManagerSecretReplicationEl {
    type O = terrars::BlockAssignable<DataSecretManagerSecretReplicationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSecretManagerSecretReplicationEl {}

impl BuildDataSecretManagerSecretReplicationEl {
    pub fn build(self) -> DataSecretManagerSecretReplicationEl {
        DataSecretManagerSecretReplicationEl {
            auto: core::default::Default::default(),
            user_managed: core::default::Default::default(),
        }
    }
}

pub struct DataSecretManagerSecretReplicationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSecretManagerSecretReplicationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSecretManagerSecretReplicationElRef {
        DataSecretManagerSecretReplicationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSecretManagerSecretReplicationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto` after provisioning.\n"]
    pub fn auto(&self) -> terrars::ListRef<DataSecretManagerSecretReplicationElAutoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto", self.base))
    }

    #[doc = "Get a reference to the value of field `user_managed` after provisioning.\n"]
    pub fn user_managed(&self) -> terrars::ListRef<DataSecretManagerSecretReplicationElUserManagedElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_managed", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSecretManagerSecretRotationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_rotation_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rotation_period: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSecretManagerSecretRotationEl {
    #[doc = "Set the field `next_rotation_time`.\n"]
    pub fn set_next_rotation_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.next_rotation_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rotation_period`.\n"]
    pub fn set_rotation_period(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rotation_period = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSecretManagerSecretRotationEl {
    type O = terrars::BlockAssignable<DataSecretManagerSecretRotationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSecretManagerSecretRotationEl {}

impl BuildDataSecretManagerSecretRotationEl {
    pub fn build(self) -> DataSecretManagerSecretRotationEl {
        DataSecretManagerSecretRotationEl {
            next_rotation_time: core::default::Default::default(),
            rotation_period: core::default::Default::default(),
        }
    }
}

pub struct DataSecretManagerSecretRotationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSecretManagerSecretRotationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSecretManagerSecretRotationElRef {
        DataSecretManagerSecretRotationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSecretManagerSecretRotationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `next_rotation_time` after provisioning.\n"]
    pub fn next_rotation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_rotation_time", self.base))
    }

    #[doc = "Get a reference to the value of field `rotation_period` after provisioning.\n"]
    pub fn rotation_period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rotation_period", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSecretManagerSecretTopicsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSecretManagerSecretTopicsEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSecretManagerSecretTopicsEl {
    type O = terrars::BlockAssignable<DataSecretManagerSecretTopicsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSecretManagerSecretTopicsEl {}

impl BuildDataSecretManagerSecretTopicsEl {
    pub fn build(self) -> DataSecretManagerSecretTopicsEl {
        DataSecretManagerSecretTopicsEl { name: core::default::Default::default() }
    }
}

pub struct DataSecretManagerSecretTopicsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSecretManagerSecretTopicsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSecretManagerSecretTopicsElRef {
        DataSecretManagerSecretTopicsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSecretManagerSecretTopicsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
