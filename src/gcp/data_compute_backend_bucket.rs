use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataComputeBackendBucketData {
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

struct DataComputeBackendBucket_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataComputeBackendBucketData>,
}

#[derive(Clone)]
pub struct DataComputeBackendBucket(std::rc::Rc<DataComputeBackendBucket_>);

impl DataComputeBackendBucket {
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

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\nCloud Storage bucket name."]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cdn_policy` after provisioning.\nCloud CDN configuration for this Backend Bucket."]
    pub fn cdn_policy(&self) -> terrars::ListRef<DataComputeBackendBucketCdnPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cdn_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `compression_mode` after provisioning.\nCompress text responses using Brotli or gzip compression, based on the client's Accept-Encoding header. Possible values: [\"AUTOMATIC\", \"DISABLED\"]"]
    pub fn compression_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compression_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_response_headers` after provisioning.\nHeaders that the HTTP/S load balancer should add to proxied responses."]
    pub fn custom_response_headers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_response_headers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional textual description of the resource; provided by the\nclient when the resource is created."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `edge_security_policy` after provisioning.\nThe security policy associated with this backend bucket."]
    pub fn edge_security_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.edge_security_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_cdn` after provisioning.\nIf true, enable Cloud CDN for this BackendBucket."]
    pub fn enable_cdn(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_cdn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035.  Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means\nthe first character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the\nlast character, which cannot be a dash."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }
}

