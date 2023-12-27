use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataStorageBucketObjectContentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
}

struct DataStorageBucketObjectContent_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataStorageBucketObjectContentData>,
}

#[derive(Clone)]
pub struct DataStorageBucketObjectContent(Rc<DataStorageBucketObjectContent_>);

impl DataStorageBucketObjectContent {
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

    #[doc= "Set the field `content`.\nData as string to be uploaded. Must be defined if source is not. Note: The content field is marked as sensitive. To view the raw contents of the object, please define an output."]
    pub fn set_content(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().content = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `bucket` after provisioning.\nThe name of the containing bucket."]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cache_control` after provisioning.\nCache-Control directive to specify caching behavior of object data. If omitted and object is accessible to all anonymous users, the default will be public, max-age=3600"]
    pub fn cache_control(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cache_control", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content` after provisioning.\nData as string to be uploaded. Must be defined if source is not. Note: The content field is marked as sensitive. To view the raw contents of the object, please define an output."]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content_disposition` after provisioning.\nContent-Disposition of the object data."]
    pub fn content_disposition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_disposition", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content_encoding` after provisioning.\nContent-Encoding of the object data."]
    pub fn content_encoding(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_encoding", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content_language` after provisioning.\nContent-Language of the object data."]
    pub fn content_language(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_language", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content_type` after provisioning.\nContent-Type of the object data. Defaults to \"application/octet-stream\" or \"text/plain; charset=utf-8\"."]
    pub fn content_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `crc32c` after provisioning.\nBase 64 CRC32 hash of the uploaded data."]
    pub fn crc32c(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.crc32c", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `customer_encryption` after provisioning.\nEncryption key; encoded using base64."]
    pub fn customer_encryption(&self) -> ListRef<DataStorageBucketObjectContentCustomerEncryptionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.customer_encryption", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `detect_md5hash` after provisioning.\n"]
    pub fn detect_md5hash(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.detect_md5hash", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `event_based_hold` after provisioning.\nWhether an object is under event-based hold. Event-based hold is a way to retain objects until an event occurs, which is signified by the hold's release (i.e. this value is set to false). After being released (set to false), such objects will be subject to bucket-level retention (if any)."]
    pub fn event_based_hold(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_based_hold", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kms_key_name` after provisioning.\nResource name of the Cloud KMS key that will be used to encrypt the object. Overrides the object metadata's kmsKeyName value, if any."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `md5hash` after provisioning.\nBase 64 MD5 hash of the uploaded data."]
    pub fn md5hash(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.md5hash", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `media_link` after provisioning.\nA url reference to download this object."]
    pub fn media_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.media_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `metadata` after provisioning.\nUser-provided metadata, in key/value pairs."]
    pub fn metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.metadata", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the object. If you're interpolating the name of this object, see output_name instead."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `output_name` after provisioning.\nThe name of the object. Use this field in interpolations with google_storage_object_acl to recreate google_storage_object_acl resources when your google_storage_bucket_object is recreated."]
    pub fn output_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `retention` after provisioning.\nObject level retention configuration."]
    pub fn retention(&self) -> ListRef<DataStorageBucketObjectContentRetentionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.retention", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\nA url reference to this object."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source` after provisioning.\nA path to the data you want to upload. Must be defined if content is not."]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `storage_class` after provisioning.\nThe StorageClass of the new bucket object. Supported values include: MULTI_REGIONAL, REGIONAL, NEARLINE, COLDLINE, ARCHIVE. If not provided, this defaults to the bucket's default storage class or to a standard class."]
    pub fn storage_class(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_class", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `temporary_hold` after provisioning.\nWhether an object is under temporary hold. While this flag is set to true, the object is protected against deletion and overwrites."]
    pub fn temporary_hold(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.temporary_hold", self.extract_ref()))
    }
}

impl Referable for DataStorageBucketObjectContent {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataStorageBucketObjectContent { }

impl ToListMappable for DataStorageBucketObjectContent {
    type O = ListRef<DataStorageBucketObjectContentRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataStorageBucketObjectContent_ {
    fn extract_datasource_type(&self) -> String {
        "google_storage_bucket_object_content".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataStorageBucketObjectContent {
    pub tf_id: String,
    #[doc= "The name of the containing bucket."]
    pub bucket: PrimField<String>,
    #[doc= "The name of the object. If you're interpolating the name of this object, see output_name instead."]
    pub name: PrimField<String>,
}

impl BuildDataStorageBucketObjectContent {
    pub fn build(self, stack: &mut Stack) -> DataStorageBucketObjectContent {
        let out = DataStorageBucketObjectContent(Rc::new(DataStorageBucketObjectContent_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataStorageBucketObjectContentData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                bucket: self.bucket,
                content: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataStorageBucketObjectContentRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataStorageBucketObjectContentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataStorageBucketObjectContentRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `bucket` after provisioning.\nThe name of the containing bucket."]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cache_control` after provisioning.\nCache-Control directive to specify caching behavior of object data. If omitted and object is accessible to all anonymous users, the default will be public, max-age=3600"]
    pub fn cache_control(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cache_control", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content` after provisioning.\nData as string to be uploaded. Must be defined if source is not. Note: The content field is marked as sensitive. To view the raw contents of the object, please define an output."]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content_disposition` after provisioning.\nContent-Disposition of the object data."]
    pub fn content_disposition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_disposition", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content_encoding` after provisioning.\nContent-Encoding of the object data."]
    pub fn content_encoding(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_encoding", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content_language` after provisioning.\nContent-Language of the object data."]
    pub fn content_language(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_language", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content_type` after provisioning.\nContent-Type of the object data. Defaults to \"application/octet-stream\" or \"text/plain; charset=utf-8\"."]
    pub fn content_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `crc32c` after provisioning.\nBase 64 CRC32 hash of the uploaded data."]
    pub fn crc32c(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.crc32c", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `customer_encryption` after provisioning.\nEncryption key; encoded using base64."]
    pub fn customer_encryption(&self) -> ListRef<DataStorageBucketObjectContentCustomerEncryptionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.customer_encryption", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `detect_md5hash` after provisioning.\n"]
    pub fn detect_md5hash(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.detect_md5hash", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `event_based_hold` after provisioning.\nWhether an object is under event-based hold. Event-based hold is a way to retain objects until an event occurs, which is signified by the hold's release (i.e. this value is set to false). After being released (set to false), such objects will be subject to bucket-level retention (if any)."]
    pub fn event_based_hold(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.event_based_hold", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `kms_key_name` after provisioning.\nResource name of the Cloud KMS key that will be used to encrypt the object. Overrides the object metadata's kmsKeyName value, if any."]
    pub fn kms_key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `md5hash` after provisioning.\nBase 64 MD5 hash of the uploaded data."]
    pub fn md5hash(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.md5hash", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `media_link` after provisioning.\nA url reference to download this object."]
    pub fn media_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.media_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `metadata` after provisioning.\nUser-provided metadata, in key/value pairs."]
    pub fn metadata(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.metadata", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the object. If you're interpolating the name of this object, see output_name instead."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `output_name` after provisioning.\nThe name of the object. Use this field in interpolations with google_storage_object_acl to recreate google_storage_object_acl resources when your google_storage_bucket_object is recreated."]
    pub fn output_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `retention` after provisioning.\nObject level retention configuration."]
    pub fn retention(&self) -> ListRef<DataStorageBucketObjectContentRetentionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.retention", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\nA url reference to this object."]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source` after provisioning.\nA path to the data you want to upload. Must be defined if content is not."]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `storage_class` after provisioning.\nThe StorageClass of the new bucket object. Supported values include: MULTI_REGIONAL, REGIONAL, NEARLINE, COLDLINE, ARCHIVE. If not provided, this defaults to the bucket's default storage class or to a standard class."]
    pub fn storage_class(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_class", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `temporary_hold` after provisioning.\nWhether an object is under temporary hold. While this flag is set to true, the object is protected against deletion and overwrites."]
    pub fn temporary_hold(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.temporary_hold", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataStorageBucketObjectContentCustomerEncryptionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_algorithm: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_key: Option<PrimField<String>>,
}

impl DataStorageBucketObjectContentCustomerEncryptionEl {
    #[doc= "Set the field `encryption_algorithm`.\n"]
    pub fn set_encryption_algorithm(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encryption_algorithm = Some(v.into());
        self
    }

    #[doc= "Set the field `encryption_key`.\n"]
    pub fn set_encryption_key(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encryption_key = Some(v.into());
        self
    }
}

impl ToListMappable for DataStorageBucketObjectContentCustomerEncryptionEl {
    type O = BlockAssignable<DataStorageBucketObjectContentCustomerEncryptionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageBucketObjectContentCustomerEncryptionEl {}

impl BuildDataStorageBucketObjectContentCustomerEncryptionEl {
    pub fn build(self) -> DataStorageBucketObjectContentCustomerEncryptionEl {
        DataStorageBucketObjectContentCustomerEncryptionEl {
            encryption_algorithm: core::default::Default::default(),
            encryption_key: core::default::Default::default(),
        }
    }
}

pub struct DataStorageBucketObjectContentCustomerEncryptionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataStorageBucketObjectContentCustomerEncryptionElRef {
    fn new(shared: StackShared, base: String) -> DataStorageBucketObjectContentCustomerEncryptionElRef {
        DataStorageBucketObjectContentCustomerEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageBucketObjectContentCustomerEncryptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `encryption_algorithm` after provisioning.\n"]
    pub fn encryption_algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encryption_algorithm", self.base))
    }

    #[doc= "Get a reference to the value of field `encryption_key` after provisioning.\n"]
    pub fn encryption_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encryption_key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataStorageBucketObjectContentRetentionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retain_until_time: Option<PrimField<String>>,
}

impl DataStorageBucketObjectContentRetentionEl {
    #[doc= "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }

    #[doc= "Set the field `retain_until_time`.\n"]
    pub fn set_retain_until_time(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.retain_until_time = Some(v.into());
        self
    }
}

impl ToListMappable for DataStorageBucketObjectContentRetentionEl {
    type O = BlockAssignable<DataStorageBucketObjectContentRetentionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataStorageBucketObjectContentRetentionEl {}

impl BuildDataStorageBucketObjectContentRetentionEl {
    pub fn build(self) -> DataStorageBucketObjectContentRetentionEl {
        DataStorageBucketObjectContentRetentionEl {
            mode: core::default::Default::default(),
            retain_until_time: core::default::Default::default(),
        }
    }
}

pub struct DataStorageBucketObjectContentRetentionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataStorageBucketObjectContentRetentionElRef {
    fn new(shared: StackShared, base: String) -> DataStorageBucketObjectContentRetentionElRef {
        DataStorageBucketObjectContentRetentionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataStorageBucketObjectContentRetentionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc= "Get a reference to the value of field `retain_until_time` after provisioning.\n"]
    pub fn retain_until_time(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.retain_until_time", self.base))
    }
}
