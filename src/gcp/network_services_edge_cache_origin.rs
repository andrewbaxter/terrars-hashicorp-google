#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct NetworkServicesEdgeCacheOriginData_ {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failover_origin: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_attempts: std::option::Option<terrars::PrimField<f64>>,
    name: terrars::PrimField<std::string::String>,
    origin_address: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_conditions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_v4_authentication: std::option::Option<std::vec::Vec<NetworkServicesEdgeCacheOrigin_AwsV4Authentication>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flex_shielding: std::option::Option<std::vec::Vec<NetworkServicesEdgeCacheOrigin_FlexShielding>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_override_action: std::option::Option<std::vec::Vec<NetworkServicesEdgeCacheOrigin_OriginOverrideAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin_redirect: std::option::Option<std::vec::Vec<NetworkServicesEdgeCacheOrigin_OriginRedirect>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<std::vec::Vec<NetworkServicesEdgeCacheOrigin_Timeout>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<NetworkServicesEdgeCacheOrigin_Timeouts>,
    dynamic: NetworkServicesEdgeCacheOriginDynamic,
}

struct NetworkServicesEdgeCacheOrigin_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<NetworkServicesEdgeCacheOriginData_>,
}

#[derive(Clone)]
pub struct NetworkServicesEdgeCacheOrigin(std::rc::Rc<NetworkServicesEdgeCacheOrigin_>);

impl NetworkServicesEdgeCacheOrigin {
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