impl terrars::Referable for DataComputeBackendBucket {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataComputeBackendBucket { }

impl terrars::ToListMappable for DataComputeBackendBucket {
    type O = terrars::ListRef<DataComputeBackendBucketRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataComputeBackendBucket_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_compute_backend_bucket".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataComputeBackendBucket {
    pub tf_id: std::string::String,
    #[doc =
        "Name of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035.  Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means\nthe first character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the\nlast character, which cannot be a dash."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataComputeBackendBucket {
    pub fn build(self, stack: &mut terrars::Stack) -> DataComputeBackendBucket {
        let out = DataComputeBackendBucket(std::rc::Rc::new(DataComputeBackendBucket_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataComputeBackendBucketData {
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

pub struct DataComputeBackendBucketRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeBackendBucketRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataComputeBackendBucketRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\nCloud Storage bucket name."]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `cdn_policy` after provisioning.\nCloud CDN configuration for this Backend Bucket."]
    pub fn cdn_policy(&self) -> terrars::ListRef<DataComputeBackendBucketCdnPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cdn_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `compression_mode` after provisioning.\nCompress text responses using Brotli or gzip compression, based on the client's Accept-Encoding header. Possible values: [\"AUTOMATIC\", \"DISABLED\"]"]
    pub fn compression_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compression_mode", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_timestamp", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_response_headers` after provisioning.\nHeaders that the HTTP/S load balancer should add to proxied responses."]
    pub fn custom_response_headers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_response_headers", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nAn optional textual description of the resource; provided by the\nclient when the resource is created."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `edge_security_policy` after provisioning.\nThe security policy associated with this backend bucket."]
    pub fn edge_security_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.edge_security_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_cdn` after provisioning.\nIf true, enable Cloud CDN for this BackendBucket."]
    pub fn enable_cdn(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_cdn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035.  Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means\nthe first character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the\nlast character, which cannot be a dash."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersEl {
    #[doc = "Set the field `header_name`.\n"]
    pub fn set_header_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.header_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersEl {
    type O = terrars::BlockAssignable<DataComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersEl {}

impl BuildDataComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersEl {
    pub fn build(self) -> DataComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersEl {
        DataComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersEl {
            header_name: core::default::Default::default(),
        }
    }
}

pub struct DataComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersElRef {
        DataComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header_name` after provisioning.\n"]
    pub fn header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeBackendBucketCdnPolicyElCacheKeyPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_http_headers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_string_whitelist: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataComputeBackendBucketCdnPolicyElCacheKeyPolicyEl {
    #[doc = "Set the field `include_http_headers`.\n"]
    pub fn set_include_http_headers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.include_http_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `query_string_whitelist`.\n"]
    pub fn set_query_string_whitelist(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.query_string_whitelist = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeBackendBucketCdnPolicyElCacheKeyPolicyEl {
    type O = terrars::BlockAssignable<DataComputeBackendBucketCdnPolicyElCacheKeyPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeBackendBucketCdnPolicyElCacheKeyPolicyEl {}

impl BuildDataComputeBackendBucketCdnPolicyElCacheKeyPolicyEl {
    pub fn build(self) -> DataComputeBackendBucketCdnPolicyElCacheKeyPolicyEl {
        DataComputeBackendBucketCdnPolicyElCacheKeyPolicyEl {
            include_http_headers: core::default::Default::default(),
            query_string_whitelist: core::default::Default::default(),
        }
    }
}

pub struct DataComputeBackendBucketCdnPolicyElCacheKeyPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeBackendBucketCdnPolicyElCacheKeyPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeBackendBucketCdnPolicyElCacheKeyPolicyElRef {
        DataComputeBackendBucketCdnPolicyElCacheKeyPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeBackendBucketCdnPolicyElCacheKeyPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `include_http_headers` after provisioning.\n"]
    pub fn include_http_headers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_http_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `query_string_whitelist` after provisioning.\n"]
    pub fn query_string_whitelist(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_string_whitelist", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeBackendBucketCdnPolicyElNegativeCachingPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ttl: std::option::Option<terrars::PrimField<f64>>,
}

impl DataComputeBackendBucketCdnPolicyElNegativeCachingPolicyEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ttl`.\n"]
    pub fn set_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ttl = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeBackendBucketCdnPolicyElNegativeCachingPolicyEl {
    type O = terrars::BlockAssignable<DataComputeBackendBucketCdnPolicyElNegativeCachingPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeBackendBucketCdnPolicyElNegativeCachingPolicyEl {}

impl BuildDataComputeBackendBucketCdnPolicyElNegativeCachingPolicyEl {
    pub fn build(self) -> DataComputeBackendBucketCdnPolicyElNegativeCachingPolicyEl {
        DataComputeBackendBucketCdnPolicyElNegativeCachingPolicyEl {
            code: core::default::Default::default(),
            ttl: core::default::Default::default(),
        }
    }
}

pub struct DataComputeBackendBucketCdnPolicyElNegativeCachingPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeBackendBucketCdnPolicyElNegativeCachingPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataComputeBackendBucketCdnPolicyElNegativeCachingPolicyElRef {
        DataComputeBackendBucketCdnPolicyElNegativeCachingPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeBackendBucketCdnPolicyElNegativeCachingPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.base))
    }
}

#[derive(Serialize)]
pub struct DataComputeBackendBucketCdnPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bypass_cache_on_request_headers: std
    ::option
    ::Option<terrars::ListField<DataComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_key_policy: std::option::Option<terrars::ListField<DataComputeBackendBucketCdnPolicyElCacheKeyPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_ttl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_ttl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_ttl: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    negative_caching: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    negative_caching_policy: std
    ::option
    ::Option<terrars::ListField<DataComputeBackendBucketCdnPolicyElNegativeCachingPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_coalescing: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    serve_while_stale: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    signed_url_cache_max_age_sec: std::option::Option<terrars::PrimField<f64>>,
}

impl DataComputeBackendBucketCdnPolicyEl {
    #[doc = "Set the field `bypass_cache_on_request_headers`.\n"]
    pub fn set_bypass_cache_on_request_headers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersEl>>,
    ) -> Self {
        self.bypass_cache_on_request_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cache_key_policy`.\n"]
    pub fn set_cache_key_policy(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeBackendBucketCdnPolicyElCacheKeyPolicyEl>>,
    ) -> Self {
        self.cache_key_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cache_mode`.\n"]
    pub fn set_cache_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cache_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_ttl`.\n"]
    pub fn set_client_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.client_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_ttl`.\n"]
    pub fn set_default_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.default_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_ttl`.\n"]
    pub fn set_max_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `negative_caching`.\n"]
    pub fn set_negative_caching(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.negative_caching = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `negative_caching_policy`.\n"]
    pub fn set_negative_caching_policy(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataComputeBackendBucketCdnPolicyElNegativeCachingPolicyEl>>,
    ) -> Self {
        self.negative_caching_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `request_coalescing`.\n"]
    pub fn set_request_coalescing(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.request_coalescing = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `serve_while_stale`.\n"]
    pub fn set_serve_while_stale(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.serve_while_stale = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `signed_url_cache_max_age_sec`.\n"]
    pub fn set_signed_url_cache_max_age_sec(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.signed_url_cache_max_age_sec = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataComputeBackendBucketCdnPolicyEl {
    type O = terrars::BlockAssignable<DataComputeBackendBucketCdnPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataComputeBackendBucketCdnPolicyEl {}

impl BuildDataComputeBackendBucketCdnPolicyEl {
    pub fn build(self) -> DataComputeBackendBucketCdnPolicyEl {
        DataComputeBackendBucketCdnPolicyEl {
            bypass_cache_on_request_headers: core::default::Default::default(),
            cache_key_policy: core::default::Default::default(),
            cache_mode: core::default::Default::default(),
            client_ttl: core::default::Default::default(),
            default_ttl: core::default::Default::default(),
            max_ttl: core::default::Default::default(),
            negative_caching: core::default::Default::default(),
            negative_caching_policy: core::default::Default::default(),
            request_coalescing: core::default::Default::default(),
            serve_while_stale: core::default::Default::default(),
            signed_url_cache_max_age_sec: core::default::Default::default(),
        }
    }
}

pub struct DataComputeBackendBucketCdnPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataComputeBackendBucketCdnPolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataComputeBackendBucketCdnPolicyElRef {
        DataComputeBackendBucketCdnPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataComputeBackendBucketCdnPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bypass_cache_on_request_headers` after provisioning.\n"]
    pub fn bypass_cache_on_request_headers(
        &self,
    ) -> terrars::ListRef<DataComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bypass_cache_on_request_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_key_policy` after provisioning.\n"]
    pub fn cache_key_policy(&self) -> terrars::ListRef<DataComputeBackendBucketCdnPolicyElCacheKeyPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_key_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_mode` after provisioning.\n"]
    pub fn cache_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `client_ttl` after provisioning.\n"]
    pub fn client_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `default_ttl` after provisioning.\n"]
    pub fn default_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `max_ttl` after provisioning.\n"]
    pub fn max_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `negative_caching` after provisioning.\n"]
    pub fn negative_caching(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.negative_caching", self.base))
    }

    #[doc = "Get a reference to the value of field `negative_caching_policy` after provisioning.\n"]
    pub fn negative_caching_policy(
        &self,
    ) -> terrars::ListRef<DataComputeBackendBucketCdnPolicyElNegativeCachingPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.negative_caching_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `request_coalescing` after provisioning.\n"]
    pub fn request_coalescing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_coalescing", self.base))
    }

    #[doc = "Get a reference to the value of field `serve_while_stale` after provisioning.\n"]
    pub fn serve_while_stale(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serve_while_stale", self.base))
    }

    #[doc = "Get a reference to the value of field `signed_url_cache_max_age_sec` after provisioning.\n"]
    pub fn signed_url_cache_max_age_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.signed_url_cache_max_age_sec", self.base))
    }
}
