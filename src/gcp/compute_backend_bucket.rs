#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ComputeBackendBucketData_ {
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
    bucket_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compression_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_response_headers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    edge_security_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_cdn: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancing_scheme: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cdn_policy: std::option::Option<std::vec::Vec<ComputeBackendBucket_CdnPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    params: std::option::Option<std::vec::Vec<ComputeBackendBucket_Params>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ComputeBackendBucket_Timeouts>,
    dynamic: ComputeBackendBucketDynamic,
}

struct ComputeBackendBucket_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ComputeBackendBucketData_>,
}

#[derive(Clone)]
pub struct ComputeBackendBucket(std::rc::Rc<ComputeBackendBucket_>);

impl ComputeBackendBucket {
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
        "Set the field `compression_mode`.\nCompress text responses using Brotli or gzip compression, based on the client's Accept-Encoding header. Possible values: [\"AUTOMATIC\", \"DISABLED\"]"]
    pub fn set_compression_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().compression_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `custom_response_headers`.\nHeaders that the HTTP/S load balancer should add to proxied responses."]
    pub fn set_custom_response_headers(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().custom_response_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `description`.\nAn optional textual description of the resource; provided by the\nclient when the resource is created."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `edge_security_policy`.\nThe security policy associated with this backend bucket."]
    pub fn set_edge_security_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().edge_security_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_cdn`.\nIf true, enable Cloud CDN for this BackendBucket.\nNote: This cannot be set to true when loadBalancingScheme is set to INTERNAL_MANAGED."]
    pub fn set_enable_cdn(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_cdn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `load_balancing_scheme`.\nThe value can only be INTERNAL_MANAGED for cross-region internal layer 7 load balancer.\nIf loadBalancingScheme is not specified, the backend bucket can be used by classic global external load balancers, or global application external load balancers, or both.\nImportant: CDN cannot be enabled (enableCdn cannot be set to true) when loadBalancingScheme is set to INTERNAL_MANAGED. Possible values: [\"INTERNAL_MANAGED\"]"]
    pub fn set_load_balancing_scheme(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().load_balancing_scheme = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cdn_policy`.\n"]
    pub fn set_cdn_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeBackendBucket_CdnPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cdn_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cdn_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `params`.\n"]
    pub fn set_params(self, v: impl std::convert::Into<terrars::BlockAssignable<ComputeBackendBucket_Params>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().params = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.params = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ComputeBackendBucket_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\nCloud Storage bucket name."]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.extract_ref()))
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
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
        "Get a reference to the value of field `enable_cdn` after provisioning.\nIf true, enable Cloud CDN for this BackendBucket.\nNote: This cannot be set to true when loadBalancingScheme is set to INTERNAL_MANAGED."]
    pub fn enable_cdn(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_cdn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `load_balancing_scheme` after provisioning.\nThe value can only be INTERNAL_MANAGED for cross-region internal layer 7 load balancer.\nIf loadBalancingScheme is not specified, the backend bucket can be used by classic global external load balancers, or global application external load balancers, or both.\nImportant: CDN cannot be enabled (enableCdn cannot be set to true) when loadBalancingScheme is set to INTERNAL_MANAGED. Possible values: [\"INTERNAL_MANAGED\"]"]
    pub fn load_balancing_scheme(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.load_balancing_scheme", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `cdn_policy` after provisioning.\n"]
    pub fn cdn_policy(&self) -> terrars::ListRef<ComputeBackendBucket_CdnPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cdn_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> terrars::ListRef<ComputeBackendBucket_ParamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.params", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeBackendBucket_TimeoutsRef {
        ComputeBackendBucket_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ComputeBackendBucket {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ComputeBackendBucket { }

impl terrars::ToListMappable for ComputeBackendBucket {
    type O = terrars::ListRef<ComputeBackendBucketRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ComputeBackendBucket_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_compute_backend_bucket".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildComputeBackendBucket {
    pub tf_id: std::string::String,
    #[doc = "Cloud Storage bucket name."]
    pub bucket_name: terrars::PrimField<std::string::String>,
    #[doc =
        "Name of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035.  Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means\nthe first character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the\nlast character, which cannot be a dash."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildComputeBackendBucket {
    pub fn build(self, stack: &mut terrars::Stack) -> ComputeBackendBucket {
        let out = ComputeBackendBucket(std::rc::Rc::new(ComputeBackendBucket_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ComputeBackendBucketData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                bucket_name: self.bucket_name,
                compression_mode: core::default::Default::default(),
                custom_response_headers: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                edge_security_policy: core::default::Default::default(),
                enable_cdn: core::default::Default::default(),
                id: core::default::Default::default(),
                load_balancing_scheme: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                cdn_policy: core::default::Default::default(),
                params: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ComputeBackendBucketRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeBackendBucketRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ComputeBackendBucketRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\nCloud Storage bucket name."]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.extract_ref()))
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
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
        "Get a reference to the value of field `enable_cdn` after provisioning.\nIf true, enable Cloud CDN for this BackendBucket.\nNote: This cannot be set to true when loadBalancingScheme is set to INTERNAL_MANAGED."]
    pub fn enable_cdn(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_cdn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `load_balancing_scheme` after provisioning.\nThe value can only be INTERNAL_MANAGED for cross-region internal layer 7 load balancer.\nIf loadBalancingScheme is not specified, the backend bucket can be used by classic global external load balancers, or global application external load balancers, or both.\nImportant: CDN cannot be enabled (enableCdn cannot be set to true) when loadBalancingScheme is set to INTERNAL_MANAGED. Possible values: [\"INTERNAL_MANAGED\"]"]
    pub fn load_balancing_scheme(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.load_balancing_scheme", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `cdn_policy` after provisioning.\n"]
    pub fn cdn_policy(&self) -> terrars::ListRef<ComputeBackendBucket_CdnPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cdn_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> terrars::ListRef<ComputeBackendBucket_ParamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.params", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeBackendBucket_TimeoutsRef {
        ComputeBackendBucket_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ComputeBackendBucketDynamic {
    cdn_policy: std::option::Option<terrars::DynamicBlock<ComputeBackendBucket_CdnPolicy>>,
    params: std::option::Option<terrars::DynamicBlock<ComputeBackendBucket_Params>>,
}

#[derive(Serialize)]
pub struct ComputeBackendBucket_CdnPolicy_BypassCacheOnRequestHeaders {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeBackendBucket_CdnPolicy_BypassCacheOnRequestHeaders {
    #[doc =
        "Set the field `header_name`.\nThe header field name to match on when bypassing cache. Values are case-insensitive."]
    pub fn set_header_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.header_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeBackendBucket_CdnPolicy_BypassCacheOnRequestHeaders {
    type O = terrars::BlockAssignable<ComputeBackendBucket_CdnPolicy_BypassCacheOnRequestHeaders>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeBackendBucket_CdnPolicy_BypassCacheOnRequestHeaders {}

impl BuildComputeBackendBucket_CdnPolicy_BypassCacheOnRequestHeaders {
    pub fn build(self) -> ComputeBackendBucket_CdnPolicy_BypassCacheOnRequestHeaders {
        ComputeBackendBucket_CdnPolicy_BypassCacheOnRequestHeaders { header_name: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ComputeBackendBucket_CdnPolicy_BypassCacheOnRequestHeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeBackendBucket_CdnPolicy_BypassCacheOnRequestHeadersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeBackendBucket_CdnPolicy_BypassCacheOnRequestHeadersRef {
        ComputeBackendBucket_CdnPolicy_BypassCacheOnRequestHeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeBackendBucket_CdnPolicy_BypassCacheOnRequestHeadersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `header_name` after provisioning.\nThe header field name to match on when bypassing cache. Values are case-insensitive."]
    pub fn header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeBackendBucket_CdnPolicy_CacheKeyPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_http_headers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_string_whitelist: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ComputeBackendBucket_CdnPolicy_CacheKeyPolicy {
    #[doc =
        "Set the field `include_http_headers`.\nAllows HTTP request headers (by name) to be used in the\ncache key."]
    pub fn set_include_http_headers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.include_http_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `query_string_whitelist`.\nNames of query string parameters to include in cache keys.\nDefault parameters are always included. '&' and '=' will\nbe percent encoded and not treated as delimiters."]
    pub fn set_query_string_whitelist(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.query_string_whitelist = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeBackendBucket_CdnPolicy_CacheKeyPolicy {
    type O = terrars::BlockAssignable<ComputeBackendBucket_CdnPolicy_CacheKeyPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeBackendBucket_CdnPolicy_CacheKeyPolicy {}

impl BuildComputeBackendBucket_CdnPolicy_CacheKeyPolicy {
    pub fn build(self) -> ComputeBackendBucket_CdnPolicy_CacheKeyPolicy {
        ComputeBackendBucket_CdnPolicy_CacheKeyPolicy {
            include_http_headers: core::default::Default::default(),
            query_string_whitelist: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeBackendBucket_CdnPolicy_CacheKeyPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeBackendBucket_CdnPolicy_CacheKeyPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeBackendBucket_CdnPolicy_CacheKeyPolicyRef {
        ComputeBackendBucket_CdnPolicy_CacheKeyPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeBackendBucket_CdnPolicy_CacheKeyPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `include_http_headers` after provisioning.\nAllows HTTP request headers (by name) to be used in the\ncache key."]
    pub fn include_http_headers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_http_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `query_string_whitelist` after provisioning.\nNames of query string parameters to include in cache keys.\nDefault parameters are always included. '&' and '=' will\nbe percent encoded and not treated as delimiters."]
    pub fn query_string_whitelist(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_string_whitelist", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeBackendBucket_CdnPolicy_NegativeCachingPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ttl: std::option::Option<terrars::PrimField<f64>>,
}

impl ComputeBackendBucket_CdnPolicy_NegativeCachingPolicy {
    #[doc =
        "Set the field `code`.\nThe HTTP status code to define a TTL against. Only HTTP status codes 300, 301, 308, 404, 405, 410, 421, 451 and 501\ncan be specified as values, and you cannot specify a status code more than once."]
    pub fn set_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.code = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ttl`.\nThe TTL (in seconds) for which to cache responses with the corresponding status code. The maximum allowed value is 1800s\n(30 minutes), noting that infrequently accessed objects may be evicted from the cache before the defined TTL."]
    pub fn set_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ttl = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeBackendBucket_CdnPolicy_NegativeCachingPolicy {
    type O = terrars::BlockAssignable<ComputeBackendBucket_CdnPolicy_NegativeCachingPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeBackendBucket_CdnPolicy_NegativeCachingPolicy {}

impl BuildComputeBackendBucket_CdnPolicy_NegativeCachingPolicy {
    pub fn build(self) -> ComputeBackendBucket_CdnPolicy_NegativeCachingPolicy {
        ComputeBackendBucket_CdnPolicy_NegativeCachingPolicy {
            code: core::default::Default::default(),
            ttl: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeBackendBucket_CdnPolicy_NegativeCachingPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeBackendBucket_CdnPolicy_NegativeCachingPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ComputeBackendBucket_CdnPolicy_NegativeCachingPolicyRef {
        ComputeBackendBucket_CdnPolicy_NegativeCachingPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeBackendBucket_CdnPolicy_NegativeCachingPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `code` after provisioning.\nThe HTTP status code to define a TTL against. Only HTTP status codes 300, 301, 308, 404, 405, 410, 421, 451 and 501\ncan be specified as values, and you cannot specify a status code more than once."]
    pub fn code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ttl` after provisioning.\nThe TTL (in seconds) for which to cache responses with the corresponding status code. The maximum allowed value is 1800s\n(30 minutes), noting that infrequently accessed objects may be evicted from the cache before the defined TTL."]
    pub fn ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.base))
    }
}

#[derive(Serialize, Default)]
struct ComputeBackendBucket_CdnPolicyDynamic {
    bypass_cache_on_request_headers: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeBackendBucket_CdnPolicy_BypassCacheOnRequestHeaders>>,
    cache_key_policy: std::option::Option<terrars::DynamicBlock<ComputeBackendBucket_CdnPolicy_CacheKeyPolicy>>,
    negative_caching_policy: std
    ::option
    ::Option<terrars::DynamicBlock<ComputeBackendBucket_CdnPolicy_NegativeCachingPolicy>>,
}

#[derive(Serialize)]
pub struct ComputeBackendBucket_CdnPolicy {
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
    request_coalescing: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    serve_while_stale: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    signed_url_cache_max_age_sec: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bypass_cache_on_request_headers: std
    ::option
    ::Option<std::vec::Vec<ComputeBackendBucket_CdnPolicy_BypassCacheOnRequestHeaders>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_key_policy: std::option::Option<std::vec::Vec<ComputeBackendBucket_CdnPolicy_CacheKeyPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    negative_caching_policy: std::option::Option<std::vec::Vec<ComputeBackendBucket_CdnPolicy_NegativeCachingPolicy>>,
    dynamic: ComputeBackendBucket_CdnPolicyDynamic,
}

impl ComputeBackendBucket_CdnPolicy {
    #[doc =
        "Set the field `cache_mode`.\nSpecifies the cache setting for all responses from this backend.\nThe possible values are: USE_ORIGIN_HEADERS, FORCE_CACHE_ALL and CACHE_ALL_STATIC Possible values: [\"USE_ORIGIN_HEADERS\", \"FORCE_CACHE_ALL\", \"CACHE_ALL_STATIC\"]"]
    pub fn set_cache_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cache_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `client_ttl`.\nSpecifies the maximum allowed TTL for cached content served by this origin. When the\n'cache_mode' is set to \"USE_ORIGIN_HEADERS\", you must omit this field."]
    pub fn set_client_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.client_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `default_ttl`.\nSpecifies the default TTL for cached content served by this origin for responses\nthat do not have an existing valid TTL (max-age or s-max-age). When the 'cache_mode'\nis set to \"USE_ORIGIN_HEADERS\", you must omit this field."]
    pub fn set_default_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.default_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_ttl`.\nSpecifies the maximum allowed TTL for cached content served by this origin. When the\n'cache_mode' is set to \"USE_ORIGIN_HEADERS\", you must omit this field."]
    pub fn set_max_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `negative_caching`.\nNegative caching allows per-status code TTLs to be set, in order to apply fine-grained caching for common errors or redirects."]
    pub fn set_negative_caching(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.negative_caching = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `request_coalescing`.\nIf true then Cloud CDN will combine multiple concurrent cache fill requests into a small number of requests to the origin."]
    pub fn set_request_coalescing(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.request_coalescing = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `serve_while_stale`.\nServe existing content from the cache (if available) when revalidating content with the origin, or when an error is encountered when refreshing the cache."]
    pub fn set_serve_while_stale(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.serve_while_stale = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `signed_url_cache_max_age_sec`.\nMaximum number of seconds the response to a signed URL request will\nbe considered fresh. After this time period,\nthe response will be revalidated before being served.\nWhen serving responses to signed URL requests,\nCloud CDN will internally behave as though\nall responses from this backend had a \"Cache-Control: public,\nmax-age=[TTL]\" header, regardless of any existing Cache-Control\nheader. The actual headers served in responses will not be altered."]
    pub fn set_signed_url_cache_max_age_sec(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.signed_url_cache_max_age_sec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bypass_cache_on_request_headers`.\n"]
    pub fn set_bypass_cache_on_request_headers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ComputeBackendBucket_CdnPolicy_BypassCacheOnRequestHeaders>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bypass_cache_on_request_headers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bypass_cache_on_request_headers = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cache_key_policy`.\n"]
    pub fn set_cache_key_policy(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeBackendBucket_CdnPolicy_CacheKeyPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cache_key_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cache_key_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `negative_caching_policy`.\n"]
    pub fn set_negative_caching_policy(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ComputeBackendBucket_CdnPolicy_NegativeCachingPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.negative_caching_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.negative_caching_policy = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ComputeBackendBucket_CdnPolicy {
    type O = terrars::BlockAssignable<ComputeBackendBucket_CdnPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeBackendBucket_CdnPolicy {}

impl BuildComputeBackendBucket_CdnPolicy {
    pub fn build(self) -> ComputeBackendBucket_CdnPolicy {
        ComputeBackendBucket_CdnPolicy {
            cache_mode: core::default::Default::default(),
            client_ttl: core::default::Default::default(),
            default_ttl: core::default::Default::default(),
            max_ttl: core::default::Default::default(),
            negative_caching: core::default::Default::default(),
            request_coalescing: core::default::Default::default(),
            serve_while_stale: core::default::Default::default(),
            signed_url_cache_max_age_sec: core::default::Default::default(),
            bypass_cache_on_request_headers: core::default::Default::default(),
            cache_key_policy: core::default::Default::default(),
            negative_caching_policy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeBackendBucket_CdnPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeBackendBucket_CdnPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeBackendBucket_CdnPolicyRef {
        ComputeBackendBucket_CdnPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeBackendBucket_CdnPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cache_mode` after provisioning.\nSpecifies the cache setting for all responses from this backend.\nThe possible values are: USE_ORIGIN_HEADERS, FORCE_CACHE_ALL and CACHE_ALL_STATIC Possible values: [\"USE_ORIGIN_HEADERS\", \"FORCE_CACHE_ALL\", \"CACHE_ALL_STATIC\"]"]
    pub fn cache_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_ttl` after provisioning.\nSpecifies the maximum allowed TTL for cached content served by this origin. When the\n'cache_mode' is set to \"USE_ORIGIN_HEADERS\", you must omit this field."]
    pub fn client_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_ttl", self.base))
    }

    #[doc =
        "Get a reference to the value of field `default_ttl` after provisioning.\nSpecifies the default TTL for cached content served by this origin for responses\nthat do not have an existing valid TTL (max-age or s-max-age). When the 'cache_mode'\nis set to \"USE_ORIGIN_HEADERS\", you must omit this field."]
    pub fn default_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_ttl", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_ttl` after provisioning.\nSpecifies the maximum allowed TTL for cached content served by this origin. When the\n'cache_mode' is set to \"USE_ORIGIN_HEADERS\", you must omit this field."]
    pub fn max_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_ttl", self.base))
    }

    #[doc =
        "Get a reference to the value of field `negative_caching` after provisioning.\nNegative caching allows per-status code TTLs to be set, in order to apply fine-grained caching for common errors or redirects."]
    pub fn negative_caching(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.negative_caching", self.base))
    }

    #[doc =
        "Get a reference to the value of field `request_coalescing` after provisioning.\nIf true then Cloud CDN will combine multiple concurrent cache fill requests into a small number of requests to the origin."]
    pub fn request_coalescing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_coalescing", self.base))
    }

    #[doc =
        "Get a reference to the value of field `serve_while_stale` after provisioning.\nServe existing content from the cache (if available) when revalidating content with the origin, or when an error is encountered when refreshing the cache."]
    pub fn serve_while_stale(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serve_while_stale", self.base))
    }

    #[doc =
        "Get a reference to the value of field `signed_url_cache_max_age_sec` after provisioning.\nMaximum number of seconds the response to a signed URL request will\nbe considered fresh. After this time period,\nthe response will be revalidated before being served.\nWhen serving responses to signed URL requests,\nCloud CDN will internally behave as though\nall responses from this backend had a \"Cache-Control: public,\nmax-age=[TTL]\" header, regardless of any existing Cache-Control\nheader. The actual headers served in responses will not be altered."]
    pub fn signed_url_cache_max_age_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.signed_url_cache_max_age_sec", self.base))
    }

    #[doc = "Get a reference to the value of field `bypass_cache_on_request_headers` after provisioning.\n"]
    pub fn bypass_cache_on_request_headers(
        &self,
    ) -> terrars::ListRef<ComputeBackendBucket_CdnPolicy_BypassCacheOnRequestHeadersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bypass_cache_on_request_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `cache_key_policy` after provisioning.\n"]
    pub fn cache_key_policy(&self) -> terrars::ListRef<ComputeBackendBucket_CdnPolicy_CacheKeyPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_key_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `negative_caching_policy` after provisioning.\n"]
    pub fn negative_caching_policy(&self) -> terrars::ListRef<ComputeBackendBucket_CdnPolicy_NegativeCachingPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.negative_caching_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeBackendBucket_Params {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_manager_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl ComputeBackendBucket_Params {
    #[doc =
        "Set the field `resource_manager_tags`.\nResource manager tags to be bound to the backend bucket. Tag keys and values have the\nsame definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id},\nand values are in the format tagValues/456."]
    pub fn set_resource_manager_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resource_manager_tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ComputeBackendBucket_Params {
    type O = terrars::BlockAssignable<ComputeBackendBucket_Params>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeBackendBucket_Params {}

impl BuildComputeBackendBucket_Params {
    pub fn build(self) -> ComputeBackendBucket_Params {
        ComputeBackendBucket_Params { resource_manager_tags: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ComputeBackendBucket_ParamsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeBackendBucket_ParamsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeBackendBucket_ParamsRef {
        ComputeBackendBucket_ParamsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeBackendBucket_ParamsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `resource_manager_tags` after provisioning.\nResource manager tags to be bound to the backend bucket. Tag keys and values have the\nsame definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id},\nand values are in the format tagValues/456."]
    pub fn resource_manager_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.resource_manager_tags", self.base))
    }
}

#[derive(Serialize)]
pub struct ComputeBackendBucket_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ComputeBackendBucket_Timeouts {
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

impl terrars::ToListMappable for ComputeBackendBucket_Timeouts {
    type O = terrars::BlockAssignable<ComputeBackendBucket_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildComputeBackendBucket_Timeouts {}

impl BuildComputeBackendBucket_Timeouts {
    pub fn build(self) -> ComputeBackendBucket_Timeouts {
        ComputeBackendBucket_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ComputeBackendBucket_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ComputeBackendBucket_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ComputeBackendBucket_TimeoutsRef {
        ComputeBackendBucket_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ComputeBackendBucket_TimeoutsRef {
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
