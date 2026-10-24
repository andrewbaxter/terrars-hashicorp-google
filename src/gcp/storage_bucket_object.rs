use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct StorageBucketObjectData {
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
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_control: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_disposition: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_encoding: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_language: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    detect_md5hash: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_based_hold: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_class: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    temporary_hold: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_encryption: std::option::Option<std::vec::Vec<StorageBucketObjectCustomerEncryptionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention: std::option::Option<std::vec::Vec<StorageBucketObjectRetentionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<StorageBucketObjectTimeoutsEl>,
    dynamic: StorageBucketObjectDynamic,
}

struct StorageBucketObject_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<StorageBucketObjectData>,
}

#[derive(Clone)]
pub struct StorageBucketObject(std::rc::Rc<StorageBucketObject_>);

impl StorageBucketObject {
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
        "Set the field `cache_control`.\nCache-Control directive to specify caching behavior of object data. If omitted and object is accessible to all anonymous users, the default will be public, max-age=3600"]
    pub fn set_cache_control(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cache_control = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `content`.\nData as string to be uploaded. Must be defined if source is not. Note: The content field is marked as sensitive. To view the raw contents of the object, please define an output."]
    pub fn set_content(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().content = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `content_disposition`.\nContent-Disposition of the object data."]
    pub fn set_content_disposition(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().content_disposition = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `content_encoding`.\nContent-Encoding of the object data."]
    pub fn set_content_encoding(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().content_encoding = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `content_language`.\nContent-Language of the object data."]
    pub fn set_content_language(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().content_language = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `content_type`.\nContent-Type of the object data. Defaults to \"application/octet-stream\" or \"text/plain; charset=utf-8\"."]
    pub fn set_content_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().content_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `detect_md5hash`.\n"]
    pub fn set_detect_md5hash(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().detect_md5hash = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `event_based_hold`.\nWhether an object is under event-based hold. Event-based hold is a way to retain objects until an event occurs, which is signified by the hold's release (i.e. this value is set to false). After being released (set to false), such objects will be subject to bucket-level retention (if any)."]
    pub fn set_event_based_hold(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().event_based_hold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kms_key_name`.\nResource name of the Cloud KMS key that will be used to encrypt the object. Overrides the object metadata's kmsKeyName value, if any."]
    pub fn set_kms_key_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metadata`.\nUser-provided metadata, in key/value pairs."]
    pub fn set_metadata(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source`.\nA path to the data you want to upload. Must be defined if content is not."]
    pub fn set_source(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().source = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `storage_class`.\nThe StorageClass of the new bucket object. Supported values include: MULTI_REGIONAL, REGIONAL, NEARLINE, COLDLINE, ARCHIVE. If not provided, this defaults to the bucket's default storage class or to a standard class."]
    pub fn set_storage_class(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().storage_class = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `temporary_hold`.\nWhether an object is under temporary hold. While this flag is set to true, the object is protected against deletion and overwrites."]
    pub fn set_temporary_hold(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().temporary_hold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `customer_encryption`.\n"]
    pub fn set_customer_encryption(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageBucketObjectCustomerEncryptionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().customer_encryption = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.customer_encryption = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `retention`.\n"]
    pub fn set_retention(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<StorageBucketObjectRetentionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().retention = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.retention = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<StorageBucketObjectTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nThe name of the containing bucket."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cache_control` after provisioning.\nCache-Control directive to specify caching behavior of object data. If omitted and object is accessible to all anonymous users, the default will be public, max-age=3600"]
    pub fn cache_control(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_control", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `content` after provisioning.\nData as string to be uploaded. Must be defined if source is not. Note: The content field is marked as sensitive. To view the raw contents of the object, please define an output."]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `content_disposition` after provisioning.\nContent-Disposition of the object data."]
    pub fn content_disposition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_disposition", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `content_encoding` after provisioning.\nContent-Encoding of the object data."]
    pub fn content_encoding(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_encoding", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `content_language` after provisioning.\nContent-Language of the object data."]
    pub fn content_language(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_language", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `content_type` after provisioning.\nContent-Type of the object data. Defaults to \"application/octet-stream\" or \"text/plain; charset=utf-8\"."]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `crc32c` after provisioning.\nBase 64 CRC32 hash of the uploaded data."]
    pub fn crc32c(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.crc32c", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `detect_md5hash` after provisioning.\n"]
    pub fn detect_md5hash(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.detect_md5hash", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `event_based_hold` after provisioning.\nWhether an object is under event-based hold. Event-based hold is a way to retain objects until an event occurs, which is signified by the hold's release (i.e. this value is set to false). After being released (set to false), such objects will be subject to bucket-level retention (if any)."]
    pub fn event_based_hold(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_based_hold", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nResource name of the Cloud KMS key that will be used to encrypt the object. Overrides the object metadata's kmsKeyName value, if any."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `md5hash` after provisioning.\nBase 64 MD5 hash of the uploaded data."]
    pub fn md5hash(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.md5hash", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `media_link` after provisioning.\nA url reference to download this object."]
    pub fn media_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.media_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `metadata` after provisioning.\nUser-provided metadata, in key/value pairs."]
    pub fn metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.metadata", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the object. If you're interpolating the name of this object, see output_name instead."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `output_name` after provisioning.\nThe name of the object. Use this field in interpolations with google_storage_object_acl to recreate google_storage_object_acl resources when your google_storage_bucket_object is recreated."]
    pub fn output_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nA url reference to this object."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source` after provisioning.\nA path to the data you want to upload. Must be defined if content is not."]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `storage_class` after provisioning.\nThe StorageClass of the new bucket object. Supported values include: MULTI_REGIONAL, REGIONAL, NEARLINE, COLDLINE, ARCHIVE. If not provided, this defaults to the bucket's default storage class or to a standard class."]
    pub fn storage_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_class", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `temporary_hold` after provisioning.\nWhether an object is under temporary hold. While this flag is set to true, the object is protected against deletion and overwrites."]
    pub fn temporary_hold(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.temporary_hold", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_encryption` after provisioning.\n"]
    pub fn customer_encryption(&self) -> terrars::ListRef<StorageBucketObjectCustomerEncryptionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customer_encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retention` after provisioning.\n"]
    pub fn retention(&self) -> terrars::ListRef<StorageBucketObjectRetentionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retention", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageBucketObjectTimeoutsElRef {
        StorageBucketObjectTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for StorageBucketObject {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for StorageBucketObject { }

impl terrars::ToListMappable for StorageBucketObject {
    type O = terrars::ListRef<StorageBucketObjectRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for StorageBucketObject_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_storage_bucket_object".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildStorageBucketObject {
    pub tf_id: std::string::String,
    #[doc = "The name of the containing bucket."]
    pub bucket: terrars::PrimField<std::string::String>,
    #[doc = "The name of the object. If you're interpolating the name of this object, see output_name instead."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildStorageBucketObject {
    pub fn build(self, stack: &mut terrars::Stack) -> StorageBucketObject {
        let out = StorageBucketObject(std::rc::Rc::new(StorageBucketObject_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(StorageBucketObjectData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                bucket: self.bucket,
                cache_control: core::default::Default::default(),
                content: core::default::Default::default(),
                content_disposition: core::default::Default::default(),
                content_encoding: core::default::Default::default(),
                content_language: core::default::Default::default(),
                content_type: core::default::Default::default(),
                detect_md5hash: core::default::Default::default(),
                event_based_hold: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_name: core::default::Default::default(),
                metadata: core::default::Default::default(),
                name: self.name,
                source: core::default::Default::default(),
                storage_class: core::default::Default::default(),
                temporary_hold: core::default::Default::default(),
                customer_encryption: core::default::Default::default(),
                retention: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct StorageBucketObjectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBucketObjectRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl StorageBucketObjectRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nThe name of the containing bucket."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cache_control` after provisioning.\nCache-Control directive to specify caching behavior of object data. If omitted and object is accessible to all anonymous users, the default will be public, max-age=3600"]
    pub fn cache_control(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_control", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `content` after provisioning.\nData as string to be uploaded. Must be defined if source is not. Note: The content field is marked as sensitive. To view the raw contents of the object, please define an output."]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `content_disposition` after provisioning.\nContent-Disposition of the object data."]
    pub fn content_disposition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_disposition", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `content_encoding` after provisioning.\nContent-Encoding of the object data."]
    pub fn content_encoding(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_encoding", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `content_language` after provisioning.\nContent-Language of the object data."]
    pub fn content_language(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_language", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `content_type` after provisioning.\nContent-Type of the object data. Defaults to \"application/octet-stream\" or \"text/plain; charset=utf-8\"."]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `crc32c` after provisioning.\nBase 64 CRC32 hash of the uploaded data."]
    pub fn crc32c(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.crc32c", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `detect_md5hash` after provisioning.\n"]
    pub fn detect_md5hash(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.detect_md5hash", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `event_based_hold` after provisioning.\nWhether an object is under event-based hold. Event-based hold is a way to retain objects until an event occurs, which is signified by the hold's release (i.e. this value is set to false). After being released (set to false), such objects will be subject to bucket-level retention (if any)."]
    pub fn event_based_hold(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_based_hold", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nResource name of the Cloud KMS key that will be used to encrypt the object. Overrides the object metadata's kmsKeyName value, if any."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `md5hash` after provisioning.\nBase 64 MD5 hash of the uploaded data."]
    pub fn md5hash(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.md5hash", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `media_link` after provisioning.\nA url reference to download this object."]
    pub fn media_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.media_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `metadata` after provisioning.\nUser-provided metadata, in key/value pairs."]
    pub fn metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.metadata", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the object. If you're interpolating the name of this object, see output_name instead."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `output_name` after provisioning.\nThe name of the object. Use this field in interpolations with google_storage_object_acl to recreate google_storage_object_acl resources when your google_storage_bucket_object is recreated."]
    pub fn output_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\nA url reference to this object."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source` after provisioning.\nA path to the data you want to upload. Must be defined if content is not."]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `storage_class` after provisioning.\nThe StorageClass of the new bucket object. Supported values include: MULTI_REGIONAL, REGIONAL, NEARLINE, COLDLINE, ARCHIVE. If not provided, this defaults to the bucket's default storage class or to a standard class."]
    pub fn storage_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_class", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `temporary_hold` after provisioning.\nWhether an object is under temporary hold. While this flag is set to true, the object is protected against deletion and overwrites."]
    pub fn temporary_hold(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.temporary_hold", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_encryption` after provisioning.\n"]
    pub fn customer_encryption(&self) -> terrars::ListRef<StorageBucketObjectCustomerEncryptionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customer_encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retention` after provisioning.\n"]
    pub fn retention(&self) -> terrars::ListRef<StorageBucketObjectRetentionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retention", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StorageBucketObjectTimeoutsElRef {
        StorageBucketObjectTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct StorageBucketObjectCustomerEncryptionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_algorithm: std::option::Option<terrars::PrimField<std::string::String>>,
    encryption_key: terrars::PrimField<std::string::String>,
}

impl StorageBucketObjectCustomerEncryptionEl {
    #[doc = "Set the field `encryption_algorithm`.\nThe encryption algorithm. Default: AES256"]
    pub fn set_encryption_algorithm(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.encryption_algorithm = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StorageBucketObjectCustomerEncryptionEl {
    type O = terrars::BlockAssignable<StorageBucketObjectCustomerEncryptionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBucketObjectCustomerEncryptionEl {
    #[doc = "Base64 encoded customer supplied encryption key."]
    pub encryption_key: terrars::PrimField<std::string::String>,
}

impl BuildStorageBucketObjectCustomerEncryptionEl {
    pub fn build(self) -> StorageBucketObjectCustomerEncryptionEl {
        StorageBucketObjectCustomerEncryptionEl {
            encryption_algorithm: core::default::Default::default(),
            encryption_key: self.encryption_key,
        }
    }
}

pub struct StorageBucketObjectCustomerEncryptionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBucketObjectCustomerEncryptionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageBucketObjectCustomerEncryptionElRef {
        StorageBucketObjectCustomerEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBucketObjectCustomerEncryptionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `encryption_algorithm` after provisioning.\nThe encryption algorithm. Default: AES256"]
    pub fn encryption_algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_algorithm", self.base))
    }

    #[doc =
        "Get a reference to the value of field `encryption_key` after provisioning.\nBase64 encoded customer supplied encryption key."]
    pub fn encryption_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_key", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageBucketObjectRetentionEl {
    mode: terrars::PrimField<std::string::String>,
    retain_until_time: terrars::PrimField<std::string::String>,
}

impl StorageBucketObjectRetentionEl { }

impl terrars::ToListMappable for StorageBucketObjectRetentionEl {
    type O = terrars::BlockAssignable<StorageBucketObjectRetentionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBucketObjectRetentionEl {
    #[doc = "The object retention mode. Supported values include: \"Unlocked\", \"Locked\"."]
    pub mode: terrars::PrimField<std::string::String>,
    #[doc = "Time in RFC 3339 (e.g. 2030-01-01T02:03:04Z) until which object retention protects this object."]
    pub retain_until_time: terrars::PrimField<std::string::String>,
}

impl BuildStorageBucketObjectRetentionEl {
    pub fn build(self) -> StorageBucketObjectRetentionEl {
        StorageBucketObjectRetentionEl {
            mode: self.mode,
            retain_until_time: self.retain_until_time,
        }
    }
}

pub struct StorageBucketObjectRetentionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBucketObjectRetentionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageBucketObjectRetentionElRef {
        StorageBucketObjectRetentionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBucketObjectRetentionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nThe object retention mode. Supported values include: \"Unlocked\", \"Locked\"."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retain_until_time` after provisioning.\nTime in RFC 3339 (e.g. 2030-01-01T02:03:04Z) until which object retention protects this object."]
    pub fn retain_until_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retain_until_time", self.base))
    }
}

#[derive(Serialize)]
pub struct StorageBucketObjectTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl StorageBucketObjectTimeoutsEl {
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

impl terrars::ToListMappable for StorageBucketObjectTimeoutsEl {
    type O = terrars::BlockAssignable<StorageBucketObjectTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStorageBucketObjectTimeoutsEl {}

impl BuildStorageBucketObjectTimeoutsEl {
    pub fn build(self) -> StorageBucketObjectTimeoutsEl {
        StorageBucketObjectTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct StorageBucketObjectTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StorageBucketObjectTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StorageBucketObjectTimeoutsElRef {
        StorageBucketObjectTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StorageBucketObjectTimeoutsElRef {
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
struct StorageBucketObjectDynamic {
    customer_encryption: std::option::Option<terrars::DynamicBlock<StorageBucketObjectCustomerEncryptionEl>>,
    retention: std::option::Option<terrars::DynamicBlock<StorageBucketObjectRetentionEl>>,
}