    #[doc = "Set the field `description`.\nA human-readable description of the resource."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `failover_origin`.\nThe Origin resource to try when the current origin cannot be reached.\nAfter maxAttempts is reached, the configured failoverOrigin will be used to fulfil the request.\n\nThe value of timeout.maxAttemptsTimeout dictates the timeout across all origins.\nA reference to a Topic resource."]
    pub fn set_failover_origin(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().failover_origin = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nSet of label tags associated with the EdgeCache resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_attempts`.\nThe maximum number of attempts to cache fill from this origin. Another attempt is made when a cache fill fails with one of the retryConditions.\n\nOnce maxAttempts to this origin have failed the failoverOrigin will be used, if one is specified. That failoverOrigin may specify its own maxAttempts,\nretryConditions and failoverOrigin to control its own cache fill failures.\n\nThe total number of allowed attempts to cache fill across this and failover origins is limited to four.\nThe total time allowed for cache fill attempts across this and failover origins can be controlled with maxAttemptsTimeout.\n\nThe last valid, non-retried response from all origins will be returned to the client.\nIf no origin returns a valid response, an HTTP 502 will be returned to the client.\n\nDefaults to 1. Must be a value greater than 0 and less than 4."]
    pub fn set_max_attempts(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_attempts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `port`.\nThe port to connect to the origin on.\nDefaults to port 443 for HTTP2 and HTTPS protocols, and port 80 for HTTP."]
    pub fn set_port(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `protocol`.\nThe protocol to use to connect to the configured origin. Defaults to HTTP2, and it is strongly recommended that users use HTTP2 for both security & performance.\n\nWhen using HTTP2 or HTTPS as the protocol, a valid, publicly-signed, unexpired TLS (SSL) certificate must be presented by the origin server. Possible values: [\"HTTP2\", \"HTTPS\", \"HTTP\"]"]
    pub fn set_protocol(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `retry_conditions`.\nSpecifies one or more retry conditions for the configured origin.\n\nIf the failure mode during a connection attempt to the origin matches the configured retryCondition(s),\nthe origin request will be retried up to maxAttempts times. The failoverOrigin, if configured, will then be used to satisfy the request.\n\nThe default retryCondition is \"CONNECT_FAILURE\".\n\nretryConditions apply to this origin, and not subsequent failoverOrigin(s),\nwhich may specify their own retryConditions and maxAttempts.\n\nValid values are:\n\n- CONNECT_FAILURE: Retry on failures connecting to origins, for example due to connection timeouts.\n- HTTP_5XX: Retry if the origin responds with any 5xx response code, or if the origin does not respond at all, example: disconnects, reset, read timeout, connection failure, and refused streams.\n- GATEWAY_ERROR: Similar to 5xx, but only applies to response codes 502, 503 or 504.\n- RETRIABLE_4XX: Retry for retriable 4xx response codes, which include HTTP 409 (Conflict) and HTTP 429 (Too Many Requests)\n- NOT_FOUND: Retry if the origin returns a HTTP 404 (Not Found). This can be useful when generating video content, and the segment is not available yet.\n- FORBIDDEN: Retry if the origin returns a HTTP 403 (Forbidden). Possible values: [\"CONNECT_FAILURE\", \"HTTP_5XX\", \"GATEWAY_ERROR\", \"RETRIABLE_4XX\", \"NOT_FOUND\", \"FORBIDDEN\"]"]
    pub fn set_retry_conditions(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().retry_conditions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `aws_v4_authentication`.\n"]
    pub fn set_aws_v4_authentication(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkServicesEdgeCacheOrigin_AwsV4Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().aws_v4_authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.aws_v4_authentication = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `flex_shielding`.\n"]
    pub fn set_flex_shielding(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkServicesEdgeCacheOrigin_FlexShielding>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().flex_shielding = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.flex_shielding = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `origin_override_action`.\n"]
    pub fn set_origin_override_action(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkServicesEdgeCacheOrigin_OriginOverrideAction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().origin_override_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.origin_override_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `origin_redirect`.\n"]
    pub fn set_origin_redirect(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkServicesEdgeCacheOrigin_OriginRedirect>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().origin_redirect = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.origin_redirect = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkServicesEdgeCacheOrigin_Timeout>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().timeout = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.timeout = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<NetworkServicesEdgeCacheOrigin_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `failover_origin` after provisioning.\nThe Origin resource to try when the current origin cannot be reached.\nAfter maxAttempts is reached, the configured failoverOrigin will be used to fulfil the request.\n\nThe value of timeout.maxAttemptsTimeout dictates the timeout across all origins.\nA reference to a Topic resource."]
    pub fn failover_origin(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failover_origin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the EdgeCache resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_attempts` after provisioning.\nThe maximum number of attempts to cache fill from this origin. Another attempt is made when a cache fill fails with one of the retryConditions.\n\nOnce maxAttempts to this origin have failed the failoverOrigin will be used, if one is specified. That failoverOrigin may specify its own maxAttempts,\nretryConditions and failoverOrigin to control its own cache fill failures.\n\nThe total number of allowed attempts to cache fill across this and failover origins is limited to four.\nThe total time allowed for cache fill attempts across this and failover origins can be controlled with maxAttemptsTimeout.\n\nThe last valid, non-retried response from all origins will be returned to the client.\nIf no origin returns a valid response, an HTTP 502 will be returned to the client.\n\nDefaults to 1. Must be a value greater than 0 and less than 4."]
    pub fn max_attempts(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_attempts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource; provided by the client when the resource is created.\nThe name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]* which means the first character must be a letter,\nand all following characters must be a dash, underscore, letter or digit."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `origin_address` after provisioning.\nA fully qualified domain name (FQDN) or IP address reachable over the public Internet, or the address of a Google Cloud Storage bucket.\n\nThis address will be used as the origin for cache requests - e.g. FQDN: media-backend.example.com, IPv4: 35.218.1.1, IPv6: 2607:f8b0:4012:809::200e, Cloud Storage: gs://bucketname\n\nWhen providing an FQDN (hostname), it must be publicly resolvable (e.g. via Google public DNS) and IP addresses must be publicly routable.  It must not contain a protocol (e.g., https://) and it must not contain any slashes.\nIf a Cloud Storage bucket is provided, it must be in the canonical \"gs://bucketname\" format. Other forms, such as \"storage.googleapis.com\", will be rejected."]
    pub fn origin_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe port to connect to the origin on.\nDefaults to port 443 for HTTP2 and HTTPS protocols, and port 80 for HTTP."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `protocol` after provisioning.\nThe protocol to use to connect to the configured origin. Defaults to HTTP2, and it is strongly recommended that users use HTTP2 for both security & performance.\n\nWhen using HTTP2 or HTTPS as the protocol, a valid, publicly-signed, unexpired TLS (SSL) certificate must be presented by the origin server. Possible values: [\"HTTP2\", \"HTTPS\", \"HTTP\"]"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `retry_conditions` after provisioning.\nSpecifies one or more retry conditions for the configured origin.\n\nIf the failure mode during a connection attempt to the origin matches the configured retryCondition(s),\nthe origin request will be retried up to maxAttempts times. The failoverOrigin, if configured, will then be used to satisfy the request.\n\nThe default retryCondition is \"CONNECT_FAILURE\".\n\nretryConditions apply to this origin, and not subsequent failoverOrigin(s),\nwhich may specify their own retryConditions and maxAttempts.\n\nValid values are:\n\n- CONNECT_FAILURE: Retry on failures connecting to origins, for example due to connection timeouts.\n- HTTP_5XX: Retry if the origin responds with any 5xx response code, or if the origin does not respond at all, example: disconnects, reset, read timeout, connection failure, and refused streams.\n- GATEWAY_ERROR: Similar to 5xx, but only applies to response codes 502, 503 or 504.\n- RETRIABLE_4XX: Retry for retriable 4xx response codes, which include HTTP 409 (Conflict) and HTTP 429 (Too Many Requests)\n- NOT_FOUND: Retry if the origin returns a HTTP 404 (Not Found). This can be useful when generating video content, and the segment is not available yet.\n- FORBIDDEN: Retry if the origin returns a HTTP 403 (Forbidden). Possible values: [\"CONNECT_FAILURE\", \"HTTP_5XX\", \"GATEWAY_ERROR\", \"RETRIABLE_4XX\", \"NOT_FOUND\", \"FORBIDDEN\"]"]
    pub fn retry_conditions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_conditions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aws_v4_authentication` after provisioning.\n"]
    pub fn aws_v4_authentication(&self) -> terrars::ListRef<NetworkServicesEdgeCacheOrigin_AwsV4AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_v4_authentication", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `flex_shielding` after provisioning.\n"]
    pub fn flex_shielding(&self) -> terrars::ListRef<NetworkServicesEdgeCacheOrigin_FlexShieldingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.flex_shielding", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `origin_override_action` after provisioning.\n"]
    pub fn origin_override_action(&self) -> terrars::ListRef<NetworkServicesEdgeCacheOrigin_OriginOverrideActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.origin_override_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `origin_redirect` after provisioning.\n"]
    pub fn origin_redirect(&self) -> terrars::ListRef<NetworkServicesEdgeCacheOrigin_OriginRedirectRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.origin_redirect", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::ListRef<NetworkServicesEdgeCacheOrigin_TimeoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesEdgeCacheOrigin_TimeoutsRef {
        NetworkServicesEdgeCacheOrigin_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for NetworkServicesEdgeCacheOrigin {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for NetworkServicesEdgeCacheOrigin { }

impl terrars::ToListMappable for NetworkServicesEdgeCacheOrigin {
    type O = terrars::ListRef<NetworkServicesEdgeCacheOriginRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for NetworkServicesEdgeCacheOrigin_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_network_services_edge_cache_origin".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetworkServicesEdgeCacheOrigin {
    pub tf_id: std::string::String,
    #[doc =
        "Name of the resource; provided by the client when the resource is created.\nThe name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]* which means the first character must be a letter,\nand all following characters must be a dash, underscore, letter or digit."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc =
        "A fully qualified domain name (FQDN) or IP address reachable over the public Internet, or the address of a Google Cloud Storage bucket.\n\nThis address will be used as the origin for cache requests - e.g. FQDN: media-backend.example.com, IPv4: 35.218.1.1, IPv6: 2607:f8b0:4012:809::200e, Cloud Storage: gs://bucketname\n\nWhen providing an FQDN (hostname), it must be publicly resolvable (e.g. via Google public DNS) and IP addresses must be publicly routable.  It must not contain a protocol (e.g., https://) and it must not contain any slashes.\nIf a Cloud Storage bucket is provided, it must be in the canonical \"gs://bucketname\" format. Other forms, such as \"storage.googleapis.com\", will be rejected."]
    pub origin_address: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesEdgeCacheOrigin {
    pub fn build(self, stack: &mut terrars::Stack) -> NetworkServicesEdgeCacheOrigin {
        let out = NetworkServicesEdgeCacheOrigin(std::rc::Rc::new(NetworkServicesEdgeCacheOrigin_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(NetworkServicesEdgeCacheOriginData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                failover_origin: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                max_attempts: core::default::Default::default(),
                name: self.name,
                origin_address: self.origin_address,
                port: core::default::Default::default(),
                project: core::default::Default::default(),
                protocol: core::default::Default::default(),
                retry_conditions: core::default::Default::default(),
                aws_v4_authentication: core::default::Default::default(),
                flex_shielding: core::default::Default::default(),
                origin_override_action: core::default::Default::default(),
                origin_redirect: core::default::Default::default(),
                timeout: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetworkServicesEdgeCacheOriginRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheOriginRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl NetworkServicesEdgeCacheOriginRef {
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
        "Get a reference to the value of field `description` after provisioning.\nA human-readable description of the resource."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `failover_origin` after provisioning.\nThe Origin resource to try when the current origin cannot be reached.\nAfter maxAttempts is reached, the configured failoverOrigin will be used to fulfil the request.\n\nThe value of timeout.maxAttemptsTimeout dictates the timeout across all origins.\nA reference to a Topic resource."]
    pub fn failover_origin(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failover_origin", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nSet of label tags associated with the EdgeCache resource.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `max_attempts` after provisioning.\nThe maximum number of attempts to cache fill from this origin. Another attempt is made when a cache fill fails with one of the retryConditions.\n\nOnce maxAttempts to this origin have failed the failoverOrigin will be used, if one is specified. That failoverOrigin may specify its own maxAttempts,\nretryConditions and failoverOrigin to control its own cache fill failures.\n\nThe total number of allowed attempts to cache fill across this and failover origins is limited to four.\nThe total time allowed for cache fill attempts across this and failover origins can be controlled with maxAttemptsTimeout.\n\nThe last valid, non-retried response from all origins will be returned to the client.\nIf no origin returns a valid response, an HTTP 502 will be returned to the client.\n\nDefaults to 1. Must be a value greater than 0 and less than 4."]
    pub fn max_attempts(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_attempts", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the resource; provided by the client when the resource is created.\nThe name must be 1-64 characters long, and match the regular expression [a-zA-Z][a-zA-Z0-9_-]* which means the first character must be a letter,\nand all following characters must be a dash, underscore, letter or digit."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `origin_address` after provisioning.\nA fully qualified domain name (FQDN) or IP address reachable over the public Internet, or the address of a Google Cloud Storage bucket.\n\nThis address will be used as the origin for cache requests - e.g. FQDN: media-backend.example.com, IPv4: 35.218.1.1, IPv6: 2607:f8b0:4012:809::200e, Cloud Storage: gs://bucketname\n\nWhen providing an FQDN (hostname), it must be publicly resolvable (e.g. via Google public DNS) and IP addresses must be publicly routable.  It must not contain a protocol (e.g., https://) and it must not contain any slashes.\nIf a Cloud Storage bucket is provided, it must be in the canonical \"gs://bucketname\" format. Other forms, such as \"storage.googleapis.com\", will be rejected."]
    pub fn origin_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_address", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nThe port to connect to the origin on.\nDefaults to port 443 for HTTP2 and HTTPS protocols, and port 80 for HTTP."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `protocol` after provisioning.\nThe protocol to use to connect to the configured origin. Defaults to HTTP2, and it is strongly recommended that users use HTTP2 for both security & performance.\n\nWhen using HTTP2 or HTTPS as the protocol, a valid, publicly-signed, unexpired TLS (SSL) certificate must be presented by the origin server. Possible values: [\"HTTP2\", \"HTTPS\", \"HTTP\"]"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `retry_conditions` after provisioning.\nSpecifies one or more retry conditions for the configured origin.\n\nIf the failure mode during a connection attempt to the origin matches the configured retryCondition(s),\nthe origin request will be retried up to maxAttempts times. The failoverOrigin, if configured, will then be used to satisfy the request.\n\nThe default retryCondition is \"CONNECT_FAILURE\".\n\nretryConditions apply to this origin, and not subsequent failoverOrigin(s),\nwhich may specify their own retryConditions and maxAttempts.\n\nValid values are:\n\n- CONNECT_FAILURE: Retry on failures connecting to origins, for example due to connection timeouts.\n- HTTP_5XX: Retry if the origin responds with any 5xx response code, or if the origin does not respond at all, example: disconnects, reset, read timeout, connection failure, and refused streams.\n- GATEWAY_ERROR: Similar to 5xx, but only applies to response codes 502, 503 or 504.\n- RETRIABLE_4XX: Retry for retriable 4xx response codes, which include HTTP 409 (Conflict) and HTTP 429 (Too Many Requests)\n- NOT_FOUND: Retry if the origin returns a HTTP 404 (Not Found). This can be useful when generating video content, and the segment is not available yet.\n- FORBIDDEN: Retry if the origin returns a HTTP 403 (Forbidden). Possible values: [\"CONNECT_FAILURE\", \"HTTP_5XX\", \"GATEWAY_ERROR\", \"RETRIABLE_4XX\", \"NOT_FOUND\", \"FORBIDDEN\"]"]
    pub fn retry_conditions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_conditions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aws_v4_authentication` after provisioning.\n"]
    pub fn aws_v4_authentication(&self) -> terrars::ListRef<NetworkServicesEdgeCacheOrigin_AwsV4AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_v4_authentication", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `flex_shielding` after provisioning.\n"]
    pub fn flex_shielding(&self) -> terrars::ListRef<NetworkServicesEdgeCacheOrigin_FlexShieldingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.flex_shielding", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `origin_override_action` after provisioning.\n"]
    pub fn origin_override_action(&self) -> terrars::ListRef<NetworkServicesEdgeCacheOrigin_OriginOverrideActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.origin_override_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `origin_redirect` after provisioning.\n"]
    pub fn origin_redirect(&self) -> terrars::ListRef<NetworkServicesEdgeCacheOrigin_OriginRedirectRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.origin_redirect", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::ListRef<NetworkServicesEdgeCacheOrigin_TimeoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkServicesEdgeCacheOrigin_TimeoutsRef {
        NetworkServicesEdgeCacheOrigin_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct NetworkServicesEdgeCacheOriginDynamic {
    aws_v4_authentication: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkServicesEdgeCacheOrigin_AwsV4Authentication>>,
    flex_shielding: std::option::Option<terrars::DynamicBlock<NetworkServicesEdgeCacheOrigin_FlexShielding>>,
    origin_override_action: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkServicesEdgeCacheOrigin_OriginOverrideAction>>,
    origin_redirect: std::option::Option<terrars::DynamicBlock<NetworkServicesEdgeCacheOrigin_OriginRedirect>>,
    timeout: std::option::Option<terrars::DynamicBlock<NetworkServicesEdgeCacheOrigin_Timeout>>,
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheOrigin_AwsV4Authentication {
    access_key_id: terrars::PrimField<std::string::String>,
    origin_region: terrars::PrimField<std::string::String>,
    secret_access_key_version: terrars::PrimField<std::string::String>,
}

impl NetworkServicesEdgeCacheOrigin_AwsV4Authentication { }

impl terrars::ToListMappable for NetworkServicesEdgeCacheOrigin_AwsV4Authentication {
    type O = terrars::BlockAssignable<NetworkServicesEdgeCacheOrigin_AwsV4Authentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheOrigin_AwsV4Authentication {
    #[doc = "The access key ID your origin uses to identify the key."]
    pub access_key_id: terrars::PrimField<std::string::String>,
    #[doc = "The name of the AWS region that your origin is in."]
    pub origin_region: terrars::PrimField<std::string::String>,
    #[doc =
        "The Secret Manager secret version of the secret access key used by your origin.\n\nThis is the resource name of the secret version in the format 'projects/*/secrets/*/versions/*' where the '*' values are replaced by the project, secret, and version you require."]
    pub secret_access_key_version: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesEdgeCacheOrigin_AwsV4Authentication {
    pub fn build(self) -> NetworkServicesEdgeCacheOrigin_AwsV4Authentication {
        NetworkServicesEdgeCacheOrigin_AwsV4Authentication {
            access_key_id: self.access_key_id,
            origin_region: self.origin_region,
            secret_access_key_version: self.secret_access_key_version,
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesEdgeCacheOrigin_AwsV4AuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheOrigin_AwsV4AuthenticationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheOrigin_AwsV4AuthenticationRef {
        NetworkServicesEdgeCacheOrigin_AwsV4AuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheOrigin_AwsV4AuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_key_id` after provisioning.\nThe access key ID your origin uses to identify the key."]
    pub fn access_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_key_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `origin_region` after provisioning.\nThe name of the AWS region that your origin is in."]
    pub fn origin_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin_region", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_access_key_version` after provisioning.\nThe Secret Manager secret version of the secret access key used by your origin.\n\nThis is the resource name of the secret version in the format 'projects/*/secrets/*/versions/*' where the '*' values are replaced by the project, secret, and version you require."]
    pub fn secret_access_key_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_access_key_version", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheOrigin_FlexShielding {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flex_shielding_regions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl NetworkServicesEdgeCacheOrigin_FlexShielding {
    #[doc =
        "Set the field `flex_shielding_regions`.\nWhenever possible, content will be fetched from origin and cached in or\nnear the specified origin. Best effort.\n\nYou must specify exactly one FlexShieldingRegion. Possible values include\n'AFRICA_SOUTH1', 'ME_CENTRAL1', 'EUROPE_WEST3', and 'US_EAST5'. See\n[FlexShieldingRegion](https://cloud.google.com/media-cdn/docs/reference/rest/v1/projects.locations.edgeCacheOrigins#FlexShieldingRegion)\nfor the full list of supported regions."]
    pub fn set_flex_shielding_regions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.flex_shielding_regions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkServicesEdgeCacheOrigin_FlexShielding {
    type O = terrars::BlockAssignable<NetworkServicesEdgeCacheOrigin_FlexShielding>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheOrigin_FlexShielding {}

impl BuildNetworkServicesEdgeCacheOrigin_FlexShielding {
    pub fn build(self) -> NetworkServicesEdgeCacheOrigin_FlexShielding {
        NetworkServicesEdgeCacheOrigin_FlexShielding { flex_shielding_regions: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesEdgeCacheOrigin_FlexShieldingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheOrigin_FlexShieldingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheOrigin_FlexShieldingRef {
        NetworkServicesEdgeCacheOrigin_FlexShieldingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheOrigin_FlexShieldingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `flex_shielding_regions` after provisioning.\nWhenever possible, content will be fetched from origin and cached in or\nnear the specified origin. Best effort.\n\nYou must specify exactly one FlexShieldingRegion. Possible values include\n'AFRICA_SOUTH1', 'ME_CENTRAL1', 'EUROPE_WEST3', and 'US_EAST5'. See\n[FlexShieldingRegion](https://cloud.google.com/media-cdn/docs/reference/rest/v1/projects.locations.edgeCacheOrigins#FlexShieldingRegion)\nfor the full list of supported regions."]
    pub fn flex_shielding_regions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.flex_shielding_regions", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheOrigin_OriginOverrideAction_HeaderAction_RequestHeadersToAdd {
    header_name: terrars::PrimField<std::string::String>,
    header_value: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replace: std::option::Option<terrars::PrimField<bool>>,
}

impl NetworkServicesEdgeCacheOrigin_OriginOverrideAction_HeaderAction_RequestHeadersToAdd {
    #[doc =
        "Set the field `replace`.\nWhether to replace all existing headers with the same name.\n\nBy default, added header values are appended\nto the response or request headers with the\nsame field names. The added values are\nseparated by commas.\n\nTo overwrite existing values, set 'replace' to 'true'."]
    pub fn set_replace(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.replace = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkServicesEdgeCacheOrigin_OriginOverrideAction_HeaderAction_RequestHeadersToAdd {
    type O =
        terrars
        ::BlockAssignable<NetworkServicesEdgeCacheOrigin_OriginOverrideAction_HeaderAction_RequestHeadersToAdd>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheOrigin_OriginOverrideAction_HeaderAction_RequestHeadersToAdd {
    #[doc = "The name of the header to add."]
    pub header_name: terrars::PrimField<std::string::String>,
    #[doc = "The value of the header to add."]
    pub header_value: terrars::PrimField<std::string::String>,
}

impl BuildNetworkServicesEdgeCacheOrigin_OriginOverrideAction_HeaderAction_RequestHeadersToAdd {
    pub fn build(self) -> NetworkServicesEdgeCacheOrigin_OriginOverrideAction_HeaderAction_RequestHeadersToAdd {
        NetworkServicesEdgeCacheOrigin_OriginOverrideAction_HeaderAction_RequestHeadersToAdd {
            header_name: self.header_name,
            header_value: self.header_value,
            replace: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesEdgeCacheOrigin_OriginOverrideAction_HeaderAction_RequestHeadersToAddRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheOrigin_OriginOverrideAction_HeaderAction_RequestHeadersToAddRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheOrigin_OriginOverrideAction_HeaderAction_RequestHeadersToAddRef {
        NetworkServicesEdgeCacheOrigin_OriginOverrideAction_HeaderAction_RequestHeadersToAddRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheOrigin_OriginOverrideAction_HeaderAction_RequestHeadersToAddRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header_name` after provisioning.\nThe name of the header to add."]
    pub fn header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }

    #[doc = "Get a reference to the value of field `header_value` after provisioning.\nThe value of the header to add."]
    pub fn header_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }

    #[doc =
        "Get a reference to the value of field `replace` after provisioning.\nWhether to replace all existing headers with the same name.\n\nBy default, added header values are appended\nto the response or request headers with the\nsame field names. The added values are\nseparated by commas.\n\nTo overwrite existing values, set 'replace' to 'true'."]
    pub fn replace(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replace", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkServicesEdgeCacheOrigin_OriginOverrideAction_HeaderActionDynamic {
    request_headers_to_add: std
    ::option
    ::Option<
        terrars::DynamicBlock<NetworkServicesEdgeCacheOrigin_OriginOverrideAction_HeaderAction_RequestHeadersToAdd>,
    >,
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheOrigin_OriginOverrideAction_HeaderAction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_headers_to_add: std
    ::option
    ::Option<std::vec::Vec<NetworkServicesEdgeCacheOrigin_OriginOverrideAction_HeaderAction_RequestHeadersToAdd>>,
    dynamic: NetworkServicesEdgeCacheOrigin_OriginOverrideAction_HeaderActionDynamic,
}

impl NetworkServicesEdgeCacheOrigin_OriginOverrideAction_HeaderAction {
    #[doc = "Set the field `request_headers_to_add`.\n"]
    pub fn set_request_headers_to_add(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            NetworkServicesEdgeCacheOrigin_OriginOverrideAction_HeaderAction_RequestHeadersToAdd,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.request_headers_to_add = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.request_headers_to_add = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkServicesEdgeCacheOrigin_OriginOverrideAction_HeaderAction {
    type O = terrars::BlockAssignable<NetworkServicesEdgeCacheOrigin_OriginOverrideAction_HeaderAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheOrigin_OriginOverrideAction_HeaderAction {}

impl BuildNetworkServicesEdgeCacheOrigin_OriginOverrideAction_HeaderAction {
    pub fn build(self) -> NetworkServicesEdgeCacheOrigin_OriginOverrideAction_HeaderAction {
        NetworkServicesEdgeCacheOrigin_OriginOverrideAction_HeaderAction {
            request_headers_to_add: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesEdgeCacheOrigin_OriginOverrideAction_HeaderActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheOrigin_OriginOverrideAction_HeaderActionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheOrigin_OriginOverrideAction_HeaderActionRef {
        NetworkServicesEdgeCacheOrigin_OriginOverrideAction_HeaderActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheOrigin_OriginOverrideAction_HeaderActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `request_headers_to_add` after provisioning.\n"]
    pub fn request_headers_to_add(
        &self,
    ) -> terrars::ListRef<NetworkServicesEdgeCacheOrigin_OriginOverrideAction_HeaderAction_RequestHeadersToAddRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_headers_to_add", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheOrigin_OriginOverrideAction_UrlRewrite {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_rewrite: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkServicesEdgeCacheOrigin_OriginOverrideAction_UrlRewrite {
    #[doc =
        "Set the field `host_rewrite`.\nPrior to forwarding the request to the selected\norigin, the request's host header is replaced with\ncontents of the hostRewrite.\n\nThis value must be between 1 and 255 characters."]
    pub fn set_host_rewrite(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host_rewrite = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkServicesEdgeCacheOrigin_OriginOverrideAction_UrlRewrite {
    type O = terrars::BlockAssignable<NetworkServicesEdgeCacheOrigin_OriginOverrideAction_UrlRewrite>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheOrigin_OriginOverrideAction_UrlRewrite {}

impl BuildNetworkServicesEdgeCacheOrigin_OriginOverrideAction_UrlRewrite {
    pub fn build(self) -> NetworkServicesEdgeCacheOrigin_OriginOverrideAction_UrlRewrite {
        NetworkServicesEdgeCacheOrigin_OriginOverrideAction_UrlRewrite {
            host_rewrite: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesEdgeCacheOrigin_OriginOverrideAction_UrlRewriteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheOrigin_OriginOverrideAction_UrlRewriteRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheOrigin_OriginOverrideAction_UrlRewriteRef {
        NetworkServicesEdgeCacheOrigin_OriginOverrideAction_UrlRewriteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheOrigin_OriginOverrideAction_UrlRewriteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `host_rewrite` after provisioning.\nPrior to forwarding the request to the selected\norigin, the request's host header is replaced with\ncontents of the hostRewrite.\n\nThis value must be between 1 and 255 characters."]
    pub fn host_rewrite(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_rewrite", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkServicesEdgeCacheOrigin_OriginOverrideActionDynamic {
    header_action: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkServicesEdgeCacheOrigin_OriginOverrideAction_HeaderAction>>,
    url_rewrite: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkServicesEdgeCacheOrigin_OriginOverrideAction_UrlRewrite>>,
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheOrigin_OriginOverrideAction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_action: std::option::Option<std::vec::Vec<NetworkServicesEdgeCacheOrigin_OriginOverrideAction_HeaderAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url_rewrite: std::option::Option<std::vec::Vec<NetworkServicesEdgeCacheOrigin_OriginOverrideAction_UrlRewrite>>,
    dynamic: NetworkServicesEdgeCacheOrigin_OriginOverrideActionDynamic,
}

impl NetworkServicesEdgeCacheOrigin_OriginOverrideAction {
    #[doc = "Set the field `header_action`.\n"]
    pub fn set_header_action(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<NetworkServicesEdgeCacheOrigin_OriginOverrideAction_HeaderAction>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.header_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.header_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `url_rewrite`.\n"]
    pub fn set_url_rewrite(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<NetworkServicesEdgeCacheOrigin_OriginOverrideAction_UrlRewrite>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.url_rewrite = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.url_rewrite = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for NetworkServicesEdgeCacheOrigin_OriginOverrideAction {
    type O = terrars::BlockAssignable<NetworkServicesEdgeCacheOrigin_OriginOverrideAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheOrigin_OriginOverrideAction {}

impl BuildNetworkServicesEdgeCacheOrigin_OriginOverrideAction {
    pub fn build(self) -> NetworkServicesEdgeCacheOrigin_OriginOverrideAction {
        NetworkServicesEdgeCacheOrigin_OriginOverrideAction {
            header_action: core::default::Default::default(),
            url_rewrite: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesEdgeCacheOrigin_OriginOverrideActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheOrigin_OriginOverrideActionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheOrigin_OriginOverrideActionRef {
        NetworkServicesEdgeCacheOrigin_OriginOverrideActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheOrigin_OriginOverrideActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header_action` after provisioning.\n"]
    pub fn header_action(&self) -> terrars::ListRef<NetworkServicesEdgeCacheOrigin_OriginOverrideAction_HeaderActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header_action", self.base))
    }

    #[doc = "Get a reference to the value of field `url_rewrite` after provisioning.\n"]
    pub fn url_rewrite(&self) -> terrars::ListRef<NetworkServicesEdgeCacheOrigin_OriginOverrideAction_UrlRewriteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.url_rewrite", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheOrigin_OriginRedirect {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect_conditions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl NetworkServicesEdgeCacheOrigin_OriginRedirect {
    #[doc =
        "Set the field `redirect_conditions`.\nThe set of redirect response codes that the CDN\nfollows. Values of\n[RedirectConditions](https://cloud.google.com/media-cdn/docs/reference/rest/v1/projects.locations.edgeCacheOrigins#redirectconditions)\nare accepted."]
    pub fn set_redirect_conditions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.redirect_conditions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkServicesEdgeCacheOrigin_OriginRedirect {
    type O = terrars::BlockAssignable<NetworkServicesEdgeCacheOrigin_OriginRedirect>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheOrigin_OriginRedirect {}

impl BuildNetworkServicesEdgeCacheOrigin_OriginRedirect {
    pub fn build(self) -> NetworkServicesEdgeCacheOrigin_OriginRedirect {
        NetworkServicesEdgeCacheOrigin_OriginRedirect { redirect_conditions: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesEdgeCacheOrigin_OriginRedirectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheOrigin_OriginRedirectRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkServicesEdgeCacheOrigin_OriginRedirectRef {
        NetworkServicesEdgeCacheOrigin_OriginRedirectRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheOrigin_OriginRedirectRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `redirect_conditions` after provisioning.\nThe set of redirect response codes that the CDN\nfollows. Values of\n[RedirectConditions](https://cloud.google.com/media-cdn/docs/reference/rest/v1/projects.locations.edgeCacheOrigins#redirectconditions)\nare accepted."]
    pub fn redirect_conditions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redirect_conditions", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheOrigin_Timeout {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connect_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_attempts_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkServicesEdgeCacheOrigin_Timeout {
    #[doc =
        "Set the field `connect_timeout`.\nThe maximum duration to wait for a single origin connection to be established, including DNS lookup, TLS handshake and TCP/QUIC connection establishment.\n\nDefaults to 5 seconds. The timeout must be a value between 1s and 15s.\n\nThe connectTimeout capped by the deadline set by the request's maxAttemptsTimeout.  The last connection attempt may have a smaller connectTimeout in order to adhere to the overall maxAttemptsTimeout."]
    pub fn set_connect_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connect_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_attempts_timeout`.\nThe maximum time across all connection attempts to the origin, including failover origins, before returning an error to the client. A HTTP 504 will be returned if the timeout is reached before a response is returned.\n\nDefaults to 15 seconds. The timeout must be a value between 1s and 30s.\n\nIf a failoverOrigin is specified, the maxAttemptsTimeout of the first configured origin sets the deadline for all connection attempts across all failoverOrigins."]
    pub fn set_max_attempts_timeout(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.max_attempts_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `read_timeout`.\nThe maximum duration to wait between reads of a single HTTP connection/stream.\n\nDefaults to 15 seconds.  The timeout must be a value between 1s and 30s.\n\nThe readTimeout is capped by the responseTimeout.  All reads of the HTTP connection/stream must be completed by the deadline set by the responseTimeout.\n\nIf the response headers have already been written to the connection, the response will be truncated and logged."]
    pub fn set_read_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.read_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `response_timeout`.\nThe maximum duration to wait for the last byte of a response to arrive when reading from the HTTP connection/stream.\n\nDefaults to 30 seconds. The timeout must be a value between 1s and 120s.\n\nThe responseTimeout starts after the connection has been established.\n\nThis also applies to HTTP Chunked Transfer Encoding responses, and/or when an open-ended Range request is made to the origin. Origins that take longer to write additional bytes to the response than the configured responseTimeout will result in an error being returned to the client.\n\nIf the response headers have already been written to the connection, the response will be truncated and logged."]
    pub fn set_response_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.response_timeout = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkServicesEdgeCacheOrigin_Timeout {
    type O = terrars::BlockAssignable<NetworkServicesEdgeCacheOrigin_Timeout>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheOrigin_Timeout {}

impl BuildNetworkServicesEdgeCacheOrigin_Timeout {
    pub fn build(self) -> NetworkServicesEdgeCacheOrigin_Timeout {
        NetworkServicesEdgeCacheOrigin_Timeout {
            connect_timeout: core::default::Default::default(),
            max_attempts_timeout: core::default::Default::default(),
            read_timeout: core::default::Default::default(),
            response_timeout: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesEdgeCacheOrigin_TimeoutRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheOrigin_TimeoutRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkServicesEdgeCacheOrigin_TimeoutRef {
        NetworkServicesEdgeCacheOrigin_TimeoutRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheOrigin_TimeoutRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `connect_timeout` after provisioning.\nThe maximum duration to wait for a single origin connection to be established, including DNS lookup, TLS handshake and TCP/QUIC connection establishment.\n\nDefaults to 5 seconds. The timeout must be a value between 1s and 15s.\n\nThe connectTimeout capped by the deadline set by the request's maxAttemptsTimeout.  The last connection attempt may have a smaller connectTimeout in order to adhere to the overall maxAttemptsTimeout."]
    pub fn connect_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connect_timeout", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_attempts_timeout` after provisioning.\nThe maximum time across all connection attempts to the origin, including failover origins, before returning an error to the client. A HTTP 504 will be returned if the timeout is reached before a response is returned.\n\nDefaults to 15 seconds. The timeout must be a value between 1s and 30s.\n\nIf a failoverOrigin is specified, the maxAttemptsTimeout of the first configured origin sets the deadline for all connection attempts across all failoverOrigins."]
    pub fn max_attempts_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_attempts_timeout", self.base))
    }

    #[doc =
        "Get a reference to the value of field `read_timeout` after provisioning.\nThe maximum duration to wait between reads of a single HTTP connection/stream.\n\nDefaults to 15 seconds.  The timeout must be a value between 1s and 30s.\n\nThe readTimeout is capped by the responseTimeout.  All reads of the HTTP connection/stream must be completed by the deadline set by the responseTimeout.\n\nIf the response headers have already been written to the connection, the response will be truncated and logged."]
    pub fn read_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_timeout", self.base))
    }

    #[doc =
        "Get a reference to the value of field `response_timeout` after provisioning.\nThe maximum duration to wait for the last byte of a response to arrive when reading from the HTTP connection/stream.\n\nDefaults to 30 seconds. The timeout must be a value between 1s and 120s.\n\nThe responseTimeout starts after the connection has been established.\n\nThis also applies to HTTP Chunked Transfer Encoding responses, and/or when an open-ended Range request is made to the origin. Origins that take longer to write additional bytes to the response than the configured responseTimeout will result in an error being returned to the client.\n\nIf the response headers have already been written to the connection, the response will be truncated and logged."]
    pub fn response_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response_timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkServicesEdgeCacheOrigin_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkServicesEdgeCacheOrigin_Timeouts {
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

impl terrars::ToListMappable for NetworkServicesEdgeCacheOrigin_Timeouts {
    type O = terrars::BlockAssignable<NetworkServicesEdgeCacheOrigin_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkServicesEdgeCacheOrigin_Timeouts {}

impl BuildNetworkServicesEdgeCacheOrigin_Timeouts {
    pub fn build(self) -> NetworkServicesEdgeCacheOrigin_Timeouts {
        NetworkServicesEdgeCacheOrigin_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkServicesEdgeCacheOrigin_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkServicesEdgeCacheOrigin_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkServicesEdgeCacheOrigin_TimeoutsRef {
        NetworkServicesEdgeCacheOrigin_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkServicesEdgeCacheOrigin_TimeoutsRef {
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
